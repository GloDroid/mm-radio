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

#include <memory>

#include <android-base/logging.h>
#include <libmm-glib.h>

template <typename T>
struct GObjectDeleter {
    void operator()(T* ptr) {
        if (ptr) {
            g_object_unref(ptr);
        }
    }
};

template <typename T>
using SharedGObject = std::shared_ptr<T>;

template <typename T>
inline SharedGObject<T> makeSharedGObject(T* ptr) {
    return SharedGObject<T>(ptr, GObjectDeleter<T>());
}

using SharedGDBusConnection = SharedGObject<GDBusConnection>;
using SharedMmManager = SharedGObject<MMManager>;
using SharedGdbusObject = SharedGObject<GDBusObject>;
using SharedMmObject = SharedGObject<MMObject>;

inline auto makeSharedGDBusConnection(GDBusConnection* connection) {
    return SharedGObject<GDBusConnection>(connection, GObjectDeleter<GDBusConnection>());
}

inline auto makeSharedMmManager(MMManager* manager) {
    return SharedGObject<MMManager>(manager, GObjectDeleter<MMManager>());
}

inline auto makeSharedGdbusObject(GDBusObject* object) {
    return SharedGObject<GDBusObject>(object, GObjectDeleter<GDBusObject>());
}

inline auto makeSharedMmObject(MMObject* object) {
    return SharedGObject<MMObject>(object, GObjectDeleter<MMObject>());
}
