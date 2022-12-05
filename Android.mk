# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#


LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)
LOCAL_VENDOR_MODULE := true
LOCAL_MODULE := android.hardware.mm-radio-service
LOCAL_MODULE_RELATIVE_PATH := hw
LOCAL_VINTF_FRAGMENTS := android.hardware.radio.xml
LOCAL_INIT_RC := android.hardware.radio.rc

LOCAL_C_INCLUDES := $(LOCAL_PATH)/.

LOCAL_SRC_FILES := \
    src/RadioConfig.cpp \
    src/RadioData.cpp \
    src/RadioMessaging.cpp \
    src/RadioModem.cpp \
    src/RadioNetwork.cpp \
    src/RadioSim.cpp \
    src/RadioVoice.cpp \
    src/service.cpp \

LOCAL_CFLAGS += \
    -Wno-unused-parameter \
    -Wno-missing-field-initializers \

LOCAL_SHARED_LIBRARIES := \
    libbase \
    libcutils \
    libutils \
    libbinder_ndk \
    android.hardware.radio-V1-ndk \
    android.hardware.radio.config-V1-ndk \
    android.hardware.radio.data-V1-ndk \
    android.hardware.radio.messaging-V1-ndk \
    android.hardware.radio.modem-V1-ndk \
    android.hardware.radio.network-V1-ndk \
    android.hardware.radio.sim-V1-ndk \
    android.hardware.radio.voice-V1-ndk \

include $(BUILD_EXECUTABLE)
