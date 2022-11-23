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

#include <aidl/android/hardware/radio/config/BnRadioConfig.h>

namespace android::hardware::radio::mm {

class RadioConfig : public aidl::android::hardware::radio::config::BnRadioConfig {
    ::ndk::ScopedAStatus getHalDeviceCapabilities(int32_t serial) override;
    ::ndk::ScopedAStatus getNumOfLiveModems(int32_t serial) override;
    ::ndk::ScopedAStatus getPhoneCapability(int32_t serial) override;
    ::ndk::ScopedAStatus getSimSlotsStatus(int32_t serial) override;
    ::ndk::ScopedAStatus setNumOfLiveModems(int32_t serial, int8_t numOfLiveModems) override;
    ::ndk::ScopedAStatus setPreferredDataModem(int32_t serial, int8_t modemId) override;
    ::ndk::ScopedAStatus setResponseFunctions(
            const std::shared_ptr<aidl::android::hardware::radio::config::IRadioConfigResponse>&
                    radioConfigResponse,
            const std::shared_ptr<aidl::android::hardware::radio::config::IRadioConfigIndication>&
                    radioConfigIndication) override;
    ::ndk::ScopedAStatus setSimSlotsMapping(
            int32_t serial,
            const std::vector<aidl::android::hardware::radio::config::SlotPortMapping>& slotMap)
            override;

    std::shared_ptr<aidl::android::hardware::radio::config::IRadioConfigResponse> mResponse;
    std::shared_ptr<aidl::android::hardware::radio::config::IRadioConfigIndication> mIndication;

  public:
    RadioConfig();
};

}  // namespace android::hardware::radio::mm
