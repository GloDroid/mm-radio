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

#include <functional>
#include <memory>

#include "MmRaii.h"
#include "ModemObject.h"

class ModemManager {
  public:
    static auto createInstance() -> std::shared_ptr<ModemManager>;

    void setModemCallbacks(std::function<void(std::shared_ptr<ModemObject>&)> modemAdded,
                           std::function<void(std::shared_ptr<ModemObject>&)> modemRemoved) {
        mModemAddedCallback = std::move(modemAdded);
        mModemRemovedCallback = std::move(modemRemoved);
    }

    void invokeAddedCallbackForAvailableModems();

  private:
    ModemManager() = default;

    static void modemAdded(MMManager* manager, MMObject* object, ModemManager* self);
    static void modemRemoved(MMManager* manager, MMObject* object, ModemManager* self);

    static auto openConnection() -> SharedGDBusConnection;
    static auto createManager(GDBusConnection* connection) -> SharedMmManager;

    SharedGDBusConnection mConnection;
    SharedMmManager mManager;

    std::function<void(std::shared_ptr<ModemObject>&)> mModemAddedCallback;
    std::function<void(std::shared_ptr<ModemObject>&)> mModemRemovedCallback;

    std::vector<std::shared_ptr<ModemObject>> mModems;
};
