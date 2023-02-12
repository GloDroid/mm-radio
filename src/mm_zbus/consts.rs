/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

#![allow(dead_code)]

// MMModemState
pub(crate) mod mm_modem_state {
    pub(crate) const FAILED: i32 = -1;
    pub(crate) const UNKNOWN: i32 = 0;
    pub(crate) const INITIALIZING: i32 = 1;
    pub(crate) const LOCKED: i32 = 2;
    pub(crate) const DISABLED: i32 = 3;
    pub(crate) const DISABLING: i32 = 4;
    pub(crate) const ENABLING: i32 = 5;
    pub(crate) const ENABLED: i32 = 6;
    pub(crate) const SEARCHING: i32 = 7;
    pub(crate) const REGISTERED: i32 = 8;
    pub(crate) const DISCONNECTING: i32 = 9;
    pub(crate) const CONNECTING: i32 = 10;
    pub(crate) const CONNECTED: i32 = 11;
}

// MMModem3gppUssdSessionState
pub(crate) mod mm_modem_3gpp_ussd_session_state {
    pub(crate) const UNKNOWN: u32 = 0;
    pub(crate) const IDLE: u32 = 1;
    pub(crate) const ACTIVE: u32 = 2;
    pub(crate) const USER_RESPONSE: u32 = 3;
}

// MMCallDirection
pub(crate) mod mm_call_direction {
    pub(crate) const UNKNOWN: i32 = 0;
    pub(crate) const INCOMING: i32 = 1;
    pub(crate) const OUTGOING: i32 = 2;
}

// MMCallState
pub(crate) mod mm_call_state {
    pub(crate) const UNKNOWN: i32 = 0;
    pub(crate) const DIALING: i32 = 1;
    pub(crate) const RINGING_OUT: i32 = 2;
    pub(crate) const RINGING_IN: i32 = 3;
    pub(crate) const ACTIVE: i32 = 4;
    pub(crate) const HELD: i32 = 5;
    pub(crate) const WAITING: i32 = 6;
    pub(crate) const TERMINATED: i32 = 7;
}

// MMSmsState
pub(crate) mod mm_sms_state {
    pub(crate) const UNKNOWN: u32 = 0;
    pub(crate) const STORED: u32 = 1;
    pub(crate) const RECEIVING: u32 = 2;
    pub(crate) const RECEIVED: u32 = 3;
    pub(crate) const SENDING: u32 = 4;
    pub(crate) const SENT: u32 = 5;
}
