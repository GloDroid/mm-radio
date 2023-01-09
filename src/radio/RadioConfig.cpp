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

#include "RadioConfig.h"

#include <aidl/android/hardware/radio/sim/BnRadioSim.h>

#include "debug.h"

#define RADIO_MODULE "Config"

namespace android::hardware::radio::mm {

using ::aidl::android::hardware::radio::RadioIndicationType;
using ::ndk::ScopedAStatus;
namespace aidl = ::aidl::android::hardware::radio::config;
constexpr auto ok = &ScopedAStatus::ok;

ScopedAStatus RadioConfig::getHalDeviceCapabilities(int32_t serial) {
    LOG_CALL << serial;
    mResponse->getHalDeviceCapabilitiesResponse(okay(serial), true);
    return ok();
}

ScopedAStatus RadioConfig::getNumOfLiveModems(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getNumOfLiveModemsResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioConfig::getPhoneCapability(int32_t serial) {
    LOG_STUB << serial;
    auto cap = (aidl::PhoneCapability){
            .maxActiveData = 1,
            .maxActiveInternetData = 1,
            .isInternetLingeringSupported = false,
            .logicalModemIds = std::vector<uint8_t>(0),
    };

    mResponse->getPhoneCapabilityResponse(okay(serial), cap);

    return ok();
}

ScopedAStatus RadioConfig::getSimSlotsStatus(int32_t serial) {
    LOG_STUB << serial;
    mResponse->getSimSlotsStatusResponse(okay(serial), mGetSimSlotStatusCb());
    return ok();
}

ScopedAStatus RadioConfig::setNumOfLiveModems(int32_t serial, int8_t /*numOfLiveModems*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setNumOfLiveModemsResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioConfig::setPreferredDataModem(int32_t serial, int8_t modemId) {
    LOG_STUB << serial << " modemId: " << (int)modemId;
    auto resp = okay(serial);
    if (modemId < 0) resp.error = RadioError::INVALID_ARGUMENTS;
    mResponse->setPreferredDataModemResponse(resp);
    return ok();
}

ScopedAStatus RadioConfig::setResponseFunctions(
        const std::shared_ptr<aidl::IRadioConfigResponse>& radioConfigResponse,
        const std::shared_ptr<aidl::IRadioConfigIndication>& radioConfigIndication) {
    LOG_CALL << radioConfigResponse << ' ' << radioConfigIndication;

    CHECK(radioConfigResponse);
    CHECK(radioConfigIndication);

    mResponse = radioConfigResponse;
    mIndication = radioConfigIndication;

    return ok();
}

ScopedAStatus RadioConfig::setSimSlotsMapping(  //
        int32_t serial, const std::vector<aidl::SlotPortMapping>& /*slotMap*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setSimSlotsMappingResponse(okay(serial));
    return ok();
}

// Internal API:
void RadioConfig::simSlotStatusChanged() {
    LOG_CALL;
    if (mIndication)
        mIndication->simSlotsStatusChanged(RadioIndicationType::UNSOLICITED, mGetSimSlotStatusCb());
}

}  // namespace android::hardware::radio::mm
