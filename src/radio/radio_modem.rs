/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use crate::{
    mm_zbus::{consts::*, mm_modem_proxy::ModemProxy, mm_sim_proxy::SimProxy},
    utils::iradio::{
        declare_async_iradio, def, entry_check, not_implemented, okay, shared, sharedmut,
    },
};
use android_hardware_radio::aidl::android::hardware::radio::{
    RadioAccessFamily::*, RadioIndicationType::*, RadioTechnology::*,
};
use android_hardware_radio_modem::aidl::android::hardware::radio::modem::{
    DeviceStateType::*, HardwareConfig::*, HardwareConfigModem::*, HardwareConfigSim::*,
    IRadioModem::*, IRadioModemIndication::*, IRadioModemResponse::*, NvItem::*, NvWriteItem::*,
    RadioCapability::*, RadioState::*, ResetNvType::*,
};
use async_std::task::{block_on, spawn};
use async_std::{stream::StreamExt, sync::RwLock};
use binder::{BinderFeatures, Strong};
use futures::{select, FutureExt};
use log::info;
use std::sync::Arc;
use zbus::export::async_trait::async_trait;

#[derive(Default)]
pub struct RadioModemShared {
    modem_bound: bool,
    response: Option<binder::Strong<dyn IRadioModemResponse>>,
    indication: Option<binder::Strong<dyn IRadioModemIndication>>,

    modem_proxy: Option<ModemProxy<'static>>,
    sim_proxy: Option<SimProxy<'static>>,

    radio_capability: RadioCapability,
}

#[derive(Default)]
pub struct RadioModem {
    pub shared: Arc<RwLock<RadioModemShared>>,
}

impl RadioModemShared {
    pub fn bind(shared_in: &Arc<RwLock<RadioModemShared>>, modem_proxy: &ModemProxy<'static>) {
        /* Setup shared structure */
        {
            let mut shared = block_on(shared_in.write());
            shared.modem_proxy = Some(modem_proxy.clone());
            let conn = modem_proxy.connection();
            let sim_path = block_on(modem_proxy.sim());
            if let Ok(sim_path) = sim_path {
                shared.sim_proxy = Some(
                    block_on(SimProxy::builder(conn).path(sim_path).unwrap().build()).unwrap(),
                );
            }
            shared.modem_bound = true;
        }
        /* Register listeners */
        {
            let shared = shared_in.clone();
            spawn(async move {
                let mm_modem_c = shared.read().await.modem_proxy.as_ref().unwrap().clone();
                let mut st_prop = mm_modem_c.receive_state_changed().await;
                loop {
                    select! {
                        e = st_prop.next().fuse() => {
                            let state = e.unwrap().get().await.unwrap();
                            info!("Signal: {}", state);
                            shared.read().await.notify_radio_state().await;
                        },
                        complete => break,
                    };
                }

                info!("Exit");
            });
        }
        /* Notify the framework */
        {
            let shared = block_on(shared_in.read());
            block_on(shared.notify_framework());
        }
    }

    pub fn unbind(shared_in: &Arc<RwLock<RadioModemShared>>) {
        let mut shared = block_on(shared_in.write());
        shared.modem_bound = false;
        shared.modem_proxy = None;
        shared.sim_proxy = None;
    }

    pub fn is_initialized(&self) -> bool {
        self.response.is_some() && self.indication.is_some() && self.modem_proxy.is_some()
    }

    async fn notify_radio_state(&self) {
        if !self.is_initialized() {
            return;
        }
        let ind = self.indication.as_ref().unwrap();
        let state = self.modem_proxy.as_ref().unwrap().state().await.unwrap();
        let enabled = state == mm_modem_state::ENABLED || state == mm_modem_state::REGISTERED;
        ind.radioStateChanged(
            RadioIndicationType::UNSOLICITED,
            if enabled { RadioState::ON } else { RadioState::OFF },
        )
        .unwrap();
    }

    pub async fn notify_framework(&self) {
        if !self.is_initialized() {
            return;
        }
        let ind = self.indication.as_ref().unwrap();
        ind.radioCapabilityIndication(RadioIndicationType::UNSOLICITED, &self.radio_capability)
            .unwrap();
        ind.rilConnected(RadioIndicationType::UNSOLICITED).unwrap();
        self.notify_radio_state().await;
    }
}

#[async_trait]
impl IRadioModemAsyncServer for RadioModem {
    async fn enableModem(&self, serial: i32, _on: bool) -> binder::Result<()> {
        not_implemented!(&self, serial, enableModemResponse)
    }
    async fn getBasebandVersion(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getBasebandVersionResponse, "");
        okay!(&self, serial, getBasebandVersionResponse, "10000")
    }
    async fn getDeviceIdentity(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getDeviceIdentityResponse, "", "", "", "");
        let shared = shared!(&self);
        if shared.sim_proxy.is_none() {
            return okay!(&self, serial, getDeviceIdentityResponse, "", "", "", "");
        }
        let sim_proxy = shared.sim_proxy.as_ref().unwrap();
        let imei = sim_proxy.sim_identifier().await.unwrap();
        okay!(&self, serial, getDeviceIdentityResponse, imei.as_str(), "00", "00", "00")
    }

    async fn getHardwareConfig(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getHardwareConfigResponse, def());
        let hc = vec![
            HardwareConfig {
                r#type: TYPE_MODEM,
                uuid: "MODEM-UUID".to_string(),
                state: STATE_ENABLED,
                modem: vec![HardwareConfigModem {
                    maxDataCalls: 0,
                    maxStandby: 1,
                    maxVoiceCalls: 1,
                    rilModel: 0,
                    rat: RadioTechnology(1 << 14), /* LTE bit */
                }],
                ..Default::default()
            },
            HardwareConfig {
                r#type: TYPE_SIM,
                uuid: "SIM-UUID".to_string(),
                state: STATE_ENABLED,
                sim: vec![HardwareConfigSim { modemUuid: "MODEM-UUID".to_string() }],
                ..Default::default()
            },
        ];
        okay!(&self, serial, getHardwareConfigResponse, &hc)
    }

    async fn getModemActivityInfo(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getModemActivityInfoResponse, &def())
    }
    async fn getModemStackStatus(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getModemStackStatusResponse, def());
        okay!(&self, serial, getModemStackStatusResponse, false)
    }
    async fn getRadioCapability(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getRadioCapabilityResponse, &def());
        let shared = shared!(&self);
        okay!(&self, serial, getRadioCapabilityResponse, &shared.radio_capability)
    }
    async fn nvReadItem(&self, serial: i32, _: NvItem) -> binder::Result<()> {
        not_implemented!(&self, serial, nvReadItemResponse, "")
    }
    async fn nvResetConfig(&self, serial: i32, _: ResetNvType) -> binder::Result<()> {
        not_implemented!(&self, serial, nvResetConfigResponse)
    }
    async fn nvWriteCdmaPrl(&self, serial: i32, _: &[u8]) -> binder::Result<()> {
        not_implemented!(&self, serial, nvWriteCdmaPrlResponse)
    }
    async fn nvWriteItem(&self, serial: i32, _: &NvWriteItem) -> binder::Result<()> {
        not_implemented!(&self, serial, nvWriteItemResponse)
    }

    async fn requestShutdown(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, requestShutdownResponse)
    }

    async fn responseAcknowledgement(&self) -> binder::Result<()> {
        Ok(())
    }

    async fn sendDeviceState(
        &self,
        serial: i32,
        _: DeviceStateType,
        _: bool,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, sendDeviceStateResponse)
    }

    async fn setRadioCapability(&self, serial: i32, rc: &RadioCapability) -> binder::Result<()> {
        entry_check!(&self, serial, setRadioCapabilityResponse, &def());
        sharedmut!(&self).radio_capability = RadioCapability {
            session: rc.session,
            phase: rc.phase,
            raf: rc.raf,
            logicalModemUuid: rc.logicalModemUuid.clone(),
            status: rc.status,
        };
        let shared = shared!(&self);
        okay!(&self, serial, setRadioCapabilityResponse, &shared.radio_capability)
    }

    async fn setRadioPower(
        &self,
        serial: i32,
        power_on: bool,
        _for_emergency_call: bool,
        _preferred_for_emergency_call: bool,
    ) -> binder::Result<()> {
        entry_check!(&self, serial, setRadioPowerResponse);
        info!("RadioModem: setRadioPower: {}", power_on);

        let shared = self.shared.clone();
        spawn(async move {
            let shared = shared.read().await;
            let mp = shared.modem_proxy.clone().unwrap();
            mp.enable(power_on).await.unwrap();
        });

        okay!(&self, serial, setRadioPowerResponse)
    }

    async fn setResponseFunctions(
        &self,
        radio_response: &binder::Strong<dyn IRadioModemResponse>,
        radio_indication: &binder::Strong<dyn IRadioModemIndication>,
    ) -> binder::Result<()> {
        info!("RadioModem: setResponseFunctions");

        let mut shared = sharedmut!(&self);
        shared.response = Some(radio_response.clone());
        shared.indication = Some(radio_indication.clone());

        shared.radio_capability = RadioCapability {
            logicalModemUuid: "com.mm-radio.lm0".to_string(),
            phase: PHASE_CONFIGURED,
            raf: RadioAccessFamily::LTE.0,
            status: STATUS_SUCCESS,
            session: 0,
        };

        shared.notify_framework().await;

        Ok(())
    }
}

declare_async_iradio!(RadioModem, IRadioModem, BnRadioModem);
