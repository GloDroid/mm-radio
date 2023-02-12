/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    mm_zbus::{mm_modem_proxy::ModemProxy, mm_proxy::ModemManager1Proxy},
    radio::{
        radio_data::{RadioData, RadioDataShared},
        radio_messaging::{RadioMessaging, RadioMessagingShared},
        radio_modem::{RadioModem, RadioModemShared},
        radio_network::{RadioNetwork, RadioNetworkShared},
        radio_sim::{RadioSim, RadioSimShared},
        radio_voice::{RadioVoice, RadioVoiceShared},
    },
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
use log::{error, info};
use zbus::{
    fdo::{Error, ObjectManagerProxy},
    Connection,
};

macro_rules! add_binder_service {
    ($inst: expr, $obj:ident, $name: expr) => {{
        let service = $obj::new_native_binder($inst);
        let service_name = $obj::get_descriptor().to_string() + format!("/{}", $name).as_str();
        binder::add_service(&service_name, service.as_binder())
            .map_err(|e| format!("Failed to add service {} to binder: {:?}", service_name, e))
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

pub struct RadioHalManager<'a> {
    _proxy: ModemManager1Proxy<'a>,
    omproxy: ObjectManagerProxy<'a>,
    objects: Mutex<Vec<String>>,
    frontends: Mutex<HashMap<String /*slot*/, FrontendBinding>>,
}

pub(crate) fn create_radio_hal_manager<'a>(
    connection: &Connection,
) -> Result<Arc<RadioHalManager<'a>>, Error> {
    let proxy = block_on(ModemManager1Proxy::new(connection))?;
    let omproxy = block_on(
        ObjectManagerProxy::builder(connection)
            .destination("org.freedesktop.ModemManager1")?
            .path("/org/freedesktop/ModemManager1")?
            .build(),
    )?;

    let rhm = Arc::new(RadioHalManager {
        _proxy: proxy,
        omproxy,
        objects: Mutex::new(Vec::new()),
        frontends: Mutex::new(HashMap::new()),
    });

    Ok(rhm)
}

pub(crate) struct Callbacks {
    pub modem_added: fn(rhm: &RadioHalManager, path: &str),
    pub modem_removed: fn(rhm: &RadioHalManager, path: &str),
}

pub(crate) fn register_frontend_element(rhm: &RadioHalManager, name: &str) -> Result<(), String> {
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

    rhm.frontends.lock().unwrap().insert(name.to_string(), feb);

    Ok(())
}

pub(crate) fn bind_modems(rhm: &Arc<RadioHalManager<'static>>) {
    let cbks = Callbacks {
        modem_added: |rhm, path: &str| {
            info!("Adding modem: {}", path);
            let frontends = rhm.frontends.lock().unwrap();
            /* Search for free slot */
            let slot = frontends.iter().find(|(_, feb)| feb.modem_path.borrow().is_none());

            if slot.is_none() {
                error!("No free slot for modem: {}", path);
                return;
            }

            let (slot, feb) = slot.unwrap();

            feb.modem_path.swap(&RefCell::new(Some(path.to_string())));

            let modem_proxy = {
                let builder = ModemProxy::builder(rhm.omproxy.connection());
                block_on(builder.path(path.to_string()).unwrap().build()).unwrap()
            };

            RadioDataShared::bind(&feb.radio_data_shared, &modem_proxy);
            RadioMessagingShared::bind(&feb.radio_messaging_shared, &modem_proxy);
            RadioModemShared::bind(&feb.radio_modem_shared, &modem_proxy);
            RadioNetworkShared::bind(&feb.radio_network_shared, &modem_proxy);
            RadioSimShared::bind(&feb.radio_sim_shared, &modem_proxy);
            RadioVoiceShared::bind(&feb.radio_voice_shared, &modem_proxy);
            info!("Modem \"{}\" bound to slot \"{}\"", path, slot);
        },
        modem_removed: |rhm, path| {
            info!("Removing modem \"{}\"", path);
            let frontends = rhm.frontends.lock().unwrap();

            let feb = frontends.iter().find(|(_, feb)| {
                let feb_path = feb.modem_path.borrow();
                feb_path.is_some() && feb_path.as_ref().unwrap() == path
            });

            if feb.is_none() {
                error!("Modem \"{}\" was not bound", path);
                return;
            }
            let (slot, feb) = feb.unwrap();

            RadioDataShared::unbind(&feb.radio_data_shared);
            RadioMessagingShared::unbind(&feb.radio_messaging_shared);
            RadioModemShared::unbind(&feb.radio_modem_shared);
            RadioNetworkShared::unbind(&feb.radio_network_shared);
            RadioSimShared::unbind(&feb.radio_sim_shared);
            RadioVoiceShared::unbind(&feb.radio_voice_shared);
            info!("Modem \"{}\" unbound from slot \"{}\"", path, slot);
        },
    };
    spawn_object_manager(rhm, cbks).unwrap();
}

fn spawn_object_manager(
    rhmarg: &Arc<RadioHalManager<'static>>,
    cbks: Callbacks,
) -> Result<(), zbus::Error> {
    let rhm = rhmarg.clone();

    let mo = block_on(rhm.omproxy.get_managed_objects())?;
    for (path, _) in mo {
        (cbks.modem_added)(&rhm, &path);
        rhm.objects.lock().unwrap().push(path.to_string());
    }

    spawn(async move {
        let omproxy = rhm.omproxy.clone();
        let added = block_on(omproxy.receive_interfaces_added()).unwrap();
        let removed = block_on(omproxy.receive_interfaces_removed()).unwrap();
        let mut added = added;
        let mut removed = removed;

        loop {
            select! {
                e = added.next().fuse() => {
                    let e = e.unwrap();
                    let path = e.path().unwrap();
                    let mut objects = rhm.objects.lock().unwrap();
                    let index = objects.iter().position(|x| x == &path.to_string());
                    if index.is_some() {
                        (cbks.modem_removed)(&rhm, &path);
                    } else {
                        objects.push(path.to_string());
                    }
                    (cbks.modem_added)(&rhm, &path);
                },
                e = removed.next().fuse() => {
                    let e = e.unwrap();
                    (cbks.modem_removed)(&rhm, &e.path().unwrap());
                    let mut objects = rhm.objects.lock().unwrap();
                    let index = objects.iter().position(|x| x == &e.path().unwrap().to_string());
                    if index.is_some() {
                        objects.remove(index.unwrap());
                    }
                }
                complete => {
                    info!("Complete");
                    break;
                },
            };
        }
    });

    Ok(())
}
