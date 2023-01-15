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

#pragma once

#include <list>

#include <aidl/android/hardware/radio/messaging/BnRadioMessaging.h>

#include "mm/ModemMessaging.h"

namespace android::hardware::radio::mm {

class RadioMessaging : public aidl::android::hardware::radio::messaging::BnRadioMessaging {
    ::ndk::ScopedAStatus acknowledgeIncomingGsmSmsWithPdu(int32_t serial, bool success,
                                                          const std::string& ackPdu) override;
    ::ndk::ScopedAStatus acknowledgeLastIncomingCdmaSms(
            int32_t serial,
            const ::aidl::android::hardware::radio::messaging::CdmaSmsAck& smsAck) override;
    ::ndk::ScopedAStatus acknowledgeLastIncomingGsmSms(
            int32_t serial, bool success,
            ::aidl::android::hardware::radio::messaging::SmsAcknowledgeFailCause cause) override;
    ::ndk::ScopedAStatus deleteSmsOnRuim(int32_t serial, int32_t index) override;
    ::ndk::ScopedAStatus deleteSmsOnSim(int32_t serial, int32_t index) override;
    ::ndk::ScopedAStatus getCdmaBroadcastConfig(int32_t serial) override;
    ::ndk::ScopedAStatus getGsmBroadcastConfig(int32_t serial) override;
    ::ndk::ScopedAStatus getSmscAddress(int32_t serial) override;
    ::ndk::ScopedAStatus reportSmsMemoryStatus(int32_t serial, bool available) override;
    ::ndk::ScopedAStatus responseAcknowledgement() override;
    ::ndk::ScopedAStatus sendCdmaSms(
            int32_t serial,
            const ::aidl::android::hardware::radio::messaging::CdmaSmsMessage& sms) override;
    ::ndk::ScopedAStatus sendCdmaSmsExpectMore(
            int32_t serial,
            const ::aidl::android::hardware::radio::messaging::CdmaSmsMessage& sms) override;
    ::ndk::ScopedAStatus sendImsSms(
            int32_t serial,
            const ::aidl::android::hardware::radio::messaging::ImsSmsMessage& message) override;
    ::ndk::ScopedAStatus sendSms(
            int32_t serial,
            const ::aidl::android::hardware::radio::messaging::GsmSmsMessage& message) override;
    ::ndk::ScopedAStatus sendSmsExpectMore(
            int32_t serial,
            const ::aidl::android::hardware::radio::messaging::GsmSmsMessage& message) override;
    ::ndk::ScopedAStatus setCdmaBroadcastActivation(int32_t serial, bool activate) override;
    ::ndk::ScopedAStatus setCdmaBroadcastConfig(
            int32_t serial,
            const std::vector<
                    ::aidl::android::hardware::radio::messaging::CdmaBroadcastSmsConfigInfo>&
                    configInfo) override;
    ::ndk::ScopedAStatus setGsmBroadcastActivation(int32_t serial, bool activate) override;
    ::ndk::ScopedAStatus setGsmBroadcastConfig(
            int32_t serial,
            const std::vector<
                    ::aidl::android::hardware::radio::messaging::GsmBroadcastSmsConfigInfo>&
                    configInfo) override;
    ::ndk::ScopedAStatus setResponseFunctions(
            const std::shared_ptr<
                    ::aidl::android::hardware::radio::messaging::IRadioMessagingResponse>&
                    radioMessagingResponse,
            const std::shared_ptr<
                    ::aidl::android::hardware::radio::messaging::IRadioMessagingIndication>&
                    radioMessagingIndication) override;
    ::ndk::ScopedAStatus setSmscAddress(int32_t serial, const std::string& smsc) override;
    ::ndk::ScopedAStatus writeSmsToRuim(
            int32_t serial,
            const ::aidl::android::hardware::radio::messaging::CdmaSmsWriteArgs& cdmaSms) override;
    ::ndk::ScopedAStatus writeSmsToSim(
            int32_t serial,
            const ::aidl::android::hardware::radio::messaging::SmsWriteArgs& smsWriteArgs) override;

    std::shared_ptr<::aidl::android::hardware::radio::messaging::IRadioMessagingResponse> mResponse;
    std::shared_ptr<::aidl::android::hardware::radio::messaging::IRadioMessagingIndication>
            mIndication;

    std::shared_ptr<ModemMessaging> mModemMessaging;

    int mOutgoingSmsIndex = 0;

    std::mutex mIncomingSmsListMutex;
    std::list<std::shared_ptr<ModemSms>> mIncomingSmsList;

  public:
    void bindModem(std::shared_ptr<ModemMessaging> modemMessaging) {
        mModemMessaging = std::move(modemMessaging);
        if (mIndication) mModemMessaging->queryMessages();
    }

    void smsReceived(const std::shared_ptr<ModemSms>& sms);
    void reportNextIncomingSmsLocked();
};

}  // namespace android::hardware::radio::mm
