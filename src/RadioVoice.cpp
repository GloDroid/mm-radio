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

#include "RadioVoice.h"

#include "debug.h"

#define RADIO_MODULE "Voice"

namespace android::hardware::radio::mm {

using ::ndk::ScopedAStatus;
namespace aidl = ::aidl::android::hardware::radio::voice;
constexpr auto ok = &ScopedAStatus::ok;

ScopedAStatus RadioVoice::acceptCall(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::cancelPendingUssd(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::conference(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::dial(int32_t serial, const aidl::Dial& dialInfo) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::emergencyDial(  //
        int32_t serial, const aidl::Dial& info, int32_t categories,
        const std::vector<std::string>& urns, aidl::EmergencyCallRouting routing,
        bool knownUserIntentEmerg, bool isTesting) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::exitEmergencyCallbackMode(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::explicitCallTransfer(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::getCallForwardStatus(int32_t serial,
                                               const aidl::CallForwardInfo& callInfo) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::getCallWaiting(int32_t serial, int32_t serviceClass) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::getClip(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::getClir(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::getCurrentCalls(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::getLastCallFailCause(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::getMute(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::getPreferredVoicePrivacy(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::getTtyMode(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::handleStkCallSetupRequestFromSim(int32_t serial, bool accept) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::hangup(int32_t serial, int32_t gsmIndex) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::hangupForegroundResumeBackground(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::hangupWaitingOrBackground(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::isVoNrEnabled(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::rejectCall(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::responseAcknowledgement() {
    LOG_UNIMPLEMENTED;
    return ok();
}

ScopedAStatus RadioVoice::sendBurstDtmf(int32_t serial, const std::string& dtmf, int32_t on,
                                        int32_t off) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::sendCdmaFeatureCode(int32_t serial, const std::string& featureCode) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::sendDtmf(int32_t serial, const std::string& s) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::sendUssd(int32_t serial, const std::string& ussd) {
    LOG_UNIMPLEMENTED << serial << ' ' << ussd;
    return ok();
}

ScopedAStatus RadioVoice::separateConnection(int32_t serial, int32_t gsmIndex) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::setCallForward(int32_t serial, const aidl::CallForwardInfo& callInfo) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::setCallWaiting(int32_t serial, bool enable, int32_t serviceClass) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::setClir(int32_t serial, int32_t status) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::setMute(int32_t serial, bool enable) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::setPreferredVoicePrivacy(int32_t serial, bool enable) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::setResponseFunctions(
        const std::shared_ptr<aidl::IRadioVoiceResponse>& response,
        const std::shared_ptr<aidl::IRadioVoiceIndication>& indication) {
    LOG_CALL << response << ' ' << indication;

    mResponse = response;
    mIndication = indication;

    return ok();
}

ScopedAStatus RadioVoice::setTtyMode(int32_t serial, aidl::TtyMode mode) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ndk::ScopedAStatus RadioVoice::setVoNrEnabled(int32_t serial, [[maybe_unused]] bool enable) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::startDtmf(int32_t serial, const std::string& s) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::stopDtmf(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioVoice::switchWaitingOrHoldingAndActive(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

}  // namespace android::hardware::radio::mm
