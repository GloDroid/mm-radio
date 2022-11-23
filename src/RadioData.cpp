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

#include "RadioData.h"

#include "debug.h"

#define RADIO_MODULE "Data"

namespace android::hardware::radio::mm {

using ::ndk::ScopedAStatus;
namespace aidl = ::aidl::android::hardware::radio::data;
namespace aidlCommon = ::aidl::android::hardware::radio;
constexpr auto ok = &ScopedAStatus::ok;

ScopedAStatus RadioData::allocatePduSessionId(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioData::cancelHandover(int32_t serial, int32_t callId) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioData::deactivateDataCall(int32_t serial, int32_t cid,
                                            aidl::DataRequestReason reason) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioData::getDataCallList(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioData::getSlicingConfig(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioData::releasePduSessionId(int32_t serial, int32_t id) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioData::responseAcknowledgement() {
    LOG_UNIMPLEMENTED;
    return ok();
}

ScopedAStatus RadioData::setDataAllowed(int32_t serial, bool allow) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioData::setDataProfile(int32_t serial,
                                        const std::vector<aidl::DataProfileInfo>& profiles) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioData::setDataThrottling(int32_t serial, aidl::DataThrottlingAction dta,
                                           int64_t completionDurationMs) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioData::setInitialAttachApn(int32_t serial,
                                             const std::optional<aidl::DataProfileInfo>& info) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioData::setResponseFunctions(
        const std::shared_ptr<aidl::IRadioDataResponse>& response,
        const std::shared_ptr<aidl::IRadioDataIndication>& indication) {
    LOG_CALL << response << ' ' << indication;

    mResponse = response;
    mIndication = indication;

    return ok();
}

ScopedAStatus RadioData::setupDataCall(int32_t serial, aidlCommon::AccessNetwork accessNetwork,
                                       const aidl::DataProfileInfo& dataProfileInfo,
                                       bool roamingAllowed, aidl::DataRequestReason reason,
                                       const std::vector<aidl::LinkAddress>& addresses,
                                       const std::vector<std::string>& dnses, int32_t pduSessId,
                                       const std::optional<aidl::SliceInfo>& sliceInfo,
                                       bool matchAllRuleAllowed) {
    LOG_UNIMPLEMENTED;
    return ok();
}

ScopedAStatus RadioData::startHandover(int32_t serial, int32_t callId) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioData::startKeepalive(int32_t serial, const aidl::KeepaliveRequest& keepalive) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

ScopedAStatus RadioData::stopKeepalive(int32_t serial, int32_t sessionHandle) {
    LOG_UNIMPLEMENTED << serial;
    return ok();
}

}  // namespace android::hardware::radio::mm
