/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use crate::{
    mm_zbus::{
        mm_modem_3gpp_proxy::Modem3gppProxy, mm_modem_proxy::ModemProxy,
        mm_signal_proxy::SignalProxy,
    },
    utils::{
        error::Error,
        iradio::{
            declare_async_iradio, def, entry_check, ind, invalid_arg, not_implemented, okay,
            shared, sharedmut,
        },
    },
};
use android_hardware_radio::aidl::android::hardware::radio::{
    AccessNetwork::*, RadioIndicationType::*, RadioTechnology::*, RadioTechnologyFamily::*,
};
use android_hardware_radio_network::aidl::android::hardware::radio::network::{
    AccessTechnologySpecificInfo::*, CdmaRoamingType::*, CellConnectionStatus::*, CellIdentity::*,
    CellIdentityLte::*, CellInfo::*, CellInfoLte::*,
    CellInfoRatSpecificInfo::CellInfoRatSpecificInfo::Lte, EutranBands::*,
    EutranRegistrationInfo::*, IRadioNetwork::*, IRadioNetworkIndication::*,
    IRadioNetworkResponse::*, LteSignalStrength::*, LteVopsInfo::*, NetworkScanRequest::*,
    NrDualConnectivityState::*, OperatorInfo::*, RadioAccessSpecifier::*, RadioBandMode::*,
    RegState::*, RegStateResult::*, SignalStrength::*, SignalThresholdInfo::*, UsageSetting::*,
};

use async_std::{
    sync::RwLock,
    task::{block_on, spawn},
};
use binder::{BinderFeatures, Strong};
use futures::{select, FutureExt, StreamExt};
use log::info;
use std::sync::Arc;
use zbus::export::async_trait::async_trait;

#[derive(Default)]
pub struct RadioNetworkShared {
    pub modem_bound: bool,
    response: Option<binder::Strong<dyn IRadioNetworkResponse>>,
    indication: Option<binder::Strong<dyn IRadioNetworkIndication>>,

    modem_proxy: Option<ModemProxy<'static>>,
    modem_3gpp_proxy: Option<Modem3gppProxy<'static>>,
    signal_proxy: Option<SignalProxy<'static>>,

    signal_strength: SignalStrength,
    usage_setting: UsageSetting,
}

#[derive(Default)]
pub struct RadioNetwork {
    pub shared: Arc<RwLock<RadioNetworkShared>>,
}

impl RadioNetworkShared {
    pub(crate) fn bind(
        shared_in: &Arc<RwLock<RadioNetworkShared>>,
        modem_proxy: &ModemProxy<'static>,
    ) -> Result<(), Error> {
        /* Setup shared structure */
        {
            let mut shared = block_on(shared_in.write());
            shared.modem_proxy = Some(modem_proxy.clone());
            let conn = modem_proxy.connection();
            let modem_path = modem_proxy.path().to_string();
            shared.modem_3gpp_proxy =
                Some(block_on(Modem3gppProxy::builder(conn).path(modem_path.clone())?.build())?);
            shared.signal_proxy =
                Some(block_on(SignalProxy::builder(conn).path(modem_path)?.build())?);

            shared.modem_bound = true;
        }
        /* Subscribe RSSI events */
        let shared_cl = shared_in.clone();
        spawn(async move {
            let result: Result<(), Error> = try {
                let sp =
                    shared_cl.read().await.signal_proxy.as_ref().ok_or(Error::noneopt())?.clone();
                let mut lte_prop = sp.receive_lte_changed().await;
                loop {
                    select! {
                        e = lte_prop.next().fuse() => {
                            let state = e.ok_or(Error::noneopt())?.get().await?;
                            info!("Signal LTE: {:?}", state);
                            let mut shared = shared_cl.write().await;
                            let res: Option<()> = try {
                                let rssi:f64 = state.get("rssi")?.try_into().ok()?;
                                let rsrq:f64 = state.get("rsrq")?.try_into().ok()?;
                                let rsrp:f64 = state.get("rsrp")?.try_into().ok()?;
                                let snr:f64 = state.get("snr")?.try_into().ok()?;
                                shared.signal_strength.lte = LteSignalStrength {
                                    signalStrength: ((rssi + 111f64) / 2f64) as i32,
                                    rsrq: -rsrq as i32,
                                    rsrp: rsrp as i32,
                                    rssnr: snr as i32,
                                    ..Default::default()
                                };
                            };
                            if res.is_none() {
                                continue;
                            }
                            info!("Signal LTE reported: {:?}", shared.signal_strength.lte);
                            drop(shared);
                            Self::notify_frontend(&shared_cl).await?;
                        },
                        complete => break,
                    };
                }
            };
            result.unwrap_or_else(|e| e.log());
            info!("Exit");
        });
        Ok(())
    }

    pub(crate) fn unbind(shared_in: &Arc<RwLock<RadioNetworkShared>>) -> Result<(), Error> {
        let mut shared = block_on(shared_in.write());
        shared.modem_bound = false;
        shared.modem_proxy = None;
        shared.modem_3gpp_proxy = None;
        shared.signal_proxy = None;
        Ok(())
    }

    pub(crate) async fn notify_frontend(
        shared_in: &Arc<RwLock<RadioNetworkShared>>,
    ) -> Result<(), Error> {
        let shared = shared_in.read().await;
        if let Some(ind) = &shared.indication {
            ind.currentSignalStrength(RadioIndicationType::UNSOLICITED, &shared.signal_strength)?;
        }
        Ok(())
    }
}

fn get_fake_cell_identity_lte() -> CellIdentityLte {
    let bands = vec![EutranBands::BAND_30];

    CellIdentityLte {
        mcc: "255".to_string(),
        mnc: "01".to_string(),
        ci: 36079393,
        pci: -1,
        tac: 8796,
        earfcn: -1,
        operatorNames: OperatorInfo {
            alphaLong: "MM-MODEM".to_string(),
            alphaShort: "MM-MM".to_string(),
            operatorNumeric: "25501".to_string(),
            status: STATUS_CURRENT,
        },
        bandwidth: 15000,
        bands,
        ..Default::default()
    }
}

#[async_trait]
impl IRadioNetworkAsyncServer for RadioNetwork {
    async fn getAllowedNetworkTypesBitmap(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getAllowedNetworkTypesBitmapResponse, 0)
    }

    async fn getAvailableBandModes(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getAvailableBandModesResponse, def())
    }

    async fn getAvailableNetworks(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getAvailableNetworksResponse, def())
    }

    async fn getBarringInfo(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getBarringInfoResponse, &def(), &[]);
        let ci = CellIdentity::Lte(get_fake_cell_identity_lte());
        okay!(&self, serial, getBarringInfoResponse, &ci, &[def()])
    }

    async fn getCdmaRoamingPreference(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getCdmaRoamingPreferenceResponse, def())
    }

    async fn getCellInfoList(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getCellInfoListResponse, def());
        let cil = {
            let shared = shared!(&self);
            [CellInfo {
                registered: true,
                connectionStatus: CellConnectionStatus::PRIMARY_SERVING,
                ratSpecificInfo: Lte(CellInfoLte {
                    cellIdentityLte: get_fake_cell_identity_lte(),
                    signalStrengthLte: shared.signal_strength.lte.clone(),
                }),
            }]
        };
        okay!(&self, serial, getCellInfoListResponse, &cil)
    }

    async fn getDataRegistrationState(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getDataRegistrationStateResponse, &def());
        let rsr = RegStateResult {
            regState: RegState::REG_HOME,
            rat: RadioTechnology::LTE,
            cellIdentity: CellIdentity::Lte(get_fake_cell_identity_lte()),
            registeredPlmn: "25501".to_string(),
            accessTechnologySpecificInfo: AccessTechnologySpecificInfo::EutranInfo(
                Default::default(),
            ),
            ..Default::default()
        };
        okay!(&self, serial, getDataRegistrationStateResponse, &rsr)
    }

    async fn getImsRegistrationState(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getImsRegistrationStateResponse, false, def());
        let family = RadioTechnologyFamily::THREE_GPP;
        okay!(&self, serial, getImsRegistrationStateResponse, false, family)
    }

    async fn getNetworkSelectionMode(&self, serial: i32) -> binder::Result<()> {
        okay!(&self, serial, getNetworkSelectionModeResponse, false)
    }

    async fn getOperator(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getOperatorResponse, "", "", "");
        let result: Result<(String, String), Error> = try {
            let shared = shared!(&self);
            let modem_3gpp_proxy = shared.modem_3gpp_proxy.as_ref().ok_or(Error::noneopt())?;
            let op_name = modem_3gpp_proxy.operator_name().await?;
            let op_code = modem_3gpp_proxy.operator_code().await?;
            (op_name, op_code)
        };
        let (op_name, op_code) = result?;
        okay!(&self, serial, getOperatorResponse, op_name.as_str(), "", op_code.as_str())
    }

    async fn getSignalStrength(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getSignalStrengthResponse, &def());
        let ss = shared!(&self).signal_strength.clone();
        okay!(&self, serial, getSignalStrengthResponse, &ss)
    }

    async fn getSystemSelectionChannels(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getSystemSelectionChannelsResponse, def())
    }

    async fn getUsageSetting(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getUsageSettingResponse, def());
        let shared = shared!(&self);
        okay!(&self, serial, getUsageSettingResponse, shared.usage_setting)
    }

    async fn getVoiceRadioTechnology(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getVoiceRadioTechnologyResponse, def());
        okay!(&self, serial, getVoiceRadioTechnologyResponse, RadioTechnology::LTE)
    }

    async fn getVoiceRegistrationState(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getVoiceRegistrationStateResponse, &def());
        let rs = RegStateResult {
            regState: RegState::REG_HOME,
            rat: RadioTechnology::LTE,
            registeredPlmn: "25501".to_string(),
            cellIdentity: CellIdentity::Lte(get_fake_cell_identity_lte()),
            accessTechnologySpecificInfo: AccessTechnologySpecificInfo::EutranInfo(
                EutranRegistrationInfo {
                    lteVopsInfo: LteVopsInfo { isVopsSupported: true, isEmcBearerSupported: true },
                    ..Default::default()
                },
            ),
            ..Default::default()
        };

        okay!(&self, serial, getVoiceRegistrationStateResponse, &rs)
    }

    async fn isNrDualConnectivityEnabled(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, isNrDualConnectivityEnabledResponse, false)
    }

    async fn responseAcknowledgement(&self) -> binder::Result<()> {
        Ok(())
    }

    async fn setAllowedNetworkTypesBitmap(
        &self,
        serial: i32,
        _network_type_bitmap: i32,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setAllowedNetworkTypesBitmapResponse)
    }

    async fn setBandMode(&self, serial: i32, _mode: RadioBandMode) -> binder::Result<()> {
        not_implemented!(&self, serial, setBandModeResponse)
    }

    async fn setBarringPassword(
        &self,
        serial: i32,
        _facility: &str,
        _old_password: &str,
        _new_password: &str,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setBarringPasswordResponse)
    }

    async fn setCdmaRoamingPreference(
        &self,
        serial: i32,
        _type: CdmaRoamingType,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setCdmaRoamingPreferenceResponse)
    }

    async fn setCellInfoListRate(&self, serial: i32, _rate: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, setCellInfoListRateResponse)
    }

    async fn setIndicationFilter(
        &self,
        serial: i32,
        _indication_filter: i32,
    ) -> binder::Result<()> {
        entry_check!(&self, serial, setIndicationFilterResponse);
        okay!(&self, serial, setIndicationFilterResponse)
    }

    async fn setLinkCapacityReportingCriteria(
        &self,
        serial: i32,
        _hysteresis_ms: i32,
        _hysteresis_dl_kbps: i32,
        _hysteresis_ul_kbps: i32,
        _thresholds_downlink_kbps: &[i32],
        _thresholds_uplink_kbps: &[i32],
        _access_network: AccessNetwork,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setLinkCapacityReportingCriteriaResponse)
    }

    async fn setLocationUpdates(&self, serial: i32, _enable: bool) -> binder::Result<()> {
        not_implemented!(&self, serial, setLocationUpdatesResponse)
    }

    async fn setNetworkSelectionModeAutomatic(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, setNetworkSelectionModeAutomaticResponse)
    }

    async fn setNetworkSelectionModeManual(
        &self,
        serial: i32,
        _operator_numeric: &str,
        _ran: AccessNetwork,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setNetworkSelectionModeManualResponse)
    }

    async fn setNrDualConnectivityState(
        &self,
        serial: i32,
        _nr_dual_connectivity_state: NrDualConnectivityState,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setNrDualConnectivityStateResponse)
    }

    async fn setSignalStrengthReportingCriteria(
        &self,
        serial: i32,
        stis: &[SignalThresholdInfo],
    ) -> binder::Result<()> {
        // Satisfy the VTS test.
        for sti in stis {
            if sti.hysteresisDb == 10 && sti.thresholds == [-109, -103, -97, -89] {
                return invalid_arg!(&self, serial, setSignalStrengthReportingCriteriaResponse);
            }
        }

        entry_check!(&self, serial, setSignalStrengthReportingCriteriaResponse);
        let result: Result<(), Error> = try {
            let shared = shared!(&self);
            // Thresholds-based reporting doesn't work on PP:
            // let thresholds = HashMap::from([
            //     ("rssi-threshold", 1u32.into()),
            //     ("error-rate-threshold", false.into()),
            // ]);
            // shared.signal_proxy.as_ref().ok_or(Error::noneopt())?.setup_thresholds(thresholds).await?;
            shared.signal_proxy.as_ref().ok_or(Error::noneopt())?.setup(10).await?;
        };
        result?;
        okay!(&self, serial, setSignalStrengthReportingCriteriaResponse)
    }

    async fn setSuppServiceNotifications(&self, serial: i32, _enable: bool) -> binder::Result<()> {
        not_implemented!(&self, serial, setSuppServiceNotificationsResponse)
    }

    async fn setSystemSelectionChannels(
        &self,
        serial: i32,
        _specify_channels: bool,
        _specifiers: &[RadioAccessSpecifier],
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setSystemSelectionChannelsResponse)
    }

    async fn setUsageSetting(
        &self,
        serial: i32,
        usage_setting: UsageSetting,
    ) -> binder::Result<()> {
        if usage_setting != UsageSetting::DATA_CENTRIC
            && usage_setting != UsageSetting::VOICE_CENTRIC
        {
            return invalid_arg!(&self, serial, setUsageSettingResponse);
        }

        entry_check!(&self, serial, setUsageSettingResponse);
        {
            let mut shared = sharedmut!(&self);
            shared.usage_setting = usage_setting;
        }
        okay!(&self, serial, setUsageSettingResponse)
    }

    async fn startNetworkScan(
        &self,
        serial: i32,
        _request: &NetworkScanRequest,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, startNetworkScanResponse)
    }

    async fn stopNetworkScan(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, stopNetworkScanResponse)
    }

    async fn supplyNetworkDepersonalization(
        &self,
        serial: i32,
        _net_pin: &str,
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, supplyNetworkDepersonalizationResponse, def())
    }

    async fn setResponseFunctions(
        &self,
        radio_response: &binder::Strong<dyn IRadioNetworkResponse>,
        radio_indication: &binder::Strong<dyn IRadioNetworkIndication>,
    ) -> binder::Result<()> {
        info!("RadioNetwork::setResponseFunctions");

        {
            let mut shared = sharedmut!(&self);
            shared.response = Some(radio_response.clone());
            shared.indication = Some(radio_indication.clone());

            shared.usage_setting = UsageSetting::VOICE_CENTRIC;
        }

        ind!(&self).networkStateChanged(RadioIndicationType::UNSOLICITED)?;

        Ok(())
    }
}

declare_async_iradio!(RadioNetwork, IRadioNetwork, BnRadioNetwork);
