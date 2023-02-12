/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use android_hardware_radio::aidl::android::hardware::radio::{
    RadioError::*, RadioResponseInfo::*, RadioResponseType::RadioResponseType,
};

pub fn def<T: Default>() -> T {
    std::default::Default::default()
}

pub(crate) fn respond(serial: i32, error: RadioError) -> RadioResponseInfo {
    RadioResponseInfo { serial, r#type: RadioResponseType::SOLICITED, error }
}

pub(crate) fn resp_ok(serial: i32) -> RadioResponseInfo {
    respond(serial, RadioError::NONE)
}

pub(crate) fn resp_notsup(serial: i32) -> RadioResponseInfo {
    respond(serial, RadioError::REQUEST_NOT_SUPPORTED)
}

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let pos = name.rfind("::{{closure}}").unwrap();
        let name = &name[..pos];
        let pos = name.rfind("::").unwrap() + 2;
        &name[pos..]
    }};
}

macro_rules! sharedmut {
    (&$self:ident) => {
        $self.shared.write().await
    };
}

macro_rules! shared {
    (&$self:ident) => {
        $self.shared.read().await
    };
}

macro_rules! ind {
    (&$self:ident) => {
        shared!(&$self).indication.as_ref().ok_or(binder::StatusCode::NO_INIT)?
    };
}

macro_rules! resp {
    (&$self:ident) => {{
        shared!(&$self).response.as_ref().ok_or(binder::StatusCode::NO_INIT)?
    }};
}

macro_rules! entry_check {
    (&$self: ident, $serial: expr, $respfn: ident $(, $opt:expr)*) => {
        use crate::utils::iradio::{function, respond};
        use log::{error, info};
        use android_hardware_radio::aidl::android::hardware::radio::RadioError::*;

        let shared = shared!(&$self);
        if shared.response.is_none() {
            error!("{}: Response functions wasn't set", function!());
            return Err(binder::Status::from(binder::StatusCode::NO_INIT));
        }
        if !shared.modem_bound {
            info!("{}: Modem wasn't bound", function!());
            shared
                .response
                .as_ref()
                .unwrap()
                .$respfn(&respond($serial, RadioError::RADIO_NOT_AVAILABLE), $($opt), *)?;
            return Ok(());
        }
        info!("{}: {}", function!(), $serial);
        drop(shared);
    };
}

macro_rules! not_implemented {
    (&$self: ident, $serial: expr, $respfn: ident $(, $opt:expr)*) => {{
        use crate::utils::iradio::{entry_check, resp_notsup};
        entry_check!(&$self, $serial, $respfn $(, $opt)*);
        let shared = shared!(&$self);
        shared
            .response
            .as_ref()
            .unwrap()
            .$respfn(&resp_notsup($serial), $($opt), *)?;
        Ok(())
    }};
}

macro_rules! okay {
    (&$self: ident, $serial: expr, $respfn: ident $(, $opt:expr)*) => {{
        use crate::utils::iradio::resp_ok;
        let shared = shared!(&$self);
        shared
            .response
            .as_ref()
            .unwrap()
            .$respfn(&resp_ok($serial), $($opt), *)?;
        Ok(())
    }};
}

macro_rules! err {
    (&$self: ident, $serial: expr, $err: expr, $respfn: ident $(, $opt:expr)*) => {{
        use crate::utils::iradio::respond;
        let shared = shared!(&$self);
        shared
            .response
            .as_ref()
            .unwrap()
            .$respfn(&respond($serial, $err), $($opt), *)?;
        Ok(())
    }};
}

pub(crate) use {entry_check, err, function, ind, not_implemented, okay, resp, shared, sharedmut};

macro_rules! declare_async_iradio {
    ($st:ty, $sti:ident, $stbn:ident) => {
        impl $st {
            pub fn new_native_binder(obj: $st) -> Strong<dyn $sti> {
                $stbn::new_async_binder(
                    obj,
                    crate::utils::binder_async::AsyncStdRuntime,
                    binder::BinderFeatures {
                        set_requesting_sid: true,
                        ..BinderFeatures::default()
                    },
                )
            }
        }

        impl binder::Interface for $st {}
    };
}

pub(crate) use declare_async_iradio;
