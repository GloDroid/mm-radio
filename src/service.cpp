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
#include <android/binder_manager.h>
#include <android/binder_process.h>

#include "RadioConfig.h"
#include "RadioData.h"
#include "RadioMessaging.h"
#include "RadioModem.h"
#include "RadioNetwork.h"
#include "RadioSim.h"
#include "RadioVoice.h"

namespace android::hardware::radio::service {

using namespace std::string_literals;

static std::vector<std::shared_ptr<ndk::ICInterface>> gPublishedHals;

template <typename T>
static void publishRadioHal(const std::string& slot) {
    const auto instance = T::descriptor + "/"s + slot;
    if (!AServiceManager_isDeclared(instance.c_str())) {
        LOG(INFO) << instance << " is not declared in VINTF (this may be intentional)";
        return;
    }
    LOG(DEBUG) << "Publishing " << instance;

    auto aidlHal = ndk::SharedRefBase::make<T>();
    gPublishedHals.push_back(aidlHal);
    const auto status = AServiceManager_addService(aidlHal->asBinder().get(), instance.c_str());
    CHECK_EQ(status, STATUS_OK);
}

static void publishRadio(std::string slot) {
    publishRadioHal<mm::RadioData>(slot);
    publishRadioHal<mm::RadioMessaging>(slot);
    publishRadioHal<mm::RadioModem>(slot);
    publishRadioHal<mm::RadioNetwork>(slot);
    publishRadioHal<mm::RadioSim>(slot);
    publishRadioHal<mm::RadioVoice>(slot);
}

static void publishRadioConfig() {
    auto aidlHal = ndk::SharedRefBase::make<mm::RadioConfig>();
    gPublishedHals.push_back(aidlHal);
    const auto instance = mm::RadioConfig::descriptor + "/default"s;
    const auto status = AServiceManager_addService(aidlHal->asBinder().get(), instance.c_str());
    CHECK_EQ(status, STATUS_OK);
}

static void main() {
    // Use "logcat -b radio *:F mm-radio:V" to get logs
    base::InitLogging(nullptr, base::LogdLogger(base::RADIO));
    base::SetDefaultTag("mm-radio");
    base::SetMinimumLogSeverity(base::VERBOSE);
    LOG(DEBUG) << "ModemManager-based Radio HAL service starting...";

    publishRadioConfig();

    publishRadio("slot1");

    LOG(DEBUG) << "MM Radio HAL service is operational";
    ABinderProcess_joinThreadPool();
    LOG(FATAL) << "MM Radio HAL service has stopped";
}

}  // namespace android::hardware::radio::service

int main() {
    android::hardware::radio::service::main();
    return EXIT_FAILURE;  // should not reach
}
