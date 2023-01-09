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

#include "mm/MmRaii.h"
#include "mm/Modem.h"
#include "mm/ModemManager.h"

#include "radio/RadioConfig.h"
#include "radio/RadioData.h"
#include "radio/RadioMessaging.h"
#include "radio/RadioModem.h"
#include "radio/RadioNetwork.h"
#include "radio/RadioSim.h"
#include "radio/RadioVoice.h"

#include <map>
#include <memory>
#include <thread>

namespace android::hardware::radio::mm {

class FrontendElements {
  public:
    std::shared_ptr<RadioData> mRadioData;
    std::shared_ptr<RadioMessaging> mRadioMessaging;
    std::shared_ptr<RadioModem> mRadioModem;
    std::shared_ptr<RadioNetwork> mRadioNetwork;
    std::shared_ptr<RadioSim> mRadioSim;
    std::shared_ptr<RadioVoice> mRadioVoice;
};

class RadioHalManager {
  public:
    static std::shared_ptr<RadioHalManager> getInstance();

    void schedule(std::function<void()> task);

    bool registerFrontendRadioConfig();
    bool registerFrontendElements(int slotId);

    void bindModemsToFrontends() { mModemManager->invokeAddedCallbackForAvailableModems(); }

  private:
    static auto createInstance() -> std::shared_ptr<RadioHalManager>;

    static void glibLoopThreadFunc(const std::shared_ptr<RadioHalManager>& instance);

    RadioHalManager() = default;

    void handleNewModem(const std::shared_ptr<ModemObject>& mObj);
    void handleModemRemoved(const std::shared_ptr<ModemObject>& mObj);

    auto getNextUnoccupiedFrontendSlot() -> std::optional<int>;

    std::shared_ptr<ModemManager> mModemManager;

    std::map<int /* slotId */, FrontendElements> mFrontendElements;
    std::shared_ptr<RadioConfig> mRadioConfig;

    std::map<int /* slotId */, std::shared_ptr<ModemObject>> mSlotIdToModemObjectMap;

    auto getSimSlotStatus() -> SimSlotStatusList;
};

}  // namespace android::hardware::radio::mm