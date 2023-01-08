// SPDX-License-Identifier: Apache-2.0

#pragma once

#include "MmRaii.h"

#include <libmm-glib.h>

using UssdCallback = std::function<void(MMModem3gppUssdSessionState, const std::string&)>;

class ModemUssd {
  public:
    static std::shared_ptr<ModemUssd> createInstance(SharedMmObject& mmObject);

    void sendUssd(const std::string& code);
    void cancelSession();

    void setUssdReceivedCallback(UssdCallback ussdReceivedCallback) {
        mUssdReceivedCallback = std::move(ussdReceivedCallback);
    }

  private:
    ModemUssd() = default;

    bool isSessionActive();

    std::string mResponseStr;

    void commonCallback(gchar* response, bool success);

    static auto commonCallbackFunc(ModemUssd* self) -> int;
    static void initiateCallback(MMModem3gppUssd* mmModem3gppUssd, GAsyncResult* result,
                                 ModemUssd* self);
    static void responseCallback(MMModem3gppUssd* mmModem3gppUssd, GAsyncResult* result,
                                 ModemUssd* self);
    static void cancelCallback(MMModem3gppUssd* mmModem3gppUssd, GAsyncResult* result,
                               ModemUssd* self);
    UssdCallback mUssdReceivedCallback;

    SharedGObject<MMModem3gppUssd> mMmModem3gppUssd;
};
