// SPDX-License-Identifier: Apache-2.0
//
// (c) 2023 GloDroid project (https://github.com/GloDroid/mm-radio)

#include "ModemUssd.h"

#include <cutils/log.h>

auto ModemUssd::commonCallbackFunc(ModemUssd* self) -> int {
    auto state = mm_modem_3gpp_ussd_get_state(self->mMmModem3gppUssd.get());
    if (self->mUssdReceivedCallback != nullptr)
        self->mUssdReceivedCallback(state, self->mResponseStr);

    return G_SOURCE_REMOVE;
}

void ModemUssd::initiateCallback(MMModem3gppUssd* mmModem3gppUssd, GAsyncResult* result,
                                 ModemUssd* self) {
    GError* error = nullptr;
    gchar* response = nullptr;

    response = mm_modem_3gpp_ussd_initiate_finish(mmModem3gppUssd, result, &error);
    if (error != nullptr) {
        RLOGE("Failed to initiate USSD: %s", error->message);
        g_error_free(error);
        self->commonCallback(response, false);
        return;
    }
    self->commonCallback(response, true);
}

void ModemUssd::responseCallback(MMModem3gppUssd* mmModem3gppUssd, GAsyncResult* result,
                                 ModemUssd* self) {
    GError* error = nullptr;
    gchar* response = nullptr;

    response = mm_modem_3gpp_ussd_initiate_finish(mmModem3gppUssd, result, &error);
    if (error != nullptr) {
        RLOGE("Failed to respond USSD: %s", error->message);
        g_error_free(error);
        self->commonCallback(response, false);
        return;
    }
    self->commonCallback(response, true);
}

void ModemUssd::cancelCallback(MMModem3gppUssd* mmModem3gppUssd, GAsyncResult* result,
                               ModemUssd* /*self*/) {
    GError* error = nullptr;
    mm_modem_3gpp_ussd_cancel_finish(mmModem3gppUssd, result, &error);
    if (error != nullptr) {
        RLOGE("Failed to cancel USSD: %s", error->message);
        g_error_free(error);
    }
}

std::shared_ptr<ModemUssd> ModemUssd::createInstance(SharedMmObject& mmObject) {
    auto inst = std::shared_ptr<ModemUssd>(new ModemUssd());
    inst->mMmModem3gppUssd =
            makeSharedGObject<MMModem3gppUssd>(mm_object_get_modem_3gpp_ussd(mmObject.get()));

    return inst;
}

void ModemUssd::sendUssd(const std::string& code) {
    if (!isSessionActive())
        mm_modem_3gpp_ussd_initiate(mMmModem3gppUssd.get(), code.c_str(), nullptr,
                                    (GAsyncReadyCallback)initiateCallback, this);
    else
        mm_modem_3gpp_ussd_respond(mMmModem3gppUssd.get(), code.c_str(), nullptr,
                                   (GAsyncReadyCallback)responseCallback, this);
}

void ModemUssd::cancelSession() {
    mm_modem_3gpp_ussd_cancel(mMmModem3gppUssd.get(), nullptr, (GAsyncReadyCallback)cancelCallback,
                              this);
}

bool ModemUssd::isSessionActive() {
    auto state = mm_modem_3gpp_ussd_get_state(mMmModem3gppUssd.get());
    switch (state) {
        case MM_MODEM_3GPP_USSD_SESSION_STATE_ACTIVE:
        case MM_MODEM_3GPP_USSD_SESSION_STATE_USER_RESPONSE:
            return true;
        default:
            return false;
    }
}

void ModemUssd::commonCallback(gchar* response, bool success) {
    /* internal state that mean responce error */
    auto state = MM_MODEM_3GPP_USSD_SESSION_STATE_UNKNOWN;

    RLOGD("USSD response: %s", response);
    mResponseStr = (response != nullptr) ? response : "";

    if (!success) {
        if (mUssdReceivedCallback != nullptr) mUssdReceivedCallback(state, mResponseStr);
    } else {
        // Call callback delayed to allow modem manager to change state
        constexpr int kDelayMs = 100;
        g_timeout_add(kDelayMs, (GSourceFunc)ModemUssd::commonCallbackFunc, this);
    }

    g_free(response);
}
