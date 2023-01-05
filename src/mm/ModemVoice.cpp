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

#include "ModemVoice.h"

#include <cutils/log.h>

auto ModemCall::createInstance(MMCall* call) -> std::shared_ptr<ModemCall> {
    auto mmCall = std::shared_ptr<ModemCall>(new ModemCall());
    mmCall->mMmCall = makeSharedGObject<MMCall>(call);

    g_signal_connect(mmCall->mMmCall.get(), "state-changed", G_CALLBACK(stateChangedCallback),
                     mmCall.get());

    mmCall->mCallState = mm_call_get_state(mmCall->mMmCall.get());

    return mmCall;
}

auto ModemCall::getPath() -> std::string {
    return mm_call_get_path(mMmCall.get());
}

auto ModemCall::getNumber() -> std::string {
    return mm_call_get_number(mMmCall.get());
}

auto ModemCall::getState() -> MMCallState {
    return mCallState;
}

auto ModemCall::getDirection() -> MMCallDirection {
    return mm_call_get_direction(mMmCall.get());
}

auto ModemCall::isMultiparty() -> bool {
    return mm_call_get_multiparty(mMmCall.get()) != 0;
}

auto ModemCall::getAudioPort() -> std::string {
    const auto* port = mm_call_get_audio_port(mMmCall.get());
    return (port != nullptr) ? mm_call_get_audio_port(mMmCall.get()) : "none";
}

auto ModemCall::getIndex() -> int32_t {
    auto path = getPath();
    auto indexStr = path.substr(path.find_last_of('/') + 1);
    return std::stoi(indexStr);
}

void ModemCall::sendDtmf(const std::string& dtmf) {
    GError* error = nullptr;
    mm_call_send_dtmf_sync(mMmCall.get(), dtmf.c_str(), nullptr, &error);
    if (error != nullptr) {
        RLOGE("Failed to send DTMF: %s", error->message);
        g_error_free(error);
    }
}

void ModemCall::hangup() {
    GError* error = nullptr;
    mm_call_hangup_sync(mMmCall.get(), nullptr, &error);
    if (error != nullptr) {
        RLOGE("Failed to hangup call: %s", error->message);
        g_error_free(error);
    }
}

void ModemCall::accept() {
    GError* error = nullptr;
    mm_call_accept_sync(mMmCall.get(), nullptr, &error);
    if (error != nullptr) {
        RLOGE("Failed to accept call: %s", error->message);
        g_error_free(error);
    }
}

void ModemCall::start() {
    GError* error = nullptr;
    mm_call_start_sync(mMmCall.get(), nullptr, &error);
    if (error != nullptr) {
        RLOGE("Failed to start call: %s", error->message);
        g_error_free(error);
    }
}

void ModemCall::dump() {
    RLOGD("Path: %s", getPath().c_str());
    RLOGD("  Call: %s", getNumber().c_str());
    RLOGD("  State: %s", mm_call_state_get_string(getState()));
    RLOGD("  Direction: %s", mm_call_direction_get_string(getDirection()));
    RLOGD("  Multiparty: %s", isMultiparty() ? "yes" : "no");
    RLOGD("  Audio port: %s", getAudioPort().c_str());
}

ModemCall::~ModemCall() {
    g_signal_handlers_disconnect_by_func(mMmCall.get(), (void*)stateChangedCallback, this);
}

void ModemCall::setNotifyFrontend(std::function<void()> notifyFrontend) {
    mNotifyFrontend = std::move(notifyFrontend);
}

void ModemCall::stateChangedCallback(MMCall* /*call*/, MMCallState /*oldState*/,
                                     MMCallState newState, MMCallStateReason /*reason*/,
                                     ModemCall* modemCall) {
    modemCall->mCallState = newState;
    modemCall->dump();
    // Framework doesn't like when we notify it about dialing state
    if (modemCall->mNotifyFrontend && newState != MM_CALL_STATE_DIALING) {
        modemCall->mNotifyFrontend();
    }
}

std::shared_ptr<ModemVoice> ModemVoice::createInstance(SharedMmObject& mmObject) {
    auto mmVoice = std::shared_ptr<ModemVoice>(new ModemVoice());
    mmVoice->mMmModemVoice =
            makeSharedGObject<MMModemVoice>(mm_object_get_modem_voice(mmObject.get()));

    // Register for call state changes
    g_signal_connect(mmVoice->mMmModemVoice.get(), "call-added", G_CALLBACK(callAddedCallback),
                     mmVoice.get());

    g_signal_connect(mmVoice->mMmModemVoice.get(), "call-deleted", G_CALLBACK(callDeletedCallback),
                     mmVoice.get());

    mmVoice->queryCalls();

    return mmVoice;
}

auto ModemVoice::createCall(const std::string& number) -> std::shared_ptr<ModemCall> {
    GError* error = nullptr;
    MMCallProperties* properties = mm_call_properties_new();
    mm_call_properties_set_number(properties, number.c_str());

    MMCall* call =
            mm_modem_voice_create_call_sync(mMmModemVoice.get(), properties, nullptr, &error);
    if (error != nullptr) {
        RLOGE("Failed to create call: %s", error->message);
        g_error_free(error);
        return nullptr;
    }

    return ModemCall::createInstance(call);
}

auto& ModemVoice::getCallsList() {
    return mCalls;
}

auto ModemVoice::getNotTerminatedCallsList() -> std::vector<std::shared_ptr<ModemCall>> {
    auto calls = getCallsList();
    auto out = std::vector<std::shared_ptr<ModemCall>>();
    for (auto& [_, call] : calls) {
        if (call->getState() != MM_CALL_STATE_TERMINATED) out.emplace_back(call);
    }
    return out;
}

auto ModemVoice::getActiveCall() -> std::shared_ptr<ModemCall> {
    return getCallByState(MM_CALL_STATE_ACTIVE);
}

auto ModemVoice::getIncomingCall() -> std::shared_ptr<ModemCall> {
    return getCallByState(MM_CALL_STATE_RINGING_IN);
}

auto ModemVoice::getCallByState(MMCallState state) -> std::shared_ptr<ModemCall> {
    auto calls = getCallsList();
    for (auto& [_, call] : calls) {
        if (call->getState() == state) {
            return call;
        }
    }

    return {};
}

ModemVoice::~ModemVoice() {
    g_signal_handlers_disconnect_by_func(mMmModemVoice.get(), (gpointer)callAddedCallback, this);
    g_signal_handlers_disconnect_by_func(mMmModemVoice.get(), (gpointer)callDeletedCallback, this);
}

void ModemVoice::setNotifyFrontend(std::function<void()> notifyFrontend) {
    mNotifyFrontend = std::move(notifyFrontend);
}

int ModemVoice::pathToIndex(const std::string& path) {
    auto pos = path.find_last_of('/');
    if (pos == std::string::npos) {
        return -1;
    }

    return std::stoi(path.substr(pos + 1));
}

void ModemVoice::queryCalls() {
    gchar** callPaths = NULL;

    callPaths = mm_gdbus_modem_voice_dup_calls(MM_GDBUS_MODEM_VOICE(mMmModemVoice.get()));

    if (callPaths == nullptr) return;

    // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
    for (guint i = 0; callPaths[i] != nullptr; i++) {
        // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
        addCall(callPaths[i]);
    }
}

int ModemVoice::addCall(const gchar* path) {
    RLOGD("Adding call %s", path);
    auto index = pathToIndex(path);
    if (index == -1) return -1;
    if (mCalls.count(index) != 0) return -1;
    GObject* call = nullptr;
    call = (GObject*)g_initable_new(
            MM_TYPE_CALL, NULL, NULL, "g-flags", G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START, "g-name",
            MM_DBUS_SERVICE, "g-connection",
            g_dbus_proxy_get_connection(G_DBUS_PROXY(mMmModemVoice.get())), "g-object-path", path,
            "g-interface-name", "org.freedesktop.ModemManager1.Call", NULL);
    if (call == NULL) {
        RLOGE("Failed to create call object for %s", path);
        return -1;
    }
    auto modemCall = ModemCall::createInstance(MM_CALL(call));
    modemCall->setNotifyFrontend([this]() {
        if (mNotifyFrontend) mNotifyFrontend();
    });
    mCalls[index] = modemCall;

    RLOGD("Added call %s", path);

    return index;
}

void ModemVoice::callAddedCallback(MMModemVoice* /*mmModemVoice*/, const gchar* callPath,
                                   ModemVoice* self) {
    auto index = self->addCall(callPath);
    RLOGD("Call added: %s, index: %d", callPath, index);
    if (index != -1) {
        auto& call = self->mCalls[index];
        if (call->getState() == MM_CALL_STATE_RINGING_IN && self->mNotifyFrontend) {
            RLOGD("Call is ringing in, notifying frontend");
            self->mNotifyFrontend();
        }
    }
}

void ModemVoice::callDeletedCallback(MMModemVoice* /*mmModemVoice*/, const gchar* callPath,
                                     ModemVoice* self) {
    RLOGD("Call deleted: %s", callPath);

    auto index = pathToIndex(callPath);
    if (index != -1 && self->mCalls.count(index) != 0) {
        self->mCalls.erase(index);
        if (self->mNotifyFrontend) self->mNotifyFrontend();
    }
}
