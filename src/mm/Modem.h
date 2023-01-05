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
#include <mm-modem.h>

#include <mm-sim.h>

class ModemSim {
  public:
    static std::shared_ptr<ModemSim> createInstance(SharedGObject<MMSim>& mMmSim);

    std::string getIdentifier();
    std::string getImsi();
    std::string getEid();
    std::string getOperatorIdentifier();
    std::string getOperatorName();

    bool isActive();

  private:
    ModemSim() = default;

    SharedGObject<MMSim> mMmSim;
};

using ModemStateChangedFn = std::function<void(bool)>;

class Modem {
  public:
    static std::shared_ptr<Modem> createInstance(SharedMmObject& mmObject);

    std::string getEquipmentId();
    std::string getDeviceIdentifier();
    std::string getPlugin();
    std::string getPrimaryPort();
    std::string getRevision();
    std::string getManufacturer();
    std::string getModel();
    std::string getDevice();

    void dump();
    void enable();
    void disable();

    std::shared_ptr<ModemSim> getSim();

    void refreshModemState();

    void setModemStateChangedCb(ModemStateChangedFn callback);

  private:
    Modem() = default;

    static void enableCallback(MMModem* modem, GAsyncResult* result, Modem* self);
    static void disableCallback(MMModem* modem, GAsyncResult* result, Modem* self);

    ModemStateChangedFn mModemStateChangedCb;
    SharedGObject<MMModem> mMmModem;
};
