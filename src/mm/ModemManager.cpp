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

#define LOG_TAG "mm-radio: modem-manager"

#include "ModemManager.h"

#include <memory>
#include <vector>

#include <cutils/log.h>
#include <glib.h>
#include <libmm-glib.h>

#include "ModemObject.h"

auto ModemManager::createInstance() -> std::shared_ptr<ModemManager> {
    auto modemManager = std::shared_ptr<ModemManager>(new ModemManager());
    modemManager->mConnection = ModemManager::openConnection();
    if (!modemManager->mConnection) {
        RLOGE("Failed to open connection to ModemManager");
        return {};
    }
    RLOGD("Opened connection to ModemManager");

    modemManager->mManager = ModemManager::createManager(modemManager->mConnection.get());
    if (!modemManager->mManager) {
        RLOGE("Failed to create ModemManager");
        return {};
    }
    RLOGD("Created ModemManager instance");

    auto* objects =
            g_dbus_object_manager_get_objects(G_DBUS_OBJECT_MANAGER(modemManager->mManager.get()));
    for (auto* object = objects; object != nullptr; object = object->next) {
        ModemManager::modemAdded(modemManager->mManager.get(), MM_OBJECT(object->data),
                                 modemManager.get());
    }

    g_signal_connect(modemManager->mManager.get(), "object-added", G_CALLBACK(modemAdded),
                     modemManager.get());
    g_signal_connect(modemManager->mManager.get(), "object-removed", G_CALLBACK(modemRemoved),
                     modemManager.get());

    return modemManager;
}

SharedGDBusConnection ModemManager::openConnection() {
    GError* error = nullptr;
    auto connection = makeSharedGDBusConnection(g_bus_get_sync(G_BUS_TYPE_SYSTEM, nullptr, &error));
    if (error != nullptr) {
        RLOGE("Failed to open connection: %s", error->message);
        g_error_free(error);
        return {};
    }
    return connection;
}

SharedMmManager ModemManager::createManager(GDBusConnection* connection) {
    GError* error = nullptr;
    auto manager = makeSharedMmManager(mm_manager_new_sync(
            connection, G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_NONE, nullptr, &error));
    if (error != nullptr) {
        RLOGE("Failed to create manager: %s", error->message);
        g_error_free(error);
        return {};
    }
    return manager;
}

void ModemManager::modemAdded(MMManager* /*manager*/, MMObject* object, ModemManager* self) {
    RLOGD("Modem added");
    auto mmObject = makeSharedMmObject(object);
    auto modem = ModemObject::createInstance(mmObject);
    if (!modem) {
        RLOGE("Failed to create ModemObject");
        return;
    }
    self->mModems.emplace_back(modem);

    if (self->mModemAddedCallback) self->mModemAddedCallback(modem);
}

void ModemManager::modemRemoved(MMManager* /*manager*/, MMObject* object, ModemManager* self) {
    RLOGD("Modem removed");
    auto it = std::find_if(self->mModems.begin(), self->mModems.end(),
                           [object](const std::shared_ptr<ModemObject>& modem) {
                               return modem->getPath() == mm_object_get_path(object);
                           });
    if (it == self->mModems.end()) {
        RLOGE("Modem not found");
        return;
    }

    if (self->mModemRemovedCallback) self->mModemRemovedCallback(*it);

    self->mModems.erase(it);
}

void ModemManager::invokeAddedCallbackForAvailableModems() {
    for (auto& modem : mModems) {
        if (mModemAddedCallback) mModemAddedCallback(modem);
    }
}