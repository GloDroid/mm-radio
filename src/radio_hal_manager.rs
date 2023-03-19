/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2023 The GloDroid Project
 */

use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    mm_zbus::mm_modem_proxy::ModemProxy,
    radio::{
        radio_data::{RadioData, RadioDataShared},
        radio_messaging::{RadioMessaging, RadioMessagingShared},
        radio_modem::{RadioModem, RadioModemShared},
        radio_network::{RadioNetwork, RadioNetworkShared},
        radio_sim::{RadioSim, RadioSimShared},
        radio_voice::{RadioVoice, RadioVoiceShared},
    },
    utils::error::Error,
};
use android_hardware_radio_data::aidl::android::hardware::radio::data::IRadioData::*;
use android_hardware_radio_messaging::aidl::android::hardware::radio::messaging::IRadioMessaging::*;
use android_hardware_radio_modem::aidl::android::hardware::radio::modem::IRadioModem::*;
use android_hardware_radio_network::aidl::android::hardware::radio::network::IRadioNetwork::*;
use android_hardware_radio_sim::aidl::android::hardware::radio::sim::IRadioSim::*;
use android_hardware_radio_voice::aidl::android::hardware::radio::voice::IRadioVoice::*;

use async_std::{
    stream::StreamExt,
    sync::RwLock,
    task::{block_on, spawn},
};

use futures::{select, FutureExt};
use log::info;
use zbus::{fdo::ObjectManagerProxy, Connection};

macro_rules! add_binder_service {
    ($inst: expr, $obj:ident, $name: expr) => {{
        let service = $obj::new_native_binder($inst);
        let service_name = $obj::get_descriptor().to_string() + format!("/{}", $name).as_str();
        binder::add_service(&service_name, service.as_binder())
            .map_err(|e| {
                Error::new(
                    format!("Failed to add service {} to binder: {:?}", service_name, e).as_str(),
                )
            })
            .map(|_| service)
    }};
}

pub(crate) use add_binder_service;

struct FrontendBinding {
    radio_data_shared: Arc<RwLock<RadioDataShared>>,
    radio_messaging_shared: Arc<RwLock<RadioMessagingShared>>,
    radio_modem_shared: Arc<RwLock<RadioModemShared>>,
    radio_network_shared: Arc<RwLock<RadioNetworkShared>>,
    radio_sim_shared: Arc<RwLock<RadioSimShared>>,
    radio_voice_shared: Arc<RwLock<RadioVoiceShared>>,

    modem_path: RefCell<Option<String>>,
}

pub struct RadioHalManager {
    objects: Mutex<Vec<String>>,
    frontends: Mutex<HashMap<String /*slot*/, FrontendBinding>>,
}

pub(crate) fn create_radio_hal_manager() -> Result<Arc<RadioHalManager>, Error> {
    let rhm = Arc::new(RadioHalManager {
        objects: Mutex::new(Vec::new()),
        frontends: Mutex::new(HashMap::new()),
    });

    Ok(rhm)
}

pub(crate) struct Callbacks {
    pub modem_added:
        fn(connection: &Connection, rhm: &RadioHalManager, path: &str) -> Result<(), Error>,
    pub modem_removed: fn(rhm: &RadioHalManager, path: &str) -> Result<(), Error>,
}

pub(crate) fn register_frontend_element(rhm: &RadioHalManager, name: &str) -> Result<(), Error> {
    let rd = RadioData::default();
    let rm: RadioMessaging = Default::default();
    let rmdm: RadioModem = Default::default();
    let rn: RadioNetwork = Default::default();
    let rs: RadioSim = Default::default();
    let rv: RadioVoice = Default::default();

    let feb = FrontendBinding {
        radio_data_shared: rd.shared.clone(),
        radio_messaging_shared: rm.shared.clone(),
        radio_modem_shared: rmdm.shared.clone(),
        radio_network_shared: rn.shared.clone(),
        radio_sim_shared: rs.shared.clone(),
        radio_voice_shared: rv.shared.clone(),

        modem_path: RefCell::new(None),
    };

    add_binder_service!(rd, RadioData, name)?;
    add_binder_service!(rm, RadioMessaging, name)?;
    add_binder_service!(rmdm, RadioModem, name)?;
    add_binder_service!(rn, RadioNetwork, name)?;
    add_binder_service!(rs, RadioSim, name)?;
    add_binder_service!(rv, RadioVoice, name)?;

    rhm.frontends.lock().map_err(|_| Error::poisoned())?.insert(name.to_string(), feb);

    Ok(())
}

pub(crate) fn bind_modems(rhm: &Arc<RadioHalManager>) -> Result<(), Error> {
    let cbks = Callbacks {
        modem_added: |connection, rhm, path: &str| -> Result<(), Error> {
            info!("Adding modem: {}", path);
            let frontends = rhm.frontends.lock().map_err(|_| Error::poisoned())?;
            /* Search for free slot */
            let slot = frontends.iter().find(|(_, feb)| feb.modem_path.borrow().is_none());

            let (slot, feb) =
                slot.ok_or(Error::new(format!("No free slot for modem: {path}").as_str()))?;

            feb.modem_path.swap(&RefCell::new(Some(path.to_string())));

            let modem_proxy = {
                let builder = ModemProxy::builder(connection);
                block_on(builder.path(path.to_string())?.build())?
            };

            RadioDataShared::bind(&feb.radio_data_shared, &modem_proxy)?;
            RadioMessagingShared::bind(&feb.radio_messaging_shared, &modem_proxy)?;
            RadioModemShared::bind(&feb.radio_modem_shared, &modem_proxy)?;
            RadioNetworkShared::bind(&feb.radio_network_shared, &modem_proxy)?;
            RadioSimShared::bind(&feb.radio_sim_shared, &modem_proxy)?;
            RadioVoiceShared::bind(&feb.radio_voice_shared, &modem_proxy)?;
            info!("Modem \"{}\" bound to slot \"{}\"", path, slot);
            Ok(())
        },
        modem_removed: |rhm, path| -> Result<(), Error> {
            info!("Removing modem \"{}\"", path);
            let frontends = rhm.frontends.lock().map_err(|_| Error::poisoned())?;

            let feb = frontends.iter().find(|(_, feb)| {
                if let Some(fp) = feb.modem_path.borrow().as_deref() {
                    fp == path
                } else {
                    false
                }
            });

            let (slot, feb) =
                feb.ok_or(Error::new(format!("Modem \"{path}\" was not bound").as_str()))?;

            RadioDataShared::unbind(&feb.radio_data_shared)?;
            RadioMessagingShared::unbind(&feb.radio_messaging_shared)?;
            RadioModemShared::unbind(&feb.radio_modem_shared)?;
            RadioNetworkShared::unbind(&feb.radio_network_shared)?;
            RadioSimShared::unbind(&feb.radio_sim_shared)?;
            RadioVoiceShared::unbind(&feb.radio_voice_shared)?;

            feb.modem_path.swap(&RefCell::new(None));
            info!("Modem \"{}\" unbound from slot \"{}\"", path, slot);
            Ok(())
        },
    };
    spawn_object_manager(rhm, cbks)?;
    Ok(())
}

fn spawn_object_manager(rhmarg: &Arc<RadioHalManager>, cbks: Callbacks) -> Result<(), Error> {
    let rhm = rhmarg.clone();

    spawn(async move {
        loop {
            let result: Result<(), Error> = try {
                let connection = block_on(zbus::Connection::system())?;

                let omproxy = block_on(
                    ObjectManagerProxy::builder(&connection)
                        .destination("org.freedesktop.ModemManager1")?
                        .path("/org/freedesktop/ModemManager1")?
                        .build(),
                )?;

                let mo = block_on(omproxy.get_managed_objects())?;
                for (path, _) in mo {
                    let result = (cbks.modem_added)(&connection, &rhm, &path);
                    if let Err(e) = result {
                        e.log();
                        continue;
                    }
                    rhm.objects.lock().map_err(|_| Error::poisoned())?.push(path.to_string());
                }

                let omproxy = omproxy.clone();
                let added = block_on(omproxy.receive_interfaces_added())?;
                let removed = block_on(omproxy.receive_interfaces_removed())?;
                let mut added = added;
                let mut removed = removed;
                loop {
                    select! {
                        e = added.next().fuse() => {
                            let e = e.ok_or(Error::noneopt())?;
                            let path = e.args()?.object_path;
                            let mut objects = rhm.objects.lock().map_err(|_| Error::poisoned())?;
                            let index = objects.iter().position(|x| x == &path.to_string());
                            if index.is_some() {
                                let result = (cbks.modem_removed)(&rhm, &path);
                                if let Err(e) = result {
                                    e.log();
                                    continue;
                                }
                            }
                            let result = (cbks.modem_added)(&connection, &rhm, &path);
                            if let Err(e) = result {
                                e.log();
                                continue;
                            }
                            if index.is_none() {
                                objects.push(path.to_string());
                            }
                        },
                        e = removed.next().fuse() => {
                            let e = e.ok_or(Error::noneopt())?;
                            let path = e.args()?.object_path;
                            let result = (cbks.modem_removed)(&rhm, &path);
                            if let Err(e) = result {
                                e.log();
                                continue;
                            }
                            let mut objects = rhm.objects.lock().map_err(|_| Error::poisoned())?;
                            let path = e.path().ok_or(Error::noneopt())?.to_string();
                            let index = objects.iter().position(|x| x == &path);
                            if let Some(index) = index {
                                objects.remove(index);
                            }
                        }
                        complete => {
                            info!("Complete");
                            break;
                        },
                    };
                }
            };
            if let Err(err) = result {
                err.log();
                info!("Failed to subscribe to modemmanager events. Retry in 10s.");
                async_std::task::sleep(Duration::from_secs(10)).await;
                continue;
            };
            break;
        }
    });

    Ok(())
}
