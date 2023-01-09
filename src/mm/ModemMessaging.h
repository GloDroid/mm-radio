// SPDX-License-Identifier: Apache-2.0

#pragma once

#include "MmRaii.h"

#include <libmm-glib.h>

class ModemMessaging {
  public:
    static auto createInstance(SharedMmObject& mmObject) -> std::shared_ptr<ModemMessaging>;

  private:
    ModemMessaging() = default;

    SharedGObject<MMModemMessaging> mMmModemMessaging;
};
