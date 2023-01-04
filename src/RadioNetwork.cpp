/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * Copyright (C) 2021 The Android Open Source Project
 * Copyright (C) 2022 GloDroid project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "RadioNetwork.h"

#include <thread>

#include <unistd.h>

#include "debug.h"

#define RADIO_MODULE "Network"

namespace android::hardware::radio::mm {

using ::aidl::android::hardware::radio::AccessNetwork;
using ::aidl::android::hardware::radio::RadioIndicationType;
using ::aidl::android::hardware::radio::RadioTechnology;
using ::aidl::android::hardware::radio::RadioTechnologyFamily;
using ::ndk::ScopedAStatus;
namespace aidl = ::aidl::android::hardware::radio::network;
constexpr auto ok = &ScopedAStatus::ok;

ScopedAStatus RadioNetwork::getAllowedNetworkTypesBitmap(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getAllowedNetworkTypesBitmapResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioNetwork::getAvailableBandModes(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getAvailableBandModesResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioNetwork::getAvailableNetworks(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getAvailableNetworksResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioNetwork::getBarringInfo(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getBarringInfoResponse(notSupported(serial), {}, {});
    return ok();
}

ScopedAStatus RadioNetwork::getCdmaRoamingPreference(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getCdmaRoamingPreferenceResponse(notSupported(serial), {});
    return ok();
}

static auto getFakeCellIdentityLte() {
    auto bands = std::vector<aidl::EutranBands>();
    bands.emplace_back(aidl::EutranBands(1000));

    return (aidl::CellIdentityLte){
            .mcc = "255",
            .mnc = "01",
            .ci = 36079393,
            .pci = -1,
            .tac = 8796,
            .earfcn = -1,
            .operatorNames =
                    (aidl::OperatorInfo){
                            .alphaLong = "MM-MODEM",
                            .alphaShort = "MM-MM",
                            .operatorNumeric = "25501",
                            .status = aidl::OperatorInfo::STATUS_CURRENT,
                    },
            .bandwidth = 15000,
            .bands = bands,
    };
}

ScopedAStatus RadioNetwork::getCellInfoList(int32_t serial) {
    LOG_STUB << serial;
    auto cellInfoList = std::vector<aidl::CellInfo>();
    auto cellInfo = (aidl::CellInfo){
            .registered = true,
            .connectionStatus = aidl::CellConnectionStatus::PRIMARY_SERVING,
            .ratSpecificInfo =
                    (aidl::CellInfoLte){
                            .cellIdentityLte = getFakeCellIdentityLte(),
                            .signalStrengthLte =
                                    (aidl::LteSignalStrength){
                                            .signalStrength = 14,
                                            .rsrp = 16,
                                            .rsrq = 4,
                                            .rssnr = 2,
                                            .cqi = INT_MAX,
                                            .timingAdvance = INT_MAX,
                                            .cqiTableIndex = INT_MAX,
                                    },
                    },
    };

    cellInfoList.emplace_back(cellInfo);
    mResponse->getCellInfoListResponse(okay(serial), cellInfoList);
    return ok();
}

ScopedAStatus RadioNetwork::getDataRegistrationState(int32_t serial) {
    LOG_STUB << serial;
    auto result = (aidl::RegStateResult){
            .regState = aidl::RegState::REG_HOME,
            .rat = RadioTechnology::LTE,
            .cellIdentity = getFakeCellIdentityLte(),
            .registeredPlmn = "25501",
            .accessTechnologySpecificInfo = (aidl::EutranRegistrationInfo){},
    };
    mResponse->getDataRegistrationStateResponse(okay(serial), result);
    return ok();
}

ScopedAStatus RadioNetwork::getImsRegistrationState(int32_t serial) {
    LOG_STUB << serial;
    mResponse->getImsRegistrationStateResponse(okay(serial), false,
                                               RadioTechnologyFamily::THREE_GPP2);
    return ok();
}

ScopedAStatus RadioNetwork::getNetworkSelectionMode(int32_t serial) {
    LOG_STUB << serial;
    mResponse->getNetworkSelectionModeResponse(okay(serial), false);
    return ok();
}

ScopedAStatus RadioNetwork::getOperator(int32_t serial) {
    LOG_STUB << serial;
    mResponse->getOperatorResponse(okay(serial), "MM-MODEM", "MM-MM", {});
    return ok();
}

ScopedAStatus RadioNetwork::getSignalStrength(int32_t serial) {
    LOG_STUB << serial;
    auto strength = (aidl::SignalStrength){
            .gsm =
                    (aidl::GsmSignalStrength){
                            .signalStrength = INT_MAX,
                            .bitErrorRate = INT_MAX,
                            .timingAdvance = INT_MAX,
                    },
            .cdma =
                    (aidl::CdmaSignalStrength){
                            .dbm = INT_MAX,
                            .ecio = INT_MAX,
                    },
            .evdo =
                    (aidl::EvdoSignalStrength){
                            .dbm = INT_MAX,
                            .ecio = INT_MAX,
                            .signalNoiseRatio = INT_MAX,
                    },
            .lte =
                    (aidl::LteSignalStrength){
                            .signalStrength = 14,
                            .rsrp = 16,
                            .rsrq = 4,
                            .rssnr = 2,
                            .cqi = INT_MAX,
                            .timingAdvance = INT_MAX,
                            .cqiTableIndex = INT_MAX,
                    },
            .tdscdma =
                    (aidl::TdscdmaSignalStrength){
                            .signalStrength = INT_MAX,
                            .bitErrorRate = INT_MAX,
                            .rscp = INT_MAX,
                    },
            .wcdma =
                    (aidl::WcdmaSignalStrength){
                            .signalStrength = INT_MAX,
                            .bitErrorRate = INT_MAX,
                            .rscp = INT_MAX,
                            .ecno = INT_MAX,
                    },
            .nr =
                    (aidl::NrSignalStrength){
                            .ssRsrp = INT_MAX,
                            .ssRsrq = INT_MAX,
                            .ssSinr = INT_MAX,
                            .csiRsrp = INT_MAX,
                            .csiRsrq = INT_MAX,
                            .csiSinr = INT_MAX,
                            .csiCqiTableIndex = INT_MAX,
                    },
    };

    mResponse->getSignalStrengthResponse(okay(serial), strength);
    return ok();
}

ScopedAStatus RadioNetwork::getSystemSelectionChannels(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getSystemSelectionChannelsResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioNetwork::getVoiceRadioTechnology(int32_t serial) {
    LOG_STUB << serial;
    mResponse->getVoiceRadioTechnologyResponse(okay(serial), RadioTechnology::LTE);
    return ok();
}

ScopedAStatus RadioNetwork::getVoiceRegistrationState(int32_t serial) {
    LOG_STUB << serial;
    auto result = (aidl::RegStateResult){
            .regState = aidl::RegState::REG_HOME,
            .rat = RadioTechnology::LTE,
            .cellIdentity = getFakeCellIdentityLte(),
            .registeredPlmn = "25501",
            .accessTechnologySpecificInfo =
                    (aidl::EutranRegistrationInfo){
                            .lteVopsInfo =
                                    (aidl::LteVopsInfo){
                                            .isVopsSupported = true,
                                            .isEmcBearerSupported = true,
                                    },
                    },
    };
    mResponse->getVoiceRegistrationStateResponse(okay(serial), result);
    return ok();
}

ScopedAStatus RadioNetwork::isNrDualConnectivityEnabled(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->isNrDualConnectivityEnabledResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioNetwork::responseAcknowledgement() {
    LOG_UNIMPLEMENTED;
    return ok();
}

ScopedAStatus RadioNetwork::setAllowedNetworkTypesBitmap(int32_t serial, int32_t /*ntype*/) {
    LOG_STUB << serial;
    mResponse->setAllowedNetworkTypesBitmapResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioNetwork::setBandMode(int32_t serial, aidl::RadioBandMode /*mode*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setBandModeResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioNetwork::setBarringPassword(int32_t serial, const std::string& /*facility*/,
                                               const std::string& /*oldPw*/,
                                               const std::string& /*newPw*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setBarringPasswordResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioNetwork::setCdmaRoamingPreference(int32_t serial,
                                                     aidl::CdmaRoamingType /*type*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setCdmaRoamingPreferenceResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioNetwork::setCellInfoListRate(int32_t serial, int32_t /*rate*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setCellInfoListRateResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioNetwork::setIndicationFilter(int32_t serial, int32_t /*indFilter*/) {
    LOG_STUB << serial;
    mResponse->setIndicationFilterResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioNetwork::setLinkCapacityReportingCriteria(  //
        int32_t serial, int32_t /*hysteresisMs*/, int32_t /*hysteresisDlKbps*/,
        int32_t /*hysteresisUlKbps*/, const std::vector<int32_t>& /*thrDownlinkKbps*/,
        const std::vector<int32_t>& /*thrUplinkKbps*/, AccessNetwork /*accessNetwork*/) {
    LOG_STUB << serial;
    mResponse->setLinkCapacityReportingCriteriaResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioNetwork::setLocationUpdates(int32_t serial, bool /*enable*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setLocationUpdatesResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioNetwork::setNetworkSelectionModeAutomatic(int32_t serial) {
    LOG_STUB << serial;
    mResponse->setNetworkSelectionModeAutomaticResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioNetwork::setNetworkSelectionModeManual(  //
        int32_t serial, const std::string& /*opNumeric*/, AccessNetwork /*ran*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setNetworkSelectionModeManualResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioNetwork::setNrDualConnectivityState(int32_t serial,
                                                       aidl::NrDualConnectivityState /*st*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setNrDualConnectivityStateResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioNetwork::setResponseFunctions(
        const std::shared_ptr<aidl::IRadioNetworkResponse>& response,
        const std::shared_ptr<aidl::IRadioNetworkIndication>& indication) {
    LOG_STUB << response << ' ' << indication;

    mResponse = response;
    mIndication = indication;

    mIndication->networkStateChanged(RadioIndicationType::UNSOLICITED);

    return ok();
}

ScopedAStatus RadioNetwork::setSignalStrengthReportingCriteria(
        int32_t serial, const std::vector<aidl::SignalThresholdInfo>& infos) {
    LOG_STUB << serial;
    auto resp = okay(serial);
    if (infos.empty()) resp.error = RadioError::INVALID_ARGUMENTS;
    mResponse->setSignalStrengthReportingCriteriaResponse(resp);
    return ok();
}

ScopedAStatus RadioNetwork::setSuppServiceNotifications(int32_t serial, bool /*enable*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setSuppServiceNotificationsResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioNetwork::setSystemSelectionChannels(  //
        int32_t serial, bool /*specifyCh*/,
        const std::vector<aidl::RadioAccessSpecifier>& /*specifiers*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setSystemSelectionChannelsResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioNetwork::startNetworkScan(int32_t serial,
                                             const aidl::NetworkScanRequest& /*req*/) {
    LOG_UNIMPLEMENTED << serial;
    auto resp = okay(serial);
    resp.error = RadioError::REQUEST_NOT_SUPPORTED;
    mResponse->startNetworkScanResponse(resp);
    return ok();
}

ScopedAStatus RadioNetwork::stopNetworkScan(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->stopNetworkScanResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioNetwork::supplyNetworkDepersonalization(int32_t serial,
                                                           const std::string& /*nPin*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->supplyNetworkDepersonalizationResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioNetwork::setUsageSetting(int32_t serial, aidl::UsageSetting usageSetting) {
    LOG_UNIMPLEMENTED << serial;
    state.usageSetting = usageSetting;
    mResponse->setUsageSettingResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioNetwork::getUsageSetting(int32_t serial) {
    LOG_STUB << serial;
    mResponse->getUsageSettingResponse(okay(serial), state.usageSetting);
    return ok();
}

}  // namespace android::hardware::radio::mm
