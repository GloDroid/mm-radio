/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use crate::mm_zbus::mm_modem_proxy::ModemProxy;
use crate::utils::iradio::{declare_async_iradio, entry_check, not_implemented, shared, sharedmut};
use android_hardware_radio::aidl::android::hardware::radio::AccessNetwork::AccessNetwork;
use android_hardware_radio_data::aidl::android::hardware::radio::data::DataRequestReason::DataRequestReason as DRR;
use android_hardware_radio_data::aidl::android::hardware::radio::data::{
    DataProfileInfo::*, DataThrottlingAction::*, IRadioData::*, IRadioDataIndication::*,
    IRadioDataResponse::*, KeepaliveRequest::*, LinkAddress::*, SliceInfo::*,
};
use async_std::sync::RwLock;
use async_std::task::block_on;
use binder::{BinderFeatures, Strong};
use log::info;
use std::sync::Arc;
use zbus::export::async_trait::async_trait;

#[derive(Default)]
pub struct RadioDataShared {
    modem_bound: bool,
    response: Option<binder::Strong<dyn IRadioDataResponse>>,
    indication: Option<binder::Strong<dyn IRadioDataIndication>>,

    modem_proxy: Option<ModemProxy<'static>>,
}

#[derive(Default)]
pub struct RadioData {
    pub shared: Arc<RwLock<RadioDataShared>>,
}

impl RadioDataShared {
    pub fn bind(shared_in: &Arc<RwLock<RadioDataShared>>, modem_proxy: &ModemProxy<'static>) {
        /* Setup shared structure */
        {
            let mut shared = block_on(shared_in.write());
            shared.modem_proxy = Some(modem_proxy.clone());
            shared.modem_bound = true;
        }
        Self::notify_framework(shared_in);
    }

    pub fn unbind(shared_in: &Arc<RwLock<RadioDataShared>>) {
        {
            let mut shared = block_on(shared_in.write());
            shared.modem_proxy = None;
            shared.modem_bound = false;
        }
        Self::notify_framework(shared_in);
    }

    fn is_initialized(&self) -> bool {
        self.response.is_some() && self.indication.is_some() && self.modem_bound
    }

    fn notify_framework(shared_in: &Arc<RwLock<RadioDataShared>>) {
        let shared = block_on(shared_in.read());
        if !shared.is_initialized() {
            return;
        }
        let ind = shared.indication.as_ref().unwrap();
        ind.dataCallListChanged(Default::default(), Default::default()).unwrap();
    }
}

#[async_trait]
impl IRadioDataAsyncServer for RadioData {
    async fn allocatePduSessionId(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, allocatePduSessionIdResponse, 0)
    }
    async fn cancelHandover(&self, serial: i32, _call_id: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, cancelHandoverResponse)
    }
    async fn deactivateDataCall(&self, serial: i32, _cid: i32, _reason: DRR) -> binder::Result<()> {
        not_implemented!(&self, serial, deactivateDataCallResponse)
    }
    async fn getDataCallList(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getDataCallListResponse, &[])
    }
    async fn getSlicingConfig(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getSlicingConfigResponse, &Default::default())
    }
    async fn releasePduSessionId(&self, serial: i32, _id: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, releasePduSessionIdResponse)
    }
    async fn responseAcknowledgement(&self) -> binder::Result<()> {
        Ok(())
    }
    async fn setDataAllowed(&self, serial: i32, _allow: bool) -> binder::Result<()> {
        not_implemented!(&self, serial, setDataAllowedResponse)
    }
    async fn setDataProfile(
        &self,
        serial: i32,
        _profiles: &[DataProfileInfo],
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setDataProfileResponse)
    }

    async fn setDataThrottling(
        &self,
        serial: i32,
        _data_throttling_action: DataThrottlingAction,
        _completion_duration_millis: i64,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setDataThrottlingResponse)
    }

    async fn setInitialAttachApn(
        &self,
        serial: i32,
        _data_profile_info: Option<&DataProfileInfo>,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setInitialAttachApnResponse)
    }

    async fn setupDataCall(
        &self,
        serial: i32,
        _access_network: AccessNetwork,
        _data_profile_info: &DataProfileInfo,
        _roaming_allowed: bool,
        _reason: DRR,
        _addresses: &[LinkAddress],
        _dnses: &[String],
        _pdu_session_id: i32,
        _slice_info: Option<&SliceInfo>,
        _match_all_rule_allowed: bool,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setupDataCallResponse, &Default::default())
    }

    async fn startHandover(&self, serial: i32, _call_id: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, startHandoverResponse)
    }

    async fn startKeepalive(
        &self,
        serial: i32,
        _keepalive: &KeepaliveRequest,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, startKeepaliveResponse, &Default::default())
    }

    async fn stopKeepalive(&self, serial: i32, _session_handle: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, stopKeepaliveResponse)
    }

    async fn setResponseFunctions(
        &self,
        radio_response: &binder::Strong<dyn IRadioDataResponse>,
        radio_indication: &binder::Strong<dyn IRadioDataIndication>,
    ) -> binder::Result<()> {
        info!("RadioData: setResponseFunctions");

        {
            let mut shared = sharedmut!(&self);
            shared.response = Some(radio_response.clone());
            shared.indication = Some(radio_indication.clone());
        }

        RadioDataShared::notify_framework(&self.shared);

        Ok(())
    }
}

declare_async_iradio!(RadioData, IRadioData, BnRadioData);
