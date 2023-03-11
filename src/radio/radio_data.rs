/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use crate::mm_zbus::consts::mm_bearer_ip_family;
use crate::mm_zbus::mm_bearer_proxy::BearerProxy;
use crate::mm_zbus::mm_modem_proxy::ModemProxy;
use crate::mm_zbus::mm_simple_proxy::SimpleProxy;
use crate::utils::error::Error;
use crate::utils::iradio::{
    declare_async_iradio, entry_check, not_implemented, okay, resp_err, shared, sharedmut,
};
use android_hardware_radio::aidl::android::hardware::radio::AccessNetwork::AccessNetwork;
use android_hardware_radio_data::aidl::android::hardware::radio::data::DataRequestReason::DataRequestReason as DRR;
use android_hardware_radio_data::aidl::android::hardware::radio::data::{
    DataCallFailCause::*, DataProfileInfo::*, DataThrottlingAction::*, IRadioData::*,
    IRadioDataIndication::*, IRadioDataResponse::*, KeepaliveRequest::*, LinkAddress::*,
    PdpProtocolType::*, SetupDataCallResult::*, SliceInfo::*,
};
use async_std::sync::RwLock;
use async_std::task::block_on;
use binder::{BinderFeatures, Strong};
use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use zbus::export::async_trait::async_trait;

#[derive(Default)]
pub struct RadioDataShared {
    modem_bound: bool,
    response: Option<binder::Strong<dyn IRadioDataResponse>>,
    indication: Option<binder::Strong<dyn IRadioDataIndication>>,

    modem_proxy: Option<ModemProxy<'static>>,
    simple_proxy: Option<SimpleProxy<'static>>,
}

#[derive(Default)]
pub struct RadioData {
    pub shared: Arc<RwLock<RadioDataShared>>,
}

impl RadioDataShared {
    pub(crate) fn bind(
        shared_in: &Arc<RwLock<RadioDataShared>>,
        modem_proxy: &ModemProxy<'static>,
    ) -> Result<(), Error> {
        /* Setup shared structure */
        {
            let mut shared = block_on(shared_in.write());
            shared.modem_proxy = Some(modem_proxy.clone());
            let conn = modem_proxy.connection();
            let path = modem_proxy.path().to_string();
            shared.simple_proxy = Some(block_on(SimpleProxy::builder(conn).path(path)?.build())?);
            shared.modem_bound = true;
        }
        Self::notify_framework(shared_in)?;
        Ok(())
    }

    pub(crate) fn unbind(shared_in: &Arc<RwLock<RadioDataShared>>) -> Result<(), Error> {
        {
            let mut shared = block_on(shared_in.write());
            shared.modem_proxy = None;
            shared.simple_proxy = None;
            shared.modem_bound = false;
        }
        Self::notify_framework(shared_in)?;
        Ok(())
    }

    fn is_initialized(&self) -> bool {
        self.response.is_some() && self.indication.is_some() && self.modem_bound
    }

    fn notify_framework(shared_in: &Arc<RwLock<RadioDataShared>>) -> Result<(), Error> {
        let shared = block_on(shared_in.read());
        if !shared.is_initialized() {
            return Ok(());
        }
        let ind = shared.indication.as_ref().ok_or(Error::noneopt())?;
        ind.dataCallListChanged(Default::default(), Default::default())?;
        Ok(())
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
        entry_check!(&self, serial, getDataCallListResponse, &[]);
        okay!(&self, serial, getDataCallListResponse, &[])
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
        entry_check!(&self, serial, setDataProfileResponse);
        info!("setDataProfile: {:?}", _profiles);
        okay!(&self, serial, setDataProfileResponse)
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
        entry_check!(&self, serial, setInitialAttachApnResponse);
        info!("setInitialAttachApn: {:?}", _data_profile_info);
        okay!(&self, serial, setInitialAttachApnResponse)
    }

    async fn setupDataCall(
        &self,
        serial: i32,
        _access_network: AccessNetwork,
        data_profile_info: &DataProfileInfo,
        _roaming_allowed: bool,
        _reason: DRR,
        _addresses: &[LinkAddress],
        _dnses: &[String],
        _pdu_session_id: i32,
        _slice_info: Option<&SliceInfo>,
        _match_all_rule_allowed: bool,
    ) -> binder::Result<()> {
        entry_check!(&self, serial, setupDataCallResponse, &Default::default());
        info!("setupDataCall: {:?}", data_profile_info);
        let result: Result<SetupDataCallResult, Error> = try {
            let shared = shared!(&self);
            let mut props = HashMap::new();
            props.insert("apn", data_profile_info.apn.clone().into());
            props.insert("user", data_profile_info.user.clone().into());
            props.insert("password", data_profile_info.password.clone().into());
            let ip_family = match data_profile_info.protocol {
                PdpProtocolType::IPV4V6 => mm_bearer_ip_family::IPV4V6,
                PdpProtocolType::IP => mm_bearer_ip_family::IPV4,
                PdpProtocolType::IPV6 => mm_bearer_ip_family::IPV6,
                PdpProtocolType::NON_IP => mm_bearer_ip_family::NON_IP,
                _ => mm_bearer_ip_family::ANY,
            };
            props.insert("ip-type", ip_family.into());
            let result = shared.simple_proxy.as_ref().ok_or(Error::noneopt())?.connect(props).await;
            if result.is_err() {
                error!("setupDataCall error: {:?}", result);
                return resp_err!(
                    &self,
                    serial,
                    RadioError::INTERNAL_ERR,
                    setupDataCallResponse,
                    &Default::default()
                );
            }

            let path = result?;
            let conn = shared.simple_proxy.as_ref().ok_or(Error::noneopt())?.connection();
            let bearer_proxy = BearerProxy::builder(conn).path(path)?.build().await?;

            let connected = bearer_proxy.connected().await?;
            info!("setupDataCall connected: {connected}");

            let ipv4 = bearer_proxy.ip4_config().await?;
            info!("setupDataCall ipv4: {:?}", ipv4);
            let addrv4 = LinkAddress {
                address: ipv4.get("address").ok_or(Error::noneopt())?.to_owned().try_into()?,
                ..Default::default()
            };

            let mut dnses: Vec<String> = Vec::new();
            for i in 1..=2 {
                if let Some(opt) = ipv4.get(&format!("dns{i}")) {
                    dnses.push(opt.to_owned().try_into()?);
                }
            }

            let mtu_v6: Option<i32> = try {
                let ipv6 = bearer_proxy.ip6_config().await.ok()?;
                TryInto::<u32>::try_into(ipv6.get("mtu")?).ok()? as i32
            };

            let mtu_v6 = mtu_v6.unwrap_or(1500);

            SetupDataCallResult {
                cid: 1,
                active: DATA_CONNECTION_STATUS_ACTIVE,
                r#type: data_profile_info.protocol,
                ifname: bearer_proxy.interface().await?,
                addresses: vec![addrv4],
                dnses,
                gateways: vec![ipv4
                    .get("gateway")
                    .ok_or(Error::noneopt())?
                    .to_owned()
                    .try_into()?],
                pcscf: vec![],
                mtuV4: TryInto::<u32>::try_into(ipv4.get("mtu").ok_or(Error::noneopt())?)? as i32,
                mtuV6: mtu_v6,
                pduSessionId: 0,
                cause: DataCallFailCause::NONE,
                defaultQos: Default::default(),
                handoverFailureMode: HANDOVER_FAILURE_MODE_NO_FALLBACK_RETRY_SETUP_NORMAL,
                qosSessions: vec![],
                sliceInfo: None,
                suggestedRetryTime: 0,
                trafficDescriptors: vec![],
            }
        };
        let result = result?;
        okay!(&self, serial, setupDataCallResponse, &result)
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

        RadioDataShared::notify_framework(&self.shared)?;

        Ok(())
    }
}

declare_async_iradio!(RadioData, IRadioData, BnRadioData);
