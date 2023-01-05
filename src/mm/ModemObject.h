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
#include "Modem.h"
#include "ModemVoice.h"

class ModemObject {
  public:
    static std::shared_ptr<ModemObject> createInstance(SharedMmObject& mmObject) {
        auto modemInstance = std::shared_ptr<ModemObject>(new ModemObject(mmObject));

        modemInstance->mModem = Modem::createInstance(mmObject);
        modemInstance->mModemVoice = ModemVoice::createInstance(mmObject);

        modemInstance->mModemPath = mm_object_get_path(mmObject.get());

        return modemInstance;
    }

    auto getIndex() -> int {
        return std::stoi(mModemPath.substr(mModemPath.find_last_of('/') + 1));
    }

    auto& getPath() { return mModemPath; }
    auto& getModem() { return mModem; }
    auto& getVoice() { return mModemVoice; }

  private:
    explicit ModemObject(SharedMmObject& mmObject) : mMmObject(mmObject) {}

    std::string mModemPath;

    SharedMmObject mMmObject;

    std::shared_ptr<Modem> mModem;
    std::shared_ptr<ModemVoice> mModemVoice;
};
