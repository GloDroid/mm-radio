/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use crate::{
    mm_zbus::{mm_modem_3gpp_proxy::Modem3gppProxy, mm_modem_proxy::ModemProxy},
    utils::iradio::{
        declare_async_iradio, def, entry_check, ind, not_implemented, okay, shared, sharedmut,
    },
};
use android_hardware_radio::aidl::android::hardware::radio::{
    AccessNetwork::*, RadioIndicationType::*, RadioTechnology::*, RadioTechnologyFamily::*,
};
use android_hardware_radio_network::aidl::android::hardware::radio::network::{
    AccessTechnologySpecificInfo::*, CdmaRoamingType::*, CdmaSignalStrength::*,
    CellConnectionStatus::*, CellIdentity::*, CellIdentityLte::*, CellInfo::*, CellInfoLte::*,
    CellInfoRatSpecificInfo::CellInfoRatSpecificInfo::Lte, EutranBands::*,
    EutranRegistrationInfo::*, EvdoSignalStrength::*, GsmSignalStrength::*, IRadioNetwork::*,
    IRadioNetworkIndication::*, IRadioNetworkResponse::*, LteSignalStrength::*, LteVopsInfo::*,
    NetworkScanRequest::*, NrDualConnectivityState::*, NrSignalStrength::*, OperatorInfo::*,
    RadioAccessSpecifier::*, RadioBandMode::*, RegState::*, RegStateResult::*, SignalStrength::*,
    SignalThresholdInfo::*, TdscdmaSignalStrength::*, UsageSetting::*, WcdmaSignalStrength::*,
};

use async_std::{sync::RwLock, task::block_on};
use binder::{BinderFeatures, Strong};
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

    usage_setting: UsageSetting,
}

#[derive(Default)]
pub struct RadioNetwork {
    pub shared: Arc<RwLock<RadioNetworkShared>>,
}

impl RadioNetworkShared {
    pub fn bind(shared_in: &Arc<RwLock<RadioNetworkShared>>, modem_proxy: &ModemProxy<'static>) {
        /* Setup shared structure */
        {
            let mut shared = block_on(shared_in.write());
            shared.modem_proxy = Some(modem_proxy.clone());
            let conn = modem_proxy.connection();
            let modem_path = modem_proxy.path().to_string();
            shared.modem_3gpp_proxy = Some(
                block_on(Modem3gppProxy::builder(conn).path(modem_path).unwrap().build()).unwrap(),
            );
            shared.modem_bound = true;
        }
    }

    pub fn unbind(shared_in: &Arc<RwLock<RadioNetworkShared>>) {
        let mut shared = block_on(shared_in.write());
        shared.modem_bound = false;
        shared.modem_proxy = None;
        shared.modem_3gpp_proxy = None;
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
        not_implemented!(&self, serial, getBarringInfoResponse, &def(), &[])
    }

    async fn getCdmaRoamingPreference(&self, serial: i32) -> binder::Result<()> {
        not_implemented!(&self, serial, getCdmaRoamingPreferenceResponse, def())
    }

    async fn getCellInfoList(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getCellInfoListResponse, def());
        let cil = [CellInfo {
            registered: true,
            connectionStatus: CellConnectionStatus::PRIMARY_SERVING,
            ratSpecificInfo: Lte(CellInfoLte {
                cellIdentityLte: get_fake_cell_identity_lte(),
                signalStrengthLte: LteSignalStrength {
                    signalStrength: 14,
                    rsrp: 16,
                    rsrq: 4,
                    rssnr: 2,
                    cqi: std::i32::MAX,
                    timingAdvance: std::i32::MAX,
                    cqiTableIndex: std::i32::MAX,
                },
            }),
        }];
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
        let shared = shared!(&self);
        let modem_3gpp_proxy = shared.modem_3gpp_proxy.as_ref().unwrap();
        let op_name = modem_3gpp_proxy.operator_name().await.unwrap();
        let op_code = modem_3gpp_proxy.operator_code().await.unwrap();
        okay!(&self, serial, getOperatorResponse, op_name.as_str(), "", op_code.as_str())
    }

    async fn getSignalStrength(&self, serial: i32) -> binder::Result<()> {
        entry_check!(&self, serial, getSignalStrengthResponse, &def());
        let ss = SignalStrength {
            gsm: GsmSignalStrength {
                timingAdvance: std::i32::MAX,
                signalStrength: std::i32::MAX,
                bitErrorRate: std::i32::MAX,
            },
            cdma: CdmaSignalStrength { dbm: std::i32::MAX, ecio: std::i32::MAX },
            evdo: EvdoSignalStrength {
                dbm: std::i32::MAX,
                ecio: std::i32::MAX,
                signalNoiseRatio: std::i32::MAX,
            },
            lte: LteSignalStrength {
                signalStrength: 14,
                rsrp: 16,
                rsrq: 4,
                rssnr: 2,
                cqi: std::i32::MAX,
                timingAdvance: std::i32::MAX,
                cqiTableIndex: std::i32::MAX,
            },
            tdscdma: TdscdmaSignalStrength {
                bitErrorRate: std::i32::MAX,
                signalStrength: std::i32::MAX,
                rscp: std::i32::MAX,
            },
            wcdma: WcdmaSignalStrength {
                ecno: std::i32::MAX,
                rscp: std::i32::MAX,
                signalStrength: std::i32::MAX,
                bitErrorRate: std::i32::MAX,
            },
            nr: NrSignalStrength {
                csiCqiTableIndex: std::i32::MAX,
                csiRsrp: std::i32::MAX,
                csiRsrq: std::i32::MAX,
                csiSinr: std::i32::MAX,
                ssRsrp: std::i32::MAX,
                ssRsrq: std::i32::MAX,
                ssSinr: std::i32::MAX,
                ..Default::default()
            },
        };
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
        not_implemented!(&self, serial, setIndicationFilterResponse)
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
        _signal_threshold_infos: &[SignalThresholdInfo],
    ) -> binder::Result<()> {
        not_implemented!(&self, serial, setSignalStrengthReportingCriteriaResponse)
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
        }

        ind!(&self).networkStateChanged(RadioIndicationType::UNSOLICITED)?;

        Ok(())
    }
}

declare_async_iradio!(RadioNetwork, IRadioNetwork, BnRadioNetwork);
