/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use crate::{
    mm_zbus::{
        consts::mm_sms_state, mm_messaging_proxy::MessagingProxy, mm_modem_proxy::ModemProxy,
        mm_sms_proxy::SmsProxy,
    },
    utils::{
        iradio::{
            declare_async_iradio, def, entry_check, err, not_implemented, okay, shared, sharedmut,
        },
        sms_deliver_encode::sms_deliver_encode,
        sms_submit_decode::sms_submit_decode,
    },
};
use android_hardware_radio::aidl::android::hardware::radio::RadioIndicationType::RadioIndicationType;
use android_hardware_radio_messaging::aidl::android::hardware::radio::messaging::{
    CdmaBroadcastSmsConfigInfo::*, CdmaSmsAck::*, CdmaSmsMessage::*, CdmaSmsWriteArgs::*,
    GsmBroadcastSmsConfigInfo::*, GsmSmsMessage::*, IRadioMessaging::*,
    IRadioMessagingIndication::*, IRadioMessagingResponse::*, ImsSmsMessage::*,
    SmsAcknowledgeFailCause::*, SmsWriteArgs::*,
};

use async_std::{
    sync::RwLock,
    task::{block_on, spawn},
};
use binder::{BinderFeatures, Strong};
use futures::{select, FutureExt, StreamExt};
use log::info;
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use zbus::{export::async_trait::async_trait, zvariant::OwnedObjectPath};

#[derive(Default)]
pub struct RadioMessagingShared {
    modem_bound: bool,
    response: Option<binder::Strong<dyn IRadioMessagingResponse>>,
    indication: Option<binder::Strong<dyn IRadioMessagingIndication>>,

    modem_proxy: Option<ModemProxy<'static>>,
    messaging_proxy: Option<MessagingProxy<'static>>,

    sms_deliver_queue: VecDeque<OwnedObjectPath>,
}

#[derive(Default)]
pub struct RadioMessaging {
    pub shared: Arc<RwLock<RadioMessagingShared>>,
}

impl RadioMessagingShared {
    pub fn bind(shared_in: &Arc<RwLock<RadioMessagingShared>>, modem_proxy: &ModemProxy<'static>) {
        /* Setup shared structure */
        {
            let mut shared = block_on(shared_in.write());
            shared.modem_proxy = Some(modem_proxy.clone());
            let conn = modem_proxy.connection();
            let path = modem_proxy.path().to_string();
            shared.messaging_proxy =
                Some(block_on(MessagingProxy::builder(conn).path(path).unwrap().build()).unwrap());
            shared.modem_bound = true;
        }

        let mut received_sms = VecDeque::new();

        /* Query RECEIVED SMS from the Modem Manager */
        {
            let shared = block_on(shared_in.read());
            let messages = block_on(shared.messaging_proxy.as_ref().unwrap().messages()).unwrap();

            let conn = shared.modem_proxy.as_ref().unwrap().connection();
            for path in messages {
                let sms_proxy =
                    block_on(SmsProxy::builder(conn).path(path.clone()).unwrap().build()).unwrap();
                let state = block_on(sms_proxy.state());
                if state.is_ok() && state.unwrap() == mm_sms_state::RECEIVED {
                    received_sms.push_back(path);
                }
            }
        }

        /* Put them into shared structure */
        {
            let mut shared = block_on(shared_in.write());
            shared.sms_deliver_queue = received_sms;
        }

        /* Subscribe for new received SMS events */
        {
            let shared_in_c = shared_in.clone();
            spawn(async move {
                let mproxy = {
                    let shared = shared_in_c.read().await;
                    shared.messaging_proxy.as_ref().unwrap().clone()
                };
                let mut added_signal = mproxy.receive_added().await.unwrap();
                select! {
                    added = added_signal.next().fuse() => {
                        let added = added.unwrap();
                        let received = added.args().unwrap().received;
                        if received {
                            let path = added.args().unwrap().path;
                            info!("Received new SMS: {}", path);
                            {
                                let mut shared = shared_in_c.write().await;
                                shared.sms_deliver_queue.push_back(path.into());
                            }
                            RadioMessagingShared::forward_next_sms(&shared_in_c).await;
                        }
                    }
                }
            });
        }

        Self::notify_framework(shared_in);
    }

    pub fn unbind(shared_in: &Arc<RwLock<RadioMessagingShared>>) {
        let mut shared = block_on(shared_in.write());
        shared.modem_bound = false;
        shared.modem_proxy = None;
    }

    async fn forward_next_sms(shared_in: &Arc<RwLock<RadioMessagingShared>>) {
        let shared = shared_in.read().await;
        if !shared.modem_bound || shared.response.is_none() || shared.indication.is_none() {
            return;
        }

        let path = match shared.sms_deliver_queue.front() {
            Some(path) => path,
            None => return,
        };

        let conn = shared.modem_proxy.as_ref().unwrap().connection();
        let sms_proxy = SmsProxy::builder(conn).path(path).unwrap().build().await.unwrap();

        let number = sms_proxy.number().await.unwrap();
        let text = sms_proxy.text().await.unwrap();
        let timestamp = sms_proxy.timestamp().await.unwrap();

        info!("Received message from '{}' at '{}' with text '{}'", number, timestamp, text);

        let pdu = sms_deliver_encode(number.as_str(), text.as_str(), timestamp.as_str());

        info!("DELIVER PDU: {}", pdu);

        let mut pdu_bytes = Vec::new();
        let full_pdu = format!("00{pdu}"); // Add null SMSC field
        for i in (0..full_pdu.len()).step_by(2) {
            let byte = &full_pdu[i..i + 2];
            let byte = u8::from_str_radix(byte, 16).unwrap();
            pdu_bytes.push(byte);
        }

        let ind = shared.indication.as_ref().unwrap();
        ind.newSms(RadioIndicationType::UNSOLICITED, &pdu_bytes).unwrap();
    }

    fn notify_framework(shared_in: &Arc<RwLock<RadioMessagingShared>>) {
        block_on(Self::forward_next_sms(shared_in));
    }
}

#[async_trait]
impl IRadioMessagingAsyncServer for RadioMessaging {
    async fn acknowledgeIncomingGsmSmsWithPdu(
        &self,
        serial: i32,
        _success: bool,
        _ack_pdu: &str,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, acknowledgeIncomingGsmSmsWithPduResponse)
    }

    async fn acknowledgeLastIncomingGsmSms(
        &self,
        serial: i32,
        _success: bool,
        _cause: SmsAcknowledgeFailCause,
    ) -> binder::Result<()> {
        entry_check!(&self, serial, acknowledgeLastIncomingGsmSmsResponse);

        let sms_path = sharedmut!(&self).sms_deliver_queue.pop_front();

        if let Some(path) = sms_path {
            {
                let shared = shared!(&self);
                shared.messaging_proxy.as_ref().unwrap().delete(&path).await.unwrap();
            }
            RadioMessagingShared::forward_next_sms(&self.shared).await;
        }

        okay!(&self, serial, acknowledgeLastIncomingGsmSmsResponse)
    }

    async fn acknowledgeLastIncomingCdmaSms(
        &self,
        serial: i32,
        _sms_ack: &CdmaSmsAck,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, acknowledgeLastIncomingCdmaSmsResponse)
    }

    async fn deleteSmsOnSim(&self, serial: i32, _index: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, deleteSmsOnSimResponse)
    }

    async fn deleteSmsOnRuim(&self, serial: i32, _index: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, deleteSmsOnRuimResponse)
    }

    async fn getGsmBroadcastConfig(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getGsmBroadcastConfigResponse, def())
    }

    async fn getCdmaBroadcastConfig(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getCdmaBroadcastConfigResponse, def())
    }

    async fn getSmscAddress(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getSmscAddressResponse, def())
    }

    async fn responseAcknowledgement(&self) -> binder::Result<()> {
        Ok(())
    }

    async fn sendCdmaSms(&self, serial: i32, _sms: &CdmaSmsMessage) -> binder::Result<()> {
        not_implemented!(&self, serial, sendCdmaSmsResponse, &def())
    }

    async fn reportSmsMemoryStatus(&self, serial: i32, _available: bool) -> binder::Result<()> {
        not_implemented!(&self, serial, reportSmsMemoryStatusResponse)
    }

    async fn sendCdmaSmsExpectMore(
        &self,
        serial: i32,
        _sms: &CdmaSmsMessage,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, sendCdmaSmsExpectMoreResponse, &def())
    }

    async fn sendImsSms(&self, serial: i32, _message: &ImsSmsMessage) -> binder::Result<()> {
        not_implemented!(&self, serial, sendImsSmsResponse, &def())
    }

    async fn sendSms(&self, serial: i32, message: &GsmSmsMessage) -> binder::Result<()> {
        entry_check!(&self, serial, sendSmsResponse, &def());
        info!("Sending SMS: {:?}", message);

        let decoded = sms_submit_decode(message.pdu.as_str());
        if decoded.is_none() {
            error!("Failed to decode SMS");
            return err!(&self, serial, RadioError::INVALID_ARGUMENTS, sendSmsResponse, &def());
        }

        let (number, text) = decoded.unwrap();

        info!("Sending SMS to '{}' with text '{}'", number, text);

        let shared = shared!(&self);
        let messaging_proxy = shared.messaging_proxy.as_ref().unwrap();

        let mut sms_props = HashMap::new();
        sms_props.insert("number", number.into());
        sms_props.insert("text", text.into());

        let path = messaging_proxy.create(sms_props).await.unwrap();
        info!("SMS created at {}", path.to_string());
        let conn = shared.modem_proxy.as_ref().unwrap().connection();
        let sms_proxy = SmsProxy::builder(conn).path(path).unwrap().build().await.unwrap();
        let shared_c = self.shared.clone();
        spawn(async move {
            let status = sms_proxy.send().await;
            let shared = shared_c.read().await;
            if status.is_err() {
                error!("Failed to send SMS: {}, {}", serial, status.unwrap_err());
                shared
                    .response
                    .as_ref()
                    .unwrap()
                    .sendSmsResponse(&respond(serial, RadioError::SMS_SEND_FAIL_RETRY), &def())
                    .unwrap();
                return;
            }
            info!("SMS sent: {}", serial);
            shared
                .response
                .as_ref()
                .unwrap()
                .sendSmsResponse(&respond(serial, RadioError::SMS_SEND_FAIL_RETRY), &def())
                .unwrap();
        });

        Ok(())
    }

    async fn sendSmsExpectMore(&self, serial: i32, _message: &GsmSmsMessage) -> binder::Result<()> {
        not_implemented!(&self, serial, sendSmsExpectMoreResponse, &def())
    }

    async fn setCdmaBroadcastActivation(&self, serial: i32, _activate: bool) -> binder::Result<()> {
        not_implemented!(&self, serial, setCdmaBroadcastActivationResponse)
    }

    async fn setCdmaBroadcastConfig(
        &self,
        serial: i32,
        _config_info: &[CdmaBroadcastSmsConfigInfo],
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setCdmaBroadcastConfigResponse)
    }

    async fn setGsmBroadcastActivation(&self, serial: i32, _activate: bool) -> binder::Result<()> {
        not_implemented!(&self, serial, setGsmBroadcastActivationResponse)
    }

    async fn setGsmBroadcastConfig(
        &self,
        serial: i32,
        _config_info: &[GsmBroadcastSmsConfigInfo],
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setGsmBroadcastConfigResponse)
    }

    async fn setSmscAddress(&self, serial: i32, _smsc: &str) -> binder::Result<()> {
        not_implemented!(&self, serial, setSmscAddressResponse)
    }

    async fn writeSmsToRuim(
        &self,
        serial: i32,
        _cdma_sms: &CdmaSmsWriteArgs,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, writeSmsToRuimResponse, 0)
    }

    async fn writeSmsToSim(
        &self,
        serial: i32,
        _sms_write_args: &SmsWriteArgs,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, writeSmsToSimResponse, 0)
    }

    async fn setResponseFunctions(
        &self,
        radio_response: &binder::Strong<dyn IRadioMessagingResponse>,
        radio_indication: &binder::Strong<dyn IRadioMessagingIndication>,
    ) -> binder::Result<()> {
        info!("RadioMessaging: setResponseFunctions");

        {
            let mut shared = sharedmut!(&self);
            shared.response = Some(radio_response.clone());
            shared.indication = Some(radio_indication.clone());
        }

        RadioMessagingShared::notify_framework(&self.shared);

        Ok(())
    }
}

declare_async_iradio!(RadioMessaging, IRadioMessaging, BnRadioMessaging);
