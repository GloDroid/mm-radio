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

#include "debug.h"

#define RADIO_MODULE "Network"

namespace android::hardware::radio::mm {

using ::aidl::android::hardware::radio::AccessNetwork;
using ::ndk::ScopedAStatus;
namespace aidl = ::aidl::android::hardware::radio::network;
constexpr auto ok = &ScopedAStatus::ok;

ScopedAStatus RadioNetwork::getAllowedNetworkTypesBitmap(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getAvailableBandModes(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getAvailableNetworks(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getBarringInfo(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getCdmaRoamingPreference(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getCellInfoList(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getDataRegistrationState(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getImsRegistrationState(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getNetworkSelectionMode(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getOperator(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getSignalStrength(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getSystemSelectionChannels(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getVoiceRadioTechnology(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getVoiceRegistrationState(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::isNrDualConnectivityEnabled(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::responseAcknowledgement() {
    LOG_UNIMPLEMENTED;
    return ok();
}

ScopedAStatus RadioNetwork::setAllowedNetworkTypesBitmap(int32_t serial, int32_t ntype) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::setBandMode(int32_t serial, aidl::RadioBandMode mode) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::setBarringPassword(int32_t serial, const std::string& facility,
                                               const std::string& oldPw, const std::string& newPw) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::setCdmaRoamingPreference(int32_t serial, aidl::CdmaRoamingType type) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::setCellInfoListRate(int32_t serial, int32_t rate) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::setIndicationFilter(int32_t serial, int32_t indFilter) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::setLinkCapacityReportingCriteria(  //
        int32_t serial, int32_t hysteresisMs, int32_t hysteresisDlKbps, int32_t hysteresisUlKbps,
        const std::vector<int32_t>& thrDownlinkKbps, const std::vector<int32_t>& thrUplinkKbps,
        AccessNetwork accessNetwork) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::setLocationUpdates(int32_t serial, bool enable) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::setNetworkSelectionModeAutomatic(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::setNetworkSelectionModeManual(  //
        int32_t serial, const std::string& opNumeric, AccessNetwork ran) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::setNrDualConnectivityState(int32_t serial,
                                                       aidl::NrDualConnectivityState st) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::setResponseFunctions(
        const std::shared_ptr<aidl::IRadioNetworkResponse>& response,
        const std::shared_ptr<aidl::IRadioNetworkIndication>& indication) {
    LOG_CALL << response << ' ' << indication;

    mResponse = response;
    mIndication = indication;

    return ok();
}

ScopedAStatus RadioNetwork::setSignalStrengthReportingCriteria(
        int32_t serial, const std::vector<aidl::SignalThresholdInfo>& infos) {
    LOG_UNIMPLEMENTED << serial;
    if (infos.size() == 0) {
        LOG(ERROR) << "Threshold info array is empty - dropping setSignalStrengthReportingCriteria";
        return ok();
    }
    return ok();
}

ScopedAStatus RadioNetwork::setSuppServiceNotifications(int32_t serial, bool enable) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::setSystemSelectionChannels(  //
        int32_t serial, bool specifyCh, const std::vector<aidl::RadioAccessSpecifier>& specifiers) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::startNetworkScan(int32_t serial, const aidl::NetworkScanRequest& req) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::stopNetworkScan(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::supplyNetworkDepersonalization(int32_t ser, const std::string& nPin) {
    LOG_UNIMPLEMENTED << ser;
    return ok();
}

ScopedAStatus RadioNetwork::setUsageSetting(int32_t serial, aidl::UsageSetting) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioNetwork::getUsageSetting(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

}  // namespace android::hardware::radio::mm
