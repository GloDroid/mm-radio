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

#include "Modem.h"

#include <cutils/log.h>

std::shared_ptr<ModemSim> ModemSim::createInstance(SharedGObject<MMSim>& mMmSim) {
    auto modemInstance = std::shared_ptr<ModemSim>(new ModemSim());
    modemInstance->mMmSim = mMmSim;

    return modemInstance;
}

std::string ModemSim::getIdentifier() {
    return mm_sim_get_identifier(mMmSim.get());
}
std::string ModemSim::getImsi() {
    return mm_sim_get_imsi(mMmSim.get());
}
std::string ModemSim::getEid() {
    const auto* eid = mm_sim_get_eid(mMmSim.get());
    return (eid != nullptr) ? eid : "";
}
std::string ModemSim::getOperatorIdentifier() {
    return mm_sim_get_operator_identifier(mMmSim.get());
}
std::string ModemSim::getOperatorName() {
    return mm_sim_get_operator_name(mMmSim.get());
}

bool ModemSim::isActive() {
    return mm_sim_get_active(mMmSim.get()) != 0;
}

std::shared_ptr<Modem> Modem::createInstance(SharedMmObject& mmObject) {
    auto modemInstance = std::shared_ptr<Modem>(new Modem());
    modemInstance->mMmModem = makeSharedGObject<MMModem>(mm_object_get_modem(mmObject.get()));

    modemInstance->mModemStateChangedCb = [](bool) {
        RLOGE("Unhandled modem state changed callback");
    };

    return modemInstance;
}

std::string Modem::getEquipmentId() {
    return mm_modem_get_equipment_identifier(mMmModem.get());
}
std::string Modem::getDeviceIdentifier() {
    return mm_modem_get_device_identifier(mMmModem.get());
}
std::string Modem::getPlugin() {
    return mm_modem_get_plugin(mMmModem.get());
}
std::string Modem::getPrimaryPort() {
    return mm_modem_get_primary_port(mMmModem.get());
}
std::string Modem::getRevision() {
    return mm_modem_get_revision(mMmModem.get());
}
std::string Modem::getManufacturer() {
    return mm_modem_get_manufacturer(mMmModem.get());
}
std::string Modem::getModel() {
    return mm_modem_get_model(mMmModem.get());
}
std::string Modem::getDevice() {
    return mm_modem_get_device(mMmModem.get());
}

void Modem::dump() {
    RLOGD("Modem:");
    RLOGD("  Equipment ID: %s", getEquipmentId().c_str());
    RLOGD("  Device ID: %s", getDeviceIdentifier().c_str());
    RLOGD("  Plugin: %s", getPlugin().c_str());
    RLOGD("  Primary port: %s", getPrimaryPort().c_str());
    RLOGD("  Revision: %s", getRevision().c_str());
    RLOGD("  Manufacturer: %s", getManufacturer().c_str());
    RLOGD("  Model: %s", getModel().c_str());
    RLOGD("  Device: %s", getDevice().c_str());
}

void Modem::enable() {
    mm_modem_enable(mMmModem.get(), nullptr, (GAsyncReadyCallback)enableCallback, this);
}

void Modem::disable() {
    mm_modem_disable(mMmModem.get(), nullptr, (GAsyncReadyCallback)disableCallback, this);
}

std::shared_ptr<ModemSim> Modem::getSim() {
    GError* error = nullptr;
    auto sim = makeSharedGObject(mm_modem_get_sim_sync(mMmModem.get(), nullptr, &error));
    if (error != nullptr) {
        RLOGE("Failed to get SIM: %s", error->message);
        g_error_free(error);
        return {};
    }
    return ModemSim::createInstance(sim);
}

void Modem::setModemStateChangedCb(ModemStateChangedFn callback) {
    mModemStateChangedCb = std::move(callback);
}

void Modem::enableCallback(MMModem* modem, GAsyncResult* result, Modem* self) {
    GError* error = nullptr;
    mm_modem_enable_finish(modem, result, &error);
    if (error != nullptr) {
        RLOGE("Failed to enable modem: %s", error->message);
        g_error_free(error);
        return;
    }
    if (self->mModemStateChangedCb) self->mModemStateChangedCb(true);
}

void Modem::disableCallback(MMModem* modem, GAsyncResult* result, Modem* self) {
    GError* error = nullptr;
    mm_modem_disable_finish(modem, result, &error);
    if (error != nullptr) {
        RLOGE("Failed to disable modem: %s", error->message);
        g_error_free(error);
        return;
    }
    if (self->mModemStateChangedCb) self->mModemStateChangedCb(false);
}

void Modem::refreshModemState() {
    if (!mModemStateChangedCb) return;

    auto state = mm_modem_get_state(mMmModem.get());
    mModemStateChangedCb(state == MM_MODEM_STATE_ENABLED);
}