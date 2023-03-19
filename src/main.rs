/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2023 The GloDroid Project
 */

#![warn(clippy::unwrap_used, clippy::expect_used)]
#![feature(async_closure)]
#![feature(try_blocks)]

mod mm_zbus;
mod radio;
mod radio_hal_manager;
mod utils;
use crate::{
    radio::radio_config::RadioConfig,
    radio_hal_manager::{bind_modems, create_radio_hal_manager, register_frontend_element},
};
use android_hardware_radio_config::aidl::android::hardware::radio::config::IRadioConfig::*;

use android_logger::LogId;
use async_std::task::block_on;
use log::{error, info};
use utils::error::Error;

fn main() {
    // Initialize android logging.
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("mm-radio")
            .with_max_level(log::LevelFilter::Debug)
            .with_log_buffer(LogId::Radio)
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{} ({}:{})",
                    record.args(),
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                )
            }),
    );

    // Redirect panic messages to logcat.
    std::panic::set_hook(Box::new(|panic_info| {
        error!("{}", panic_info);
    }));

    let result: Result<(), Error> = try {
        // Android use different path for dbus socket, set it via env variable.
        if std::env::var("DBUS_SYSTEM_BUS_ADDRESS").is_err() {
            std::env::set_var(
                "DBUS_SYSTEM_BUS_ADDRESS",
                "unix:path=/mnt/var/run/dbus/system_bus_socket",
            );
        }

        info!("mm-radio is starting.");

        let connection = block_on(zbus::Connection::system())?;
        let rhm = create_radio_hal_manager(&connection)?;

        radio_hal_manager::add_binder_service!(RadioConfig::default(), RadioConfig, "default")?;
        const MAX_SLOT: usize = 3;
        const FIRST_SLOT: usize = 1;
        let mut slots_registered = 0;
        for slot in FIRST_SLOT..MAX_SLOT {
            if register_frontend_element(&rhm, format!("slot{slot}").as_str()).is_err() {
                break;
            }
            slots_registered += 1;
        }

        if slots_registered == 0 {
            panic!("Failed to register any slot, please check your manifest");
        }

        info!("Registered {} slots for modems", slots_registered);

        bind_modems(&rhm)?;

        info!("Successfully registered mm-radio service.");

        info!("Joining thread pool now.");
        binder::ProcessState::join_thread_pool();

        panic!("Should not reach here.");
    };
    result.unwrap_or_else(|e| e.log());
}
