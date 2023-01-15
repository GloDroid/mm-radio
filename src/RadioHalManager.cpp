/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * Copyright (C) 2023 GloDroid project
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

#define LOG_TAG "mm-radio: hal-manager"

#include "RadioHalManager.h"

#include <android/binder_manager.h>
#include <cutils/log.h>
#include <string>

namespace android::hardware::radio::mm {

using namespace std::string_literals;
using ::aidl::android::hardware::radio::config::SimPortInfo;
using ::aidl::android::hardware::radio::config::SimSlotStatus;
using ::aidl::android::hardware::radio::sim::CardStatus;

std::shared_ptr<RadioHalManager> RadioHalManager::getInstance() {
    static const std::shared_ptr<RadioHalManager> kInstance = createInstance();
    return kInstance;
}

auto RadioHalManager::getSimSlotStatus() -> SimSlotStatusList {
    SimSlotStatusList slots{};

    for (auto& fe : mFrontendElements) {
        std::shared_ptr<ModemSim> simObj{};
        ModemSim* sim{};
        if (mSlotIdToModemObjectMap.count(fe.first) != 0) {
            auto modemObj = mSlotIdToModemObjectMap[fe.first];
            simObj = modemObj->getModem()->getSim();
            if (simObj && simObj->isActive()) sim = simObj.get();
        }

        auto slotStatus = (SimSlotStatus){
                .cardState = (sim != nullptr) ? int(CardStatus::STATE_PRESENT)
                                              : int(CardStatus::STATE_ABSENT),
                .atr = {},
                .eid = (sim != nullptr) ? sim->getEid() : ""s,
        };

        if (sim != nullptr) {
            slotStatus.portInfo.emplace_back((SimPortInfo){
                    .iccId = sim->getIdentifier(),
                    .logicalSlotId = fe.first,
                    .portActive = true,
            });
        }

        slots.emplace_back(slotStatus);
    }

    return slots;
}

void RadioHalManager::glibLoopThreadFunc(const std::shared_ptr<RadioHalManager>& /*instance*/) {
    GMainLoop* loop = g_main_loop_new(nullptr, FALSE);
    g_main_loop_run(loop);
}

auto RadioHalManager::createInstance() -> std::shared_ptr<RadioHalManager> {
    auto instance = std::shared_ptr<RadioHalManager>(new RadioHalManager());

    instance->mModemManager = ModemManager::createInstance();

    auto newModemCb = [instance](std::shared_ptr<ModemObject>& mObject) {
        instance->handleNewModem(mObject);
    };

    auto removedModemCb = [instance](std::shared_ptr<ModemObject>& mObject) {
        instance->handleModemRemoved(mObject);
    };

    instance->mModemManager->setModemCallbacks(newModemCb, removedModemCb);

    std::thread(&RadioHalManager::glibLoopThreadFunc, instance).detach();

    return instance;
}

auto RadioHalManager::getNextUnoccupiedFrontendSlot() -> std::optional<int> {
    for (auto& fe : mFrontendElements) {
        if (mSlotIdToModemObjectMap.count(fe.first) == 0) return fe.first;
    }
    return {};
}

void RadioHalManager::handleNewModem(const std::shared_ptr<ModemObject>& mObj) {
    RLOGD("New modem detected: %s", mObj->getPath().c_str());
    auto slotId = getNextUnoccupiedFrontendSlot();
    if (!slotId) {
        RLOGE("No unoccupied slots found");
        return;
    }

    RLOGD("Binding modem %d to slot %d", mObj->getIndex(), *slotId);

    auto& modem = mObj->getModem();
    auto sim = modem->getSim();
    if (!sim) {
        RLOGE("Modem %d does not have a SIM object", mObj->getIndex());
        return;
    }

    mSlotIdToModemObjectMap[*slotId] = mObj;

    auto& fe = mFrontendElements[*slotId];
    fe.mRadioSim->bindModem(sim);
    auto& voice = mObj->getVoice();
    auto& ussd = mObj->getUssd();
    auto& messaging = mObj->getMessaging();
    fe.mRadioVoice->bindModem(voice, ussd);
    fe.mRadioModem->bindModem(modem);
    fe.mRadioMessaging->bindModem(messaging);
    voice->setNotifyFrontend([fe]() { fe.mRadioVoice->callStateChanged(); });
    modem->setModemStateChangedCb([fe](bool on) { fe.mRadioModem->radioStateChanged(on); });
    ussd->setUssdReceivedCallback([fe](MMModem3gppUssdSessionState state, const std::string& ussd) {
        fe.mRadioVoice->ussdReceived(state, ussd);
    });
    messaging->setSmsReceivedCallback([fe](const std::shared_ptr<ModemSms>& sms) {  //
        fe.mRadioMessaging->smsReceived(sms);
    });

    mRadioConfig->simSlotStatusChanged();
    fe.mRadioModem->rilConnected();
}

void RadioHalManager::handleModemRemoved(const std::shared_ptr<ModemObject>& mObj) {
    RLOGD("Modem removed: %s", mObj->getPath().c_str());
    auto slotMObjPair = std::find_if(
            mSlotIdToModemObjectMap.begin(), mSlotIdToModemObjectMap.end(),
            [mObj](const auto& pair) { return pair.second->getIndex() == mObj->getIndex(); });

    if (slotMObjPair == mSlotIdToModemObjectMap.end()) {
        RLOGE("Modem %d not found in slot map", mObj->getIndex());
        return;
    }

    auto slotId = slotMObjPair->first;

    RLOGD("Unbinding modem %d from slot %d", mObj->getIndex(), slotId);

    auto& fe = mFrontendElements[slotId];
    fe.mRadioSim->bindModem({});
    fe.mRadioVoice->bindModem({}, {});
    fe.mRadioModem->bindModem({});
    fe.mRadioMessaging->bindModem({});
    mSlotIdToModemObjectMap.erase(slotId);

    mRadioConfig->simSlotStatusChanged();
    fe.mRadioModem->rilConnected();
}

// HAL Registration

template <typename T>
static bool publishRadioHal(int slot, const std::shared_ptr<T>& aidlHal) {
    auto instance = T::descriptor + "/slot"s + std::to_string(slot);
    if (!AServiceManager_isDeclared(instance.c_str())) {
        RLOGV("%s is not declared in VINTF (this may be intentional)", instance.c_str());
        return false;
    }
    RLOGD("Publishing %s", instance.c_str());

    auto status = AServiceManager_addService(aidlHal->asBinder().get(), instance.c_str());
    return status == STATUS_OK;
}

bool RadioHalManager::registerFrontendRadioConfig() {
    mRadioConfig = ndk::SharedRefBase::make<RadioConfig>();
    mRadioConfig->bindManager([this]() -> SimSlotStatusList { return getSimSlotStatus(); });
    auto instance = RadioConfig::descriptor + "/default"s;
    auto status = AServiceManager_addService(mRadioConfig->asBinder().get(), instance.c_str());
    return status == STATUS_OK;
}

bool RadioHalManager::registerFrontendElements(int slotId) {
    auto elements = (FrontendElements){
            .mRadioData = ndk::SharedRefBase::make<RadioData>(),
            .mRadioMessaging = ndk::SharedRefBase::make<RadioMessaging>(),
            .mRadioModem = ndk::SharedRefBase::make<RadioModem>(),
            .mRadioNetwork = ndk::SharedRefBase::make<RadioNetwork>(),
            .mRadioSim = ndk::SharedRefBase::make<RadioSim>(),
            .mRadioVoice = ndk::SharedRefBase::make<RadioVoice>(),
    };

    if (!publishRadioHal(slotId, elements.mRadioData)) return false;
    if (!publishRadioHal(slotId, elements.mRadioMessaging)) return false;
    if (!publishRadioHal(slotId, elements.mRadioModem)) return false;
    if (!publishRadioHal(slotId, elements.mRadioNetwork)) return false;
    if (!publishRadioHal(slotId, elements.mRadioSim)) return false;
    if (!publishRadioHal(slotId, elements.mRadioVoice)) return false;

    mFrontendElements[slotId] = elements;

    return true;
}

}  // namespace android::hardware::radio::mm