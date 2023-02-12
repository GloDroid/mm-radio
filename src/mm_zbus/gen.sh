#!/bin/bash -e

#
# mm-radio HAL (https://github.com/GloDroid/mm-radio)
#
# SPDX-License-Identifier: Apache-2.0
# Copyright (C) 2023 The GloDroid Project
#

MM_INTROSPECTION_DIR=$1

zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Bearer.xml > mm_bearer_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Call.xml > mm_call_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.Firmware.xml > mm_firmware_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.Location.xml > mm_location_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.Messaging.xml > mm_messaging_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.Modem3gpp.ProfileManager.xml > mm_profile_manager_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.Modem3gpp.Ussd.xml > mm_ussd_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.Modem3gpp.xml > mm_modem_3gpp_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.ModemCdma.xml > mm_modem_cdma_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.Oma.xml > mm_oma_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.Sar.xml > mm_sar_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.Signal.xml > mm_signal_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.Simple.xml > mm_simple_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.Time.xml > mm_time_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.Voice.xml > mm_voice_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Modem.xml > mm_modem_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Sim.xml > mm_sim_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.Sms.xml > mm_sms_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/org.freedesktop.ModemManager1.xml > mm_proxy.rs
zbus-xmlgen ${MM_INTROSPECTION_DIR}/wip-org.freedesktop.ModemManager1.Modem.Contacts.xml > mm_contacts_proxy.rs
