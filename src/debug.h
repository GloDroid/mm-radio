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

#include <android-base/logging.h>

namespace android::hardware::radio::mm {

namespace debug {

static constexpr bool kSuperVerbose = true;

#define LOG_CALL \
    if constexpr (debug::kSuperVerbose) LOG(VERBOSE) << (RADIO_MODULE ".") << __func__ << ' '

#define LOG_UNIMPLEMENTED LOG(WARNING) << (RADIO_MODULE ".") << __func__ << " not implemented "
#define LOG_STUB LOG(WARNING) << (RADIO_MODULE ".") << __func__ << " stub "

}  // namespace debug

using RadioResponseType = aidl::android::hardware::radio::RadioResponseType;
using RadioResponseInfo = aidl::android::hardware::radio::RadioResponseInfo;
using RadioError = aidl::android::hardware::radio::RadioError;

static inline RadioResponseInfo notSupported(int32_t serial) {
    return {
            .type = RadioResponseType::SOLICITED,
            .serial = serial,
            .error = RadioError::REQUEST_NOT_SUPPORTED,
    };
}

static inline RadioResponseInfo okay(int32_t serial) {
    return {
            .type = RadioResponseType::SOLICITED,
            .serial = serial,
            .error = RadioError::NONE,
    };
}

}  // namespace android::hardware::radio::mm
