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

#include <android-base/logging.h>
#include <android/binder_process.h>

#include "RadioHalManager.h"

namespace android::hardware::radio::service {

using android::hardware::radio::mm::RadioHalManager;

static void main() {
    // Use "logcat -b radio *:F mm-radio:V" to get logs
    base::InitLogging(nullptr, base::LogdLogger(base::RADIO));
    base::SetDefaultTag("mm-radio");
    base::SetMinimumLogSeverity(base::VERBOSE);
    LOG(DEBUG) << "ModemManager-based Radio HAL service starting...";

    auto radioHalManager = RadioHalManager::getInstance();
    if (!radioHalManager->registerFrontendRadioConfig())
        LOG(FATAL) << "Failed to register RadioConfig, please check your manifest";

    constexpr int kMaxSlots = 3;
    constexpr int kFirstSlot = 1;
    int slot = 0;
    for (slot = 0; slot < kMaxSlots; slot++) {
        if (!radioHalManager->registerFrontendElements(slot + kFirstSlot)) break;
    }

    if (slot == 0) LOG(FATAL) << "Failed to register any slot, please check your manifest";

    LOG(DEBUG) << "Registered " << slot << " slots for modems";

    radioHalManager->bindModemsToFrontends();

    LOG(DEBUG) << "MM Radio HAL service is operational";
    ABinderProcess_joinThreadPool();
    LOG(FATAL) << "MM Radio HAL service has stopped";
}

}  // namespace android::hardware::radio::service

int main() {
    android::hardware::radio::service::main();
    return EXIT_FAILURE;  // should not reach
}
