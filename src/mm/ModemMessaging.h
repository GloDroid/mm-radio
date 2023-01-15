// SPDX-License-Identifier: Apache-2.0

#pragma once

#include <map>

#include "MmRaii.h"

#include <libmm-glib.h>

class ModemSms {
  public:
    static auto createInstance(MMSms* sms) -> std::shared_ptr<ModemSms>;

    auto getPath() -> std::string;
    auto getIndex() -> int32_t;

    auto getNumber() -> std::string;
    auto getText() -> std::string;
    auto getTimestamp() -> std::string;
    auto getState() -> MMSmsState;

  private:
    ModemSms() = default;

    SharedGObject<MMSms> mMmSms;
};

class ModemMessaging {
  public:
    static auto createInstance(SharedMmObject& mmObject) -> std::shared_ptr<ModemMessaging>;

    auto sendSms(std::string& smsc, std::string& number, std::string& text) -> int;

    auto deleteSms(int index) -> int;

    void setSmsReceivedCallback(std::function<void(std::shared_ptr<ModemSms>)> callback) {
        mSmsReceivedCallback = std::move(callback);
    }

    ~ModemMessaging();

    void queryMessages();

  private:
    ModemMessaging() = default;

    static void smsAddedCallback(MMModemMessaging* modem, const gchar* smsPath, gboolean received,
                                 ModemMessaging* self);
    static void smsDeletedCallback(MMModemMessaging* modem, const gchar* smsPath,
                                   ModemMessaging* self);

    void addMessage(const gchar* messagePath);

    std::function<void(std::shared_ptr<ModemSms>)> mSmsReceivedCallback;

    std::mutex mMessagesMutex;
    std::map<int /*index*/, std::shared_ptr<ModemSms>> mMessages;

    SharedGObject<MMModemMessaging> mMmModemMessaging;
};
