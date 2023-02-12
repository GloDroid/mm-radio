/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use crate::{
    mm_zbus::{
        consts::*, mm_call_proxy::CallProxy, mm_modem_proxy::ModemProxy, mm_ussd_proxy::UssdProxy,
        mm_voice_proxy::VoiceProxy,
    },
    utils::iradio::{
        declare_async_iradio, def, entry_check, ind, not_implemented, okay, shared, sharedmut,
    },
};
use android_hardware_radio::aidl::android::hardware::radio::RadioIndicationType::*;
use android_hardware_radio_voice::aidl::android::hardware::radio::voice::{
    Call as VoiceCall, CallForwardInfo::*, Dial::*, EmergencyCallRouting::*, IRadioVoice::*,
    IRadioVoiceIndication::*, IRadioVoiceResponse::*, LastCallFailCause::*,
    LastCallFailCauseInfo::*, TtyMode::*, UssdModeType::*,
};

use async_std::{
    channel::Sender,
    sync::RwLock,
    task::{block_on, spawn},
};
use binder::{BinderFeatures, Strong};
use futures::{select, FutureExt, StreamExt};
use log::info;
use std::{collections::HashMap, sync::Arc};
use zbus::{export::async_trait::async_trait, Connection};

struct Call {
    proxy: CallProxy<'static>,
    stop_monitoring: Sender<()>,
}

impl Drop for Call {
    fn drop(&mut self) {
        block_on(self.stop_monitoring.send(())).unwrap();
    }
}

#[derive(Default)]
pub struct RadioVoiceShared {
    modem_bound: bool,
    response: Option<binder::Strong<dyn IRadioVoiceResponse>>,
    indication: Option<binder::Strong<dyn IRadioVoiceIndication>>,

    modem_proxy: Option<ModemProxy<'static>>,
    ussd_proxy: Option<UssdProxy<'static>>,
    voice_proxy: Option<VoiceProxy<'static>>,

    calls: HashMap<String, Call>,
}

#[derive(Default)]
pub struct RadioVoice {
    pub shared: Arc<RwLock<RadioVoiceShared>>,
}

async fn build_call_proxy(conn: Connection, path: String) -> CallProxy<'static> {
    CallProxy::builder(&conn).path(path).unwrap().build().await.unwrap()
}

impl Call {
    async fn new(
        shared_in: &Arc<RwLock<RadioVoiceShared>>,
        path: &str,
        conn: Connection,
    ) -> Option<Self> {
        let (stop_monitoring, stop_monitoring_rx) = async_std::channel::bounded(1);

        let call = Call { proxy: build_call_proxy(conn, path.to_string()).await, stop_monitoring };

        let call_state = call.proxy.state().await.ok()?;
        if call_state == mm_call_state::TERMINATED {
            return None;
        }

        let mm_call_c = call.proxy.clone();
        let shared = shared_in.clone();
        spawn(async move {
            let mut call_state = mm_call_c.receive_state_changed().await;
            let mut stop = stop_monitoring_rx;
            {
                let shared = shared.read().await;
                shared.notify_call_state_changed();
            }
            info!("Call monitoring started");
            loop {
                select! {
                    state = call_state.next().fuse() => {
                        let new_state = state.unwrap().get().await.unwrap();
                        info!("Call state: {}", new_state);
                        let shared = shared.read().await;
                        shared.notify_call_state_changed();
                    },
                    _ = stop.next().fuse() => break,
                    complete => break,
                };
            }

            info!("Call monitoring stopped");
        });

        Some(call)
    }

    async fn call_state_mm_to_aidl(&self) -> Option<i32> {
        let mm_state = self.proxy.state().await.ok()?;

        use {mm_call_state::*, VoiceCall::*};
        match mm_state {
            ACTIVE => Some(STATE_ACTIVE),
            DIALING => Some(STATE_DIALING),
            RINGING_IN => Some(STATE_INCOMING),
            RINGING_OUT => Some(STATE_ALERTING),
            HELD => Some(STATE_HOLDING),
            WAITING => Some(STATE_WAITING),
            _ => None,
        }
    }

    fn get_index(&self) -> i32 {
        let path = self.proxy.path().as_str();
        path.split('/').last().unwrap().parse().unwrap()
    }
}

impl RadioVoiceShared {
    pub fn bind(shared_in: &Arc<RwLock<RadioVoiceShared>>, modem_proxy: &ModemProxy<'static>) {
        /* Setup shared structure */
        {
            let mut shared = block_on(shared_in.write());
            shared.modem_proxy = Some(modem_proxy.clone());
            let conn = modem_proxy.connection();
            let path = modem_proxy.path().to_string();
            shared.ussd_proxy = Some(
                block_on(UssdProxy::builder(conn).path(path.clone()).unwrap().build()).unwrap(),
            );
            shared.voice_proxy =
                Some(block_on(VoiceProxy::builder(conn).path(path).unwrap().build()).unwrap());
            shared.modem_bound = true;
            shared.calls = HashMap::new();
        }
        /* Register event listeners */
        {
            let shared = shared_in.clone();
            spawn(async move {
                let mm_voice_c = shared.read().await.voice_proxy.as_ref().unwrap().clone();
                let mut call_added = mm_voice_c.receive_call_added().await.unwrap();
                let mut call_deleted = mm_voice_c.receive_call_deleted().await.unwrap();
                loop {
                    select! {
                        path = call_added.next().fuse() => Self::add_call(&shared, path.unwrap().args().unwrap().call_path().to_string()).await,
                        path = call_deleted.next().fuse() => Self::delete_call(&shared, path.unwrap().args().unwrap().call_path().to_string()).await,
                    }
                }
            });
        }
        /* Query calls from the Modem Manager */
        {
            let calls = {
                let shared = block_on(shared_in.read());
                block_on(shared.voice_proxy.as_ref().unwrap().list_calls()).unwrap()
            };
            let shared = shared_in.clone();
            spawn(async move {
                for call in calls {
                    Self::add_call(&shared, call.to_string()).await;
                }
            });
        }
    }

    pub fn unbind(shared_in: &Arc<RwLock<RadioVoiceShared>>) {
        let mut shared = block_on(shared_in.write());
        shared.modem_bound = false;
        shared.modem_proxy = None;
        shared.ussd_proxy = None;
        shared.voice_proxy = None;
    }

    async fn add_call(shared_in: &Arc<RwLock<RadioVoiceShared>>, path: String) {
        let mut shared = shared_in.write().await;
        if shared.calls.contains_key(&path) {
            info!("Call already exists");
            return;
        }
        let conn = shared.modem_proxy.as_ref().unwrap().connection().clone();
        if let Some(c) = Call::new(shared_in, &path, conn).await {
            info!("New call: {}", path);

            shared.calls.insert(path.clone(), c);
            info!("Call added: {:?}", path);
        };
    }

    async fn delete_call(shared: &Arc<RwLock<RadioVoiceShared>>, path: String) {
        let mut shared = shared.write().await;
        info!("Call deleted: {:?}", path);
        if !shared.calls.contains_key(&path) {
            return;
        }
        shared.calls.remove(&path);
    }

    fn notify_call_state_changed(&self) -> Option<()> {
        let indication = self.indication.as_ref()?;
        indication.callStateChanged(RadioIndicationType::UNSOLICITED).ok()
    }

    async fn find_call(&self, mm_state: i32) -> Option<&Call> {
        for c in self.calls.values() {
            let state = c.call_state_mm_to_aidl().await;
            if state == Some(mm_state) {
                return Some(c);
            }
        }
        None
    }
}

#[async_trait]
impl IRadioVoiceAsyncServer for RadioVoice {
    async fn acceptCall(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, acceptCallResponse);
        let shared = shared!(&self);
        let call = shared.find_call(mm_call_state::RINGING_IN).await;
        if let Some(call) = call {
            call.proxy.accept().await.unwrap();
        }
        okay!(&self, serial, acceptCallResponse)
    }
    async fn cancelPendingUssd(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, cancelPendingUssdResponse);
        let shared = shared!(&self);
        let ussd_proxy = shared.ussd_proxy.as_ref().unwrap();
        ussd_proxy.cancel().await.unwrap_or_default();
        okay!(&self, serial, cancelPendingUssdResponse)
    }
    async fn conference(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, conferenceResponse);
        let shared = shared!(&self);
        let call = shared.find_call(mm_call_state::HELD).await;
        if let Some(call) = call {
            call.proxy.join_multiparty().await.unwrap();
        }
        okay!(&self, serial, conferenceResponse)
    }
    async fn dial(&self, serial: i32, dial_info: &Dial) -> binder::Result<()> {
        entry_check!(&self, serial, dialResponse);
        let shared = shared!(&self);
        let voice_proxy = shared.voice_proxy.as_ref().unwrap();
        let mut call_props = HashMap::new();
        call_props.insert("number", dial_info.address.clone().into());
        let path = voice_proxy.create_call(call_props).await.unwrap();
        drop(shared);

        RadioVoiceShared::add_call(&self.shared, path.to_string()).await;

        let shared = shared!(&self);
        let call = shared.calls.get(&path.to_string()).unwrap();
        call.proxy.start().await.unwrap();

        info!("Dialing: {:?}", path);
        okay!(&self, serial, dialResponse)
    }
    async fn emergencyDial(
        &self,
        serial: i32,
        _dial_info: &Dial,
        _categories: i32,
        _urns: &[String],
        _routing: EmergencyCallRouting,
        _has_known_user_intent_emergency: bool,
        _is_testing: bool,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, emergencyDialResponse)
    }
    async fn exitEmergencyCallbackMode(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, exitEmergencyCallbackModeResponse)
    }
    async fn explicitCallTransfer(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, explicitCallTransferResponse);
        let shared = shared!(&self);
        shared.voice_proxy.as_ref().unwrap().transfer().await.unwrap();
        okay!(&self, serial, explicitCallTransferResponse)
    }
    async fn getCallForwardStatus(
        &self,
        serial: i32,
        _call_info: &CallForwardInfo,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, getCallForwardStatusResponse, def())
    }
    async fn getCallWaiting(&self, serial: i32, _service_class: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getCallWaitingResponse, false, 0);
        let shared = shared!(&self);
        let waiting = shared.voice_proxy.as_ref().unwrap().call_waiting_query().await.unwrap();
        okay!(&self, serial, getCallWaitingResponse, waiting, 0)
    }
    async fn getClip(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getClipResponse, def())
    }
    async fn getClir(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getClirResponse, def(), def())
    }
    async fn getCurrentCalls(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getCurrentCallsResponse, def());
        let mut calls = Vec::new();

        for call in shared!(&self).calls.values() {
            let proxy = &call.proxy;

            let call_state = call.call_state_mm_to_aidl().await;
            if call_state.is_none() {
                continue;
            }

            let vcall = VoiceCall::Call {
                state: call_state.unwrap(),
                index: call.get_index() + 1,
                isMpty: proxy.multiparty().await.unwrap(),
                isMT: proxy.direction().await.unwrap() == mm_call_direction::INCOMING,
                isVoice: true,
                number: proxy.number().await.unwrap(),
                numberPresentation: VoiceCall::PRESENTATION_ALLOWED,
                namePresentation: VoiceCall::PRESENTATION_ALLOWED,
                ..Default::default()
            };
            calls.push(vcall);
        }

        okay!(&self, serial, getCurrentCallsResponse, &calls)
    }
    async fn getLastCallFailCause(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getLastCallFailCauseResponse, &def());
        let lcfc = LastCallFailCauseInfo {
            causeCode: LastCallFailCause::OUT_OF_SERVICE,
            vendorCause: "Out of service".to_string(),
        };

        okay!(&self, serial, getLastCallFailCauseResponse, &lcfc)
    }
    async fn getMute(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getMuteResponse, def())
    }
    async fn getPreferredVoicePrivacy(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getPreferredVoicePrivacyResponse, def())
    }
    async fn getTtyMode(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getTtyModeResponse, def())
    }
    async fn handleStkCallSetupRequestFromSim(
        &self,
        serial: i32,
        _accept: bool,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, handleStkCallSetupRequestFromSimResponse)
    }
    async fn hangup(&self, serial: i32, gsm_index: i32) -> binder::Result<()> {
        entry_check!(&self, serial, hangupConnectionResponse);
        let path = format!("/org/freedesktop/ModemManager1/Call/{}", gsm_index - 1);
        info!("hangup path: {}", path);
        let shared = shared!(&self);
        let call = shared.calls.get(&path);
        if let Some(call) = call {
            call.proxy.hangup().await.unwrap();
        }
        okay!(&self, serial, hangupConnectionResponse)
    }
    async fn hangupForegroundResumeBackground(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, hangupForegroundResumeBackgroundResponse);
        let shared = shared!(&self);
        shared.voice_proxy.as_ref().unwrap().hangup_and_accept().await.unwrap();
        okay!(&self, serial, hangupForegroundResumeBackgroundResponse)
    }
    async fn hangupWaitingOrBackground(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, hangupWaitingOrBackgroundResponse);
        let shared = shared!(&self);
        shared.voice_proxy.as_ref().unwrap().hangup_all().await.unwrap();
        okay!(&self, serial, hangupWaitingOrBackgroundResponse)
    }
    async fn isVoNrEnabled(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, isVoNrEnabledResponse, def())
    }
    async fn rejectCall(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, rejectCallResponse);
        /* "Send UDUB (user determined user busy) to ringing or waiting call answer)" */
        let shared = shared!(&self);
        let call = shared.find_call(VoiceCall::STATE_INCOMING).await;
        if let Some(call) = call {
            call.proxy.hangup().await.unwrap();
        }
        let call = shared.find_call(VoiceCall::STATE_WAITING).await;
        if let Some(call) = call {
            call.proxy.hangup().await.ok();
        }

        okay!(&self, serial, rejectCallResponse)
    }
    async fn responseAcknowledgement(&self) -> binder::Result<()> {
        Ok(())
    }
    async fn sendBurstDtmf(
        &self,
        serial: i32,
        _dtmf: &str,
        _on: i32,
        _off: i32,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, sendBurstDtmfResponse)
    }
    async fn sendCdmaFeatureCode(&self, serial: i32, _feature_code: &str) -> binder::Result<()> {
        not_implemented!(&self, serial, sendCdmaFeatureCodeResponse)
    }
    async fn sendDtmf(&self, serial: i32, s: &str) -> binder::Result<()> {
        entry_check!(&self, serial, sendDtmfResponse);
        let shared = shared!(&self);
        let call = shared.find_call(VoiceCall::STATE_ACTIVE).await;
        if let Some(call) = call {
            call.proxy.send_dtmf(s).await.ok();
        }
        okay!(&self, serial, sendDtmfResponse)
    }
    async fn sendUssd(&self, serial: i32, ussd: &str) -> binder::Result<()> {
        entry_check!(&self, serial, sendUssdResponse);
        let shared = shared!(&self);
        let ussd_proxy = shared.ussd_proxy.as_ref().unwrap();
        let state = ussd_proxy.state().await.unwrap();
        let response = if state == 3
        /*UserResponce*/
        {
            ussd_proxy.respond(ussd).await.unwrap()
        } else {
            ussd_proxy.initiate(ussd).await.unwrap()
        };
        let state = ussd_proxy.state().await.unwrap();
        drop(shared);

        let mode_type = match state {
            mm_modem_3gpp_ussd_session_state::IDLE => UssdModeType::NOTIFY,
            mm_modem_3gpp_ussd_session_state::USER_RESPONSE => UssdModeType::REQUEST,
            _ => UssdModeType::LOCAL_CLIENT,
        };
        ind!(&self).onUssd(RadioIndicationType::UNSOLICITED, mode_type, &response)?;
        okay!(&self, serial, sendUssdResponse)
    }
    async fn separateConnection(&self, serial: i32, gsm_index: i32) -> binder::Result<()> {
        entry_check!(&self, serial, separateConnectionResponse);
        let shared = shared!(&self);
        let path = format!("/org/freedesktop/ModemManager1/Call/{}", gsm_index - 1);
        info!("separateConnection path: {}", path);
        let call = shared.calls.get(&path);
        if let Some(call) = call {
            call.proxy.leave_multiparty().await.unwrap();
        }
        okay!(&self, serial, separateConnectionResponse)
    }
    async fn setCallForward(
        &self,
        serial: i32,
        _call_info: &CallForwardInfo,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setCallForwardResponse)
    }
    async fn setCallWaiting(
        &self,
        serial: i32,
        enable: bool,
        _svc_class: i32,
    ) -> binder::Result<()> {
        entry_check!(&self, serial, setCallWaitingResponse);
        let shared = shared!(&self);
        let vp = shared.voice_proxy.as_ref().unwrap();
        vp.call_waiting_setup(enable).await.unwrap();
        okay!(&self, serial, setCallWaitingResponse)
    }
    async fn setClir(&self, serial: i32, _status: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, setClirResponse)
    }
    async fn setMute(&self, serial: i32, _enable: bool) -> binder::Result<()> {
        not_implemented!(&self, serial, setMuteResponse)
    }
    async fn setPreferredVoicePrivacy(&self, serial: i32, _enable: bool) -> binder::Result<()> {
        not_implemented!(&self, serial, setPreferredVoicePrivacyResponse)
    }
    async fn setTtyMode(&self, serial: i32, _mode: TtyMode) -> binder::Result<()> {
        not_implemented!(&self, serial, setTtyModeResponse)
    }
    async fn setVoNrEnabled(&self, serial: i32, _enable: bool) -> binder::Result<()> {
        not_implemented!(&self, serial, setVoNrEnabledResponse)
    }
    async fn startDtmf(&self, serial: i32, s: &str) -> binder::Result<()> {
        entry_check!(&self, serial, startDtmfResponse);
        let shared = shared!(&self);
        let call = shared.find_call(VoiceCall::STATE_ACTIVE).await;
        if let Some(call) = call {
            call.proxy.send_dtmf(s).await.unwrap();
        }
        okay!(&self, serial, startDtmfResponse)
    }
    async fn stopDtmf(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, stopDtmfResponse);
        okay!(&self, serial, stopDtmfResponse)
    }
    async fn switchWaitingOrHoldingAndActive(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, switchWaitingOrHoldingAndActiveResponse);
        let shared = shared!(&self);
        let vp = shared.voice_proxy.as_ref().unwrap();
        vp.hold_and_accept().await.unwrap();
        okay!(&self, serial, switchWaitingOrHoldingAndActiveResponse)
    }

    async fn setResponseFunctions(
        &self,
        radio_response: &binder::Strong<dyn IRadioVoiceResponse>,
        radio_indication: &binder::Strong<dyn IRadioVoiceIndication>,
    ) -> binder::Result<()> {
        info!("RadioVoice: setResponseFunctions");

        let mut shared = sharedmut!(&self);
        shared.response = Some(radio_response.clone());
        shared.indication = Some(radio_indication.clone());

        Ok(())
    }
}

declare_async_iradio!(RadioVoice, IRadioVoice, BnRadioVoice);
