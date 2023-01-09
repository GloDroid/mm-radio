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

#include <aidl/android/hardware/radio/RadioAccessFamily.h>

#include "debug.h"

#define RADIO_MODULE "Modem"

namespace android::hardware::radio::mm {

using ::ndk::ScopedAStatus;
namespace aidl = ::aidl::android::hardware::radio::modem;
using ::aidl::android::hardware::radio::RadioAccessFamily;
using ::aidl::android::hardware::radio::RadioIndicationType;
using ::aidl::android::hardware::radio::RadioTechnology;
using ::aidl::android::hardware::radio::modem::RadioState;

constexpr auto ok = &ScopedAStatus::ok;

ScopedAStatus RadioModem::enableModem(int32_t serial, bool /*on*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->enableModemResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioModem::getBasebandVersion(int32_t serial) {
    LOG_STUB << serial;
    mResponse->getBasebandVersionResponse(okay(serial), "10000");
    return ok();
}

ScopedAStatus RadioModem::getDeviceIdentity(int32_t serial) {
    LOG_STUB << serial;
    if (!mModemSim) {
        mResponse->getDeviceIdentityResponse(error(serial, RadioError::SIM_ABSENT), {}, {}, {}, {});
        return ok();
    }

    mResponse->getDeviceIdentityResponse(okay(serial), mModemSim->getIdentifier(), "00", "00",
                                         "00");
    return ok();
}

ScopedAStatus RadioModem::getHardwareConfig(int32_t serial) {
    LOG_CALL << serial;

    auto hwConfig = std::vector<aidl::HardwareConfig>();
    auto modem = (aidl::HardwareConfig){
            .type = aidl::HardwareConfig::TYPE_MODEM,
            .uuid = "MODEM-UUID-TODO",                     // TODO(nobody)
            .state = aidl::HardwareConfig::STATE_ENABLED,  // STATE_STANDBY,
    };

    auto modemConfig = (aidl::HardwareConfigModem){
            .rilModel = 0,  // Single RIL access
            .rat = (RadioTechnology)(1 << (uint32_t)RadioTechnology::LTE),
            .maxVoiceCalls = 1,
            .maxDataCalls = 0,
            .maxStandby = 1,
    };

    auto sim = (aidl::HardwareConfig){
            .type = aidl::HardwareConfig::TYPE_SIM,
            .uuid = "SIM-UUID-TODO",  // TODO(nobody)
            .state = aidl::HardwareConfig::STATE_ENABLED,
    };
    auto simConfig = (aidl::HardwareConfigSim){
            .modemUuid = "MODEM-UUID-TODO",  // TODO(nobody)
    };

    modem.modem.emplace_back(modemConfig);
    sim.sim.emplace_back(simConfig);
    hwConfig.emplace_back(modem);
    hwConfig.emplace_back(sim);

    mResponse->getHardwareConfigResponse(okay(serial), hwConfig);

    return ok();
}

ScopedAStatus RadioModem::getModemActivityInfo(int32_t serial) {
    LOG_STUB << serial;

    mResponse->getModemActivityInfoResponse(notSupported(serial), {});

    return ok();
}

ScopedAStatus RadioModem::getModemStackStatus(int32_t serial) {
    LOG_STUB << serial;
    mResponse->getModemStackStatusResponse(okay(serial), false);
    return ok();
}

ScopedAStatus RadioModem::getRadioCapability(int32_t serial) {
    LOG_CALL << serial;
    mResponse->getRadioCapabilityResponse(okay(serial), mRadioCapability);
    return ok();
}

ScopedAStatus RadioModem::nvReadItem(int32_t serial, aidl::NvItem /*temId*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->nvReadItemResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioModem::nvResetConfig(int32_t serial, aidl::ResetNvType /*resetType*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->nvResetConfigResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioModem::nvWriteCdmaPrl(int32_t serial, const std::vector<uint8_t>& /*prl*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->nvWriteCdmaPrlResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioModem::nvWriteItem(int32_t serial, const aidl::NvWriteItem& /*item*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->nvWriteItemResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioModem::requestShutdown(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->requestShutdownResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioModem::responseAcknowledgement() {
    LOG_UNIMPLEMENTED;
    return ok();
}

ScopedAStatus RadioModem::sendDeviceState(int32_t serial, aidl::DeviceStateType type, bool state) {
    LOG_UNIMPLEMENTED << serial << " type: " << aidl::toString(type) << " state: " << state;
    mResponse->sendDeviceStateResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioModem::setRadioCapability(int32_t serial, const aidl::RadioCapability& rc) {
    LOG_CALL << serial << ' ' << rc.toString();
    mRadioCapability = rc;
    mResponse->setRadioCapabilityResponse(notSupported(serial), mRadioCapability);
    return ok();
}

ScopedAStatus RadioModem::setRadioPower(int32_t serial, bool powerOn, bool /*forEmergencyCall*/,
                                        bool /*preferredForEmergencyCall*/) {
    LOG_CALL << serial << " PowerOn: " << powerOn;

    if (!mModem) {
        mResponse->setRadioPowerResponse(error(serial, RadioError::SIM_ABSENT));
        return ok();
    }

    mResponse->setRadioPowerResponse(okay(serial));
    if (powerOn)
        mModem->enable();
    else
        mModem->disable();

    return ok();
}

ScopedAStatus RadioModem::setResponseFunctions(
        const std::shared_ptr<aidl::IRadioModemResponse>& response,
        const std::shared_ptr<aidl::IRadioModemIndication>& indication) {
    LOG_STUB << response << ' ' << indication;

    mResponse = response;
    mIndication = indication;

    mRadioCapability = (aidl::RadioCapability){
            .phase = aidl::RadioCapability::PHASE_CONFIGURED,
            .raf = (int32_t)RadioAccessFamily::LTE,
            .logicalModemUuid = "com.mm-radio.lm0",
            .status = aidl::RadioCapability::STATUS_SUCCESS,
    };

    rilConnected();

    return ok();
}

// Internal communication

void RadioModem::radioStateChanged(bool on) {
    LOG(VERBOSE) << "Radio state changed: " << on;
    mIndication->radioStateChanged(RadioIndicationType::UNSOLICITED,
                                   on ? RadioState::ON : RadioState::OFF);
}

void RadioModem::rilConnected() {
    if (!mIndication) return;

    LOG_CALL;

    if (!mModem) {
        mIndication->radioStateChanged(RadioIndicationType::UNSOLICITED, RadioState::UNAVAILABLE);
    } else {
        mIndication->rilConnected(RadioIndicationType::UNSOLICITED);
        mModem->refreshModemState();
        mIndication->radioCapabilityIndication(RadioIndicationType::UNSOLICITED, mRadioCapability);
    }
}

}  // namespace android::hardware::radio::mm
