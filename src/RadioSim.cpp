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

#include "RadioSim.h"

#include <map>

#include "debug.h"

#define RADIO_MODULE "Sim"

namespace android::hardware::radio::mm {

using ::aidl::android::hardware::radio::RadioIndicationType;
using ::aidl::android::hardware::radio::sim::AppStatus;
using ::ndk::ScopedAStatus;
namespace aidl = ::aidl::android::hardware::radio::sim;
constexpr auto ok = &ScopedAStatus::ok;

ScopedAStatus RadioSim::areUiccApplicationsEnabled(int32_t serial) {
    LOG_STUB << serial;
    mResponse->areUiccApplicationsEnabledResponse(okay(serial), true);
    return ok();
}

ScopedAStatus RadioSim::changeIccPin2ForApp(int32_t serial, const std::string& /*oldPin2*/,
                                            const std::string& /*newPin2*/,
                                            const std::string& /*aid*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->changeIccPin2ForAppResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioSim::changeIccPinForApp(int32_t serial, const std::string& /*oldPin*/,
                                           const std::string& /*newPin*/,
                                           const std::string& /*aid*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->changeIccPinForAppResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioSim::enableUiccApplications(int32_t serial, bool /*enable*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->enableUiccApplicationsResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioSim::getAllowedCarriers(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getAllowedCarriersResponse(notSupported(serial), {}, {});
    return ok();
}

ScopedAStatus RadioSim::getCdmaSubscription(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getCdmaSubscriptionResponse(notSupported(serial), {}, {}, {}, {}, {});
    return ok();
}

ScopedAStatus RadioSim::getCdmaSubscriptionSource(int32_t serial) {
    LOG_STUB << serial;
    mResponse->getCdmaSubscriptionSourceResponse(okay(serial),
                                                 aidl::CdmaSubscriptionSource::RUIM_SIM);
    return ok();
}

ScopedAStatus RadioSim::getFacilityLockForApp(  //
        int32_t serial, const std::string& /*facility*/, const std::string& /*password*/,
        int32_t /*serviceClass*/, const std::string& /*appId*/) {
    LOG_STUB << serial;
    mResponse->getFacilityLockForAppResponse(okay(serial), 0);
    return ok();
}

ScopedAStatus RadioSim::getIccCardStatus(int32_t serial) {
    LOG_STUB << serial;

    auto cardStatus = (aidl::CardStatus){
            .cardState = aidl::CardStatus::STATE_PRESENT,
            .universalPinState = aidl::PinState::UNKNOWN,
            .gsmUmtsSubscriptionAppIndex = 0,
            .cdmaSubscriptionAppIndex = -1,
            .imsSubscriptionAppIndex = -1,
            .atr = {},
            .iccid = "8938003992840681512F",
    };
    cardStatus.applications.emplace_back((AppStatus){
            .appType = AppStatus::APP_TYPE_USIM,
            .appState = AppStatus::APP_STATE_READY,
            .aidPtr = "0xA0 0x00 0x00 0x00 0x87 0x10 0x02 0xFF 0xFF 0xFF 0xFF 0x89",
            .appLabelPtr = "",
            .pin1 = aidl::PinState::UNKNOWN,
            .pin2 = aidl::PinState::UNKNOWN,
    });
    cardStatus.applications.emplace_back((AppStatus){
            .appType = AppStatus::APP_TYPE_RUIM,
    });
    cardStatus.applications.emplace_back((AppStatus){
            .appType = AppStatus::APP_TYPE_ISIM,
    });
    mResponse->getIccCardStatusResponse(okay(serial), cardStatus);
    return ok();
}

ScopedAStatus RadioSim::getImsiForApp(int32_t serial, const std::string& aid) {
    LOG_STUB << serial << "aid: " << aid;
    mResponse->getImsiForAppResponse(okay(serial), "255010899987259");
    return ok();
}

ScopedAStatus RadioSim::getSimPhonebookCapacity(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getSimPhonebookCapacityResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioSim::getSimPhonebookRecords(int32_t serial) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->getSimPhonebookRecordsResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioSim::iccCloseLogicalChannel(int32_t serial, int32_t channelId) {
    LOG_STUB << serial;
    auto resp = okay(serial);
    if (channelId < 1) resp.error = RadioError::INVALID_ARGUMENTS;
    mResponse->iccCloseLogicalChannelResponse(resp);
    return ok();
}

// NOLINTNEXTLINE(cppcoreguidelines-avoid-non-const-global-variables)
std::map<aidl::IccIo, aidl::IccIoResult> fakeIccProfile = {
        // EF_DIR
        {{0xc0, 0x2f00, "3F00", 0x0, 0x0, 0x0f},
         {144, 0, "621A8205422100300483022F008A01058B032F0601800200C08801F0"}},
        {{0xb2, 0x2f00, "3F00", 0x1, 0x4, 0x30},
         {144, 0,
          "61184F10A0000003431002FF86FF0389FFFFFFFF50044353494DFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
          "FFFFFFFF"}},
        {{0xb2, 0x2f00, "3F00", 0x2, 0x4, 0x30},
         {144, 0,
          "61184F10A0000003431002FF86FF0389FFFFFFFF50044353494DFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
          "FFFFFFFF"}},
        {{0xb2, 0x2f00, "3F00", 0x3, 0x4, 0x30},
         {144, 0,
          "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
          "FFFFFFFF"}},
        {{0xb2, 0x2f00, "3F00", 0x4, 0x4, 0x30},
         {144, 0,
          "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
          "FFFFFFFF"}},
        {{0xc0, 0x2fe2, "3F00", 0x0, 0x0, 0x0f},  // 0xc0=get response 0x2fe2=EF_ICCID
         {144, 0, "000082020000040000000000000000"}},
        {{0xb0, 0x2fe2, "3F00", 0x0, 0x0, 0x0a},  // ]0x2fe2=EF_ICCID
         {144, 0, "9883"}},
        {{0xb0, 0x2fe2, "3F00", 0x0, 0x0, 33282},  //  0x2fe2=EF_ICCID
         {144, 0, "9883"}},
        {{0xc0, 0x2f05, "3F00", 0x0, 0x0, 0x0f},  // EF_PL
         {144, 0, "62178202412183022F058A01058B032F060280020004880128"}},
        {{0xb0, 0x2f05, "3F00", 0x0, 0x0, 0x04}, {144, 0, "FFFFFFFF"}},
        // USIM
        {{0xc0, 0x6f40, "3F007FFF", 0x0, 0x0, 0x0f},  // EF_MSISDN
         {144, 0, "621982054221001C0283026F408A01058B036F0605800200388800"}},
        {{0xb2, 0x6f40, "3F007FFF", 0x1, 0x4, 0x1c},
         {144, 0, "000000000000000000000000000007915155214365F7FFFFFFFFFFFF"}},
        {{0xc0, 0x6fc9, "3F007FFF", 0x0, 0x0, 0x0f},  // EF_MBI
         {144, 0, "62198205422100050183026FC98A01058B036F0602800200058800"}},
        {{0xb2, 0x6fc9, "3F007FFF", 0x1, 0x4, 0x05}, {144, 0, "0100000000"}},
        {{0xc0, 0x6fc7, "3F007FFF", 0x0, 0x0, 0x0f},  // EF_MBDN
         {144, 0, "621982054221001C0283026F408A01058B036F06058002003E8800"}},
        {{0xb2, 0x6fc7, "3F007FFF", 0x1, 0x4, 0x1c},
         {144, 0, "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF07915155674523F1FFFFFFFFFFFF"}},
        {{0xc0, 0x6FAD, "3F007FFF", 0x0, 0x0, 0x0f},  // EF_AD
         {144, 0, "62178202412183026FAD8A01058B036F060180020004880118"}},
        {{0xb2, 0x6FAD, "3F007FFF", 0x0, 0x0, 0x04}, {144, 0, "00000003"}},
        {{0xc0, 0x6FCA, "3F007FFF", 0x0, 0x0, 0x0f},  // EF_MWIS
         {144, 0, "62198205422100050183026FCA8A01058B036F060E800200058800"}},
        {{0xb2, 0x6FCA, "3F007FFF", 0x1, 0x4, 0x05}, {144, 0, "0000000000"}},
        {{0xc0, 0x6F11, "3F007FFF", 0x0, 0x0, 0x0f},  // EF_VOICE_MAIL_INDICATOR_CPHS
         {106, 130, ""}},
        {{0xc0, 0x6F7B, "3F007FFF", 0x0, 0x0, 0x0f},  // EF_FPLMN
         {144, 0, "621C8202412183026F7BA5038001718A01058B036F06038002001E880168"}},
        {{0xb2, 0x6F7B, "3F007FFF", 0x0, 0x0, 0x1e},
         {144, 0, "64F00064F02064F04064F07064F080FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"}},
        // Other
        {{0xc0, 0x6f3c, "3F007FFF", 0x0, 0x0, 0x0f}, {144, 0, "0000820500000400000000000001B0"}},
        {{0xc0, 0x6f05, "3F007FFF", 0x0, 0x0, 0x0f}, {144, 0, "FFFF"}},
        {{0xc0, 0x6f07, "3F007FFF", 0x0, 0x0, 0x0f}, {144, 0, "0829"}},
        {{0xc0, 0x6f17, "3F007FFF", 0x0, 0x0, 0x0f}, {106, 130, ""}},
        {{0xc0, 0x2f05, "3F00", 0x0, 0x0, 0x0f}, {144, 0, "000082020000040000000000000000"}},
};

ScopedAStatus RadioSim::iccIoForApp(int32_t serial, const aidl::IccIo& iccIo) {
    LOG_STUB << serial;
    auto iccIoKey = (aidl::IccIo){
            .command = iccIo.command,
            .fileId = iccIo.fileId,
            .path = iccIo.path,
            .p1 = iccIo.p1,
            .p2 = iccIo.p2,
            .p3 = iccIo.p3,
    };

    if (fakeIccProfile.count(iccIoKey) != 0) {
        mResponse->iccIoForAppResponse(okay(serial), fakeIccProfile[iccIoKey]);
    } else {
        const aidl::IccIoResult kFileNotFound = {106, 130, ""};
        mResponse->iccIoForAppResponse(okay(serial), kFileNotFound);
    }

    return ok();
}

ScopedAStatus RadioSim::iccOpenLogicalChannel(int32_t serial, const std::string& /*aid*/,
                                              int32_t /*p2*/) {
    LOG_STUB << serial;
    static int channel = 1;
    mResponse->iccOpenLogicalChannelResponse(okay(serial), channel++, {});
    return ok();
}

ScopedAStatus RadioSim::iccTransmitApduBasicChannel(int32_t serial,
                                                    const aidl::SimApdu& /*message*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->iccTransmitApduBasicChannelResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioSim::iccTransmitApduLogicalChannel(int32_t serial,
                                                      const aidl::SimApdu& /*message*/) {
    LOG_STUB << serial;
    mResponse->iccTransmitApduLogicalChannelResponse(okay(serial), aidl::IccIoResult());
    return ok();
}

ScopedAStatus RadioSim::reportStkServiceIsRunning(int32_t serial) {
    LOG_STUB << serial;
    mResponse->reportStkServiceIsRunningResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioSim::requestIccSimAuthentication(  //
        int32_t serial, int32_t /*authContext*/, const std::string& /*authData*/,
        const std::string& /*aid*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->requestIccSimAuthenticationResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioSim::responseAcknowledgement() {
    LOG_UNIMPLEMENTED;
    return ok();
}

ScopedAStatus RadioSim::sendEnvelope(int32_t serial, const std::string& /*command*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->sendEnvelopeResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioSim::sendEnvelopeWithStatus(int32_t serial, const std::string& /*contents*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->sendEnvelopeWithStatusResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioSim::sendTerminalResponseToSim(int32_t serial,
                                                  const std::string& /*commandResponse*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->sendTerminalResponseToSimResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioSim::setAllowedCarriers(  //
        int32_t serial, const aidl::CarrierRestrictions& /*carriers*/,
        aidl::SimLockMultiSimPolicy /*mp*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setAllowedCarriersResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioSim::setCarrierInfoForImsiEncryption(
        int32_t serial, const aidl::ImsiEncryptionInfo& /*imsiEncryptionInfo*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setCarrierInfoForImsiEncryptionResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioSim::setCdmaSubscriptionSource(int32_t serial,
                                                  aidl::CdmaSubscriptionSource /*cdmaSub*/) {
    LOG_STUB << serial;
    mResponse->setCdmaSubscriptionSourceResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioSim::setFacilityLockForApp(  //
        int32_t serial, const std::string& /*facility*/, bool /*lockState*/,
        const std::string& /*password*/, int32_t /*serviceClass*/, const std::string& /*appId*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setFacilityLockForAppResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioSim::setResponseFunctions(
        const std::shared_ptr<aidl::IRadioSimResponse>& response,
        const std::shared_ptr<aidl::IRadioSimIndication>& indication) {
    LOG_STUB << response << ' ' << indication;

    mResponse = response;
    mIndication = indication;

    mIndication->simStatusChanged(RadioIndicationType::UNSOLICITED);
    mIndication->subscriptionStatusChanged(RadioIndicationType::UNSOLICITED, true);

    return ok();
}

ScopedAStatus RadioSim::setSimCardPower(int32_t serial, aidl::CardPowerState /*powerUp*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setSimCardPowerResponse(okay(serial));
    return ok();
}

ScopedAStatus RadioSim::setUiccSubscription(int32_t serial,
                                            const aidl::SelectUiccSub& /*uiccSub*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->setUiccSubscriptionResponse(notSupported(serial));
    return ok();
}

ScopedAStatus RadioSim::supplyIccPin2ForApp(int32_t serial, const std::string& /*pin2*/,
                                            const std::string& /*aid*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->supplyIccPin2ForAppResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioSim::supplyIccPinForApp(int32_t serial, const std::string& /*pin*/,
                                           const std::string& /*aid*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->supplyIccPinForAppResponse(notSupported(serial), {});
    return ok();
}

ScopedAStatus RadioSim::supplyIccPuk2ForApp(int32_t serial, const std::string& /*puk2*/,
                                            const std::string& /*pin2*/,
                                            const std::string& /*aid*/) {
    LOG_UNIMPLEMENTED << serial;
    auto resp = okay(serial);
    resp.error = RadioError::INVALID_SIM_STATE;
    mResponse->supplyIccPuk2ForAppResponse(resp, {});
    return ok();
}

ScopedAStatus RadioSim::supplyIccPukForApp(int32_t serial, const std::string& /*puk*/,
                                           const std::string& /*pin*/, const std::string& /*aid*/) {
    LOG_UNIMPLEMENTED << serial;
    auto resp = okay(serial);
    resp.error = RadioError::INVALID_SIM_STATE;
    mResponse->supplyIccPukForAppResponse(resp, {});
    return ok();
}

ScopedAStatus RadioSim::supplySimDepersonalization(int32_t serial, aidl::PersoSubstate /*pss*/,
                                                   const std::string& /*controlKey*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->supplySimDepersonalizationResponse(notSupported(serial), {}, {});
    return ok();
}

ScopedAStatus RadioSim::updateSimPhonebookRecords(int32_t serial,
                                                  const aidl::PhonebookRecordInfo& /*recordInfo*/) {
    LOG_UNIMPLEMENTED << serial;
    mResponse->updateSimPhonebookRecordsResponse(notSupported(serial), {});
    return ok();
}

}  // namespace android::hardware::radio::mm
