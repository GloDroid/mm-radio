/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use crate::{
    mm_zbus::{mm_modem_proxy::ModemProxy, mm_sim_proxy::SimProxy},
    utils::iradio::{
        declare_async_iradio, def, entry_check, not_implemented, okay, shared, sharedmut,
    },
};
use android_hardware_radio::aidl::android::hardware::radio::RadioIndicationType::RadioIndicationType;
use android_hardware_radio_sim::aidl::android::hardware::radio::sim::{
    AppStatus::*, CardPowerState::*, CardStatus::*, CarrierRestrictions::*,
    CdmaSubscriptionSource::*, IRadioSim::*, IRadioSimIndication::*, IRadioSimResponse::*,
    IccIo::*, ImsiEncryptionInfo::*, PersoSubstate::*, PhonebookRecordInfo::*, PinState::*,
    SelectUiccSub::*, SimApdu::*, SimLockMultiSimPolicy::*,
};

use async_std::{sync::RwLock, task::block_on};
use binder::{BinderFeatures, Strong};
use log::info;
use std::sync::Arc;
use zbus::export::async_trait::async_trait;

use super::fake_icc_profile;

#[derive(Default)]
pub struct RadioSimShared {
    modem_bound: bool,
    response: Option<binder::Strong<dyn IRadioSimResponse>>,
    indication: Option<binder::Strong<dyn IRadioSimIndication>>,

    modem_proxy: Option<ModemProxy<'static>>,
    sim_proxy: Option<SimProxy<'static>>,

    channel: i32,
}

#[derive(Default)]
pub struct RadioSim {
    pub shared: Arc<RwLock<RadioSimShared>>,
}

impl RadioSimShared {
    pub fn bind(shared_in: &Arc<RwLock<RadioSimShared>>, modem_proxy: &ModemProxy<'static>) {
        /* Setup shared structure */
        {
            let mut shared = block_on(shared_in.write());
            shared.modem_bound = true;
            shared.modem_proxy = Some(modem_proxy.clone());
            let conn = modem_proxy.connection();
            let sim_path = block_on(modem_proxy.sim());
            if let Ok(sim_path) = sim_path {
                shared.sim_proxy = Some(
                    block_on(SimProxy::builder(conn).path(sim_path).unwrap().build()).unwrap(),
                );
            }
        }
        /* Notify framework */
        {
            let shared = block_on(shared_in.read());
            shared.notify_framework();
        }
    }

    pub fn unbind(shared_in: &Arc<RwLock<RadioSimShared>>) {
        {
            let mut shared = block_on(shared_in.write());
            shared.modem_proxy = None;
            shared.sim_proxy = None;
            shared.modem_bound = false;
        }
        let shared = block_on(shared_in.read());
        shared.notify_framework();
    }

    fn is_initialized(&self) -> bool {
        self.response.is_some() && self.indication.is_some() && self.modem_proxy.is_some()
    }

    fn notify_framework(&self) {
        if !self.is_initialized() {
            return;
        }
        let ind = self.indication.as_ref().unwrap();
        ind.simStatusChanged(RadioIndicationType::UNSOLICITED).unwrap();
        ind.subscriptionStatusChanged(RadioIndicationType::UNSOLICITED, true).unwrap();
    }
}

#[async_trait]
impl IRadioSimAsyncServer for RadioSim {
    async fn areUiccApplicationsEnabled(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, areUiccApplicationsEnabledResponse, false)
    }
    async fn changeIccPin2ForApp(
        &self,
        serial: i32,
        _old_pin2: &str,
        _new_pin2: &str,
        _aid: &str,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, changeIccPin2ForAppResponse, 0)
    }
    async fn changeIccPinForApp(
        &self,
        serial: i32,
        _old_pin: &str,
        _new_pin: &str,
        _aid: &str,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, changeIccPinForAppResponse, 0)
    }
    async fn enableUiccApplications(&self, serial: i32, _enable: bool) -> binder::Result<()> {
        entry_check!(&self, serial, enableUiccApplicationsResponse);
        okay!(&self, serial, enableUiccApplicationsResponse)
    }
    async fn getAllowedCarriers(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getAllowedCarriersResponse, &def(), def())
    }
    async fn getCdmaSubscription(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getCdmaSubscriptionResponse, "", def(), def(), def(), def())
    }
    async fn getCdmaSubscriptionSource(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getCdmaSubscriptionSourceResponse, def());
        okay!(&self, serial, getCdmaSubscriptionSourceResponse, CdmaSubscriptionSource::RUIM_SIM)
    }
    async fn getFacilityLockForApp(
        &self,
        serial: i32,
        _facility: &str,
        _password: &str,
        _service_class: i32,
        _app_id: &str,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, getFacilityLockForAppResponse, 0)
    }
    async fn getIccCardStatus(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getIccCardStatusResponse, &def());
        let shared = shared!(&self);
        if shared.sim_proxy.is_none() {
            return okay!(&self, serial, getIccCardStatusResponse, &def());
        }
        let sim_proxy = shared.sim_proxy.as_ref().unwrap();

        let cs = CardStatus {
            cardState: if sim_proxy.active().await.unwrap() { STATE_PRESENT } else { STATE_ABSENT },
            universalPinState: PinState::UNKNOWN,
            applications: vec![
                AppStatus {
                    appType: APP_TYPE_USIM,
                    appState: APP_STATE_READY,
                    aidPtr: "0xA0 0x00 0x00 0x00 0x87 0x10 0x02 0xFF 0xFF 0xFF 0xFF 0x89"
                        .to_string(),
                    appLabelPtr: "".to_string(),
                    pin1: PinState::UNKNOWN,
                    pin2: PinState::UNKNOWN,
                    ..Default::default()
                },
                AppStatus { appType: APP_TYPE_RUIM, ..Default::default() },
                AppStatus { appType: APP_TYPE_ISIM, ..Default::default() },
            ],
            gsmUmtsSubscriptionAppIndex: 0,
            cdmaSubscriptionAppIndex: -1,
            imsSubscriptionAppIndex: -1,
            iccid: sim_proxy.sim_identifier().await.unwrap().to_string(),
            eid: sim_proxy.eid().await.unwrap(),
            ..Default::default()
        };
        okay!(&self, serial, getIccCardStatusResponse, &cs)
    }
    async fn getImsiForApp(&self, serial: i32, _aid: &str) -> binder::Result<()> {
        entry_check!(&self, serial, getImsiForAppResponse, "");
        okay!(&self, serial, getImsiForAppResponse, "255010899987259")
    }
    async fn getSimPhonebookCapacity(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getSimPhonebookCapacityResponse, &def())
    }
    async fn getSimPhonebookRecords(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getSimPhonebookRecordsResponse)
    }
    async fn iccCloseLogicalChannel(&self, serial: i32, _channel_id: i32) -> binder::Result<()> {
        entry_check!(&self, serial, iccCloseLogicalChannelResponse);
        okay!(&self, serial, iccCloseLogicalChannelResponse)
    }
    async fn iccIoForApp(&self, serial: i32, icc_io: &IccIo) -> binder::Result<()> {
        entry_check!(&self, serial, iccIoForAppResponse, &def());
        let icc_resp = fake_icc_profile::get_default_sim_io(icc_io);
        okay!(&self, serial, iccIoForAppResponse, &icc_resp)
    }
    async fn iccOpenLogicalChannel(&self, serial: i32, _aid: &str, _p2: i32) -> binder::Result<()> {
        entry_check!(&self, serial, iccOpenLogicalChannelResponse, 0, &[]);
        sharedmut!(&self).channel += 1;
        let shared = shared!(&self);
        okay!(&self, serial, iccOpenLogicalChannelResponse, shared.channel, &[])
    }
    async fn iccTransmitApduBasicChannel(
        &self,
        serial: i32,
        _message: &SimApdu,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, iccTransmitApduBasicChannelResponse, &def())
    }
    async fn iccTransmitApduLogicalChannel(
        &self,
        serial: i32,
        _message: &SimApdu,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, iccTransmitApduLogicalChannelResponse, &def())
    }
    async fn reportStkServiceIsRunning(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, reportStkServiceIsRunningResponse);
        okay!(&self, serial, reportStkServiceIsRunningResponse)
    }
    async fn requestIccSimAuthentication(
        &self,
        serial: i32,
        _auth_context: i32,
        _auth_data: &str,
        _aid: &str,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, requestIccSimAuthenticationResponse, &def())
    }
    async fn responseAcknowledgement(&self) -> binder::Result<()> {
        Ok(())
    }
    async fn sendEnvelope(&self, serial: i32, _contents: &str) -> binder::Result<()> {
        not_implemented!(&self, serial, sendEnvelopeResponse, "")
    }
    async fn sendEnvelopeWithStatus(&self, serial: i32, _contents: &str) -> binder::Result<()> {
        not_implemented!(&self, serial, sendEnvelopeWithStatusResponse, &def())
    }
    async fn sendTerminalResponseToSim(&self, serial: i32, _contents: &str) -> binder::Result<()> {
        not_implemented!(&self, serial, sendTerminalResponseToSimResponse)
    }
    async fn setAllowedCarriers(
        &self,
        serial: i32,
        _carriers: &CarrierRestrictions,
        _multi_sim_policy: SimLockMultiSimPolicy,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setAllowedCarriersResponse)
    }
    async fn setCarrierInfoForImsiEncryption(
        &self,
        serial: i32,
        _imsi_encryption_info: &ImsiEncryptionInfo,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setCarrierInfoForImsiEncryptionResponse)
    }
    async fn setCdmaSubscriptionSource(
        &self,
        serial: i32,
        _cdma_sub: CdmaSubscriptionSource,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setCdmaSubscriptionSourceResponse)
    }
    async fn setFacilityLockForApp(
        &self,
        serial: i32,
        _facility: &str,
        _lock_state: bool,
        _password: &str,
        _service_class: i32,
        _app_id: &str,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setFacilityLockForAppResponse, 0)
    }
    async fn setSimCardPower(&self, serial: i32, _power_up: CardPowerState) -> binder::Result<()> {
        not_implemented!(&self, serial, setSimCardPowerResponse)
    }
    async fn setUiccSubscription(
        &self,
        serial: i32,
        _uicc_sub: &SelectUiccSub,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setUiccSubscriptionResponse)
    }
    async fn supplyIccPin2ForApp(
        &self,
        serial: i32,
        _pin2: &str,
        _aid: &str,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, supplyIccPin2ForAppResponse, 0)
    }
    async fn supplyIccPinForApp(&self, serial: i32, _pin: &str, _aid: &str) -> binder::Result<()> {
        not_implemented!(&self, serial, supplyIccPinForAppResponse, 0)
    }
    async fn supplyIccPuk2ForApp(
        &self,
        serial: i32,
        _puk2: &str,
        _pin2: &str,
        _aid: &str,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, supplyIccPuk2ForAppResponse, 0)
    }
    async fn supplyIccPukForApp(
        &self,
        serial: i32,
        _puk: &str,
        _pin: &str,
        _aid: &str,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, supplyIccPukForAppResponse, 0)
    }
    async fn supplySimDepersonalization(
        &self,
        serial: i32,
        _perso_type: PersoSubstate,
        _control_key: &str,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, supplySimDepersonalizationResponse, def(), def())
    }
    async fn updateSimPhonebookRecords(
        &self,
        serial: i32,
        _record_info: &PhonebookRecordInfo,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, updateSimPhonebookRecordsResponse, 0)
    }
    async fn setResponseFunctions(
        &self,
        radio_response: &binder::Strong<dyn IRadioSimResponse>,
        radio_indication: &binder::Strong<dyn IRadioSimIndication>,
    ) -> binder::Result<()> {
        info!("RadioSim::setResponseFunctions");

        let mut shared = sharedmut!(&self);
        shared.response = Some(radio_response.clone());
        shared.indication = Some(radio_indication.clone());
        shared.notify_framework();

        Ok(())
    }
}

declare_async_iradio!(RadioSim, IRadioSim, BnRadioSim);
