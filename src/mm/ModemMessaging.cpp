// SPDX-License-Identifier: Apache-2.0

#define LOG_TAG "mm-radio: modem-messaging"

#include "ModemMessaging.h"

#include <cutils/log.h>
#include <libmm-glib.h>

auto ModemSms::createInstance(MMSms* sms) -> std::shared_ptr<ModemSms> {
    auto self = std::shared_ptr<ModemSms>(new ModemSms());
    self->mMmSms = makeSharedGObject(sms);
    return self;
}

auto ModemSms::getPath() -> std::string {
    return mm_sms_get_path(mMmSms.get());
}

auto ModemSms::getIndex() -> int32_t {
    return pathToIndex(getPath());
}

auto ModemSms::getNumber() -> std::string {
    const auto* number = mm_sms_get_number(mMmSms.get());
    return (number != nullptr) ? number : "";
}

auto ModemSms::getText() -> std::string {
    const auto* text = mm_sms_get_text(mMmSms.get());
    return (text != nullptr) ? text : "";
}

auto ModemSms::getTimestamp() -> std::string {
    const auto* timestamp = mm_sms_get_timestamp(mMmSms.get());
    return (timestamp != nullptr) ? timestamp : "";
}

auto ModemSms::getState() -> MMSmsState {
    return mm_sms_get_state(mMmSms.get());
}

void ModemMessaging::smsAddedCallback(MMModemMessaging* /*modem*/, const gchar* smsPath,
                                      gboolean received, ModemMessaging* self) {
    RLOGI("smsAddedCallback: %s", smsPath);
    if (received == TRUE) self->addMessage(smsPath);
}

void ModemMessaging::smsDeletedCallback(MMModemMessaging* /*modem*/, const gchar* smsPath,
                                        ModemMessaging* self) {
    RLOGI("smsDeletedCallback: %s", smsPath);

    auto index = pathToIndex(smsPath);

    auto lock = std::unique_lock(self->mMessagesMutex);

    if (index != -1 && self->mMessages.count(index) != 0) self->mMessages.erase(index);
}

auto ModemMessaging::createInstance(SharedMmObject& mmObject) -> std::shared_ptr<ModemMessaging> {
    auto self = std::shared_ptr<ModemMessaging>(new ModemMessaging());
    self->mMmModemMessaging = makeSharedGObject(mm_object_get_modem_messaging(mmObject.get()));

    g_signal_connect(self->mMmModemMessaging.get(), "added", G_CALLBACK(smsAddedCallback),
                     self.get());

    g_signal_connect(self->mMmModemMessaging.get(), "deleted", G_CALLBACK(smsDeletedCallback),
                     self.get());

    return self;
}

ModemMessaging::~ModemMessaging() {
    g_signal_handlers_disconnect_by_data(mMmModemMessaging.get(), this);
}

auto ModemMessaging::sendSms(std::string& smsc, std::string& number, std::string& text) -> int {
    auto smsProps = makeSharedGObject<MMSmsProperties>(mm_sms_properties_new());

    if (!smsc.empty()) mm_sms_properties_set_smsc(smsProps.get(), smsc.c_str());
    mm_sms_properties_set_number(smsProps.get(), number.c_str());
    mm_sms_properties_set_text(smsProps.get(), text.c_str());

    GError* error = nullptr;

    auto* sms =
            mm_modem_messaging_create_sync(mMmModemMessaging.get(), smsProps.get(), NULL, &error);
    if (sms == nullptr) {
        RLOGE("Failed to create SMS: %s", error->message);
        g_error_free(error);
        return -EINVAL;
    }

    if (mm_sms_send_sync(sms, NULL, &error) == FALSE) {
        RLOGE("Failed to send SMS: %s", error->message);
        g_error_free(error);
        return -EINVAL;
    }

    return 0;
}

auto ModemMessaging::deleteSms(int index) -> int {
    std::string smsPath;
    {
        auto lock = std::unique_lock(mMessagesMutex);
        if (mMessages.count(index) == 0) return -EINVAL;
        smsPath = mMessages[index]->getPath();
    }

    GError* error = nullptr;

    if (mm_modem_messaging_delete_sync(mMmModemMessaging.get(), smsPath.c_str(), NULL, &error) ==
        FALSE) {
        RLOGE("Failed to delete SMS: %s", error->message);
        g_error_free(error);
        return -EINVAL;
    }

    return 0;
}

void ModemMessaging::addMessage(const gchar* path) {
    auto lock = std::unique_lock(mMessagesMutex);

    auto index = pathToIndex(path);
    if (index == -1) return;
    if (mMessages.count(index) != 0) return;

    RLOGD("Adding message %s", path);

    auto* message = (GObject*)g_initable_new(
            MM_TYPE_SMS, NULL, NULL,                                                             //
            "g-flags", G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START,                                     //
            "g-name", MM_DBUS_SERVICE,                                                           //
            "g-connection", g_dbus_proxy_get_connection(G_DBUS_PROXY(mMmModemMessaging.get())),  //
            "g-object-path", path,                                                               //
            "g-interface-name", "org.freedesktop.ModemManager1.Sms",                             //
            NULL);

    if (message == NULL) {
        RLOGE("Failed to create message object for %s", path);
        return;
    }

    auto modemMessage = ModemSms::createInstance(MM_SMS(message));
    mMessages[index] = modemMessage;

    if (mSmsReceivedCallback) mSmsReceivedCallback(modemMessage);

    RLOGD("Added message %s", path);
}

void ModemMessaging::queryMessages() {
    auto* smsPaths = mm_gdbus_modem_messaging_dup_messages(
            MM_GDBUS_MODEM_MESSAGING(mMmModemMessaging.get()));

    if (smsPaths == nullptr) return;

    // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
    for (guint i = 0; smsPaths[i] != nullptr; i++) {
        // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
        addMessage(smsPaths[i]);
    }

    g_strfreev(smsPaths);
}