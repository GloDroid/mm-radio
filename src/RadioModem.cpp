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

#include "RadioModem.h"

#include "debug.h"

#define RADIO_MODULE "Modem"

namespace android::hardware::radio::mm {

using ::ndk::ScopedAStatus;
namespace aidl = ::aidl::android::hardware::radio::modem;
constexpr auto ok = &ScopedAStatus::ok;

ScopedAStatus RadioModem::enableModem(int32_t serial, bool on) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::getBasebandVersion(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::getDeviceIdentity(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::getHardwareConfig(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::getModemActivityInfo(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::getModemStackStatus(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::getRadioCapability(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::nvReadItem(int32_t serial, aidl::NvItem itemId) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::nvResetConfig(int32_t serial, aidl::ResetNvType resetType) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::nvWriteCdmaPrl(int32_t serial, const std::vector<uint8_t>& prl) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::nvWriteItem(int32_t serial, const aidl::NvWriteItem& item) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::requestShutdown(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::responseAcknowledgement() {
    LOG_UNIMPLEMENTED;
    return ok();
}

ScopedAStatus RadioModem::sendDeviceState(int32_t serial, aidl::DeviceStateType type, bool state) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::setRadioCapability(int32_t serial, const aidl::RadioCapability& rc) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::setRadioPower(int32_t serial, bool powerOn, bool forEmergencyCall,
                                        bool preferredForEmergencyCall) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioModem::setResponseFunctions(
        const std::shared_ptr<aidl::IRadioModemResponse>& response,
        const std::shared_ptr<aidl::IRadioModemIndication>& indication) {
    LOG_CALL << response << ' ' << indication;

    mResponse = response;
    mIndication = indication;

    return ok();
}

}  // namespace android::hardware::radio::mm
