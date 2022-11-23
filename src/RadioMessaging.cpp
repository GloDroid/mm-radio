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

#include "RadioMessaging.h"

#include "debug.h"

#define RADIO_MODULE "Messaging"

namespace android::hardware::radio::mm {

using ::ndk::ScopedAStatus;
namespace aidl = ::aidl::android::hardware::radio::messaging;
constexpr auto ok = &ScopedAStatus::ok;

ScopedAStatus RadioMessaging::acknowledgeIncomingGsmSmsWithPdu(  //
        int32_t serial, bool success, const std::string& ackPdu) {
    LOG_UNIMPLEMENTED << serial << ' ' << success << ' ' << ackPdu;
    return ok();
}

ScopedAStatus RadioMessaging::acknowledgeLastIncomingCdmaSms(  //
        int32_t serial, const aidl::CdmaSmsAck& smsAck) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioMessaging::acknowledgeLastIncomingGsmSms(  //
        int32_t serial, bool success, aidl::SmsAcknowledgeFailCause cause) {
    LOG_UNIMPLEMENTED << serial << ' ' << success;
    return ok();
}

ScopedAStatus RadioMessaging::deleteSmsOnRuim(int32_t serial, int32_t index) {
    LOG_UNIMPLEMENTED << serial << ' ' << index;
    return ok();
}

ScopedAStatus RadioMessaging::deleteSmsOnSim(int32_t serial, int32_t index) {
    LOG_UNIMPLEMENTED << serial << ' ' << index;
    return ok();
}

ScopedAStatus RadioMessaging::getCdmaBroadcastConfig(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioMessaging::getGsmBroadcastConfig(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioMessaging::getSmscAddress(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioMessaging::reportSmsMemoryStatus(int32_t serial, bool available) {
    LOG_UNIMPLEMENTED << serial << ' ' << available;
    return ok();
}

ScopedAStatus RadioMessaging::responseAcknowledgement() {
    LOG_UNIMPLEMENTED;
    return ok();
}

ScopedAStatus RadioMessaging::sendCdmaSms(int32_t serial, const aidl::CdmaSmsMessage& sms) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioMessaging::sendCdmaSmsExpectMore(int32_t serial, const aidl::CdmaSmsMessage& m) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioMessaging::sendImsSms(int32_t serial, const aidl::ImsSmsMessage& message) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioMessaging::sendSms(int32_t serial, const aidl::GsmSmsMessage& message) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioMessaging::sendSmsExpectMore(int32_t serial, const aidl::GsmSmsMessage& msg) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioMessaging::setCdmaBroadcastActivation(int32_t serial, bool activate) {
    LOG_UNIMPLEMENTED << serial << ' ' << activate;
    return ok();
}

ScopedAStatus RadioMessaging::setCdmaBroadcastConfig(
        int32_t serial, const std::vector<aidl::CdmaBroadcastSmsConfigInfo>& cfgInfo) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioMessaging::setGsmBroadcastActivation(int32_t serial, bool activate) {
    LOG_UNIMPLEMENTED << serial << ' ' << activate;
    return ok();
}

ScopedAStatus RadioMessaging::setGsmBroadcastConfig(
        int32_t serial, const std::vector<aidl::GsmBroadcastSmsConfigInfo>& configInfo) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioMessaging::setResponseFunctions(
        const std::shared_ptr<aidl::IRadioMessagingResponse>& response,
        const std::shared_ptr<aidl::IRadioMessagingIndication>& indication) {
    LOG_CALL << response << ' ' << indication;

    mResponse = response;
    mIndication = indication;

    return ok();
}

ScopedAStatus RadioMessaging::setSmscAddress(int32_t serial, const std::string& smsc) {
    LOG_UNIMPLEMENTED << serial << ' ' << smsc;
    return ok();
}

ScopedAStatus RadioMessaging::writeSmsToRuim(int32_t serial, const aidl::CdmaSmsWriteArgs& sms) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioMessaging::writeSmsToSim(int32_t serial, const aidl::SmsWriteArgs& smsWrArgs) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

}  // namespace android::hardware::radio::mm
