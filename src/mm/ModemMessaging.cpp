// SPDX-License-Identifier: Apache-2.0

#include "ModemMessaging.h"

#include <libmm-glib.h>

auto ModemMessaging::createInstance(SharedMmObject& mmObject) -> std::shared_ptr<ModemMessaging> {
    auto self = std::shared_ptr<ModemMessaging>(new ModemMessaging());
    self->mMmModemMessaging = makeSharedGObject(mm_object_get_modem_messaging(mmObject.get()));

    return self;
}
