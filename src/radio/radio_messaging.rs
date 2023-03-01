/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use crate::{
    mm_zbus::{
        consts::{
            mm_modem_state,
            mm_sms_state::{RECEIVED, RECEIVING},
        },
        mm_messaging_proxy::MessagingProxy,
        mm_modem_proxy::ModemProxy,
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
    channel::Sender,
    future::timeout,
    sync::{Mutex, RwLock},
    task::{block_on, spawn},
};
use binder::{BinderFeatures, Strong};
use futures::{select, FutureExt, StreamExt};
use log::{error, info};
use std::{collections::HashMap, sync::Arc, time::Duration};
use zbus::{export::async_trait::async_trait, zvariant::OwnedObjectPath};

#[derive(Default)]
pub struct RadioMessagingShared {
    modem_bound: bool,
    response: Option<binder::Strong<dyn IRadioMessagingResponse>>,
    indication: Option<binder::Strong<dyn IRadioMessagingIndication>>,

    modem_proxy: Option<ModemProxy<'static>>,
    messaging_proxy: Option<MessagingProxy<'static>>,

    incoming_sms_list: Vec<OwnedObjectPath>,
    incoming_sms_process_lock: Arc<Mutex<()>>,
    incoming_sms_confirmation: Option<Sender<bool>>,
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

        /* Subscribe for modem enable events to query pending SMS */
        {
            let shared_in_c = shared_in.clone();
            spawn(async move {
                let mproxy = {
                    let shared = shared_in_c.read().await;
                    shared.modem_proxy.as_ref().unwrap().clone()
                };
                let _state = mproxy.state().await; /* get state here to avoid getting initial state below */
                let mut st_prop = mproxy.receive_state_changed().await;
                loop {
                    select! {
                        e = st_prop.next().fuse() => {
                            let state = e.unwrap().get().await.unwrap();
                            if state == mm_modem_state::ENABLED || state == mm_modem_state::REGISTERED {
                                info!("Modem enabled, querying pending SMS");
                                let _ = Self::query_pending_sms(&shared_in_c).await;
                            }
                        },
                        complete => break,
                    };
                }
            });
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
                loop {
                    select! {
                        added = added_signal.next().fuse() => {
                            let added = added.unwrap();
                            let received = added.args().unwrap().received;
                            if received {
                                let path = added.args().unwrap().path;
                                spawn(Self::handle_message_received(shared_in_c.clone(), path.into()));
                            }
                        }
                        complete => break,
                    }
                }
            });
        }
    }

    pub fn unbind(shared_in: &Arc<RwLock<RadioMessagingShared>>) {
        let mut shared = block_on(shared_in.write());
        shared.modem_bound = false;
        shared.modem_proxy = None;
    }

    async fn query_pending_sms(
        shared_in: &Arc<RwLock<RadioMessagingShared>>,
    ) -> Result<(), zbus::Error> {
        let shared = shared_in.read().await;
        let messages = shared.messaging_proxy.as_ref().unwrap().messages().await?;

        let conn = shared.modem_proxy.as_ref().unwrap().connection();
        for path in messages {
            let sms_proxy = SmsProxy::builder(conn).path(path.clone())?.build().await?;
            let state = sms_proxy.state().await?;
            if state == RECEIVED || state == RECEIVING {
                spawn(Self::handle_message_received(shared_in.clone(), path));
            }
        }

        Ok(())
    }

    async fn delete_sms(shared_in: Arc<RwLock<RadioMessagingShared>>, path: OwnedObjectPath) {
        info!("Deleting SMS message: {path}");

        let result =
            shared_in.as_ref().read().await.messaging_proxy.as_ref().unwrap().delete(&path).await;

        if let Err(e) = result {
            info!("Failed to delete SMS message at {path}: {e}");
        }
    }

    /* must be a spawned async task */
    async fn handle_message_received(
        shared_in: Arc<RwLock<RadioMessagingShared>>,
        path: OwnedObjectPath,
    ) -> Result<(), zbus::Error> {
        /* Avoid duplicates */
        {
            let mut shared = shared_in.write().await;
            if shared.incoming_sms_list.contains(&path) {
                drop(shared);
                info!("SMS message '{path}' was already processed");
                return Ok(());
            }
            shared.incoming_sms_list.push(path.clone());
        }

        let sms_proxy = {
            let shared = shared_in.read().await;
            let conn = shared.modem_proxy.as_ref().unwrap().connection();
            SmsProxy::builder(conn).path(path.clone()).unwrap().build().await.unwrap()
        };

        info!("New SMS message receiving: {path}");

        loop {
            match sms_proxy.receive_state_changed().await.next().await.unwrap().get().await? {
                RECEIVING => continue,
                RECEIVED => break,
                s => {
                    error!("Unknown SMS '{path}' state: {s}");
                    return Ok(());
                }
            }
        }

        let number = sms_proxy.number().await?;
        let text = sms_proxy.text().await?;
        let timestamp = sms_proxy.timestamp().await?;

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

        let lock = shared_in.read().await.incoming_sms_process_lock.clone();
        loop {
            /* Lock, so multiple tasks won't report messages simultaneously */
            let _lock = lock.lock().await;
            let mut shared = shared_in.write().await;
            let ind = shared.indication.clone();
            if ind.is_none() {
                drop(shared);
                // Ugly, so contributions are welcome
                info!("No IRadio callbacks registered when reporting SMS: {path}, retry in 10 seconds...");
                async_std::task::sleep(Duration::from_secs(10)).await;
                continue;
            }
            let (sms_processed, wait_sms_processed) = async_std::channel::bounded::<bool>(1);
            shared.incoming_sms_confirmation = Some(sms_processed);
            let ind = ind.unwrap();
            ind.newSms(RadioIndicationType::UNSOLICITED, &pdu_bytes).unwrap();
            drop(shared);

            let fut = wait_sms_processed.into_future();
            let result = timeout(Duration::from_secs(5), fut).await;
            if result.is_err() {
                error!("Timeout accepting incoming SMS: {path}");
                continue;
            }
            break;
        }

        Self::delete_sms(shared_in, path).await;
        Ok(())
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

        {
            let shared = shared!(&self);
            if let Some(confirmation) = &shared.incoming_sms_confirmation {
                confirmation.send(true).await.unwrap();
            }
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
                .sendSmsResponse(&respond(serial, RadioError::NONE), &def())
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

        Ok(())
    }
}

declare_async_iradio!(RadioMessaging, IRadioMessaging, BnRadioMessaging);
