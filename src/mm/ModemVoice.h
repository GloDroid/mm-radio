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

#pragma once

#include "MmRaii.h"

#include <libmm-glib.h>

#include <map>
#include <memory>
#include <vector>

class ModemCall {
  public:
    static auto createInstance(MMCall* call) -> std::shared_ptr<ModemCall>;

    auto getPath() -> std::string;
    auto getNumber() -> std::string;
    auto getState() -> MMCallState;
    auto getDirection() -> MMCallDirection;
    auto isMultiparty() -> bool;
    auto getAudioPort() -> std::string;

    auto getIndex() -> int32_t;

    void sendDtmf(const std::string& dtmf);
    void hangup();
    void accept();
    void start();

    void dump();

    ~ModemCall();

    void setNotifyFrontend(std::function<void()> notifyFrontend);

  private:
    ModemCall() = default;

    std::function<void()> mNotifyFrontend;

    MMCallState mCallState = MM_CALL_STATE_UNKNOWN;

    static void stateChangedCallback(MMCall* call, MMCallState oldState, MMCallState newState,
                                     MMCallStateReason reason, ModemCall* modemCall);

    SharedGObject<MMCall> mMmCall;
};

class ModemVoice {
  public:
    static std::shared_ptr<ModemVoice> createInstance(SharedMmObject& mmObject);

    auto createCall(const std::string& number) -> std::shared_ptr<ModemCall>;
    auto& getCallsList();

    auto getNotTerminatedCallsList() -> std::vector<std::shared_ptr<ModemCall>>;
    auto getActiveCall() -> std::shared_ptr<ModemCall>;
    auto getIncomingCall() -> std::shared_ptr<ModemCall>;
    auto getCallByState(MMCallState state) -> std::shared_ptr<ModemCall>;
    ~ModemVoice();

    void setNotifyFrontend(std::function<void()> notifyFrontend);

  private:
    ModemVoice() = default;

    std::function<void()> mNotifyFrontend;

    void queryCalls();

    int addCall(const gchar* path);

    static void callAddedCallback(MMModemVoice* mmModemVoice, const gchar* callPath,
                                  ModemVoice* self);

    static void callDeletedCallback(MMModemVoice* mmModemVoice, const gchar* callPath,
                                    ModemVoice* self);

    std::map<int /*index*/, std::shared_ptr<ModemCall>> mCalls;

    SharedGObject<MMModemVoice> mMmModemVoice;
};
