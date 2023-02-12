/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use crate::utils::iradio::{
    declare_async_iradio, entry_check, not_implemented, okay, resp, resp_ok, shared, sharedmut,
};
use android_hardware_radio::aidl::android::hardware::radio::RadioIndicationType::*;
use android_hardware_radio_config::aidl::android::hardware::radio::config::{
    IRadioConfig::*, IRadioConfigIndication::*, IRadioConfigResponse::*, PhoneCapability::*,
    SimPortInfo::*, SimSlotStatus::*, SlotPortMapping::*,
};
use android_hardware_radio_sim::aidl::android::hardware::radio::sim::CardStatus::STATE_PRESENT;
use async_std::sync::RwLock;
use binder::{BinderFeatures, Strong};
use log::info;
use std::sync::Arc;
use zbus::export::async_trait::async_trait;

#[derive(Default)]
pub struct RadioConfigShared {
    /* modem_bound is used here only for unification of macros. Should be always true! */
    pub modem_bound: bool,
    response: Option<binder::Strong<dyn IRadioConfigResponse>>,
    indication: Option<binder::Strong<dyn IRadioConfigIndication>>,

    slot_status: Vec<SimSlotStatus>,
}

#[derive(Default)]
pub struct RadioConfig {
    shared: Arc<RwLock<RadioConfigShared>>,
}

#[async_trait]
impl IRadioConfigAsyncServer for RadioConfig {
    async fn getSimSlotsStatus(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getSimSlotsStatusResponse, Default::default());
        let shared = shared!(&self);
        okay!(&self, serial, getSimSlotsStatusResponse, &shared.slot_status)
    }
    async fn setSimSlotsMapping(
        &self,
        serial: i32,
        _smap: &[SlotPortMapping],
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setSimSlotsMappingResponse)
    }
    async fn getHalDeviceCapabilities(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getHalDeviceCapabilitiesResponse, false);
        resp!(&self).getHalDeviceCapabilitiesResponse(&resp_ok(serial), true)
    }
    async fn getNumOfLiveModems(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getNumOfLiveModemsResponse, 0);
        resp!(&self).getNumOfLiveModemsResponse(&resp_ok(serial), 0)
    }
    async fn getPhoneCapability(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getPhoneCapabilityResponse, &Default::default());
        let pc = PhoneCapability {
            isInternetLingeringSupported: false,
            maxActiveData: 1,
            maxActiveInternetData: 1,
            logicalModemIds: vec![0],
        };
        resp!(&self).getPhoneCapabilityResponse(&resp_ok(serial), &pc)
    }
    async fn setNumOfLiveModems(&self, serial: i32, _num_of_live_modems: i8) -> binder::Result<()> {
        not_implemented!(&self, serial, setNumOfLiveModemsResponse)
    }
    async fn setPreferredDataModem(&self, serial: i32, _arg_modem_id: i8) -> binder::Result<()> {
        entry_check!(&self, serial, setPreferredDataModemResponse);
        resp!(&self).setPreferredDataModemResponse(&resp_ok(serial))
    }
    async fn setResponseFunctions(
        &self,
        radio_response: &binder::Strong<dyn IRadioConfigResponse>,
        radio_indication: &binder::Strong<dyn IRadioConfigIndication>,
    ) -> binder::Result<()> {
        info!("RadioConfig::setResponseFunctions");
        let mut shared = sharedmut!(&self);
        shared.response = Some(radio_response.clone());
        shared.indication = Some(radio_indication.clone());
        shared.modem_bound = true; /* for compatibility */
        shared.slot_status = vec![SimSlotStatus {
            cardState: STATE_PRESENT,
            portInfo: vec![SimPortInfo {
                iccId: "8938003992840681512F".to_string(),
                logicalSlotId: 1,
                portActive: true,
            }],
            ..Default::default()
        }];
        radio_indication
            .simSlotsStatusChanged(RadioIndicationType::UNSOLICITED, &shared.slot_status)?;

        Ok(())
    }
}

declare_async_iradio!(RadioConfig, IRadioConfig, BnRadioConfig);
