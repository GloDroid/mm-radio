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

#include "RadioSim.h"

#include "debug.h"

#define RADIO_MODULE "Sim"

namespace android::hardware::radio::mm {

using ::ndk::ScopedAStatus;
namespace aidl = ::aidl::android::hardware::radio::sim;
constexpr auto ok = &ScopedAStatus::ok;

ScopedAStatus RadioSim::areUiccApplicationsEnabled(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::changeIccPin2ForApp(int32_t serial, const std::string& oldPin2,
                                            const std::string& newPin2, const std::string& aid) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::changeIccPinForApp(int32_t serial, const std::string& oldPin,
                                           const std::string& newPin, const std::string& aid) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::enableUiccApplications(int32_t serial, bool enable) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::getAllowedCarriers(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::getCdmaSubscription(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::getCdmaSubscriptionSource(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::getFacilityLockForApp(  //
        int32_t serial, const std::string& facility, const std::string& password,
        int32_t serviceClass, const std::string& appId) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::getIccCardStatus(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::getImsiForApp(int32_t serial, const std::string& aid) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::getSimPhonebookCapacity(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::getSimPhonebookRecords(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::iccCloseLogicalChannel(int32_t serial, int32_t channelId) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::iccIoForApp(int32_t serial, const aidl::IccIo& iccIo) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::iccOpenLogicalChannel(int32_t serial, const std::string& aid, int32_t p2) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::iccTransmitApduBasicChannel(int32_t serial, const aidl::SimApdu& message) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::iccTransmitApduLogicalChannel(int32_t serial,
                                                      const aidl::SimApdu& message) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::reportStkServiceIsRunning(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::requestIccSimAuthentication(  //
        int32_t serial, int32_t authContext, const std::string& authData, const std::string& aid) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::responseAcknowledgement() {
    LOG_UNIMPLEMENTED;
    return ok();
}

ScopedAStatus RadioSim::sendEnvelope(int32_t serial, const std::string& command) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::sendEnvelopeWithStatus(int32_t serial, const std::string& contents) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::sendTerminalResponseToSim(int32_t serial,
                                                  const std::string& commandResponse) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::setAllowedCarriers(  //
        int32_t serial, const aidl::CarrierRestrictions& carriers, aidl::SimLockMultiSimPolicy mp) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::setCarrierInfoForImsiEncryption(
        int32_t serial, const aidl::ImsiEncryptionInfo& imsiEncryptionInfo) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::setCdmaSubscriptionSource(int32_t serial,
                                                  aidl::CdmaSubscriptionSource cdmaSub) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::setFacilityLockForApp(  //
        int32_t serial, const std::string& facility, bool lockState, const std::string& password,
        int32_t serviceClass, const std::string& appId) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::setResponseFunctions(
        const std::shared_ptr<aidl::IRadioSimResponse>& response,
        const std::shared_ptr<aidl::IRadioSimIndication>& indication) {
    LOG_CALL << response << ' ' << indication;

    mResponse = response;
    mIndication = indication;

    return ok();
}

ScopedAStatus RadioSim::setSimCardPower(int32_t serial, aidl::CardPowerState powerUp) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::setUiccSubscription(int32_t serial, const aidl::SelectUiccSub& uiccSub) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::supplyIccPin2ForApp(int32_t serial, const std::string& pin2,
                                            const std::string& aid) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::supplyIccPinForApp(int32_t serial, const std::string& pin,
                                           const std::string& aid) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::supplyIccPuk2ForApp(int32_t serial, const std::string& puk2,
                                            const std::string& pin2, const std::string& aid) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::supplyIccPukForApp(int32_t serial, const std::string& puk,
                                           const std::string& pin, const std::string& aid) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::supplySimDepersonalization(int32_t serial, aidl::PersoSubstate pss,
                                                   const std::string& controlKey) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioSim::updateSimPhonebookRecords(int32_t serial,
                                                  const aidl::PhonebookRecordInfo& recordInfo) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

}  // namespace android::hardware::radio::mm
