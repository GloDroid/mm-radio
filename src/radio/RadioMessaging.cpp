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
#include "utils/sms_deliver.h"
#include "utils/sms_submit.h"

#include <libmm-glib.h>

#define RADIO_MODULE "Messaging"

namespace android::hardware::radio::mm {

using ::aidl::android::hardware::radio::RadioIndicationType;
using ::aidl::android::hardware::radio::messaging::SendSmsResult;
using ::ndk::ScopedAStatus;
namespace aidl = ::aidl::android::hardware::radio::messaging;
constexpr auto ok = &ScopedAStatus::ok;

ScopedAStatus RadioMessaging::acknowledgeIncomingGsmSmsWithPdu(  //
        int32_t serial, bool success, const std::string& ackPdu) {
    LOG_UNIMPLEMENTED << serial << ' ' << success << ' ' << ackPdu;
    mResponse->acknowledgeIncomingGsmSmsWithPduResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioMessaging::acknowledgeLastIncomingCdmaSms(  //
        int32_t serial, const aidl::CdmaSmsAck& /*smsAck*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->acknowledgeLastIncomingCdmaSmsResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioMessaging::acknowledgeLastIncomingGsmSms(  //
        int32_t serial, bool success, aidl::SmsAcknowledgeFailCause /*cause*/) {
    LOG_CALL << serial << ' ' << success;

    auto lock = std::unique_lock(mIncomingSmsListMutex);

    if (!mIncomingSmsList.empty()) {
        auto acceptedSms = mIncomingSmsList.front();
        mIncomingSmsList.pop_front();
        // Delete accepted SMS from modem
        if (mModemMessaging) mModemMessaging->deleteSms(acceptedSms->getIndex());

        reportNextIncomingSmsLocked();
    }

    mResponse->acknowledgeLastIncomingGsmSmsResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioMessaging::deleteSmsOnRuim(int32_t serial, int32_t index) {
    LOG_UNIMPLEMENTED << serial << ' ' << index;
    mResponse->deleteSmsOnRuimResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioMessaging::deleteSmsOnSim(int32_t serial, int32_t index) {
    LOG_UNIMPLEMENTED << serial << ' ' << index;
    mResponse->deleteSmsOnSimResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioMessaging::getCdmaBroadcastConfig(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getCdmaBroadcastConfigResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioMessaging::getGsmBroadcastConfig(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getGsmBroadcastConfigResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioMessaging::getSmscAddress(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getSmscAddressResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioMessaging::reportSmsMemoryStatus(int32_t serial, bool available) {
    LOG_UNIMPLEMENTED << serial << ' ' << available;
    mResponse->reportSmsMemoryStatusResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioMessaging::responseAcknowledgement() {
    LOG_UNIMPLEMENTED;
    return ok();
}

ScopedAStatus RadioMessaging::sendCdmaSms(int32_t serial, const aidl::CdmaSmsMessage& /*sms*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->sendCdmaSmsResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioMessaging::sendCdmaSmsExpectMore(int32_t serial,
                                                    const aidl::CdmaSmsMessage& /*m*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->sendCdmaSmsExpectMoreResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioMessaging::sendImsSms(int32_t serial, const aidl::ImsSmsMessage& /*message*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->sendImsSmsResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioMessaging::sendSms(int32_t serial, const aidl::GsmSmsMessage& message) {
    LOG_CALL << serial << ": " << message.toString();

    if (!mModemMessaging) {
        LOG(ERROR) << "ModemMessaging is not initialized";
        mResponse->sendSmsResponse(error(serial, RadioError::SIM_ABSENT), {});
        return ok();
    }

    LOG(INFO) << "Sending SMS to " << message.smscPdu << ": " << message.pdu;

    auto sArgs = (DecodeSmsSubmitArgs){
            .in_smsc_pdu = message.smscPdu,
            .in_pdu = message.pdu,
    };
    auto res = smsSubmitDecode(sArgs);
    if (res != 0) {
        LOG(ERROR) << "Failed to decode SMS PDU: " << message.pdu;
        mResponse->sendSmsResponse(error(serial, RadioError::INVALID_ARGUMENTS), {});
        return ok();
    }

    auto smsc = sArgs.out_smsc;
    auto dest = sArgs.out_destination;
    auto text = sArgs.out_message;
    LOG(INFO) << "Sending SMS to " << dest << ": " << text;

    auto ret = mModemMessaging->sendSms(smsc, dest, text);
    if (ret != 0) {
        LOG(ERROR) << "Failed to send SMS: " << ret;
        mResponse->sendSmsResponse(error(serial, RadioError::INVALID_ARGUMENTS), {});
        return ok();
    }

    auto sendSmsResult = (SendSmsResult){
            .messageRef = mOutgoingSmsIndex++,
            .ackPDU = "",
            .errorCode = -1,
    };

    mResponse->sendSmsResponse(okay(serial), sendSmsResult);
    return ok();
}

ScopedAStatus RadioMessaging::sendSmsExpectMore(int32_t serial,
                                                const aidl::GsmSmsMessage& /*msg*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->sendSmsExpectMoreResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioMessaging::setCdmaBroadcastActivation(int32_t serial, bool activate) {
    LOG_STUB << serial << ' ' << activate;
    mResponse->setCdmaBroadcastActivationResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioMessaging::setCdmaBroadcastConfig(
        int32_t serial, const std::vector<aidl::CdmaBroadcastSmsConfigInfo>& /*cfgInfo*/) {
    LOG_STUB << serial;
    mResponse->setCdmaBroadcastConfigResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioMessaging::setGsmBroadcastActivation(int32_t serial, bool activate) {
    LOG_STUB << serial << ' ' << activate;
    mResponse->setGsmBroadcastActivationResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioMessaging::setGsmBroadcastConfig(
        int32_t serial, const std::vector<aidl::GsmBroadcastSmsConfigInfo>& /*configInfo*/) {
    LOG_STUB << serial;
    mResponse->setGsmBroadcastConfigResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioMessaging::setResponseFunctions(
        const std::shared_ptr<aidl::IRadioMessagingResponse>& response,
        const std::shared_ptr<aidl::IRadioMessagingIndication>& indication) {
    LOG_CALL << response << ' ' << indication;

    mResponse = response;
    mIndication = indication;

    if (mModemMessaging) mModemMessaging->queryMessages();

    return ok();
}

ScopedAStatus RadioMessaging::setSmscAddress(int32_t serial, const std::string& smsc) {
    LOG_UNIMPLEMENTED << serial << ' ' << smsc;
    mResponse->setSmscAddressResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioMessaging::writeSmsToRuim(int32_t serial,
                                             const aidl::CdmaSmsWriteArgs& /*sms*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->writeSmsToRuimResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioMessaging::writeSmsToSim(int32_t serial,
                                            const aidl::SmsWriteArgs& /*smsWrArgs*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->writeSmsToSimResponse(notSupported(serial), {});
    return ok();
}

// Internal Interfaces

void RadioMessaging::smsReceived(const std::shared_ptr<ModemSms>& sms) {
    LOG_CALL;

    auto lock = std::unique_lock(mIncomingSmsListMutex);

    mIncomingSmsList.push_back(sms);
    if (mIncomingSmsList.size() == 1) {
        reportNextIncomingSmsLocked();
    }
}

void RadioMessaging::reportNextIncomingSmsLocked() {
    LOG_CALL;

    if (mIncomingSmsList.empty()) {
        return;
    }

    auto sms = mIncomingSmsList.front();

    auto address = sms->getNumber();
    auto text = sms->getText();
    auto ts = sms->getTimestamp();
    LOG_CALL << address << ": " << text << " (" << ts << ")";

    auto pdu = smsDeliverEncode(address, text, ts);

    LOG(INFO) << "PDU: " << pdu;

    auto fullpdu = "00" + pdu;
    std::vector<uint8_t> pduBytes;
    for (size_t i = 0; i < fullpdu.length(); i += 2) {
        auto byte = fullpdu.substr(i, 2);
        constexpr auto kBase = 16;
        pduBytes.push_back(std::stoi(byte, nullptr, kBase));
    }
    mIndication->newSms(RadioIndicationType::UNSOLICITED, pduBytes);
}

}  // namespace android::hardware::radio::mm
