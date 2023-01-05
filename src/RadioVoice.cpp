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

using ::aidl::android::hardware::radio::RadioIndicationType;
using ::ndk::ScopedAStatus;
namespace aidl = ::aidl::android::hardware::radio::voice;
constexpr auto ok = &ScopedAStatus::ok;

static std::optional<int> mmToAidlState(int mmState) {
    switch (mmState) {
        case MM_CALL_STATE_ACTIVE:
            return aidl::Call::STATE_ACTIVE;
        case MM_CALL_STATE_DIALING:
            return aidl::Call::STATE_DIALING;
        case MM_CALL_STATE_RINGING_IN:
            return aidl::Call::STATE_INCOMING;
        case MM_CALL_STATE_RINGING_OUT:
            return aidl::Call::STATE_ALERTING;
        case MM_CALL_STATE_HELD:
            return aidl::Call::STATE_HOLDING;
        case MM_CALL_STATE_WAITING:
            return aidl::Call::STATE_WAITING;
        default:
            return {};
    }
}

ScopedAStatus RadioVoice::acceptCall(int32_t serial) {
    LOG_CALL << serial;

    if (!mModemVoice) {
        mResponse->acceptCallResponse(error(serial, RadioError::SIM_ABSENT));
        return ok();
    }

    auto call = mModemVoice->getIncomingCall();
    if (!call) {
        mResponse->acceptCallResponse(error(serial, RadioError::INVALID_STATE));
        return ok();
    }

    call->accept();

    mResponse->acceptCallResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioVoice::cancelPendingUssd(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->cancelPendingUssdResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::conference(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->conferenceResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::dial(int32_t serial, const aidl::Dial& dialInfo) {
    LOG_CALL << serial;
    if (!mModemVoice) {
        mResponse->dialResponse(error(serial, RadioError::SIM_ABSENT));
        return ok();
    }

    auto call = mModemVoice->createCall(dialInfo.address);
    call->start();
    mResponse->dialResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioVoice::emergencyDial(  //
        int32_t serial, const aidl::Dial& /*info*/, int32_t /*categories*/,
        const std::vector<std::string>& /*urns*/, aidl::EmergencyCallRouting /*routing*/,
        bool /*knownUserIntentEmerg*/, bool /*isTesting*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->emergencyDialResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioVoice::exitEmergencyCallbackMode(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->exitEmergencyCallbackModeResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::explicitCallTransfer(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->explicitCallTransferResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::getCallForwardStatus(int32_t serial,
                                               const aidl::CallForwardInfo& /*callInfo*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getCallForwardStatusResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioVoice::getCallWaiting(int32_t serial, int32_t /*serviceClass*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getCallWaitingResponse(notSupported(serial), {}, {});
    return ok();
}

ScopedAStatus RadioVoice::getClip(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getClipResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioVoice::getClir(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getClirResponse(notSupported(serial), {}, {});
    return ok();
}

ScopedAStatus RadioVoice::getCurrentCalls(int32_t serial) {
    LOG_CALL << serial;
    if (!mModemVoice) {
        mResponse->getCurrentCallsResponse(error(serial, RadioError::SIM_ABSENT), {});
        return ok();
    }

    auto outCalls = std::vector<aidl::Call>();

    auto calls = mModemVoice->getNotTerminatedCallsList();
    for (auto& call : calls) {
        call->dump();
        auto state = mmToAidlState(call->getState());
        if (!state) {
            LOG(ERROR) << "Unknown call state: " << call->getState();
            continue;
        }

        auto outCall = (aidl::Call){
                .state = *state,
                .index = call->getIndex() + 1,
                .toa = {},
                .isMpty = call->isMultiparty(),
                .isMT = call->getDirection() == MM_CALL_DIRECTION_INCOMING,
                .als = {},
                .isVoice = true,
                .isVoicePrivacy = false,
                .number = call->getNumber(),
                .numberPresentation = aidl::Call::PRESENTATION_ALLOWED,
                .name = "",
                .namePresentation = aidl::Call::PRESENTATION_ALLOWED,
                .uusInfo = {},
                .audioQuality = {},
        };

        LOG(INFO) << "Call " << outCall.index << ": " << outCall.number;

        outCalls.emplace_back(outCall);
    };

    mResponse->getCurrentCallsResponse(okay(serial), outCalls);
    return ok();
}

ScopedAStatus RadioVoice::getLastCallFailCause(int32_t serial) {
    LOG_STUB << serial;
    mResponse->getLastCallFailCauseResponse(
            okay(serial), {aidl::LastCallFailCause::OUT_OF_SERVICE, "Calls not supported"});
    return ok();
}

ScopedAStatus RadioVoice::getMute(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getMuteResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioVoice::getPreferredVoicePrivacy(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getPreferredVoicePrivacyResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioVoice::getTtyMode(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getTtyModeResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioVoice::handleStkCallSetupRequestFromSim(int32_t serial, bool /*accept*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->handleStkCallSetupRequestFromSimResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::hangup(int32_t serial, int32_t /*gsmIndex*/) {
    LOG_CALL << serial;
    if (!mModemVoice) {
        mResponse->hangupConnectionResponse(error(serial, RadioError::SIM_ABSENT));
        return ok();
    }

    auto call = mModemVoice->getActiveCall();
    if (!call) {
        mResponse->hangupConnectionResponse(error(serial, RadioError::INVALID_STATE));
        return ok();
    }

    call->hangup();
    mResponse->hangupConnectionResponse(okay(serial));

    return ok();
}

ScopedAStatus RadioVoice::hangupForegroundResumeBackground(int32_t serial) {
    LOG_CALL << serial;
    if (!mModemVoice) {
        mResponse->hangupForegroundResumeBackgroundResponse(error(serial, RadioError::SIM_ABSENT));
        return ok();
    }

    auto calls = mModemVoice->getNotTerminatedCallsList();
    for (auto& call : calls) {
        if (call->getState() != MM_CALL_STATE_ACTIVE) {
            call->hangup();
        }
    }

    mResponse->hangupForegroundResumeBackgroundResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioVoice::hangupWaitingOrBackground(int32_t serial) {
    LOG_CALL << serial;
    if (!mModemVoice) {
        mResponse->hangupWaitingOrBackgroundResponse(error(serial, RadioError::SIM_ABSENT));
        return ok();
    }

    auto calls = mModemVoice->getNotTerminatedCallsList();
    for (auto& call : calls) {
        call->hangup();
    }

    mResponse->hangupWaitingOrBackgroundResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioVoice::isVoNrEnabled(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->isVoNrEnabledResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioVoice::rejectCall(int32_t serial) {
    LOG_CALL << serial;

    if (!mModemVoice) {
        mResponse->rejectCallResponse(error(serial, RadioError::SIM_ABSENT));
        return ok();
    }

    auto call = mModemVoice->getIncomingCall();
    if (!call) {
        mResponse->rejectCallResponse(error(serial, RadioError::INVALID_STATE));
        return ok();
    }

    call->hangup();
    mResponse->rejectCallResponse(okay(serial));

    return ok();
}

ScopedAStatus RadioVoice::responseAcknowledgement() {
    LOG_UNIMPLEMENTED;
    return ok();
}

ScopedAStatus RadioVoice::sendBurstDtmf(int32_t serial, const std::string& /*dtmf*/, int32_t /*on*/,
                                        int32_t /*off*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->sendBurstDtmfResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::sendCdmaFeatureCode(int32_t serial, const std::string& /*featureCode*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->sendCdmaFeatureCodeResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::sendDtmf(int32_t serial, const std::string& s) {
    LOG_CALL << serial << ": " << s;

    if (!mModemVoice) {
        mResponse->sendDtmfResponse(error(serial, RadioError::SIM_ABSENT));
        return ok();
    }
    auto call = mModemVoice->getActiveCall();
    if (!call) {
        mResponse->sendDtmfResponse(error(serial, RadioError::INVALID_STATE));
        return ok();
    }
    call->sendDtmf(s);
    mResponse->sendDtmfResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioVoice::sendUssd(int32_t serial, const std::string& ussd) {
    LOG_STUB << serial << ": " << ussd;
    mResponse->sendUssdResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::separateConnection(int32_t serial, int32_t /*gsmIndex*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->separateConnectionResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::setCallForward(int32_t serial,
                                         const aidl::CallForwardInfo& /*callInfo*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setCallForwardResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::setCallWaiting(int32_t serial, bool /*enable*/,
                                         int32_t /*serviceClass*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setCallWaitingResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::setClir(int32_t serial, int32_t /*status*/) {
    LOG_STUB << serial;
    mResponse->setClirResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::setMute(int32_t serial, bool enable) {
    LOG_STUB << serial << " enable: " << enable;
    mResponse->setMuteResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioVoice::setPreferredVoicePrivacy(int32_t serial, bool /*enable*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setPreferredVoicePrivacyResponse(notSupported(serial));
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

ScopedAStatus RadioVoice::setTtyMode(int32_t serial, aidl::TtyMode /*mode*/) {
    LOG_STUB << serial;
    mResponse->setTtyModeResponse(okay(serial));
    return ok();
}

ndk::ScopedAStatus RadioVoice::setVoNrEnabled(int32_t serial, [[maybe_unused]] bool enable) {
    LOG_STUB << serial;
    mResponse->setVoNrEnabledResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioVoice::startDtmf(int32_t serial, const std::string& s) {
    LOG_CALL << serial << ": " << s;

    if (!mModemVoice) {
        mResponse->startDtmfResponse(error(serial, RadioError::SIM_ABSENT));
        return ok();
    }
    auto call = mModemVoice->getActiveCall();
    if (!call) {
        mResponse->startDtmfResponse(error(serial, RadioError::INVALID_STATE));
        return ok();
    }
    call->sendDtmf(s);
    mResponse->startDtmfResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioVoice::stopDtmf(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->stopDtmfResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioVoice::switchWaitingOrHoldingAndActive(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->switchWaitingOrHoldingAndActiveResponse(notSupported(serial));
    return ok();
}

// Internal API

void RadioVoice::callStateChanged() {
    LOG_CALL;
    if (mIndication) mIndication->callStateChanged(RadioIndicationType::UNSOLICITED);
}

}  // namespace android::hardware::radio::mm
