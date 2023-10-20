#![feature(custom_inner_attributes)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod hardware {
      pub mod radio {
        pub mod sim {
          pub mod AppStatus {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#AppStatus {
              pub r#appType: i32,
              pub r#appState: i32,
              pub r#persoSubstate: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate,
              pub r#aidPtr: String,
              pub r#appLabelPtr: String,
              pub r#pin1Replaced: bool,
              pub r#pin1: crate::mangled::_7_android_8_hardware_5_radio_3_sim_8_PinState,
              pub r#pin2: crate::mangled::_7_android_8_hardware_5_radio_3_sim_8_PinState,
            }
            pub const r#APP_STATE_UNKNOWN: i32 = 0;
            pub const r#APP_STATE_DETECTED: i32 = 1;
            pub const r#APP_STATE_PIN: i32 = 2;
            pub const r#APP_STATE_PUK: i32 = 3;
            pub const r#APP_STATE_SUBSCRIPTION_PERSO: i32 = 4;
            pub const r#APP_STATE_READY: i32 = 5;
            pub const r#APP_TYPE_UNKNOWN: i32 = 0;
            pub const r#APP_TYPE_SIM: i32 = 1;
            pub const r#APP_TYPE_USIM: i32 = 2;
            pub const r#APP_TYPE_RUIM: i32 = 3;
            pub const r#APP_TYPE_CSIM: i32 = 4;
            pub const r#APP_TYPE_ISIM: i32 = 5;
            impl Default for r#AppStatus {
              fn default() -> Self {
                Self {
                  r#appType: 0,
                  r#appState: 0,
                  r#persoSubstate: Default::default(),
                  r#aidPtr: Default::default(),
                  r#appLabelPtr: Default::default(),
                  r#pin1Replaced: false,
                  r#pin1: Default::default(),
                  r#pin2: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#AppStatus {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#appType)?;
                  subparcel.write(&self.r#appState)?;
                  subparcel.write(&self.r#persoSubstate)?;
                  subparcel.write(&self.r#aidPtr)?;
                  subparcel.write(&self.r#appLabelPtr)?;
                  subparcel.write(&self.r#pin1Replaced)?;
                  subparcel.write(&self.r#pin1)?;
                  subparcel.write(&self.r#pin2)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#appType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#appState = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#persoSubstate = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#aidPtr = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#appLabelPtr = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#pin1Replaced = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#pin1 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#pin2 = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#AppStatus);
            binder::impl_deserialize_for_parcelable!(r#AppStatus);
            impl binder::binder_impl::ParcelableMetadata for r#AppStatus {
              fn get_descriptor() -> &'static str { "android.hardware.radio.sim.AppStatus" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#AppStatus as _7_android_8_hardware_5_radio_3_sim_9_AppStatus;
            }
          }
          pub mod CardPowerState {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#CardPowerState : [i32; 3] {
                r#POWER_DOWN = 0,
                r#POWER_UP = 1,
                r#POWER_UP_PASS_THROUGH = 2,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#CardPowerState as _7_android_8_hardware_5_radio_3_sim_14_CardPowerState;
            }
          }
          pub mod CardStatus {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CardStatus {
              pub r#cardState: i32,
              pub r#universalPinState: crate::mangled::_7_android_8_hardware_5_radio_3_sim_8_PinState,
              pub r#gsmUmtsSubscriptionAppIndex: i32,
              pub r#cdmaSubscriptionAppIndex: i32,
              pub r#imsSubscriptionAppIndex: i32,
              pub r#applications: Vec<crate::mangled::_7_android_8_hardware_5_radio_3_sim_9_AppStatus>,
              pub r#atr: String,
              pub r#iccid: String,
              pub r#eid: String,
              pub r#slotMap: crate::mangled::_7_android_8_hardware_5_radio_6_config_15_SlotPortMapping,
            }
            pub const r#STATE_ABSENT: i32 = 0;
            pub const r#STATE_PRESENT: i32 = 1;
            pub const r#STATE_ERROR: i32 = 2;
            pub const r#STATE_RESTRICTED: i32 = 3;
            impl Default for r#CardStatus {
              fn default() -> Self {
                Self {
                  r#cardState: 0,
                  r#universalPinState: Default::default(),
                  r#gsmUmtsSubscriptionAppIndex: 0,
                  r#cdmaSubscriptionAppIndex: 0,
                  r#imsSubscriptionAppIndex: 0,
                  r#applications: Default::default(),
                  r#atr: Default::default(),
                  r#iccid: Default::default(),
                  r#eid: Default::default(),
                  r#slotMap: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CardStatus {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cardState)?;
                  subparcel.write(&self.r#universalPinState)?;
                  subparcel.write(&self.r#gsmUmtsSubscriptionAppIndex)?;
                  subparcel.write(&self.r#cdmaSubscriptionAppIndex)?;
                  subparcel.write(&self.r#imsSubscriptionAppIndex)?;
                  subparcel.write(&self.r#applications)?;
                  subparcel.write(&self.r#atr)?;
                  subparcel.write(&self.r#iccid)?;
                  subparcel.write(&self.r#eid)?;
                  subparcel.write(&self.r#slotMap)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cardState = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#universalPinState = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#gsmUmtsSubscriptionAppIndex = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cdmaSubscriptionAppIndex = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#imsSubscriptionAppIndex = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#applications = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#atr = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#iccid = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#eid = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#slotMap = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CardStatus);
            binder::impl_deserialize_for_parcelable!(r#CardStatus);
            impl binder::binder_impl::ParcelableMetadata for r#CardStatus {
              fn get_descriptor() -> &'static str { "android.hardware.radio.sim.CardStatus" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CardStatus as _7_android_8_hardware_5_radio_3_sim_10_CardStatus;
            }
          }
          pub mod Carrier {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#Carrier {
              pub r#mcc: String,
              pub r#mnc: String,
              pub r#matchType: i32,
              pub r#matchData: String,
            }
            pub const r#MATCH_TYPE_ALL: i32 = 0;
            pub const r#MATCH_TYPE_SPN: i32 = 1;
            pub const r#MATCH_TYPE_IMSI_PREFIX: i32 = 2;
            pub const r#MATCH_TYPE_GID1: i32 = 3;
            pub const r#MATCH_TYPE_GID2: i32 = 4;
            impl Default for r#Carrier {
              fn default() -> Self {
                Self {
                  r#mcc: Default::default(),
                  r#mnc: Default::default(),
                  r#matchType: 0,
                  r#matchData: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#Carrier {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#mcc)?;
                  subparcel.write(&self.r#mnc)?;
                  subparcel.write(&self.r#matchType)?;
                  subparcel.write(&self.r#matchData)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#mcc = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mnc = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#matchType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#matchData = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Carrier);
            binder::impl_deserialize_for_parcelable!(r#Carrier);
            impl binder::binder_impl::ParcelableMetadata for r#Carrier {
              fn get_descriptor() -> &'static str { "android.hardware.radio.sim.Carrier" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Carrier as _7_android_8_hardware_5_radio_3_sim_7_Carrier;
            }
          }
          pub mod CarrierRestrictions {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CarrierRestrictions {
              pub r#allowedCarriers: Vec<crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_Carrier>,
              pub r#excludedCarriers: Vec<crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_Carrier>,
              pub r#allowedCarriersPrioritized: bool,
            }
            impl Default for r#CarrierRestrictions {
              fn default() -> Self {
                Self {
                  r#allowedCarriers: Default::default(),
                  r#excludedCarriers: Default::default(),
                  r#allowedCarriersPrioritized: false,
                }
              }
            }
            impl binder::Parcelable for r#CarrierRestrictions {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#allowedCarriers)?;
                  subparcel.write(&self.r#excludedCarriers)?;
                  subparcel.write(&self.r#allowedCarriersPrioritized)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#allowedCarriers = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#excludedCarriers = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#allowedCarriersPrioritized = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CarrierRestrictions);
            binder::impl_deserialize_for_parcelable!(r#CarrierRestrictions);
            impl binder::binder_impl::ParcelableMetadata for r#CarrierRestrictions {
              fn get_descriptor() -> &'static str { "android.hardware.radio.sim.CarrierRestrictions" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CarrierRestrictions as _7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions;
            }
          }
          pub mod CdmaSubscriptionSource {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#CdmaSubscriptionSource : [i32; 2] {
                r#RUIM_SIM = 0,
                r#NV = 1,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaSubscriptionSource as _7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource;
            }
          }
          pub mod IRadioSim {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioSim["android.hardware.radio.sim.IRadioSim"] {
                native: BnRadioSim(on_transact),
                proxy: BpRadioSim {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioSimAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioSim: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.sim.IRadioSim" }
              fn r#areUiccApplicationsEnabled(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#changeIccPin2ForApp(&self, _arg_serial: i32, _arg_oldPin2: &str, _arg_newPin2: &str, _arg_aid: &str) -> binder::Result<()>;
              fn r#changeIccPinForApp(&self, _arg_serial: i32, _arg_oldPin: &str, _arg_newPin: &str, _arg_aid: &str) -> binder::Result<()>;
              fn r#enableUiccApplications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()>;
              fn r#getAllowedCarriers(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getCdmaSubscription(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getCdmaSubscriptionSource(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<()>;
              fn r#getIccCardStatus(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getImsiForApp(&self, _arg_serial: i32, _arg_aid: &str) -> binder::Result<()>;
              fn r#getSimPhonebookCapacity(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getSimPhonebookRecords(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#iccCloseLogicalChannel(&self, _arg_serial: i32, _arg_channelId: i32) -> binder::Result<()>;
              fn r#iccIoForApp(&self, _arg_serial: i32, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_5_IccIo) -> binder::Result<()>;
              fn r#iccOpenLogicalChannel(&self, _arg_serial: i32, _arg_aid: &str, _arg_p2: i32) -> binder::Result<()>;
              fn r#iccTransmitApduBasicChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<()>;
              fn r#iccTransmitApduLogicalChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<()>;
              fn r#reportStkServiceIsRunning(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#requestIccSimAuthentication(&self, _arg_serial: i32, _arg_authContext: i32, _arg_authData: &str, _arg_aid: &str) -> binder::Result<()>;
              fn r#responseAcknowledgement(&self) -> binder::Result<()>;
              fn r#sendEnvelope(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()>;
              fn r#sendEnvelopeWithStatus(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()>;
              fn r#sendTerminalResponseToSim(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()>;
              fn r#setAllowedCarriers(&self, _arg_serial: i32, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<()>;
              fn r#setCarrierInfoForImsiEncryption(&self, _arg_serial: i32, _arg_imsiEncryptionInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_18_ImsiEncryptionInfo) -> binder::Result<()>;
              fn r#setCdmaSubscriptionSource(&self, _arg_serial: i32, _arg_cdmaSub: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()>;
              fn r#setFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_lockState: bool, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<()>;
              fn r#setResponseFunctions(&self, _arg_radioSimResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_IRadioSimResponse>, _arg_radioSimIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_IRadioSimIndication>) -> binder::Result<()>;
              fn r#setSimCardPower(&self, _arg_serial: i32, _arg_powerUp: crate::mangled::_7_android_8_hardware_5_radio_3_sim_14_CardPowerState) -> binder::Result<()>;
              fn r#setUiccSubscription(&self, _arg_serial: i32, _arg_uiccSub: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_SelectUiccSub) -> binder::Result<()>;
              fn r#supplyIccPin2ForApp(&self, _arg_serial: i32, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<()>;
              fn r#supplyIccPinForApp(&self, _arg_serial: i32, _arg_pin: &str, _arg_aid: &str) -> binder::Result<()>;
              fn r#supplyIccPuk2ForApp(&self, _arg_serial: i32, _arg_puk2: &str, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<()>;
              fn r#supplyIccPukForApp(&self, _arg_serial: i32, _arg_puk: &str, _arg_pin: &str, _arg_aid: &str) -> binder::Result<()>;
              fn r#supplySimDepersonalization(&self, _arg_serial: i32, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_controlKey: &str) -> binder::Result<()>;
              fn r#updateSimPhonebookRecords(&self, _arg_serial: i32, _arg_recordInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioSimDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioSimDefaultRef) -> IRadioSimDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioSimAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.sim.IRadioSim" }
              fn r#areUiccApplicationsEnabled(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#changeIccPin2ForApp(&self, _arg_serial: i32, _arg_oldPin2: &str, _arg_newPin2: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#changeIccPinForApp(&self, _arg_serial: i32, _arg_oldPin: &str, _arg_newPin: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#enableUiccApplications(&self, _arg_serial: i32, _arg_enable: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#getAllowedCarriers(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getCdmaSubscription(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getCdmaSubscriptionSource(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#getIccCardStatus(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getImsiForApp(&self, _arg_serial: i32, _arg_aid: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#getSimPhonebookCapacity(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getSimPhonebookRecords(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#iccCloseLogicalChannel(&self, _arg_serial: i32, _arg_channelId: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#iccIoForApp(&self, _arg_serial: i32, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_5_IccIo) -> std::future::Ready<binder::Result<()>>;
              fn r#iccOpenLogicalChannel(&self, _arg_serial: i32, _arg_aid: &str, _arg_p2: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#iccTransmitApduBasicChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> std::future::Ready<binder::Result<()>>;
              fn r#iccTransmitApduLogicalChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> std::future::Ready<binder::Result<()>>;
              fn r#reportStkServiceIsRunning(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#requestIccSimAuthentication(&self, _arg_serial: i32, _arg_authContext: i32, _arg_authData: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#responseAcknowledgement(&self) -> std::future::Ready<binder::Result<()>>;
              fn r#sendEnvelope(&self, _arg_serial: i32, _arg_contents: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#sendEnvelopeWithStatus(&self, _arg_serial: i32, _arg_contents: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#sendTerminalResponseToSim(&self, _arg_serial: i32, _arg_contents: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#setAllowedCarriers(&self, _arg_serial: i32, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> std::future::Ready<binder::Result<()>>;
              fn r#setCarrierInfoForImsiEncryption(&self, _arg_serial: i32, _arg_imsiEncryptionInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_18_ImsiEncryptionInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setCdmaSubscriptionSource(&self, _arg_serial: i32, _arg_cdmaSub: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> std::future::Ready<binder::Result<()>>;
              fn r#setFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_lockState: bool, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#setResponseFunctions(&self, _arg_radioSimResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_IRadioSimResponse>, _arg_radioSimIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_IRadioSimIndication>) -> std::future::Ready<binder::Result<()>>;
              fn r#setSimCardPower(&self, _arg_serial: i32, _arg_powerUp: crate::mangled::_7_android_8_hardware_5_radio_3_sim_14_CardPowerState) -> std::future::Ready<binder::Result<()>>;
              fn r#setUiccSubscription(&self, _arg_serial: i32, _arg_uiccSub: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_SelectUiccSub) -> std::future::Ready<binder::Result<()>>;
              fn r#supplyIccPin2ForApp(&self, _arg_serial: i32, _arg_pin2: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#supplyIccPinForApp(&self, _arg_serial: i32, _arg_pin: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#supplyIccPuk2ForApp(&self, _arg_serial: i32, _arg_puk2: &str, _arg_pin2: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#supplyIccPukForApp(&self, _arg_serial: i32, _arg_puk: &str, _arg_pin: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#supplySimDepersonalization(&self, _arg_serial: i32, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_controlKey: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#updateSimPhonebookRecords(&self, _arg_serial: i32, _arg_recordInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioSimAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.sim.IRadioSim" }
              async fn r#areUiccApplicationsEnabled(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#changeIccPin2ForApp(&self, _arg_serial: i32, _arg_oldPin2: &str, _arg_newPin2: &str, _arg_aid: &str) -> binder::Result<()>;
              async fn r#changeIccPinForApp(&self, _arg_serial: i32, _arg_oldPin: &str, _arg_newPin: &str, _arg_aid: &str) -> binder::Result<()>;
              async fn r#enableUiccApplications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()>;
              async fn r#getAllowedCarriers(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getCdmaSubscription(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getCdmaSubscriptionSource(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<()>;
              async fn r#getIccCardStatus(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getImsiForApp(&self, _arg_serial: i32, _arg_aid: &str) -> binder::Result<()>;
              async fn r#getSimPhonebookCapacity(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getSimPhonebookRecords(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#iccCloseLogicalChannel(&self, _arg_serial: i32, _arg_channelId: i32) -> binder::Result<()>;
              async fn r#iccIoForApp(&self, _arg_serial: i32, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_5_IccIo) -> binder::Result<()>;
              async fn r#iccOpenLogicalChannel(&self, _arg_serial: i32, _arg_aid: &str, _arg_p2: i32) -> binder::Result<()>;
              async fn r#iccTransmitApduBasicChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<()>;
              async fn r#iccTransmitApduLogicalChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<()>;
              async fn r#reportStkServiceIsRunning(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#requestIccSimAuthentication(&self, _arg_serial: i32, _arg_authContext: i32, _arg_authData: &str, _arg_aid: &str) -> binder::Result<()>;
              async fn r#responseAcknowledgement(&self) -> binder::Result<()>;
              async fn r#sendEnvelope(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()>;
              async fn r#sendEnvelopeWithStatus(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()>;
              async fn r#sendTerminalResponseToSim(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()>;
              async fn r#setAllowedCarriers(&self, _arg_serial: i32, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<()>;
              async fn r#setCarrierInfoForImsiEncryption(&self, _arg_serial: i32, _arg_imsiEncryptionInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_18_ImsiEncryptionInfo) -> binder::Result<()>;
              async fn r#setCdmaSubscriptionSource(&self, _arg_serial: i32, _arg_cdmaSub: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()>;
              async fn r#setFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_lockState: bool, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<()>;
              async fn r#setResponseFunctions(&self, _arg_radioSimResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_IRadioSimResponse>, _arg_radioSimIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_IRadioSimIndication>) -> binder::Result<()>;
              async fn r#setSimCardPower(&self, _arg_serial: i32, _arg_powerUp: crate::mangled::_7_android_8_hardware_5_radio_3_sim_14_CardPowerState) -> binder::Result<()>;
              async fn r#setUiccSubscription(&self, _arg_serial: i32, _arg_uiccSub: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_SelectUiccSub) -> binder::Result<()>;
              async fn r#supplyIccPin2ForApp(&self, _arg_serial: i32, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<()>;
              async fn r#supplyIccPinForApp(&self, _arg_serial: i32, _arg_pin: &str, _arg_aid: &str) -> binder::Result<()>;
              async fn r#supplyIccPuk2ForApp(&self, _arg_serial: i32, _arg_puk2: &str, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<()>;
              async fn r#supplyIccPukForApp(&self, _arg_serial: i32, _arg_puk: &str, _arg_pin: &str, _arg_aid: &str) -> binder::Result<()>;
              async fn r#supplySimDepersonalization(&self, _arg_serial: i32, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_controlKey: &str) -> binder::Result<()>;
              async fn r#updateSimPhonebookRecords(&self, _arg_serial: i32, _arg_recordInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo) -> binder::Result<()>;
            }
            impl BnRadioSim {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioSim>
              where
                T: IRadioSimAsyncServer + binder::Interface + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                struct Wrapper<T, R> {
                  _inner: T,
                  _rt: R,
                }
                impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync {
                  fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
                  fn dump(&self, _file: &std::fs::File, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_file, _args) }
                }
                impl<T, R> IRadioSim for Wrapper<T, R>
                where
                  T: IRadioSimAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#areUiccApplicationsEnabled(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#areUiccApplicationsEnabled(_arg_serial))
                  }
                  fn r#changeIccPin2ForApp(&self, _arg_serial: i32, _arg_oldPin2: &str, _arg_newPin2: &str, _arg_aid: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#changeIccPin2ForApp(_arg_serial, _arg_oldPin2, _arg_newPin2, _arg_aid))
                  }
                  fn r#changeIccPinForApp(&self, _arg_serial: i32, _arg_oldPin: &str, _arg_newPin: &str, _arg_aid: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#changeIccPinForApp(_arg_serial, _arg_oldPin, _arg_newPin, _arg_aid))
                  }
                  fn r#enableUiccApplications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#enableUiccApplications(_arg_serial, _arg_enable))
                  }
                  fn r#getAllowedCarriers(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getAllowedCarriers(_arg_serial))
                  }
                  fn r#getCdmaSubscription(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCdmaSubscription(_arg_serial))
                  }
                  fn r#getCdmaSubscriptionSource(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCdmaSubscriptionSource(_arg_serial))
                  }
                  fn r#getFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getFacilityLockForApp(_arg_serial, _arg_facility, _arg_password, _arg_serviceClass, _arg_appId))
                  }
                  fn r#getIccCardStatus(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getIccCardStatus(_arg_serial))
                  }
                  fn r#getImsiForApp(&self, _arg_serial: i32, _arg_aid: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getImsiForApp(_arg_serial, _arg_aid))
                  }
                  fn r#getSimPhonebookCapacity(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSimPhonebookCapacity(_arg_serial))
                  }
                  fn r#getSimPhonebookRecords(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSimPhonebookRecords(_arg_serial))
                  }
                  fn r#iccCloseLogicalChannel(&self, _arg_serial: i32, _arg_channelId: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#iccCloseLogicalChannel(_arg_serial, _arg_channelId))
                  }
                  fn r#iccIoForApp(&self, _arg_serial: i32, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_5_IccIo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#iccIoForApp(_arg_serial, _arg_iccIo))
                  }
                  fn r#iccOpenLogicalChannel(&self, _arg_serial: i32, _arg_aid: &str, _arg_p2: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#iccOpenLogicalChannel(_arg_serial, _arg_aid, _arg_p2))
                  }
                  fn r#iccTransmitApduBasicChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#iccTransmitApduBasicChannel(_arg_serial, _arg_message))
                  }
                  fn r#iccTransmitApduLogicalChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#iccTransmitApduLogicalChannel(_arg_serial, _arg_message))
                  }
                  fn r#reportStkServiceIsRunning(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#reportStkServiceIsRunning(_arg_serial))
                  }
                  fn r#requestIccSimAuthentication(&self, _arg_serial: i32, _arg_authContext: i32, _arg_authData: &str, _arg_aid: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#requestIccSimAuthentication(_arg_serial, _arg_authContext, _arg_authData, _arg_aid))
                  }
                  fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#responseAcknowledgement())
                  }
                  fn r#sendEnvelope(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendEnvelope(_arg_serial, _arg_contents))
                  }
                  fn r#sendEnvelopeWithStatus(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendEnvelopeWithStatus(_arg_serial, _arg_contents))
                  }
                  fn r#sendTerminalResponseToSim(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendTerminalResponseToSim(_arg_serial, _arg_contents))
                  }
                  fn r#setAllowedCarriers(&self, _arg_serial: i32, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setAllowedCarriers(_arg_serial, _arg_carriers, _arg_multiSimPolicy))
                  }
                  fn r#setCarrierInfoForImsiEncryption(&self, _arg_serial: i32, _arg_imsiEncryptionInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_18_ImsiEncryptionInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCarrierInfoForImsiEncryption(_arg_serial, _arg_imsiEncryptionInfo))
                  }
                  fn r#setCdmaSubscriptionSource(&self, _arg_serial: i32, _arg_cdmaSub: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCdmaSubscriptionSource(_arg_serial, _arg_cdmaSub))
                  }
                  fn r#setFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_lockState: bool, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setFacilityLockForApp(_arg_serial, _arg_facility, _arg_lockState, _arg_password, _arg_serviceClass, _arg_appId))
                  }
                  fn r#setResponseFunctions(&self, _arg_radioSimResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_IRadioSimResponse>, _arg_radioSimIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_IRadioSimIndication>) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setResponseFunctions(_arg_radioSimResponse, _arg_radioSimIndication))
                  }
                  fn r#setSimCardPower(&self, _arg_serial: i32, _arg_powerUp: crate::mangled::_7_android_8_hardware_5_radio_3_sim_14_CardPowerState) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setSimCardPower(_arg_serial, _arg_powerUp))
                  }
                  fn r#setUiccSubscription(&self, _arg_serial: i32, _arg_uiccSub: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_SelectUiccSub) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setUiccSubscription(_arg_serial, _arg_uiccSub))
                  }
                  fn r#supplyIccPin2ForApp(&self, _arg_serial: i32, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#supplyIccPin2ForApp(_arg_serial, _arg_pin2, _arg_aid))
                  }
                  fn r#supplyIccPinForApp(&self, _arg_serial: i32, _arg_pin: &str, _arg_aid: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#supplyIccPinForApp(_arg_serial, _arg_pin, _arg_aid))
                  }
                  fn r#supplyIccPuk2ForApp(&self, _arg_serial: i32, _arg_puk2: &str, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#supplyIccPuk2ForApp(_arg_serial, _arg_puk2, _arg_pin2, _arg_aid))
                  }
                  fn r#supplyIccPukForApp(&self, _arg_serial: i32, _arg_puk: &str, _arg_pin: &str, _arg_aid: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#supplyIccPukForApp(_arg_serial, _arg_puk, _arg_pin, _arg_aid))
                  }
                  fn r#supplySimDepersonalization(&self, _arg_serial: i32, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_controlKey: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#supplySimDepersonalization(_arg_serial, _arg_persoType, _arg_controlKey))
                  }
                  fn r#updateSimPhonebookRecords(&self, _arg_serial: i32, _arg_recordInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#updateSimPhonebookRecords(_arg_serial, _arg_recordInfo))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioSimDefault: Send + Sync {
              fn r#areUiccApplicationsEnabled(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#changeIccPin2ForApp(&self, _arg_serial: i32, _arg_oldPin2: &str, _arg_newPin2: &str, _arg_aid: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#changeIccPinForApp(&self, _arg_serial: i32, _arg_oldPin: &str, _arg_newPin: &str, _arg_aid: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#enableUiccApplications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getAllowedCarriers(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCdmaSubscription(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCdmaSubscriptionSource(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getIccCardStatus(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getImsiForApp(&self, _arg_serial: i32, _arg_aid: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSimPhonebookCapacity(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSimPhonebookRecords(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#iccCloseLogicalChannel(&self, _arg_serial: i32, _arg_channelId: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#iccIoForApp(&self, _arg_serial: i32, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_5_IccIo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#iccOpenLogicalChannel(&self, _arg_serial: i32, _arg_aid: &str, _arg_p2: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#iccTransmitApduBasicChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#iccTransmitApduLogicalChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#reportStkServiceIsRunning(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#requestIccSimAuthentication(&self, _arg_serial: i32, _arg_authContext: i32, _arg_authData: &str, _arg_aid: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendEnvelope(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendEnvelopeWithStatus(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendTerminalResponseToSim(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setAllowedCarriers(&self, _arg_serial: i32, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCarrierInfoForImsiEncryption(&self, _arg_serial: i32, _arg_imsiEncryptionInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_18_ImsiEncryptionInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCdmaSubscriptionSource(&self, _arg_serial: i32, _arg_cdmaSub: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_lockState: bool, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setResponseFunctions(&self, _arg_radioSimResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_IRadioSimResponse>, _arg_radioSimIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_IRadioSimIndication>) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setSimCardPower(&self, _arg_serial: i32, _arg_powerUp: crate::mangled::_7_android_8_hardware_5_radio_3_sim_14_CardPowerState) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setUiccSubscription(&self, _arg_serial: i32, _arg_uiccSub: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_SelectUiccSub) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#supplyIccPin2ForApp(&self, _arg_serial: i32, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#supplyIccPinForApp(&self, _arg_serial: i32, _arg_pin: &str, _arg_aid: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#supplyIccPuk2ForApp(&self, _arg_serial: i32, _arg_puk2: &str, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#supplyIccPukForApp(&self, _arg_serial: i32, _arg_puk: &str, _arg_pin: &str, _arg_aid: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#supplySimDepersonalization(&self, _arg_serial: i32, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_controlKey: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#updateSimPhonebookRecords(&self, _arg_serial: i32, _arg_recordInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#areUiccApplicationsEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#changeIccPin2ForApp: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#changeIccPinForApp: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#enableUiccApplications: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#getAllowedCarriers: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#getCdmaSubscription: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#getCdmaSubscriptionSource: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#getFacilityLockForApp: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#getIccCardStatus: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#getImsiForApp: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#getSimPhonebookCapacity: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#getSimPhonebookRecords: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#iccCloseLogicalChannel: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#iccIoForApp: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#iccOpenLogicalChannel: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#iccTransmitApduBasicChannel: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#iccTransmitApduLogicalChannel: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
              pub const r#reportStkServiceIsRunning: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
              pub const r#requestIccSimAuthentication: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
              pub const r#responseAcknowledgement: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
              pub const r#sendEnvelope: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 20;
              pub const r#sendEnvelopeWithStatus: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 21;
              pub const r#sendTerminalResponseToSim: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 22;
              pub const r#setAllowedCarriers: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 23;
              pub const r#setCarrierInfoForImsiEncryption: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 24;
              pub const r#setCdmaSubscriptionSource: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 25;
              pub const r#setFacilityLockForApp: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 26;
              pub const r#setResponseFunctions: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 27;
              pub const r#setSimCardPower: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 28;
              pub const r#setUiccSubscription: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 29;
              pub const r#supplyIccPin2ForApp: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 30;
              pub const r#supplyIccPinForApp: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 31;
              pub const r#supplyIccPuk2ForApp: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 32;
              pub const r#supplyIccPukForApp: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 33;
              pub const r#supplySimDepersonalization: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 34;
              pub const r#updateSimPhonebookRecords: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 35;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioSimDefaultRef = Option<std::sync::Arc<dyn IRadioSimDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioSimDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "01cea196fdf8f5e41fda8dc41125f1cc2b96f757";
            impl BpRadioSim {
              fn build_parcel_areUiccApplicationsEnabled(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_areUiccApplicationsEnabled(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#areUiccApplicationsEnabled(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_changeIccPin2ForApp(&self, _arg_serial: i32, _arg_oldPin2: &str, _arg_newPin2: &str, _arg_aid: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_oldPin2)?;
                aidl_data.write(_arg_newPin2)?;
                aidl_data.write(_arg_aid)?;
                Ok(aidl_data)
              }
              fn read_response_changeIccPin2ForApp(&self, _arg_serial: i32, _arg_oldPin2: &str, _arg_newPin2: &str, _arg_aid: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#changeIccPin2ForApp(_arg_serial, _arg_oldPin2, _arg_newPin2, _arg_aid);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_changeIccPinForApp(&self, _arg_serial: i32, _arg_oldPin: &str, _arg_newPin: &str, _arg_aid: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_oldPin)?;
                aidl_data.write(_arg_newPin)?;
                aidl_data.write(_arg_aid)?;
                Ok(aidl_data)
              }
              fn read_response_changeIccPinForApp(&self, _arg_serial: i32, _arg_oldPin: &str, _arg_newPin: &str, _arg_aid: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#changeIccPinForApp(_arg_serial, _arg_oldPin, _arg_newPin, _arg_aid);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_enableUiccApplications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_enable)?;
                Ok(aidl_data)
              }
              fn read_response_enableUiccApplications(&self, _arg_serial: i32, _arg_enable: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#enableUiccApplications(_arg_serial, _arg_enable);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getAllowedCarriers(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getAllowedCarriers(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#getAllowedCarriers(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCdmaSubscription(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getCdmaSubscription(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCdmaSubscription(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCdmaSubscriptionSource(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getCdmaSubscriptionSource(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCdmaSubscriptionSource(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_facility)?;
                aidl_data.write(_arg_password)?;
                aidl_data.write(&_arg_serviceClass)?;
                aidl_data.write(_arg_appId)?;
                Ok(aidl_data)
              }
              fn read_response_getFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#getFacilityLockForApp(_arg_serial, _arg_facility, _arg_password, _arg_serviceClass, _arg_appId);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getIccCardStatus(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getIccCardStatus(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#getIccCardStatus(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getImsiForApp(&self, _arg_serial: i32, _arg_aid: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_aid)?;
                Ok(aidl_data)
              }
              fn read_response_getImsiForApp(&self, _arg_serial: i32, _arg_aid: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#getImsiForApp(_arg_serial, _arg_aid);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSimPhonebookCapacity(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getSimPhonebookCapacity(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSimPhonebookCapacity(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSimPhonebookRecords(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getSimPhonebookRecords(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSimPhonebookRecords(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_iccCloseLogicalChannel(&self, _arg_serial: i32, _arg_channelId: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_channelId)?;
                Ok(aidl_data)
              }
              fn read_response_iccCloseLogicalChannel(&self, _arg_serial: i32, _arg_channelId: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#iccCloseLogicalChannel(_arg_serial, _arg_channelId);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_iccIoForApp(&self, _arg_serial: i32, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_5_IccIo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_iccIo)?;
                Ok(aidl_data)
              }
              fn read_response_iccIoForApp(&self, _arg_serial: i32, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_5_IccIo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#iccIoForApp(_arg_serial, _arg_iccIo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_iccOpenLogicalChannel(&self, _arg_serial: i32, _arg_aid: &str, _arg_p2: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_aid)?;
                aidl_data.write(&_arg_p2)?;
                Ok(aidl_data)
              }
              fn read_response_iccOpenLogicalChannel(&self, _arg_serial: i32, _arg_aid: &str, _arg_p2: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#iccOpenLogicalChannel(_arg_serial, _arg_aid, _arg_p2);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_iccTransmitApduBasicChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_message)?;
                Ok(aidl_data)
              }
              fn read_response_iccTransmitApduBasicChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#iccTransmitApduBasicChannel(_arg_serial, _arg_message);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_iccTransmitApduLogicalChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_message)?;
                Ok(aidl_data)
              }
              fn read_response_iccTransmitApduLogicalChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#iccTransmitApduLogicalChannel(_arg_serial, _arg_message);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_reportStkServiceIsRunning(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_reportStkServiceIsRunning(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#reportStkServiceIsRunning(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_requestIccSimAuthentication(&self, _arg_serial: i32, _arg_authContext: i32, _arg_authData: &str, _arg_aid: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_authContext)?;
                aidl_data.write(_arg_authData)?;
                aidl_data.write(_arg_aid)?;
                Ok(aidl_data)
              }
              fn read_response_requestIccSimAuthentication(&self, _arg_serial: i32, _arg_authContext: i32, _arg_authData: &str, _arg_aid: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#requestIccSimAuthentication(_arg_serial, _arg_authContext, _arg_authData, _arg_aid);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_responseAcknowledgement(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_responseAcknowledgement(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#responseAcknowledgement();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendEnvelope(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_contents)?;
                Ok(aidl_data)
              }
              fn read_response_sendEnvelope(&self, _arg_serial: i32, _arg_contents: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendEnvelope(_arg_serial, _arg_contents);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendEnvelopeWithStatus(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_contents)?;
                Ok(aidl_data)
              }
              fn read_response_sendEnvelopeWithStatus(&self, _arg_serial: i32, _arg_contents: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendEnvelopeWithStatus(_arg_serial, _arg_contents);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendTerminalResponseToSim(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_contents)?;
                Ok(aidl_data)
              }
              fn read_response_sendTerminalResponseToSim(&self, _arg_serial: i32, _arg_contents: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendTerminalResponseToSim(_arg_serial, _arg_contents);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setAllowedCarriers(&self, _arg_serial: i32, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_carriers)?;
                aidl_data.write(&_arg_multiSimPolicy)?;
                Ok(aidl_data)
              }
              fn read_response_setAllowedCarriers(&self, _arg_serial: i32, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#setAllowedCarriers(_arg_serial, _arg_carriers, _arg_multiSimPolicy);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCarrierInfoForImsiEncryption(&self, _arg_serial: i32, _arg_imsiEncryptionInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_18_ImsiEncryptionInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_imsiEncryptionInfo)?;
                Ok(aidl_data)
              }
              fn read_response_setCarrierInfoForImsiEncryption(&self, _arg_serial: i32, _arg_imsiEncryptionInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_18_ImsiEncryptionInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCarrierInfoForImsiEncryption(_arg_serial, _arg_imsiEncryptionInfo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCdmaSubscriptionSource(&self, _arg_serial: i32, _arg_cdmaSub: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_cdmaSub)?;
                Ok(aidl_data)
              }
              fn read_response_setCdmaSubscriptionSource(&self, _arg_serial: i32, _arg_cdmaSub: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCdmaSubscriptionSource(_arg_serial, _arg_cdmaSub);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_lockState: bool, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_facility)?;
                aidl_data.write(&_arg_lockState)?;
                aidl_data.write(_arg_password)?;
                aidl_data.write(&_arg_serviceClass)?;
                aidl_data.write(_arg_appId)?;
                Ok(aidl_data)
              }
              fn read_response_setFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_lockState: bool, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#setFacilityLockForApp(_arg_serial, _arg_facility, _arg_lockState, _arg_password, _arg_serviceClass, _arg_appId);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setResponseFunctions(&self, _arg_radioSimResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_IRadioSimResponse>, _arg_radioSimIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_IRadioSimIndication>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_radioSimResponse)?;
                aidl_data.write(_arg_radioSimIndication)?;
                Ok(aidl_data)
              }
              fn read_response_setResponseFunctions(&self, _arg_radioSimResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_IRadioSimResponse>, _arg_radioSimIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_IRadioSimIndication>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#setResponseFunctions(_arg_radioSimResponse, _arg_radioSimIndication);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setSimCardPower(&self, _arg_serial: i32, _arg_powerUp: crate::mangled::_7_android_8_hardware_5_radio_3_sim_14_CardPowerState) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_powerUp)?;
                Ok(aidl_data)
              }
              fn read_response_setSimCardPower(&self, _arg_serial: i32, _arg_powerUp: crate::mangled::_7_android_8_hardware_5_radio_3_sim_14_CardPowerState, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#setSimCardPower(_arg_serial, _arg_powerUp);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setUiccSubscription(&self, _arg_serial: i32, _arg_uiccSub: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_SelectUiccSub) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_uiccSub)?;
                Ok(aidl_data)
              }
              fn read_response_setUiccSubscription(&self, _arg_serial: i32, _arg_uiccSub: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_SelectUiccSub, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#setUiccSubscription(_arg_serial, _arg_uiccSub);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_supplyIccPin2ForApp(&self, _arg_serial: i32, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_pin2)?;
                aidl_data.write(_arg_aid)?;
                Ok(aidl_data)
              }
              fn read_response_supplyIccPin2ForApp(&self, _arg_serial: i32, _arg_pin2: &str, _arg_aid: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#supplyIccPin2ForApp(_arg_serial, _arg_pin2, _arg_aid);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_supplyIccPinForApp(&self, _arg_serial: i32, _arg_pin: &str, _arg_aid: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_pin)?;
                aidl_data.write(_arg_aid)?;
                Ok(aidl_data)
              }
              fn read_response_supplyIccPinForApp(&self, _arg_serial: i32, _arg_pin: &str, _arg_aid: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#supplyIccPinForApp(_arg_serial, _arg_pin, _arg_aid);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_supplyIccPuk2ForApp(&self, _arg_serial: i32, _arg_puk2: &str, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_puk2)?;
                aidl_data.write(_arg_pin2)?;
                aidl_data.write(_arg_aid)?;
                Ok(aidl_data)
              }
              fn read_response_supplyIccPuk2ForApp(&self, _arg_serial: i32, _arg_puk2: &str, _arg_pin2: &str, _arg_aid: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#supplyIccPuk2ForApp(_arg_serial, _arg_puk2, _arg_pin2, _arg_aid);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_supplyIccPukForApp(&self, _arg_serial: i32, _arg_puk: &str, _arg_pin: &str, _arg_aid: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_puk)?;
                aidl_data.write(_arg_pin)?;
                aidl_data.write(_arg_aid)?;
                Ok(aidl_data)
              }
              fn read_response_supplyIccPukForApp(&self, _arg_serial: i32, _arg_puk: &str, _arg_pin: &str, _arg_aid: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#supplyIccPukForApp(_arg_serial, _arg_puk, _arg_pin, _arg_aid);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_supplySimDepersonalization(&self, _arg_serial: i32, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_controlKey: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_persoType)?;
                aidl_data.write(_arg_controlKey)?;
                Ok(aidl_data)
              }
              fn read_response_supplySimDepersonalization(&self, _arg_serial: i32, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_controlKey: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#supplySimDepersonalization(_arg_serial, _arg_persoType, _arg_controlKey);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_updateSimPhonebookRecords(&self, _arg_serial: i32, _arg_recordInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_recordInfo)?;
                Ok(aidl_data)
              }
              fn read_response_updateSimPhonebookRecords(&self, _arg_serial: i32, _arg_recordInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSim>::getDefaultImpl() {
                    return _aidl_default_impl.r#updateSimPhonebookRecords(_arg_serial, _arg_recordInfo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getInterfaceVersion(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceVersion(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                self.cached_version.store(_aidl_return, std::sync::atomic::Ordering::Relaxed);
                Ok(_aidl_return)
              }
              fn build_parcel_getInterfaceHash(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceHash(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: String = _aidl_reply.read()?;
                *self.cached_hash.lock().unwrap() = Some(_aidl_return.clone());
                Ok(_aidl_return)
              }
            }
            impl IRadioSim for BpRadioSim {
              fn r#areUiccApplicationsEnabled(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_areUiccApplicationsEnabled(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#areUiccApplicationsEnabled, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_areUiccApplicationsEnabled(_arg_serial, _aidl_reply)
              }
              fn r#changeIccPin2ForApp(&self, _arg_serial: i32, _arg_oldPin2: &str, _arg_newPin2: &str, _arg_aid: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_changeIccPin2ForApp(_arg_serial, _arg_oldPin2, _arg_newPin2, _arg_aid)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#changeIccPin2ForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_changeIccPin2ForApp(_arg_serial, _arg_oldPin2, _arg_newPin2, _arg_aid, _aidl_reply)
              }
              fn r#changeIccPinForApp(&self, _arg_serial: i32, _arg_oldPin: &str, _arg_newPin: &str, _arg_aid: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_changeIccPinForApp(_arg_serial, _arg_oldPin, _arg_newPin, _arg_aid)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#changeIccPinForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_changeIccPinForApp(_arg_serial, _arg_oldPin, _arg_newPin, _arg_aid, _aidl_reply)
              }
              fn r#enableUiccApplications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_enableUiccApplications(_arg_serial, _arg_enable)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#enableUiccApplications, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_enableUiccApplications(_arg_serial, _arg_enable, _aidl_reply)
              }
              fn r#getAllowedCarriers(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getAllowedCarriers(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAllowedCarriers, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getAllowedCarriers(_arg_serial, _aidl_reply)
              }
              fn r#getCdmaSubscription(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCdmaSubscription(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaSubscription, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCdmaSubscription(_arg_serial, _aidl_reply)
              }
              fn r#getCdmaSubscriptionSource(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCdmaSubscriptionSource(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaSubscriptionSource, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCdmaSubscriptionSource(_arg_serial, _aidl_reply)
              }
              fn r#getFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getFacilityLockForApp(_arg_serial, _arg_facility, _arg_password, _arg_serviceClass, _arg_appId)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getFacilityLockForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getFacilityLockForApp(_arg_serial, _arg_facility, _arg_password, _arg_serviceClass, _arg_appId, _aidl_reply)
              }
              fn r#getIccCardStatus(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getIccCardStatus(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getIccCardStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getIccCardStatus(_arg_serial, _aidl_reply)
              }
              fn r#getImsiForApp(&self, _arg_serial: i32, _arg_aid: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getImsiForApp(_arg_serial, _arg_aid)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getImsiForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getImsiForApp(_arg_serial, _arg_aid, _aidl_reply)
              }
              fn r#getSimPhonebookCapacity(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSimPhonebookCapacity(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSimPhonebookCapacity, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSimPhonebookCapacity(_arg_serial, _aidl_reply)
              }
              fn r#getSimPhonebookRecords(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSimPhonebookRecords(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSimPhonebookRecords, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSimPhonebookRecords(_arg_serial, _aidl_reply)
              }
              fn r#iccCloseLogicalChannel(&self, _arg_serial: i32, _arg_channelId: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_iccCloseLogicalChannel(_arg_serial, _arg_channelId)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccCloseLogicalChannel, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_iccCloseLogicalChannel(_arg_serial, _arg_channelId, _aidl_reply)
              }
              fn r#iccIoForApp(&self, _arg_serial: i32, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_5_IccIo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_iccIoForApp(_arg_serial, _arg_iccIo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccIoForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_iccIoForApp(_arg_serial, _arg_iccIo, _aidl_reply)
              }
              fn r#iccOpenLogicalChannel(&self, _arg_serial: i32, _arg_aid: &str, _arg_p2: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_iccOpenLogicalChannel(_arg_serial, _arg_aid, _arg_p2)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccOpenLogicalChannel, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_iccOpenLogicalChannel(_arg_serial, _arg_aid, _arg_p2, _aidl_reply)
              }
              fn r#iccTransmitApduBasicChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_iccTransmitApduBasicChannel(_arg_serial, _arg_message)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccTransmitApduBasicChannel, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_iccTransmitApduBasicChannel(_arg_serial, _arg_message, _aidl_reply)
              }
              fn r#iccTransmitApduLogicalChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_iccTransmitApduLogicalChannel(_arg_serial, _arg_message)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccTransmitApduLogicalChannel, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_iccTransmitApduLogicalChannel(_arg_serial, _arg_message, _aidl_reply)
              }
              fn r#reportStkServiceIsRunning(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_reportStkServiceIsRunning(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#reportStkServiceIsRunning, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_reportStkServiceIsRunning(_arg_serial, _aidl_reply)
              }
              fn r#requestIccSimAuthentication(&self, _arg_serial: i32, _arg_authContext: i32, _arg_authData: &str, _arg_aid: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_requestIccSimAuthentication(_arg_serial, _arg_authContext, _arg_authData, _arg_aid)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#requestIccSimAuthentication, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_requestIccSimAuthentication(_arg_serial, _arg_authContext, _arg_authData, _arg_aid, _aidl_reply)
              }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_responseAcknowledgement()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#responseAcknowledgement, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_responseAcknowledgement(_aidl_reply)
              }
              fn r#sendEnvelope(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendEnvelope(_arg_serial, _arg_contents)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendEnvelope, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendEnvelope(_arg_serial, _arg_contents, _aidl_reply)
              }
              fn r#sendEnvelopeWithStatus(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendEnvelopeWithStatus(_arg_serial, _arg_contents)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendEnvelopeWithStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendEnvelopeWithStatus(_arg_serial, _arg_contents, _aidl_reply)
              }
              fn r#sendTerminalResponseToSim(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendTerminalResponseToSim(_arg_serial, _arg_contents)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendTerminalResponseToSim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendTerminalResponseToSim(_arg_serial, _arg_contents, _aidl_reply)
              }
              fn r#setAllowedCarriers(&self, _arg_serial: i32, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setAllowedCarriers(_arg_serial, _arg_carriers, _arg_multiSimPolicy)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setAllowedCarriers, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setAllowedCarriers(_arg_serial, _arg_carriers, _arg_multiSimPolicy, _aidl_reply)
              }
              fn r#setCarrierInfoForImsiEncryption(&self, _arg_serial: i32, _arg_imsiEncryptionInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_18_ImsiEncryptionInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCarrierInfoForImsiEncryption(_arg_serial, _arg_imsiEncryptionInfo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCarrierInfoForImsiEncryption, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCarrierInfoForImsiEncryption(_arg_serial, _arg_imsiEncryptionInfo, _aidl_reply)
              }
              fn r#setCdmaSubscriptionSource(&self, _arg_serial: i32, _arg_cdmaSub: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCdmaSubscriptionSource(_arg_serial, _arg_cdmaSub)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaSubscriptionSource, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCdmaSubscriptionSource(_arg_serial, _arg_cdmaSub, _aidl_reply)
              }
              fn r#setFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_lockState: bool, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setFacilityLockForApp(_arg_serial, _arg_facility, _arg_lockState, _arg_password, _arg_serviceClass, _arg_appId)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setFacilityLockForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setFacilityLockForApp(_arg_serial, _arg_facility, _arg_lockState, _arg_password, _arg_serviceClass, _arg_appId, _aidl_reply)
              }
              fn r#setResponseFunctions(&self, _arg_radioSimResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_IRadioSimResponse>, _arg_radioSimIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_IRadioSimIndication>) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setResponseFunctions(_arg_radioSimResponse, _arg_radioSimIndication)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setResponseFunctions(_arg_radioSimResponse, _arg_radioSimIndication, _aidl_reply)
              }
              fn r#setSimCardPower(&self, _arg_serial: i32, _arg_powerUp: crate::mangled::_7_android_8_hardware_5_radio_3_sim_14_CardPowerState) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setSimCardPower(_arg_serial, _arg_powerUp)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSimCardPower, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setSimCardPower(_arg_serial, _arg_powerUp, _aidl_reply)
              }
              fn r#setUiccSubscription(&self, _arg_serial: i32, _arg_uiccSub: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_SelectUiccSub) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setUiccSubscription(_arg_serial, _arg_uiccSub)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setUiccSubscription, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setUiccSubscription(_arg_serial, _arg_uiccSub, _aidl_reply)
              }
              fn r#supplyIccPin2ForApp(&self, _arg_serial: i32, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_supplyIccPin2ForApp(_arg_serial, _arg_pin2, _arg_aid)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPin2ForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_supplyIccPin2ForApp(_arg_serial, _arg_pin2, _arg_aid, _aidl_reply)
              }
              fn r#supplyIccPinForApp(&self, _arg_serial: i32, _arg_pin: &str, _arg_aid: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_supplyIccPinForApp(_arg_serial, _arg_pin, _arg_aid)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPinForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_supplyIccPinForApp(_arg_serial, _arg_pin, _arg_aid, _aidl_reply)
              }
              fn r#supplyIccPuk2ForApp(&self, _arg_serial: i32, _arg_puk2: &str, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_supplyIccPuk2ForApp(_arg_serial, _arg_puk2, _arg_pin2, _arg_aid)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPuk2ForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_supplyIccPuk2ForApp(_arg_serial, _arg_puk2, _arg_pin2, _arg_aid, _aidl_reply)
              }
              fn r#supplyIccPukForApp(&self, _arg_serial: i32, _arg_puk: &str, _arg_pin: &str, _arg_aid: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_supplyIccPukForApp(_arg_serial, _arg_puk, _arg_pin, _arg_aid)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPukForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_supplyIccPukForApp(_arg_serial, _arg_puk, _arg_pin, _arg_aid, _aidl_reply)
              }
              fn r#supplySimDepersonalization(&self, _arg_serial: i32, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_controlKey: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_supplySimDepersonalization(_arg_serial, _arg_persoType, _arg_controlKey)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplySimDepersonalization, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_supplySimDepersonalization(_arg_serial, _arg_persoType, _arg_controlKey, _aidl_reply)
              }
              fn r#updateSimPhonebookRecords(&self, _arg_serial: i32, _arg_recordInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_updateSimPhonebookRecords(_arg_serial, _arg_recordInfo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#updateSimPhonebookRecords, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_updateSimPhonebookRecords(_arg_serial, _arg_recordInfo, _aidl_reply)
              }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Ok(_aidl_version); }
                let _aidl_data = self.build_parcel_getInterfaceVersion()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceVersion(_aidl_reply)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Ok(_aidl_hash.clone());
                  }
                }
                let _aidl_data = self.build_parcel_getInterfaceHash()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceHash(_aidl_reply)
              }
            }
            impl<P: binder::BinderAsyncPool> IRadioSimAsync<P> for BpRadioSim {
              fn r#areUiccApplicationsEnabled(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_areUiccApplicationsEnabled(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#areUiccApplicationsEnabled, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_areUiccApplicationsEnabled(_arg_serial, _aidl_reply))
              }
              fn r#changeIccPin2ForApp(&self, _arg_serial: i32, _arg_oldPin2: &str, _arg_newPin2: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_changeIccPin2ForApp(_arg_serial, _arg_oldPin2, _arg_newPin2, _arg_aid) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#changeIccPin2ForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_changeIccPin2ForApp(_arg_serial, _arg_oldPin2, _arg_newPin2, _arg_aid, _aidl_reply))
              }
              fn r#changeIccPinForApp(&self, _arg_serial: i32, _arg_oldPin: &str, _arg_newPin: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_changeIccPinForApp(_arg_serial, _arg_oldPin, _arg_newPin, _arg_aid) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#changeIccPinForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_changeIccPinForApp(_arg_serial, _arg_oldPin, _arg_newPin, _arg_aid, _aidl_reply))
              }
              fn r#enableUiccApplications(&self, _arg_serial: i32, _arg_enable: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_enableUiccApplications(_arg_serial, _arg_enable) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#enableUiccApplications, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_enableUiccApplications(_arg_serial, _arg_enable, _aidl_reply))
              }
              fn r#getAllowedCarriers(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getAllowedCarriers(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAllowedCarriers, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getAllowedCarriers(_arg_serial, _aidl_reply))
              }
              fn r#getCdmaSubscription(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCdmaSubscription(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaSubscription, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCdmaSubscription(_arg_serial, _aidl_reply))
              }
              fn r#getCdmaSubscriptionSource(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCdmaSubscriptionSource(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaSubscriptionSource, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCdmaSubscriptionSource(_arg_serial, _aidl_reply))
              }
              fn r#getFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getFacilityLockForApp(_arg_serial, _arg_facility, _arg_password, _arg_serviceClass, _arg_appId) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getFacilityLockForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getFacilityLockForApp(_arg_serial, _arg_facility, _arg_password, _arg_serviceClass, _arg_appId, _aidl_reply))
              }
              fn r#getIccCardStatus(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getIccCardStatus(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getIccCardStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getIccCardStatus(_arg_serial, _aidl_reply))
              }
              fn r#getImsiForApp(&self, _arg_serial: i32, _arg_aid: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getImsiForApp(_arg_serial, _arg_aid) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getImsiForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getImsiForApp(_arg_serial, _arg_aid, _aidl_reply))
              }
              fn r#getSimPhonebookCapacity(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSimPhonebookCapacity(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSimPhonebookCapacity, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSimPhonebookCapacity(_arg_serial, _aidl_reply))
              }
              fn r#getSimPhonebookRecords(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSimPhonebookRecords(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSimPhonebookRecords, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSimPhonebookRecords(_arg_serial, _aidl_reply))
              }
              fn r#iccCloseLogicalChannel(&self, _arg_serial: i32, _arg_channelId: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_iccCloseLogicalChannel(_arg_serial, _arg_channelId) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccCloseLogicalChannel, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_iccCloseLogicalChannel(_arg_serial, _arg_channelId, _aidl_reply))
              }
              fn r#iccIoForApp(&self, _arg_serial: i32, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_5_IccIo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_iccIoForApp(_arg_serial, _arg_iccIo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccIoForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_iccIoForApp(_arg_serial, _arg_iccIo, _aidl_reply))
              }
              fn r#iccOpenLogicalChannel(&self, _arg_serial: i32, _arg_aid: &str, _arg_p2: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_iccOpenLogicalChannel(_arg_serial, _arg_aid, _arg_p2) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccOpenLogicalChannel, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_iccOpenLogicalChannel(_arg_serial, _arg_aid, _arg_p2, _aidl_reply))
              }
              fn r#iccTransmitApduBasicChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_iccTransmitApduBasicChannel(_arg_serial, _arg_message) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccTransmitApduBasicChannel, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_iccTransmitApduBasicChannel(_arg_serial, _arg_message, _aidl_reply))
              }
              fn r#iccTransmitApduLogicalChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_iccTransmitApduLogicalChannel(_arg_serial, _arg_message) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccTransmitApduLogicalChannel, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_iccTransmitApduLogicalChannel(_arg_serial, _arg_message, _aidl_reply))
              }
              fn r#reportStkServiceIsRunning(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_reportStkServiceIsRunning(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#reportStkServiceIsRunning, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_reportStkServiceIsRunning(_arg_serial, _aidl_reply))
              }
              fn r#requestIccSimAuthentication(&self, _arg_serial: i32, _arg_authContext: i32, _arg_authData: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_requestIccSimAuthentication(_arg_serial, _arg_authContext, _arg_authData, _arg_aid) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#requestIccSimAuthentication, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_requestIccSimAuthentication(_arg_serial, _arg_authContext, _arg_authData, _arg_aid, _aidl_reply))
              }
              fn r#responseAcknowledgement(&self) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_responseAcknowledgement() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#responseAcknowledgement, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_responseAcknowledgement(_aidl_reply))
              }
              fn r#sendEnvelope(&self, _arg_serial: i32, _arg_contents: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendEnvelope(_arg_serial, _arg_contents) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendEnvelope, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendEnvelope(_arg_serial, _arg_contents, _aidl_reply))
              }
              fn r#sendEnvelopeWithStatus(&self, _arg_serial: i32, _arg_contents: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendEnvelopeWithStatus(_arg_serial, _arg_contents) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendEnvelopeWithStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendEnvelopeWithStatus(_arg_serial, _arg_contents, _aidl_reply))
              }
              fn r#sendTerminalResponseToSim(&self, _arg_serial: i32, _arg_contents: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendTerminalResponseToSim(_arg_serial, _arg_contents) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendTerminalResponseToSim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendTerminalResponseToSim(_arg_serial, _arg_contents, _aidl_reply))
              }
              fn r#setAllowedCarriers(&self, _arg_serial: i32, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setAllowedCarriers(_arg_serial, _arg_carriers, _arg_multiSimPolicy) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setAllowedCarriers, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setAllowedCarriers(_arg_serial, _arg_carriers, _arg_multiSimPolicy, _aidl_reply))
              }
              fn r#setCarrierInfoForImsiEncryption(&self, _arg_serial: i32, _arg_imsiEncryptionInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_18_ImsiEncryptionInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCarrierInfoForImsiEncryption(_arg_serial, _arg_imsiEncryptionInfo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCarrierInfoForImsiEncryption, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCarrierInfoForImsiEncryption(_arg_serial, _arg_imsiEncryptionInfo, _aidl_reply))
              }
              fn r#setCdmaSubscriptionSource(&self, _arg_serial: i32, _arg_cdmaSub: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCdmaSubscriptionSource(_arg_serial, _arg_cdmaSub) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaSubscriptionSource, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCdmaSubscriptionSource(_arg_serial, _arg_cdmaSub, _aidl_reply))
              }
              fn r#setFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_lockState: bool, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setFacilityLockForApp(_arg_serial, _arg_facility, _arg_lockState, _arg_password, _arg_serviceClass, _arg_appId) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setFacilityLockForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setFacilityLockForApp(_arg_serial, _arg_facility, _arg_lockState, _arg_password, _arg_serviceClass, _arg_appId, _aidl_reply))
              }
              fn r#setResponseFunctions(&self, _arg_radioSimResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_IRadioSimResponse>, _arg_radioSimIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_IRadioSimIndication>) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setResponseFunctions(_arg_radioSimResponse, _arg_radioSimIndication) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setResponseFunctions(_arg_radioSimResponse, _arg_radioSimIndication, _aidl_reply))
              }
              fn r#setSimCardPower(&self, _arg_serial: i32, _arg_powerUp: crate::mangled::_7_android_8_hardware_5_radio_3_sim_14_CardPowerState) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setSimCardPower(_arg_serial, _arg_powerUp) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSimCardPower, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setSimCardPower(_arg_serial, _arg_powerUp, _aidl_reply))
              }
              fn r#setUiccSubscription(&self, _arg_serial: i32, _arg_uiccSub: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_SelectUiccSub) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setUiccSubscription(_arg_serial, _arg_uiccSub) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setUiccSubscription, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setUiccSubscription(_arg_serial, _arg_uiccSub, _aidl_reply))
              }
              fn r#supplyIccPin2ForApp(&self, _arg_serial: i32, _arg_pin2: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_supplyIccPin2ForApp(_arg_serial, _arg_pin2, _arg_aid) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPin2ForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_supplyIccPin2ForApp(_arg_serial, _arg_pin2, _arg_aid, _aidl_reply))
              }
              fn r#supplyIccPinForApp(&self, _arg_serial: i32, _arg_pin: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_supplyIccPinForApp(_arg_serial, _arg_pin, _arg_aid) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPinForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_supplyIccPinForApp(_arg_serial, _arg_pin, _arg_aid, _aidl_reply))
              }
              fn r#supplyIccPuk2ForApp(&self, _arg_serial: i32, _arg_puk2: &str, _arg_pin2: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_supplyIccPuk2ForApp(_arg_serial, _arg_puk2, _arg_pin2, _arg_aid) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPuk2ForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_supplyIccPuk2ForApp(_arg_serial, _arg_puk2, _arg_pin2, _arg_aid, _aidl_reply))
              }
              fn r#supplyIccPukForApp(&self, _arg_serial: i32, _arg_puk: &str, _arg_pin: &str, _arg_aid: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_supplyIccPukForApp(_arg_serial, _arg_puk, _arg_pin, _arg_aid) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPukForApp, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_supplyIccPukForApp(_arg_serial, _arg_puk, _arg_pin, _arg_aid, _aidl_reply))
              }
              fn r#supplySimDepersonalization(&self, _arg_serial: i32, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_controlKey: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_supplySimDepersonalization(_arg_serial, _arg_persoType, _arg_controlKey) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplySimDepersonalization, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_supplySimDepersonalization(_arg_serial, _arg_persoType, _arg_controlKey, _aidl_reply))
              }
              fn r#updateSimPhonebookRecords(&self, _arg_serial: i32, _arg_recordInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_updateSimPhonebookRecords(_arg_serial, _arg_recordInfo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#updateSimPhonebookRecords, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_updateSimPhonebookRecords(_arg_serial, _arg_recordInfo, _aidl_reply))
              }
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Box::pin(std::future::ready(Ok(_aidl_version))); }
                let _aidl_data = match self.build_parcel_getInterfaceVersion() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceVersion(_aidl_reply)
                  }
                )
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Box::pin(std::future::ready(Ok(_aidl_hash.clone())));
                  }
                }
                let _aidl_data = match self.build_parcel_getInterfaceHash() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceHash(_aidl_reply)
                  }
                )
              }
            }
            impl IRadioSim for binder::binder_impl::Binder<BnRadioSim> {
              fn r#areUiccApplicationsEnabled(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#areUiccApplicationsEnabled(_arg_serial) }
              fn r#changeIccPin2ForApp(&self, _arg_serial: i32, _arg_oldPin2: &str, _arg_newPin2: &str, _arg_aid: &str) -> binder::Result<()> { self.0.r#changeIccPin2ForApp(_arg_serial, _arg_oldPin2, _arg_newPin2, _arg_aid) }
              fn r#changeIccPinForApp(&self, _arg_serial: i32, _arg_oldPin: &str, _arg_newPin: &str, _arg_aid: &str) -> binder::Result<()> { self.0.r#changeIccPinForApp(_arg_serial, _arg_oldPin, _arg_newPin, _arg_aid) }
              fn r#enableUiccApplications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> { self.0.r#enableUiccApplications(_arg_serial, _arg_enable) }
              fn r#getAllowedCarriers(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getAllowedCarriers(_arg_serial) }
              fn r#getCdmaSubscription(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getCdmaSubscription(_arg_serial) }
              fn r#getCdmaSubscriptionSource(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getCdmaSubscriptionSource(_arg_serial) }
              fn r#getFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<()> { self.0.r#getFacilityLockForApp(_arg_serial, _arg_facility, _arg_password, _arg_serviceClass, _arg_appId) }
              fn r#getIccCardStatus(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getIccCardStatus(_arg_serial) }
              fn r#getImsiForApp(&self, _arg_serial: i32, _arg_aid: &str) -> binder::Result<()> { self.0.r#getImsiForApp(_arg_serial, _arg_aid) }
              fn r#getSimPhonebookCapacity(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getSimPhonebookCapacity(_arg_serial) }
              fn r#getSimPhonebookRecords(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getSimPhonebookRecords(_arg_serial) }
              fn r#iccCloseLogicalChannel(&self, _arg_serial: i32, _arg_channelId: i32) -> binder::Result<()> { self.0.r#iccCloseLogicalChannel(_arg_serial, _arg_channelId) }
              fn r#iccIoForApp(&self, _arg_serial: i32, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_5_IccIo) -> binder::Result<()> { self.0.r#iccIoForApp(_arg_serial, _arg_iccIo) }
              fn r#iccOpenLogicalChannel(&self, _arg_serial: i32, _arg_aid: &str, _arg_p2: i32) -> binder::Result<()> { self.0.r#iccOpenLogicalChannel(_arg_serial, _arg_aid, _arg_p2) }
              fn r#iccTransmitApduBasicChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<()> { self.0.r#iccTransmitApduBasicChannel(_arg_serial, _arg_message) }
              fn r#iccTransmitApduLogicalChannel(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu) -> binder::Result<()> { self.0.r#iccTransmitApduLogicalChannel(_arg_serial, _arg_message) }
              fn r#reportStkServiceIsRunning(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#reportStkServiceIsRunning(_arg_serial) }
              fn r#requestIccSimAuthentication(&self, _arg_serial: i32, _arg_authContext: i32, _arg_authData: &str, _arg_aid: &str) -> binder::Result<()> { self.0.r#requestIccSimAuthentication(_arg_serial, _arg_authContext, _arg_authData, _arg_aid) }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> { self.0.r#responseAcknowledgement() }
              fn r#sendEnvelope(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()> { self.0.r#sendEnvelope(_arg_serial, _arg_contents) }
              fn r#sendEnvelopeWithStatus(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()> { self.0.r#sendEnvelopeWithStatus(_arg_serial, _arg_contents) }
              fn r#sendTerminalResponseToSim(&self, _arg_serial: i32, _arg_contents: &str) -> binder::Result<()> { self.0.r#sendTerminalResponseToSim(_arg_serial, _arg_contents) }
              fn r#setAllowedCarriers(&self, _arg_serial: i32, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<()> { self.0.r#setAllowedCarriers(_arg_serial, _arg_carriers, _arg_multiSimPolicy) }
              fn r#setCarrierInfoForImsiEncryption(&self, _arg_serial: i32, _arg_imsiEncryptionInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_18_ImsiEncryptionInfo) -> binder::Result<()> { self.0.r#setCarrierInfoForImsiEncryption(_arg_serial, _arg_imsiEncryptionInfo) }
              fn r#setCdmaSubscriptionSource(&self, _arg_serial: i32, _arg_cdmaSub: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()> { self.0.r#setCdmaSubscriptionSource(_arg_serial, _arg_cdmaSub) }
              fn r#setFacilityLockForApp(&self, _arg_serial: i32, _arg_facility: &str, _arg_lockState: bool, _arg_password: &str, _arg_serviceClass: i32, _arg_appId: &str) -> binder::Result<()> { self.0.r#setFacilityLockForApp(_arg_serial, _arg_facility, _arg_lockState, _arg_password, _arg_serviceClass, _arg_appId) }
              fn r#setResponseFunctions(&self, _arg_radioSimResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_IRadioSimResponse>, _arg_radioSimIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_IRadioSimIndication>) -> binder::Result<()> { self.0.r#setResponseFunctions(_arg_radioSimResponse, _arg_radioSimIndication) }
              fn r#setSimCardPower(&self, _arg_serial: i32, _arg_powerUp: crate::mangled::_7_android_8_hardware_5_radio_3_sim_14_CardPowerState) -> binder::Result<()> { self.0.r#setSimCardPower(_arg_serial, _arg_powerUp) }
              fn r#setUiccSubscription(&self, _arg_serial: i32, _arg_uiccSub: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_SelectUiccSub) -> binder::Result<()> { self.0.r#setUiccSubscription(_arg_serial, _arg_uiccSub) }
              fn r#supplyIccPin2ForApp(&self, _arg_serial: i32, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<()> { self.0.r#supplyIccPin2ForApp(_arg_serial, _arg_pin2, _arg_aid) }
              fn r#supplyIccPinForApp(&self, _arg_serial: i32, _arg_pin: &str, _arg_aid: &str) -> binder::Result<()> { self.0.r#supplyIccPinForApp(_arg_serial, _arg_pin, _arg_aid) }
              fn r#supplyIccPuk2ForApp(&self, _arg_serial: i32, _arg_puk2: &str, _arg_pin2: &str, _arg_aid: &str) -> binder::Result<()> { self.0.r#supplyIccPuk2ForApp(_arg_serial, _arg_puk2, _arg_pin2, _arg_aid) }
              fn r#supplyIccPukForApp(&self, _arg_serial: i32, _arg_puk: &str, _arg_pin: &str, _arg_aid: &str) -> binder::Result<()> { self.0.r#supplyIccPukForApp(_arg_serial, _arg_puk, _arg_pin, _arg_aid) }
              fn r#supplySimDepersonalization(&self, _arg_serial: i32, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_controlKey: &str) -> binder::Result<()> { self.0.r#supplySimDepersonalization(_arg_serial, _arg_persoType, _arg_controlKey) }
              fn r#updateSimPhonebookRecords(&self, _arg_serial: i32, _arg_recordInfo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo) -> binder::Result<()> { self.0.r#updateSimPhonebookRecords(_arg_serial, _arg_recordInfo) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioSim, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#areUiccApplicationsEnabled => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#areUiccApplicationsEnabled(_arg_serial);
                  Ok(())
                }
                transactions::r#changeIccPin2ForApp => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_oldPin2: String = _aidl_data.read()?;
                  let _arg_newPin2: String = _aidl_data.read()?;
                  let _arg_aid: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#changeIccPin2ForApp(_arg_serial, &_arg_oldPin2, &_arg_newPin2, &_arg_aid);
                  Ok(())
                }
                transactions::r#changeIccPinForApp => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_oldPin: String = _aidl_data.read()?;
                  let _arg_newPin: String = _aidl_data.read()?;
                  let _arg_aid: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#changeIccPinForApp(_arg_serial, &_arg_oldPin, &_arg_newPin, &_arg_aid);
                  Ok(())
                }
                transactions::r#enableUiccApplications => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_enable: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#enableUiccApplications(_arg_serial, _arg_enable);
                  Ok(())
                }
                transactions::r#getAllowedCarriers => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getAllowedCarriers(_arg_serial);
                  Ok(())
                }
                transactions::r#getCdmaSubscription => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCdmaSubscription(_arg_serial);
                  Ok(())
                }
                transactions::r#getCdmaSubscriptionSource => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCdmaSubscriptionSource(_arg_serial);
                  Ok(())
                }
                transactions::r#getFacilityLockForApp => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_facility: String = _aidl_data.read()?;
                  let _arg_password: String = _aidl_data.read()?;
                  let _arg_serviceClass: i32 = _aidl_data.read()?;
                  let _arg_appId: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getFacilityLockForApp(_arg_serial, &_arg_facility, &_arg_password, _arg_serviceClass, &_arg_appId);
                  Ok(())
                }
                transactions::r#getIccCardStatus => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getIccCardStatus(_arg_serial);
                  Ok(())
                }
                transactions::r#getImsiForApp => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_aid: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getImsiForApp(_arg_serial, &_arg_aid);
                  Ok(())
                }
                transactions::r#getSimPhonebookCapacity => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSimPhonebookCapacity(_arg_serial);
                  Ok(())
                }
                transactions::r#getSimPhonebookRecords => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSimPhonebookRecords(_arg_serial);
                  Ok(())
                }
                transactions::r#iccCloseLogicalChannel => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_channelId: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#iccCloseLogicalChannel(_arg_serial, _arg_channelId);
                  Ok(())
                }
                transactions::r#iccIoForApp => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_iccIo: crate::mangled::_7_android_8_hardware_5_radio_3_sim_5_IccIo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#iccIoForApp(_arg_serial, &_arg_iccIo);
                  Ok(())
                }
                transactions::r#iccOpenLogicalChannel => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_aid: String = _aidl_data.read()?;
                  let _arg_p2: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#iccOpenLogicalChannel(_arg_serial, &_arg_aid, _arg_p2);
                  Ok(())
                }
                transactions::r#iccTransmitApduBasicChannel => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_message: crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#iccTransmitApduBasicChannel(_arg_serial, &_arg_message);
                  Ok(())
                }
                transactions::r#iccTransmitApduLogicalChannel => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_message: crate::mangled::_7_android_8_hardware_5_radio_3_sim_7_SimApdu = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#iccTransmitApduLogicalChannel(_arg_serial, &_arg_message);
                  Ok(())
                }
                transactions::r#reportStkServiceIsRunning => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#reportStkServiceIsRunning(_arg_serial);
                  Ok(())
                }
                transactions::r#requestIccSimAuthentication => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_authContext: i32 = _aidl_data.read()?;
                  let _arg_authData: String = _aidl_data.read()?;
                  let _arg_aid: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#requestIccSimAuthentication(_arg_serial, _arg_authContext, &_arg_authData, &_arg_aid);
                  Ok(())
                }
                transactions::r#responseAcknowledgement => {
                  let _aidl_return = _aidl_service.r#responseAcknowledgement();
                  Ok(())
                }
                transactions::r#sendEnvelope => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_contents: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendEnvelope(_arg_serial, &_arg_contents);
                  Ok(())
                }
                transactions::r#sendEnvelopeWithStatus => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_contents: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendEnvelopeWithStatus(_arg_serial, &_arg_contents);
                  Ok(())
                }
                transactions::r#sendTerminalResponseToSim => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_contents: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendTerminalResponseToSim(_arg_serial, &_arg_contents);
                  Ok(())
                }
                transactions::r#setAllowedCarriers => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_carriers: crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions = _aidl_data.read()?;
                  let _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setAllowedCarriers(_arg_serial, &_arg_carriers, _arg_multiSimPolicy);
                  Ok(())
                }
                transactions::r#setCarrierInfoForImsiEncryption => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_imsiEncryptionInfo: crate::mangled::_7_android_8_hardware_5_radio_3_sim_18_ImsiEncryptionInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCarrierInfoForImsiEncryption(_arg_serial, &_arg_imsiEncryptionInfo);
                  Ok(())
                }
                transactions::r#setCdmaSubscriptionSource => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_cdmaSub: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCdmaSubscriptionSource(_arg_serial, _arg_cdmaSub);
                  Ok(())
                }
                transactions::r#setFacilityLockForApp => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_facility: String = _aidl_data.read()?;
                  let _arg_lockState: bool = _aidl_data.read()?;
                  let _arg_password: String = _aidl_data.read()?;
                  let _arg_serviceClass: i32 = _aidl_data.read()?;
                  let _arg_appId: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setFacilityLockForApp(_arg_serial, &_arg_facility, _arg_lockState, &_arg_password, _arg_serviceClass, &_arg_appId);
                  Ok(())
                }
                transactions::r#setResponseFunctions => {
                  let _arg_radioSimResponse: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_IRadioSimResponse> = _aidl_data.read()?;
                  let _arg_radioSimIndication: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_IRadioSimIndication> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setResponseFunctions(&_arg_radioSimResponse, &_arg_radioSimIndication);
                  Ok(())
                }
                transactions::r#setSimCardPower => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_powerUp: crate::mangled::_7_android_8_hardware_5_radio_3_sim_14_CardPowerState = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setSimCardPower(_arg_serial, _arg_powerUp);
                  Ok(())
                }
                transactions::r#setUiccSubscription => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_uiccSub: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_SelectUiccSub = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setUiccSubscription(_arg_serial, &_arg_uiccSub);
                  Ok(())
                }
                transactions::r#supplyIccPin2ForApp => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_pin2: String = _aidl_data.read()?;
                  let _arg_aid: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#supplyIccPin2ForApp(_arg_serial, &_arg_pin2, &_arg_aid);
                  Ok(())
                }
                transactions::r#supplyIccPinForApp => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_pin: String = _aidl_data.read()?;
                  let _arg_aid: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#supplyIccPinForApp(_arg_serial, &_arg_pin, &_arg_aid);
                  Ok(())
                }
                transactions::r#supplyIccPuk2ForApp => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_puk2: String = _aidl_data.read()?;
                  let _arg_pin2: String = _aidl_data.read()?;
                  let _arg_aid: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#supplyIccPuk2ForApp(_arg_serial, &_arg_puk2, &_arg_pin2, &_arg_aid);
                  Ok(())
                }
                transactions::r#supplyIccPukForApp => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_puk: String = _aidl_data.read()?;
                  let _arg_pin: String = _aidl_data.read()?;
                  let _arg_aid: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#supplyIccPukForApp(_arg_serial, &_arg_puk, &_arg_pin, &_arg_aid);
                  Ok(())
                }
                transactions::r#supplySimDepersonalization => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate = _aidl_data.read()?;
                  let _arg_controlKey: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#supplySimDepersonalization(_arg_serial, _arg_persoType, &_arg_controlKey);
                  Ok(())
                }
                transactions::r#updateSimPhonebookRecords => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_recordInfo: crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#updateSimPhonebookRecords(_arg_serial, &_arg_recordInfo);
                  Ok(())
                }
                transactions::r#getInterfaceVersion => {
                  let _aidl_return = _aidl_service.r#getInterfaceVersion();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getInterfaceHash => {
                  let _aidl_return = _aidl_service.r#getInterfaceHash();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
              }
            }
            pub(crate) mod mangled {
             pub use super::r#IRadioSim as _7_android_8_hardware_5_radio_3_sim_9_IRadioSim;
            }
          }
          pub mod IRadioSimIndication {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioSimIndication["android.hardware.radio.sim.IRadioSimIndication"] {
                native: BnRadioSimIndication(on_transact),
                proxy: BpRadioSimIndication {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioSimIndicationAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioSimIndication: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.sim.IRadioSimIndication" }
              fn r#carrierInfoForImsiEncryption(&self, _arg_info: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#cdmaSubscriptionSourceChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cdmaSource: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()>;
              fn r#simPhonebookChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#simPhonebookRecordsReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_PbReceivedStatus, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo]) -> binder::Result<()>;
              fn r#simRefresh(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_refreshResult: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_SimRefreshResult) -> binder::Result<()>;
              fn r#simStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#stkEventNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<()>;
              fn r#stkProactiveCommand(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<()>;
              fn r#stkSessionEnd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#subscriptionStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_activate: bool) -> binder::Result<()>;
              fn r#uiccApplicationsEnablementChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_enabled: bool) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioSimIndicationDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioSimIndicationDefaultRef) -> IRadioSimIndicationDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioSimIndicationAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.sim.IRadioSimIndication" }
              fn r#carrierInfoForImsiEncryption(&self, _arg_info: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#cdmaSubscriptionSourceChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cdmaSource: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> std::future::Ready<binder::Result<()>>;
              fn r#simPhonebookChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#simPhonebookRecordsReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_PbReceivedStatus, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#simRefresh(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_refreshResult: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_SimRefreshResult) -> std::future::Ready<binder::Result<()>>;
              fn r#simStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#stkEventNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#stkProactiveCommand(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#stkSessionEnd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#subscriptionStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_activate: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#uiccApplicationsEnablementChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_enabled: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioSimIndicationAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.sim.IRadioSimIndication" }
              async fn r#carrierInfoForImsiEncryption(&self, _arg_info: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              async fn r#cdmaSubscriptionSourceChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cdmaSource: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()>;
              async fn r#simPhonebookChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              async fn r#simPhonebookRecordsReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_PbReceivedStatus, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo]) -> binder::Result<()>;
              async fn r#simRefresh(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_refreshResult: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_SimRefreshResult) -> binder::Result<()>;
              async fn r#simStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              async fn r#stkEventNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<()>;
              async fn r#stkProactiveCommand(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<()>;
              async fn r#stkSessionEnd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              async fn r#subscriptionStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_activate: bool) -> binder::Result<()>;
              async fn r#uiccApplicationsEnablementChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_enabled: bool) -> binder::Result<()>;
            }
            impl BnRadioSimIndication {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioSimIndication>
              where
                T: IRadioSimIndicationAsyncServer + binder::Interface + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                struct Wrapper<T, R> {
                  _inner: T,
                  _rt: R,
                }
                impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync {
                  fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
                  fn dump(&self, _file: &std::fs::File, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_file, _args) }
                }
                impl<T, R> IRadioSimIndication for Wrapper<T, R>
                where
                  T: IRadioSimIndicationAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#carrierInfoForImsiEncryption(&self, _arg_info: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#carrierInfoForImsiEncryption(_arg_info))
                  }
                  fn r#cdmaSubscriptionSourceChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cdmaSource: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#cdmaSubscriptionSourceChanged(_arg_type, _arg_cdmaSource))
                  }
                  fn r#simPhonebookChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#simPhonebookChanged(_arg_type))
                  }
                  fn r#simPhonebookRecordsReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_PbReceivedStatus, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#simPhonebookRecordsReceived(_arg_type, _arg_status, _arg_records))
                  }
                  fn r#simRefresh(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_refreshResult: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_SimRefreshResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#simRefresh(_arg_type, _arg_refreshResult))
                  }
                  fn r#simStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#simStatusChanged(_arg_type))
                  }
                  fn r#stkEventNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#stkEventNotify(_arg_type, _arg_cmd))
                  }
                  fn r#stkProactiveCommand(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#stkProactiveCommand(_arg_type, _arg_cmd))
                  }
                  fn r#stkSessionEnd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#stkSessionEnd(_arg_type))
                  }
                  fn r#subscriptionStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_activate: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#subscriptionStatusChanged(_arg_type, _arg_activate))
                  }
                  fn r#uiccApplicationsEnablementChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_enabled: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#uiccApplicationsEnablementChanged(_arg_type, _arg_enabled))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioSimIndicationDefault: Send + Sync {
              fn r#carrierInfoForImsiEncryption(&self, _arg_info: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#cdmaSubscriptionSourceChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cdmaSource: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#simPhonebookChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#simPhonebookRecordsReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_PbReceivedStatus, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#simRefresh(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_refreshResult: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_SimRefreshResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#simStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#stkEventNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#stkProactiveCommand(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#stkSessionEnd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#subscriptionStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_activate: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#uiccApplicationsEnablementChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_enabled: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#carrierInfoForImsiEncryption: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#cdmaSubscriptionSourceChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#simPhonebookChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#simPhonebookRecordsReceived: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#simRefresh: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#simStatusChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#stkEventNotify: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#stkProactiveCommand: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#stkSessionEnd: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#subscriptionStatusChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#uiccApplicationsEnablementChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioSimIndicationDefaultRef = Option<std::sync::Arc<dyn IRadioSimIndicationDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioSimIndicationDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "01cea196fdf8f5e41fda8dc41125f1cc2b96f757";
            impl BpRadioSimIndication {
              fn build_parcel_carrierInfoForImsiEncryption(&self, _arg_info: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_carrierInfoForImsiEncryption(&self, _arg_info: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#carrierInfoForImsiEncryption(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_cdmaSubscriptionSourceChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cdmaSource: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_cdmaSource)?;
                Ok(aidl_data)
              }
              fn read_response_cdmaSubscriptionSourceChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cdmaSource: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#cdmaSubscriptionSourceChanged(_arg_type, _arg_cdmaSource);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_simPhonebookChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_simPhonebookChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#simPhonebookChanged(_arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_simPhonebookRecordsReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_PbReceivedStatus, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_status)?;
                aidl_data.write(_arg_records)?;
                Ok(aidl_data)
              }
              fn read_response_simPhonebookRecordsReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_PbReceivedStatus, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#simPhonebookRecordsReceived(_arg_type, _arg_status, _arg_records);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_simRefresh(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_refreshResult: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_SimRefreshResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_refreshResult)?;
                Ok(aidl_data)
              }
              fn read_response_simRefresh(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_refreshResult: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_SimRefreshResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#simRefresh(_arg_type, _arg_refreshResult);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_simStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_simStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#simStatusChanged(_arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_stkEventNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_cmd)?;
                Ok(aidl_data)
              }
              fn read_response_stkEventNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#stkEventNotify(_arg_type, _arg_cmd);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_stkProactiveCommand(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_cmd)?;
                Ok(aidl_data)
              }
              fn read_response_stkProactiveCommand(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#stkProactiveCommand(_arg_type, _arg_cmd);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_stkSessionEnd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_stkSessionEnd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#stkSessionEnd(_arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_subscriptionStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_activate: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_activate)?;
                Ok(aidl_data)
              }
              fn read_response_subscriptionStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_activate: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#subscriptionStatusChanged(_arg_type, _arg_activate);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_uiccApplicationsEnablementChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_enabled: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_enabled)?;
                Ok(aidl_data)
              }
              fn read_response_uiccApplicationsEnablementChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_enabled: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#uiccApplicationsEnablementChanged(_arg_type, _arg_enabled);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getInterfaceVersion(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceVersion(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                self.cached_version.store(_aidl_return, std::sync::atomic::Ordering::Relaxed);
                Ok(_aidl_return)
              }
              fn build_parcel_getInterfaceHash(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceHash(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: String = _aidl_reply.read()?;
                *self.cached_hash.lock().unwrap() = Some(_aidl_return.clone());
                Ok(_aidl_return)
              }
            }
            impl IRadioSimIndication for BpRadioSimIndication {
              fn r#carrierInfoForImsiEncryption(&self, _arg_info: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_carrierInfoForImsiEncryption(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#carrierInfoForImsiEncryption, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_carrierInfoForImsiEncryption(_arg_info, _aidl_reply)
              }
              fn r#cdmaSubscriptionSourceChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cdmaSource: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_cdmaSubscriptionSourceChanged(_arg_type, _arg_cdmaSource)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaSubscriptionSourceChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_cdmaSubscriptionSourceChanged(_arg_type, _arg_cdmaSource, _aidl_reply)
              }
              fn r#simPhonebookChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_simPhonebookChanged(_arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#simPhonebookChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_simPhonebookChanged(_arg_type, _aidl_reply)
              }
              fn r#simPhonebookRecordsReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_PbReceivedStatus, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_simPhonebookRecordsReceived(_arg_type, _arg_status, _arg_records)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#simPhonebookRecordsReceived, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_simPhonebookRecordsReceived(_arg_type, _arg_status, _arg_records, _aidl_reply)
              }
              fn r#simRefresh(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_refreshResult: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_SimRefreshResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_simRefresh(_arg_type, _arg_refreshResult)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#simRefresh, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_simRefresh(_arg_type, _arg_refreshResult, _aidl_reply)
              }
              fn r#simStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_simStatusChanged(_arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#simStatusChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_simStatusChanged(_arg_type, _aidl_reply)
              }
              fn r#stkEventNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_stkEventNotify(_arg_type, _arg_cmd)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#stkEventNotify, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_stkEventNotify(_arg_type, _arg_cmd, _aidl_reply)
              }
              fn r#stkProactiveCommand(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_stkProactiveCommand(_arg_type, _arg_cmd)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#stkProactiveCommand, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_stkProactiveCommand(_arg_type, _arg_cmd, _aidl_reply)
              }
              fn r#stkSessionEnd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_stkSessionEnd(_arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#stkSessionEnd, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_stkSessionEnd(_arg_type, _aidl_reply)
              }
              fn r#subscriptionStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_activate: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_subscriptionStatusChanged(_arg_type, _arg_activate)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#subscriptionStatusChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_subscriptionStatusChanged(_arg_type, _arg_activate, _aidl_reply)
              }
              fn r#uiccApplicationsEnablementChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_enabled: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_uiccApplicationsEnablementChanged(_arg_type, _arg_enabled)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#uiccApplicationsEnablementChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_uiccApplicationsEnablementChanged(_arg_type, _arg_enabled, _aidl_reply)
              }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Ok(_aidl_version); }
                let _aidl_data = self.build_parcel_getInterfaceVersion()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceVersion(_aidl_reply)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Ok(_aidl_hash.clone());
                  }
                }
                let _aidl_data = self.build_parcel_getInterfaceHash()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceHash(_aidl_reply)
              }
            }
            impl<P: binder::BinderAsyncPool> IRadioSimIndicationAsync<P> for BpRadioSimIndication {
              fn r#carrierInfoForImsiEncryption(&self, _arg_info: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_carrierInfoForImsiEncryption(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#carrierInfoForImsiEncryption, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_carrierInfoForImsiEncryption(_arg_info, _aidl_reply))
              }
              fn r#cdmaSubscriptionSourceChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cdmaSource: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_cdmaSubscriptionSourceChanged(_arg_type, _arg_cdmaSource) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaSubscriptionSourceChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_cdmaSubscriptionSourceChanged(_arg_type, _arg_cdmaSource, _aidl_reply))
              }
              fn r#simPhonebookChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_simPhonebookChanged(_arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#simPhonebookChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_simPhonebookChanged(_arg_type, _aidl_reply))
              }
              fn r#simPhonebookRecordsReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_PbReceivedStatus, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_simPhonebookRecordsReceived(_arg_type, _arg_status, _arg_records) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#simPhonebookRecordsReceived, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_simPhonebookRecordsReceived(_arg_type, _arg_status, _arg_records, _aidl_reply))
              }
              fn r#simRefresh(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_refreshResult: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_SimRefreshResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_simRefresh(_arg_type, _arg_refreshResult) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#simRefresh, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_simRefresh(_arg_type, _arg_refreshResult, _aidl_reply))
              }
              fn r#simStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_simStatusChanged(_arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#simStatusChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_simStatusChanged(_arg_type, _aidl_reply))
              }
              fn r#stkEventNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_stkEventNotify(_arg_type, _arg_cmd) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#stkEventNotify, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_stkEventNotify(_arg_type, _arg_cmd, _aidl_reply))
              }
              fn r#stkProactiveCommand(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_stkProactiveCommand(_arg_type, _arg_cmd) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#stkProactiveCommand, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_stkProactiveCommand(_arg_type, _arg_cmd, _aidl_reply))
              }
              fn r#stkSessionEnd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_stkSessionEnd(_arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#stkSessionEnd, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_stkSessionEnd(_arg_type, _aidl_reply))
              }
              fn r#subscriptionStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_activate: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_subscriptionStatusChanged(_arg_type, _arg_activate) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#subscriptionStatusChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_subscriptionStatusChanged(_arg_type, _arg_activate, _aidl_reply))
              }
              fn r#uiccApplicationsEnablementChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_enabled: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_uiccApplicationsEnablementChanged(_arg_type, _arg_enabled) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#uiccApplicationsEnablementChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_uiccApplicationsEnablementChanged(_arg_type, _arg_enabled, _aidl_reply))
              }
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Box::pin(std::future::ready(Ok(_aidl_version))); }
                let _aidl_data = match self.build_parcel_getInterfaceVersion() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceVersion(_aidl_reply)
                  }
                )
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Box::pin(std::future::ready(Ok(_aidl_hash.clone())));
                  }
                }
                let _aidl_data = match self.build_parcel_getInterfaceHash() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceHash(_aidl_reply)
                  }
                )
              }
            }
            impl IRadioSimIndication for binder::binder_impl::Binder<BnRadioSimIndication> {
              fn r#carrierInfoForImsiEncryption(&self, _arg_info: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#carrierInfoForImsiEncryption(_arg_info) }
              fn r#cdmaSubscriptionSourceChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cdmaSource: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()> { self.0.r#cdmaSubscriptionSourceChanged(_arg_type, _arg_cdmaSource) }
              fn r#simPhonebookChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#simPhonebookChanged(_arg_type) }
              fn r#simPhonebookRecordsReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_PbReceivedStatus, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo]) -> binder::Result<()> { self.0.r#simPhonebookRecordsReceived(_arg_type, _arg_status, _arg_records) }
              fn r#simRefresh(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_refreshResult: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_SimRefreshResult) -> binder::Result<()> { self.0.r#simRefresh(_arg_type, _arg_refreshResult) }
              fn r#simStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#simStatusChanged(_arg_type) }
              fn r#stkEventNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<()> { self.0.r#stkEventNotify(_arg_type, _arg_cmd) }
              fn r#stkProactiveCommand(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cmd: &str) -> binder::Result<()> { self.0.r#stkProactiveCommand(_arg_type, _arg_cmd) }
              fn r#stkSessionEnd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#stkSessionEnd(_arg_type) }
              fn r#subscriptionStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_activate: bool) -> binder::Result<()> { self.0.r#subscriptionStatusChanged(_arg_type, _arg_activate) }
              fn r#uiccApplicationsEnablementChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_enabled: bool) -> binder::Result<()> { self.0.r#uiccApplicationsEnablementChanged(_arg_type, _arg_enabled) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioSimIndication, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#carrierInfoForImsiEncryption => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#carrierInfoForImsiEncryption(_arg_info);
                  Ok(())
                }
                transactions::r#cdmaSubscriptionSourceChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_cdmaSource: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#cdmaSubscriptionSourceChanged(_arg_type, _arg_cdmaSource);
                  Ok(())
                }
                transactions::r#simPhonebookChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#simPhonebookChanged(_arg_type);
                  Ok(())
                }
                transactions::r#simPhonebookRecordsReceived => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_status: crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_PbReceivedStatus = _aidl_data.read()?;
                  let _arg_records: Vec<crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#simPhonebookRecordsReceived(_arg_type, _arg_status, &_arg_records);
                  Ok(())
                }
                transactions::r#simRefresh => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_refreshResult: crate::mangled::_7_android_8_hardware_5_radio_3_sim_16_SimRefreshResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#simRefresh(_arg_type, &_arg_refreshResult);
                  Ok(())
                }
                transactions::r#simStatusChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#simStatusChanged(_arg_type);
                  Ok(())
                }
                transactions::r#stkEventNotify => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_cmd: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#stkEventNotify(_arg_type, &_arg_cmd);
                  Ok(())
                }
                transactions::r#stkProactiveCommand => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_cmd: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#stkProactiveCommand(_arg_type, &_arg_cmd);
                  Ok(())
                }
                transactions::r#stkSessionEnd => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#stkSessionEnd(_arg_type);
                  Ok(())
                }
                transactions::r#subscriptionStatusChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_activate: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#subscriptionStatusChanged(_arg_type, _arg_activate);
                  Ok(())
                }
                transactions::r#uiccApplicationsEnablementChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_enabled: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#uiccApplicationsEnablementChanged(_arg_type, _arg_enabled);
                  Ok(())
                }
                transactions::r#getInterfaceVersion => {
                  let _aidl_return = _aidl_service.r#getInterfaceVersion();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getInterfaceHash => {
                  let _aidl_return = _aidl_service.r#getInterfaceHash();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
              }
            }
            pub(crate) mod mangled {
             pub use super::r#IRadioSimIndication as _7_android_8_hardware_5_radio_3_sim_19_IRadioSimIndication;
            }
          }
          pub mod IRadioSimResponse {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioSimResponse["android.hardware.radio.sim.IRadioSimResponse"] {
                native: BnRadioSimResponse(on_transact),
                proxy: BpRadioSimResponse {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioSimResponseAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioSimResponse: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.sim.IRadioSimResponse" }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#areUiccApplicationsEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enabled: bool) -> binder::Result<()>;
              fn r#changeIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              fn r#changeIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              fn r#enableUiccApplicationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#getAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<()>;
              fn r#getCdmaSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mdn: &str, _arg_hSid: &str, _arg_hNid: &str, _arg_min: &str, _arg_prl: &str) -> binder::Result<()>;
              fn r#getCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_source: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()>;
              fn r#getFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_response: i32) -> binder::Result<()>;
              fn r#getIccCardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cardStatus: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_10_CardStatus) -> binder::Result<()>;
              fn r#getImsiForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imsi: &str) -> binder::Result<()>;
              fn r#getSimPhonebookCapacityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_capacity: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_PhonebookCapacity) -> binder::Result<()>;
              fn r#getSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#iccCloseLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#iccIoForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()>;
              fn r#iccOpenLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_channelId: i32, _arg_selectResponse: &[u8]) -> binder::Result<()>;
              fn r#iccTransmitApduBasicChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()>;
              fn r#iccTransmitApduLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()>;
              fn r#reportStkServiceIsRunningResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#requestIccSimAuthenticationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()>;
              fn r#sendEnvelopeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_commandResponse: &str) -> binder::Result<()>;
              fn r#sendEnvelopeWithStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()>;
              fn r#sendTerminalResponseToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setCarrierInfoForImsiEncryptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_retry: i32) -> binder::Result<()>;
              fn r#setSimCardPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setUiccSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#supplyIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              fn r#supplyIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              fn r#supplyIccPuk2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              fn r#supplyIccPukForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              fn r#supplySimDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_remainingRetries: i32) -> binder::Result<()>;
              fn r#updateSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_updatedRecordIndex: i32) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioSimResponseDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioSimResponseDefaultRef) -> IRadioSimResponseDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioSimResponseAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.sim.IRadioSimResponse" }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#areUiccApplicationsEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enabled: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#changeIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#changeIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#enableUiccApplicationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> std::future::Ready<binder::Result<()>>;
              fn r#getCdmaSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mdn: &str, _arg_hSid: &str, _arg_hNid: &str, _arg_min: &str, _arg_prl: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#getCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_source: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> std::future::Ready<binder::Result<()>>;
              fn r#getFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_response: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getIccCardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cardStatus: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_10_CardStatus) -> std::future::Ready<binder::Result<()>>;
              fn r#getImsiForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imsi: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#getSimPhonebookCapacityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_capacity: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_PhonebookCapacity) -> std::future::Ready<binder::Result<()>>;
              fn r#getSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#iccCloseLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#iccIoForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> std::future::Ready<binder::Result<()>>;
              fn r#iccOpenLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_channelId: i32, _arg_selectResponse: &[u8]) -> std::future::Ready<binder::Result<()>>;
              fn r#iccTransmitApduBasicChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> std::future::Ready<binder::Result<()>>;
              fn r#iccTransmitApduLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> std::future::Ready<binder::Result<()>>;
              fn r#reportStkServiceIsRunningResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#requestIccSimAuthenticationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> std::future::Ready<binder::Result<()>>;
              fn r#sendEnvelopeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_commandResponse: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#sendEnvelopeWithStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> std::future::Ready<binder::Result<()>>;
              fn r#sendTerminalResponseToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setCarrierInfoForImsiEncryptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_retry: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#setSimCardPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setUiccSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#supplyIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#supplyIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#supplyIccPuk2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#supplyIccPukForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#supplySimDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#updateSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_updatedRecordIndex: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioSimResponseAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.sim.IRadioSimResponse" }
              async fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#areUiccApplicationsEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enabled: bool) -> binder::Result<()>;
              async fn r#changeIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              async fn r#changeIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              async fn r#enableUiccApplicationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#getAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<()>;
              async fn r#getCdmaSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mdn: &str, _arg_hSid: &str, _arg_hNid: &str, _arg_min: &str, _arg_prl: &str) -> binder::Result<()>;
              async fn r#getCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_source: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()>;
              async fn r#getFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_response: i32) -> binder::Result<()>;
              async fn r#getIccCardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cardStatus: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_10_CardStatus) -> binder::Result<()>;
              async fn r#getImsiForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imsi: &str) -> binder::Result<()>;
              async fn r#getSimPhonebookCapacityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_capacity: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_PhonebookCapacity) -> binder::Result<()>;
              async fn r#getSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#iccCloseLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#iccIoForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()>;
              async fn r#iccOpenLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_channelId: i32, _arg_selectResponse: &[u8]) -> binder::Result<()>;
              async fn r#iccTransmitApduBasicChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()>;
              async fn r#iccTransmitApduLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()>;
              async fn r#reportStkServiceIsRunningResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#requestIccSimAuthenticationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()>;
              async fn r#sendEnvelopeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_commandResponse: &str) -> binder::Result<()>;
              async fn r#sendEnvelopeWithStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()>;
              async fn r#sendTerminalResponseToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setCarrierInfoForImsiEncryptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_retry: i32) -> binder::Result<()>;
              async fn r#setSimCardPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setUiccSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#supplyIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              async fn r#supplyIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              async fn r#supplyIccPuk2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              async fn r#supplyIccPukForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              async fn r#supplySimDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_remainingRetries: i32) -> binder::Result<()>;
              async fn r#updateSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_updatedRecordIndex: i32) -> binder::Result<()>;
            }
            impl BnRadioSimResponse {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioSimResponse>
              where
                T: IRadioSimResponseAsyncServer + binder::Interface + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                struct Wrapper<T, R> {
                  _inner: T,
                  _rt: R,
                }
                impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync {
                  fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
                  fn dump(&self, _file: &std::fs::File, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_file, _args) }
                }
                impl<T, R> IRadioSimResponse for Wrapper<T, R>
                where
                  T: IRadioSimResponseAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acknowledgeRequest(_arg_serial))
                  }
                  fn r#areUiccApplicationsEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enabled: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#areUiccApplicationsEnabledResponse(_arg_info, _arg_enabled))
                  }
                  fn r#changeIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#changeIccPin2ForAppResponse(_arg_info, _arg_remainingRetries))
                  }
                  fn r#changeIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#changeIccPinForAppResponse(_arg_info, _arg_remainingRetries))
                  }
                  fn r#enableUiccApplicationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#enableUiccApplicationsResponse(_arg_info))
                  }
                  fn r#getAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getAllowedCarriersResponse(_arg_info, _arg_carriers, _arg_multiSimPolicy))
                  }
                  fn r#getCdmaSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mdn: &str, _arg_hSid: &str, _arg_hNid: &str, _arg_min: &str, _arg_prl: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCdmaSubscriptionResponse(_arg_info, _arg_mdn, _arg_hSid, _arg_hNid, _arg_min, _arg_prl))
                  }
                  fn r#getCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_source: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCdmaSubscriptionSourceResponse(_arg_info, _arg_source))
                  }
                  fn r#getFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_response: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getFacilityLockForAppResponse(_arg_info, _arg_response))
                  }
                  fn r#getIccCardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cardStatus: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_10_CardStatus) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getIccCardStatusResponse(_arg_info, _arg_cardStatus))
                  }
                  fn r#getImsiForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imsi: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getImsiForAppResponse(_arg_info, _arg_imsi))
                  }
                  fn r#getSimPhonebookCapacityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_capacity: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_PhonebookCapacity) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSimPhonebookCapacityResponse(_arg_info, _arg_capacity))
                  }
                  fn r#getSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSimPhonebookRecordsResponse(_arg_info))
                  }
                  fn r#iccCloseLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#iccCloseLogicalChannelResponse(_arg_info))
                  }
                  fn r#iccIoForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#iccIoForAppResponse(_arg_info, _arg_iccIo))
                  }
                  fn r#iccOpenLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_channelId: i32, _arg_selectResponse: &[u8]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#iccOpenLogicalChannelResponse(_arg_info, _arg_channelId, _arg_selectResponse))
                  }
                  fn r#iccTransmitApduBasicChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#iccTransmitApduBasicChannelResponse(_arg_info, _arg_result))
                  }
                  fn r#iccTransmitApduLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#iccTransmitApduLogicalChannelResponse(_arg_info, _arg_result))
                  }
                  fn r#reportStkServiceIsRunningResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#reportStkServiceIsRunningResponse(_arg_info))
                  }
                  fn r#requestIccSimAuthenticationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#requestIccSimAuthenticationResponse(_arg_info, _arg_result))
                  }
                  fn r#sendEnvelopeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_commandResponse: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendEnvelopeResponse(_arg_info, _arg_commandResponse))
                  }
                  fn r#sendEnvelopeWithStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendEnvelopeWithStatusResponse(_arg_info, _arg_iccIo))
                  }
                  fn r#sendTerminalResponseToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendTerminalResponseToSimResponse(_arg_info))
                  }
                  fn r#setAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setAllowedCarriersResponse(_arg_info))
                  }
                  fn r#setCarrierInfoForImsiEncryptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCarrierInfoForImsiEncryptionResponse(_arg_info))
                  }
                  fn r#setCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCdmaSubscriptionSourceResponse(_arg_info))
                  }
                  fn r#setFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_retry: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setFacilityLockForAppResponse(_arg_info, _arg_retry))
                  }
                  fn r#setSimCardPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setSimCardPowerResponse(_arg_info))
                  }
                  fn r#setUiccSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setUiccSubscriptionResponse(_arg_info))
                  }
                  fn r#supplyIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#supplyIccPin2ForAppResponse(_arg_info, _arg_remainingRetries))
                  }
                  fn r#supplyIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#supplyIccPinForAppResponse(_arg_info, _arg_remainingRetries))
                  }
                  fn r#supplyIccPuk2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#supplyIccPuk2ForAppResponse(_arg_info, _arg_remainingRetries))
                  }
                  fn r#supplyIccPukForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#supplyIccPukForAppResponse(_arg_info, _arg_remainingRetries))
                  }
                  fn r#supplySimDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_remainingRetries: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#supplySimDepersonalizationResponse(_arg_info, _arg_persoType, _arg_remainingRetries))
                  }
                  fn r#updateSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_updatedRecordIndex: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#updateSimPhonebookRecordsResponse(_arg_info, _arg_updatedRecordIndex))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioSimResponseDefault: Send + Sync {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#areUiccApplicationsEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enabled: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#changeIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#changeIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#enableUiccApplicationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCdmaSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mdn: &str, _arg_hSid: &str, _arg_hNid: &str, _arg_min: &str, _arg_prl: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_source: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_response: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getIccCardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cardStatus: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_10_CardStatus) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getImsiForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imsi: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSimPhonebookCapacityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_capacity: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_PhonebookCapacity) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#iccCloseLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#iccIoForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#iccOpenLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_channelId: i32, _arg_selectResponse: &[u8]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#iccTransmitApduBasicChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#iccTransmitApduLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#reportStkServiceIsRunningResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#requestIccSimAuthenticationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendEnvelopeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_commandResponse: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendEnvelopeWithStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendTerminalResponseToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCarrierInfoForImsiEncryptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_retry: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setSimCardPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setUiccSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#supplyIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#supplyIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#supplyIccPuk2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#supplyIccPukForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#supplySimDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_remainingRetries: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#updateSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_updatedRecordIndex: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#acknowledgeRequest: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#areUiccApplicationsEnabledResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#changeIccPin2ForAppResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#changeIccPinForAppResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#enableUiccApplicationsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#getAllowedCarriersResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#getCdmaSubscriptionResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#getCdmaSubscriptionSourceResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#getFacilityLockForAppResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#getIccCardStatusResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#getImsiForAppResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#getSimPhonebookCapacityResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#getSimPhonebookRecordsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#iccCloseLogicalChannelResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#iccIoForAppResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#iccOpenLogicalChannelResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#iccTransmitApduBasicChannelResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
              pub const r#iccTransmitApduLogicalChannelResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
              pub const r#reportStkServiceIsRunningResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
              pub const r#requestIccSimAuthenticationResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
              pub const r#sendEnvelopeResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 20;
              pub const r#sendEnvelopeWithStatusResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 21;
              pub const r#sendTerminalResponseToSimResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 22;
              pub const r#setAllowedCarriersResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 23;
              pub const r#setCarrierInfoForImsiEncryptionResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 24;
              pub const r#setCdmaSubscriptionSourceResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 25;
              pub const r#setFacilityLockForAppResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 26;
              pub const r#setSimCardPowerResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 27;
              pub const r#setUiccSubscriptionResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 28;
              pub const r#supplyIccPin2ForAppResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 29;
              pub const r#supplyIccPinForAppResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 30;
              pub const r#supplyIccPuk2ForAppResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 31;
              pub const r#supplyIccPukForAppResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 32;
              pub const r#supplySimDepersonalizationResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 33;
              pub const r#updateSimPhonebookRecordsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 34;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioSimResponseDefaultRef = Option<std::sync::Arc<dyn IRadioSimResponseDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioSimResponseDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "01cea196fdf8f5e41fda8dc41125f1cc2b96f757";
            impl BpRadioSimResponse {
              fn build_parcel_acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_acknowledgeRequest(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#acknowledgeRequest(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_areUiccApplicationsEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enabled: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_enabled)?;
                Ok(aidl_data)
              }
              fn read_response_areUiccApplicationsEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enabled: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#areUiccApplicationsEnabledResponse(_arg_info, _arg_enabled);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_changeIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_remainingRetries)?;
                Ok(aidl_data)
              }
              fn read_response_changeIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#changeIccPin2ForAppResponse(_arg_info, _arg_remainingRetries);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_changeIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_remainingRetries)?;
                Ok(aidl_data)
              }
              fn read_response_changeIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#changeIccPinForAppResponse(_arg_info, _arg_remainingRetries);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_enableUiccApplicationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_enableUiccApplicationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#enableUiccApplicationsResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_carriers)?;
                aidl_data.write(&_arg_multiSimPolicy)?;
                Ok(aidl_data)
              }
              fn read_response_getAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getAllowedCarriersResponse(_arg_info, _arg_carriers, _arg_multiSimPolicy);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCdmaSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mdn: &str, _arg_hSid: &str, _arg_hNid: &str, _arg_min: &str, _arg_prl: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_mdn)?;
                aidl_data.write(_arg_hSid)?;
                aidl_data.write(_arg_hNid)?;
                aidl_data.write(_arg_min)?;
                aidl_data.write(_arg_prl)?;
                Ok(aidl_data)
              }
              fn read_response_getCdmaSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mdn: &str, _arg_hSid: &str, _arg_hNid: &str, _arg_min: &str, _arg_prl: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCdmaSubscriptionResponse(_arg_info, _arg_mdn, _arg_hSid, _arg_hNid, _arg_min, _arg_prl);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_source: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_source)?;
                Ok(aidl_data)
              }
              fn read_response_getCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_source: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCdmaSubscriptionSourceResponse(_arg_info, _arg_source);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_response: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_response)?;
                Ok(aidl_data)
              }
              fn read_response_getFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_response: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getFacilityLockForAppResponse(_arg_info, _arg_response);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getIccCardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cardStatus: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_10_CardStatus) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_cardStatus)?;
                Ok(aidl_data)
              }
              fn read_response_getIccCardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cardStatus: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_10_CardStatus, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getIccCardStatusResponse(_arg_info, _arg_cardStatus);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getImsiForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imsi: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_imsi)?;
                Ok(aidl_data)
              }
              fn read_response_getImsiForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imsi: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getImsiForAppResponse(_arg_info, _arg_imsi);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSimPhonebookCapacityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_capacity: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_PhonebookCapacity) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_capacity)?;
                Ok(aidl_data)
              }
              fn read_response_getSimPhonebookCapacityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_capacity: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_PhonebookCapacity, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSimPhonebookCapacityResponse(_arg_info, _arg_capacity);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_getSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSimPhonebookRecordsResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_iccCloseLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_iccCloseLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#iccCloseLogicalChannelResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_iccIoForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_iccIo)?;
                Ok(aidl_data)
              }
              fn read_response_iccIoForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#iccIoForAppResponse(_arg_info, _arg_iccIo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_iccOpenLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_channelId: i32, _arg_selectResponse: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_channelId)?;
                aidl_data.write(_arg_selectResponse)?;
                Ok(aidl_data)
              }
              fn read_response_iccOpenLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_channelId: i32, _arg_selectResponse: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#iccOpenLogicalChannelResponse(_arg_info, _arg_channelId, _arg_selectResponse);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_iccTransmitApduBasicChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_result)?;
                Ok(aidl_data)
              }
              fn read_response_iccTransmitApduBasicChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#iccTransmitApduBasicChannelResponse(_arg_info, _arg_result);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_iccTransmitApduLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_result)?;
                Ok(aidl_data)
              }
              fn read_response_iccTransmitApduLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#iccTransmitApduLogicalChannelResponse(_arg_info, _arg_result);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_reportStkServiceIsRunningResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_reportStkServiceIsRunningResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#reportStkServiceIsRunningResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_requestIccSimAuthenticationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_result)?;
                Ok(aidl_data)
              }
              fn read_response_requestIccSimAuthenticationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#requestIccSimAuthenticationResponse(_arg_info, _arg_result);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendEnvelopeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_commandResponse: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_commandResponse)?;
                Ok(aidl_data)
              }
              fn read_response_sendEnvelopeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_commandResponse: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendEnvelopeResponse(_arg_info, _arg_commandResponse);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendEnvelopeWithStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_iccIo)?;
                Ok(aidl_data)
              }
              fn read_response_sendEnvelopeWithStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendEnvelopeWithStatusResponse(_arg_info, _arg_iccIo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendTerminalResponseToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_sendTerminalResponseToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendTerminalResponseToSimResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setAllowedCarriersResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCarrierInfoForImsiEncryptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setCarrierInfoForImsiEncryptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCarrierInfoForImsiEncryptionResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCdmaSubscriptionSourceResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_retry: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_retry)?;
                Ok(aidl_data)
              }
              fn read_response_setFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_retry: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setFacilityLockForAppResponse(_arg_info, _arg_retry);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setSimCardPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setSimCardPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setSimCardPowerResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setUiccSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setUiccSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setUiccSubscriptionResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_supplyIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_remainingRetries)?;
                Ok(aidl_data)
              }
              fn read_response_supplyIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#supplyIccPin2ForAppResponse(_arg_info, _arg_remainingRetries);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_supplyIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_remainingRetries)?;
                Ok(aidl_data)
              }
              fn read_response_supplyIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#supplyIccPinForAppResponse(_arg_info, _arg_remainingRetries);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_supplyIccPuk2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_remainingRetries)?;
                Ok(aidl_data)
              }
              fn read_response_supplyIccPuk2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#supplyIccPuk2ForAppResponse(_arg_info, _arg_remainingRetries);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_supplyIccPukForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_remainingRetries)?;
                Ok(aidl_data)
              }
              fn read_response_supplyIccPukForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#supplyIccPukForAppResponse(_arg_info, _arg_remainingRetries);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_supplySimDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_remainingRetries: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_persoType)?;
                aidl_data.write(&_arg_remainingRetries)?;
                Ok(aidl_data)
              }
              fn read_response_supplySimDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_remainingRetries: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#supplySimDepersonalizationResponse(_arg_info, _arg_persoType, _arg_remainingRetries);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_updateSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_updatedRecordIndex: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_updatedRecordIndex)?;
                Ok(aidl_data)
              }
              fn read_response_updateSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_updatedRecordIndex: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioSimResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#updateSimPhonebookRecordsResponse(_arg_info, _arg_updatedRecordIndex);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getInterfaceVersion(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceVersion(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                self.cached_version.store(_aidl_return, std::sync::atomic::Ordering::Relaxed);
                Ok(_aidl_return)
              }
              fn build_parcel_getInterfaceHash(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceHash(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: String = _aidl_reply.read()?;
                *self.cached_hash.lock().unwrap() = Some(_aidl_return.clone());
                Ok(_aidl_return)
              }
            }
            impl IRadioSimResponse for BpRadioSimResponse {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acknowledgeRequest(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeRequest, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acknowledgeRequest(_arg_serial, _aidl_reply)
              }
              fn r#areUiccApplicationsEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enabled: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_areUiccApplicationsEnabledResponse(_arg_info, _arg_enabled)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#areUiccApplicationsEnabledResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_areUiccApplicationsEnabledResponse(_arg_info, _arg_enabled, _aidl_reply)
              }
              fn r#changeIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_changeIccPin2ForAppResponse(_arg_info, _arg_remainingRetries)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#changeIccPin2ForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_changeIccPin2ForAppResponse(_arg_info, _arg_remainingRetries, _aidl_reply)
              }
              fn r#changeIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_changeIccPinForAppResponse(_arg_info, _arg_remainingRetries)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#changeIccPinForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_changeIccPinForAppResponse(_arg_info, _arg_remainingRetries, _aidl_reply)
              }
              fn r#enableUiccApplicationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_enableUiccApplicationsResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#enableUiccApplicationsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_enableUiccApplicationsResponse(_arg_info, _aidl_reply)
              }
              fn r#getAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getAllowedCarriersResponse(_arg_info, _arg_carriers, _arg_multiSimPolicy)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAllowedCarriersResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getAllowedCarriersResponse(_arg_info, _arg_carriers, _arg_multiSimPolicy, _aidl_reply)
              }
              fn r#getCdmaSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mdn: &str, _arg_hSid: &str, _arg_hNid: &str, _arg_min: &str, _arg_prl: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCdmaSubscriptionResponse(_arg_info, _arg_mdn, _arg_hSid, _arg_hNid, _arg_min, _arg_prl)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaSubscriptionResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCdmaSubscriptionResponse(_arg_info, _arg_mdn, _arg_hSid, _arg_hNid, _arg_min, _arg_prl, _aidl_reply)
              }
              fn r#getCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_source: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCdmaSubscriptionSourceResponse(_arg_info, _arg_source)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaSubscriptionSourceResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCdmaSubscriptionSourceResponse(_arg_info, _arg_source, _aidl_reply)
              }
              fn r#getFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_response: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getFacilityLockForAppResponse(_arg_info, _arg_response)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getFacilityLockForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getFacilityLockForAppResponse(_arg_info, _arg_response, _aidl_reply)
              }
              fn r#getIccCardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cardStatus: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_10_CardStatus) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getIccCardStatusResponse(_arg_info, _arg_cardStatus)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getIccCardStatusResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getIccCardStatusResponse(_arg_info, _arg_cardStatus, _aidl_reply)
              }
              fn r#getImsiForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imsi: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getImsiForAppResponse(_arg_info, _arg_imsi)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getImsiForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getImsiForAppResponse(_arg_info, _arg_imsi, _aidl_reply)
              }
              fn r#getSimPhonebookCapacityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_capacity: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_PhonebookCapacity) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSimPhonebookCapacityResponse(_arg_info, _arg_capacity)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSimPhonebookCapacityResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSimPhonebookCapacityResponse(_arg_info, _arg_capacity, _aidl_reply)
              }
              fn r#getSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSimPhonebookRecordsResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSimPhonebookRecordsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSimPhonebookRecordsResponse(_arg_info, _aidl_reply)
              }
              fn r#iccCloseLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_iccCloseLogicalChannelResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccCloseLogicalChannelResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_iccCloseLogicalChannelResponse(_arg_info, _aidl_reply)
              }
              fn r#iccIoForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_iccIoForAppResponse(_arg_info, _arg_iccIo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccIoForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_iccIoForAppResponse(_arg_info, _arg_iccIo, _aidl_reply)
              }
              fn r#iccOpenLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_channelId: i32, _arg_selectResponse: &[u8]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_iccOpenLogicalChannelResponse(_arg_info, _arg_channelId, _arg_selectResponse)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccOpenLogicalChannelResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_iccOpenLogicalChannelResponse(_arg_info, _arg_channelId, _arg_selectResponse, _aidl_reply)
              }
              fn r#iccTransmitApduBasicChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_iccTransmitApduBasicChannelResponse(_arg_info, _arg_result)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccTransmitApduBasicChannelResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_iccTransmitApduBasicChannelResponse(_arg_info, _arg_result, _aidl_reply)
              }
              fn r#iccTransmitApduLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_iccTransmitApduLogicalChannelResponse(_arg_info, _arg_result)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccTransmitApduLogicalChannelResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_iccTransmitApduLogicalChannelResponse(_arg_info, _arg_result, _aidl_reply)
              }
              fn r#reportStkServiceIsRunningResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_reportStkServiceIsRunningResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#reportStkServiceIsRunningResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_reportStkServiceIsRunningResponse(_arg_info, _aidl_reply)
              }
              fn r#requestIccSimAuthenticationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_requestIccSimAuthenticationResponse(_arg_info, _arg_result)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#requestIccSimAuthenticationResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_requestIccSimAuthenticationResponse(_arg_info, _arg_result, _aidl_reply)
              }
              fn r#sendEnvelopeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_commandResponse: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendEnvelopeResponse(_arg_info, _arg_commandResponse)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendEnvelopeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendEnvelopeResponse(_arg_info, _arg_commandResponse, _aidl_reply)
              }
              fn r#sendEnvelopeWithStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendEnvelopeWithStatusResponse(_arg_info, _arg_iccIo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendEnvelopeWithStatusResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendEnvelopeWithStatusResponse(_arg_info, _arg_iccIo, _aidl_reply)
              }
              fn r#sendTerminalResponseToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendTerminalResponseToSimResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendTerminalResponseToSimResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendTerminalResponseToSimResponse(_arg_info, _aidl_reply)
              }
              fn r#setAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setAllowedCarriersResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setAllowedCarriersResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setAllowedCarriersResponse(_arg_info, _aidl_reply)
              }
              fn r#setCarrierInfoForImsiEncryptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCarrierInfoForImsiEncryptionResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCarrierInfoForImsiEncryptionResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCarrierInfoForImsiEncryptionResponse(_arg_info, _aidl_reply)
              }
              fn r#setCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCdmaSubscriptionSourceResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaSubscriptionSourceResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCdmaSubscriptionSourceResponse(_arg_info, _aidl_reply)
              }
              fn r#setFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_retry: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setFacilityLockForAppResponse(_arg_info, _arg_retry)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setFacilityLockForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setFacilityLockForAppResponse(_arg_info, _arg_retry, _aidl_reply)
              }
              fn r#setSimCardPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setSimCardPowerResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSimCardPowerResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setSimCardPowerResponse(_arg_info, _aidl_reply)
              }
              fn r#setUiccSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setUiccSubscriptionResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setUiccSubscriptionResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setUiccSubscriptionResponse(_arg_info, _aidl_reply)
              }
              fn r#supplyIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_supplyIccPin2ForAppResponse(_arg_info, _arg_remainingRetries)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPin2ForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_supplyIccPin2ForAppResponse(_arg_info, _arg_remainingRetries, _aidl_reply)
              }
              fn r#supplyIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_supplyIccPinForAppResponse(_arg_info, _arg_remainingRetries)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPinForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_supplyIccPinForAppResponse(_arg_info, _arg_remainingRetries, _aidl_reply)
              }
              fn r#supplyIccPuk2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_supplyIccPuk2ForAppResponse(_arg_info, _arg_remainingRetries)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPuk2ForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_supplyIccPuk2ForAppResponse(_arg_info, _arg_remainingRetries, _aidl_reply)
              }
              fn r#supplyIccPukForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_supplyIccPukForAppResponse(_arg_info, _arg_remainingRetries)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPukForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_supplyIccPukForAppResponse(_arg_info, _arg_remainingRetries, _aidl_reply)
              }
              fn r#supplySimDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_remainingRetries: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_supplySimDepersonalizationResponse(_arg_info, _arg_persoType, _arg_remainingRetries)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplySimDepersonalizationResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_supplySimDepersonalizationResponse(_arg_info, _arg_persoType, _arg_remainingRetries, _aidl_reply)
              }
              fn r#updateSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_updatedRecordIndex: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_updateSimPhonebookRecordsResponse(_arg_info, _arg_updatedRecordIndex)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#updateSimPhonebookRecordsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_updateSimPhonebookRecordsResponse(_arg_info, _arg_updatedRecordIndex, _aidl_reply)
              }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Ok(_aidl_version); }
                let _aidl_data = self.build_parcel_getInterfaceVersion()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceVersion(_aidl_reply)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Ok(_aidl_hash.clone());
                  }
                }
                let _aidl_data = self.build_parcel_getInterfaceHash()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceHash(_aidl_reply)
              }
            }
            impl<P: binder::BinderAsyncPool> IRadioSimResponseAsync<P> for BpRadioSimResponse {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acknowledgeRequest(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeRequest, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acknowledgeRequest(_arg_serial, _aidl_reply))
              }
              fn r#areUiccApplicationsEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enabled: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_areUiccApplicationsEnabledResponse(_arg_info, _arg_enabled) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#areUiccApplicationsEnabledResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_areUiccApplicationsEnabledResponse(_arg_info, _arg_enabled, _aidl_reply))
              }
              fn r#changeIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_changeIccPin2ForAppResponse(_arg_info, _arg_remainingRetries) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#changeIccPin2ForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_changeIccPin2ForAppResponse(_arg_info, _arg_remainingRetries, _aidl_reply))
              }
              fn r#changeIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_changeIccPinForAppResponse(_arg_info, _arg_remainingRetries) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#changeIccPinForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_changeIccPinForAppResponse(_arg_info, _arg_remainingRetries, _aidl_reply))
              }
              fn r#enableUiccApplicationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_enableUiccApplicationsResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#enableUiccApplicationsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_enableUiccApplicationsResponse(_arg_info, _aidl_reply))
              }
              fn r#getAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getAllowedCarriersResponse(_arg_info, _arg_carriers, _arg_multiSimPolicy) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAllowedCarriersResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getAllowedCarriersResponse(_arg_info, _arg_carriers, _arg_multiSimPolicy, _aidl_reply))
              }
              fn r#getCdmaSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mdn: &str, _arg_hSid: &str, _arg_hNid: &str, _arg_min: &str, _arg_prl: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCdmaSubscriptionResponse(_arg_info, _arg_mdn, _arg_hSid, _arg_hNid, _arg_min, _arg_prl) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaSubscriptionResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCdmaSubscriptionResponse(_arg_info, _arg_mdn, _arg_hSid, _arg_hNid, _arg_min, _arg_prl, _aidl_reply))
              }
              fn r#getCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_source: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCdmaSubscriptionSourceResponse(_arg_info, _arg_source) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaSubscriptionSourceResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCdmaSubscriptionSourceResponse(_arg_info, _arg_source, _aidl_reply))
              }
              fn r#getFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_response: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getFacilityLockForAppResponse(_arg_info, _arg_response) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getFacilityLockForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getFacilityLockForAppResponse(_arg_info, _arg_response, _aidl_reply))
              }
              fn r#getIccCardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cardStatus: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_10_CardStatus) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getIccCardStatusResponse(_arg_info, _arg_cardStatus) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getIccCardStatusResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getIccCardStatusResponse(_arg_info, _arg_cardStatus, _aidl_reply))
              }
              fn r#getImsiForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imsi: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getImsiForAppResponse(_arg_info, _arg_imsi) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getImsiForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getImsiForAppResponse(_arg_info, _arg_imsi, _aidl_reply))
              }
              fn r#getSimPhonebookCapacityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_capacity: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_PhonebookCapacity) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSimPhonebookCapacityResponse(_arg_info, _arg_capacity) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSimPhonebookCapacityResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSimPhonebookCapacityResponse(_arg_info, _arg_capacity, _aidl_reply))
              }
              fn r#getSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSimPhonebookRecordsResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSimPhonebookRecordsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSimPhonebookRecordsResponse(_arg_info, _aidl_reply))
              }
              fn r#iccCloseLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_iccCloseLogicalChannelResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccCloseLogicalChannelResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_iccCloseLogicalChannelResponse(_arg_info, _aidl_reply))
              }
              fn r#iccIoForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_iccIoForAppResponse(_arg_info, _arg_iccIo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccIoForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_iccIoForAppResponse(_arg_info, _arg_iccIo, _aidl_reply))
              }
              fn r#iccOpenLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_channelId: i32, _arg_selectResponse: &[u8]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_iccOpenLogicalChannelResponse(_arg_info, _arg_channelId, _arg_selectResponse) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccOpenLogicalChannelResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_iccOpenLogicalChannelResponse(_arg_info, _arg_channelId, _arg_selectResponse, _aidl_reply))
              }
              fn r#iccTransmitApduBasicChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_iccTransmitApduBasicChannelResponse(_arg_info, _arg_result) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccTransmitApduBasicChannelResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_iccTransmitApduBasicChannelResponse(_arg_info, _arg_result, _aidl_reply))
              }
              fn r#iccTransmitApduLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_iccTransmitApduLogicalChannelResponse(_arg_info, _arg_result) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#iccTransmitApduLogicalChannelResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_iccTransmitApduLogicalChannelResponse(_arg_info, _arg_result, _aidl_reply))
              }
              fn r#reportStkServiceIsRunningResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_reportStkServiceIsRunningResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#reportStkServiceIsRunningResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_reportStkServiceIsRunningResponse(_arg_info, _aidl_reply))
              }
              fn r#requestIccSimAuthenticationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_requestIccSimAuthenticationResponse(_arg_info, _arg_result) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#requestIccSimAuthenticationResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_requestIccSimAuthenticationResponse(_arg_info, _arg_result, _aidl_reply))
              }
              fn r#sendEnvelopeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_commandResponse: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendEnvelopeResponse(_arg_info, _arg_commandResponse) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendEnvelopeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendEnvelopeResponse(_arg_info, _arg_commandResponse, _aidl_reply))
              }
              fn r#sendEnvelopeWithStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendEnvelopeWithStatusResponse(_arg_info, _arg_iccIo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendEnvelopeWithStatusResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendEnvelopeWithStatusResponse(_arg_info, _arg_iccIo, _aidl_reply))
              }
              fn r#sendTerminalResponseToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendTerminalResponseToSimResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendTerminalResponseToSimResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendTerminalResponseToSimResponse(_arg_info, _aidl_reply))
              }
              fn r#setAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setAllowedCarriersResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setAllowedCarriersResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setAllowedCarriersResponse(_arg_info, _aidl_reply))
              }
              fn r#setCarrierInfoForImsiEncryptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCarrierInfoForImsiEncryptionResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCarrierInfoForImsiEncryptionResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCarrierInfoForImsiEncryptionResponse(_arg_info, _aidl_reply))
              }
              fn r#setCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCdmaSubscriptionSourceResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaSubscriptionSourceResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCdmaSubscriptionSourceResponse(_arg_info, _aidl_reply))
              }
              fn r#setFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_retry: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setFacilityLockForAppResponse(_arg_info, _arg_retry) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setFacilityLockForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setFacilityLockForAppResponse(_arg_info, _arg_retry, _aidl_reply))
              }
              fn r#setSimCardPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setSimCardPowerResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSimCardPowerResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setSimCardPowerResponse(_arg_info, _aidl_reply))
              }
              fn r#setUiccSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setUiccSubscriptionResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setUiccSubscriptionResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setUiccSubscriptionResponse(_arg_info, _aidl_reply))
              }
              fn r#supplyIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_supplyIccPin2ForAppResponse(_arg_info, _arg_remainingRetries) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPin2ForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_supplyIccPin2ForAppResponse(_arg_info, _arg_remainingRetries, _aidl_reply))
              }
              fn r#supplyIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_supplyIccPinForAppResponse(_arg_info, _arg_remainingRetries) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPinForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_supplyIccPinForAppResponse(_arg_info, _arg_remainingRetries, _aidl_reply))
              }
              fn r#supplyIccPuk2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_supplyIccPuk2ForAppResponse(_arg_info, _arg_remainingRetries) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPuk2ForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_supplyIccPuk2ForAppResponse(_arg_info, _arg_remainingRetries, _aidl_reply))
              }
              fn r#supplyIccPukForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_supplyIccPukForAppResponse(_arg_info, _arg_remainingRetries) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyIccPukForAppResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_supplyIccPukForAppResponse(_arg_info, _arg_remainingRetries, _aidl_reply))
              }
              fn r#supplySimDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_supplySimDepersonalizationResponse(_arg_info, _arg_persoType, _arg_remainingRetries) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplySimDepersonalizationResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_supplySimDepersonalizationResponse(_arg_info, _arg_persoType, _arg_remainingRetries, _aidl_reply))
              }
              fn r#updateSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_updatedRecordIndex: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_updateSimPhonebookRecordsResponse(_arg_info, _arg_updatedRecordIndex) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#updateSimPhonebookRecordsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_updateSimPhonebookRecordsResponse(_arg_info, _arg_updatedRecordIndex, _aidl_reply))
              }
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Box::pin(std::future::ready(Ok(_aidl_version))); }
                let _aidl_data = match self.build_parcel_getInterfaceVersion() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceVersion(_aidl_reply)
                  }
                )
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Box::pin(std::future::ready(Ok(_aidl_hash.clone())));
                  }
                }
                let _aidl_data = match self.build_parcel_getInterfaceHash() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceHash(_aidl_reply)
                  }
                )
              }
            }
            impl IRadioSimResponse for binder::binder_impl::Binder<BnRadioSimResponse> {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#acknowledgeRequest(_arg_serial) }
              fn r#areUiccApplicationsEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enabled: bool) -> binder::Result<()> { self.0.r#areUiccApplicationsEnabledResponse(_arg_info, _arg_enabled) }
              fn r#changeIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> { self.0.r#changeIccPin2ForAppResponse(_arg_info, _arg_remainingRetries) }
              fn r#changeIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> { self.0.r#changeIccPinForAppResponse(_arg_info, _arg_remainingRetries) }
              fn r#enableUiccApplicationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#enableUiccApplicationsResponse(_arg_info) }
              fn r#getAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_carriers: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions, _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy) -> binder::Result<()> { self.0.r#getAllowedCarriersResponse(_arg_info, _arg_carriers, _arg_multiSimPolicy) }
              fn r#getCdmaSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mdn: &str, _arg_hSid: &str, _arg_hNid: &str, _arg_min: &str, _arg_prl: &str) -> binder::Result<()> { self.0.r#getCdmaSubscriptionResponse(_arg_info, _arg_mdn, _arg_hSid, _arg_hNid, _arg_min, _arg_prl) }
              fn r#getCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_source: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource) -> binder::Result<()> { self.0.r#getCdmaSubscriptionSourceResponse(_arg_info, _arg_source) }
              fn r#getFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_response: i32) -> binder::Result<()> { self.0.r#getFacilityLockForAppResponse(_arg_info, _arg_response) }
              fn r#getIccCardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cardStatus: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_10_CardStatus) -> binder::Result<()> { self.0.r#getIccCardStatusResponse(_arg_info, _arg_cardStatus) }
              fn r#getImsiForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imsi: &str) -> binder::Result<()> { self.0.r#getImsiForAppResponse(_arg_info, _arg_imsi) }
              fn r#getSimPhonebookCapacityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_capacity: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_PhonebookCapacity) -> binder::Result<()> { self.0.r#getSimPhonebookCapacityResponse(_arg_info, _arg_capacity) }
              fn r#getSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#getSimPhonebookRecordsResponse(_arg_info) }
              fn r#iccCloseLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#iccCloseLogicalChannelResponse(_arg_info) }
              fn r#iccIoForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> { self.0.r#iccIoForAppResponse(_arg_info, _arg_iccIo) }
              fn r#iccOpenLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_channelId: i32, _arg_selectResponse: &[u8]) -> binder::Result<()> { self.0.r#iccOpenLogicalChannelResponse(_arg_info, _arg_channelId, _arg_selectResponse) }
              fn r#iccTransmitApduBasicChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> { self.0.r#iccTransmitApduBasicChannelResponse(_arg_info, _arg_result) }
              fn r#iccTransmitApduLogicalChannelResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> { self.0.r#iccTransmitApduLogicalChannelResponse(_arg_info, _arg_result) }
              fn r#reportStkServiceIsRunningResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#reportStkServiceIsRunningResponse(_arg_info) }
              fn r#requestIccSimAuthenticationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> { self.0.r#requestIccSimAuthenticationResponse(_arg_info, _arg_result) }
              fn r#sendEnvelopeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_commandResponse: &str) -> binder::Result<()> { self.0.r#sendEnvelopeResponse(_arg_info, _arg_commandResponse) }
              fn r#sendEnvelopeWithStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_iccIo: &crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult) -> binder::Result<()> { self.0.r#sendEnvelopeWithStatusResponse(_arg_info, _arg_iccIo) }
              fn r#sendTerminalResponseToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#sendTerminalResponseToSimResponse(_arg_info) }
              fn r#setAllowedCarriersResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setAllowedCarriersResponse(_arg_info) }
              fn r#setCarrierInfoForImsiEncryptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setCarrierInfoForImsiEncryptionResponse(_arg_info) }
              fn r#setCdmaSubscriptionSourceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setCdmaSubscriptionSourceResponse(_arg_info) }
              fn r#setFacilityLockForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_retry: i32) -> binder::Result<()> { self.0.r#setFacilityLockForAppResponse(_arg_info, _arg_retry) }
              fn r#setSimCardPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setSimCardPowerResponse(_arg_info) }
              fn r#setUiccSubscriptionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setUiccSubscriptionResponse(_arg_info) }
              fn r#supplyIccPin2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> { self.0.r#supplyIccPin2ForAppResponse(_arg_info, _arg_remainingRetries) }
              fn r#supplyIccPinForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> { self.0.r#supplyIccPinForAppResponse(_arg_info, _arg_remainingRetries) }
              fn r#supplyIccPuk2ForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> { self.0.r#supplyIccPuk2ForAppResponse(_arg_info, _arg_remainingRetries) }
              fn r#supplyIccPukForAppResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> { self.0.r#supplyIccPukForAppResponse(_arg_info, _arg_remainingRetries) }
              fn r#supplySimDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate, _arg_remainingRetries: i32) -> binder::Result<()> { self.0.r#supplySimDepersonalizationResponse(_arg_info, _arg_persoType, _arg_remainingRetries) }
              fn r#updateSimPhonebookRecordsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_updatedRecordIndex: i32) -> binder::Result<()> { self.0.r#updateSimPhonebookRecordsResponse(_arg_info, _arg_updatedRecordIndex) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioSimResponse, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#acknowledgeRequest => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acknowledgeRequest(_arg_serial);
                  Ok(())
                }
                transactions::r#areUiccApplicationsEnabledResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_enabled: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#areUiccApplicationsEnabledResponse(&_arg_info, _arg_enabled);
                  Ok(())
                }
                transactions::r#changeIccPin2ForAppResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_remainingRetries: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#changeIccPin2ForAppResponse(&_arg_info, _arg_remainingRetries);
                  Ok(())
                }
                transactions::r#changeIccPinForAppResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_remainingRetries: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#changeIccPinForAppResponse(&_arg_info, _arg_remainingRetries);
                  Ok(())
                }
                transactions::r#enableUiccApplicationsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#enableUiccApplicationsResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#getAllowedCarriersResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_carriers: crate::mangled::_7_android_8_hardware_5_radio_3_sim_19_CarrierRestrictions = _aidl_data.read()?;
                  let _arg_multiSimPolicy: crate::mangled::_7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getAllowedCarriersResponse(&_arg_info, &_arg_carriers, _arg_multiSimPolicy);
                  Ok(())
                }
                transactions::r#getCdmaSubscriptionResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_mdn: String = _aidl_data.read()?;
                  let _arg_hSid: String = _aidl_data.read()?;
                  let _arg_hNid: String = _aidl_data.read()?;
                  let _arg_min: String = _aidl_data.read()?;
                  let _arg_prl: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCdmaSubscriptionResponse(&_arg_info, &_arg_mdn, &_arg_hSid, &_arg_hNid, &_arg_min, &_arg_prl);
                  Ok(())
                }
                transactions::r#getCdmaSubscriptionSourceResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_source: crate::mangled::_7_android_8_hardware_5_radio_3_sim_22_CdmaSubscriptionSource = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCdmaSubscriptionSourceResponse(&_arg_info, _arg_source);
                  Ok(())
                }
                transactions::r#getFacilityLockForAppResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_response: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getFacilityLockForAppResponse(&_arg_info, _arg_response);
                  Ok(())
                }
                transactions::r#getIccCardStatusResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_cardStatus: crate::mangled::_7_android_8_hardware_5_radio_3_sim_10_CardStatus = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getIccCardStatusResponse(&_arg_info, &_arg_cardStatus);
                  Ok(())
                }
                transactions::r#getImsiForAppResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_imsi: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getImsiForAppResponse(&_arg_info, &_arg_imsi);
                  Ok(())
                }
                transactions::r#getSimPhonebookCapacityResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_capacity: crate::mangled::_7_android_8_hardware_5_radio_3_sim_17_PhonebookCapacity = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSimPhonebookCapacityResponse(&_arg_info, &_arg_capacity);
                  Ok(())
                }
                transactions::r#getSimPhonebookRecordsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSimPhonebookRecordsResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#iccCloseLogicalChannelResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#iccCloseLogicalChannelResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#iccIoForAppResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_iccIo: crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#iccIoForAppResponse(&_arg_info, &_arg_iccIo);
                  Ok(())
                }
                transactions::r#iccOpenLogicalChannelResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_channelId: i32 = _aidl_data.read()?;
                  let _arg_selectResponse: Vec<u8> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#iccOpenLogicalChannelResponse(&_arg_info, _arg_channelId, &_arg_selectResponse);
                  Ok(())
                }
                transactions::r#iccTransmitApduBasicChannelResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_result: crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#iccTransmitApduBasicChannelResponse(&_arg_info, &_arg_result);
                  Ok(())
                }
                transactions::r#iccTransmitApduLogicalChannelResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_result: crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#iccTransmitApduLogicalChannelResponse(&_arg_info, &_arg_result);
                  Ok(())
                }
                transactions::r#reportStkServiceIsRunningResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#reportStkServiceIsRunningResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#requestIccSimAuthenticationResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_result: crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#requestIccSimAuthenticationResponse(&_arg_info, &_arg_result);
                  Ok(())
                }
                transactions::r#sendEnvelopeResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_commandResponse: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendEnvelopeResponse(&_arg_info, &_arg_commandResponse);
                  Ok(())
                }
                transactions::r#sendEnvelopeWithStatusResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_iccIo: crate::mangled::_7_android_8_hardware_5_radio_3_sim_11_IccIoResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendEnvelopeWithStatusResponse(&_arg_info, &_arg_iccIo);
                  Ok(())
                }
                transactions::r#sendTerminalResponseToSimResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendTerminalResponseToSimResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setAllowedCarriersResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setAllowedCarriersResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setCarrierInfoForImsiEncryptionResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCarrierInfoForImsiEncryptionResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setCdmaSubscriptionSourceResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCdmaSubscriptionSourceResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setFacilityLockForAppResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_retry: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setFacilityLockForAppResponse(&_arg_info, _arg_retry);
                  Ok(())
                }
                transactions::r#setSimCardPowerResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setSimCardPowerResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setUiccSubscriptionResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setUiccSubscriptionResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#supplyIccPin2ForAppResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_remainingRetries: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#supplyIccPin2ForAppResponse(&_arg_info, _arg_remainingRetries);
                  Ok(())
                }
                transactions::r#supplyIccPinForAppResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_remainingRetries: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#supplyIccPinForAppResponse(&_arg_info, _arg_remainingRetries);
                  Ok(())
                }
                transactions::r#supplyIccPuk2ForAppResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_remainingRetries: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#supplyIccPuk2ForAppResponse(&_arg_info, _arg_remainingRetries);
                  Ok(())
                }
                transactions::r#supplyIccPukForAppResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_remainingRetries: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#supplyIccPukForAppResponse(&_arg_info, _arg_remainingRetries);
                  Ok(())
                }
                transactions::r#supplySimDepersonalizationResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_persoType: crate::mangled::_7_android_8_hardware_5_radio_3_sim_13_PersoSubstate = _aidl_data.read()?;
                  let _arg_remainingRetries: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#supplySimDepersonalizationResponse(&_arg_info, _arg_persoType, _arg_remainingRetries);
                  Ok(())
                }
                transactions::r#updateSimPhonebookRecordsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_updatedRecordIndex: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#updateSimPhonebookRecordsResponse(&_arg_info, _arg_updatedRecordIndex);
                  Ok(())
                }
                transactions::r#getInterfaceVersion => {
                  let _aidl_return = _aidl_service.r#getInterfaceVersion();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getInterfaceHash => {
                  let _aidl_return = _aidl_service.r#getInterfaceHash();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
              }
            }
            pub(crate) mod mangled {
             pub use super::r#IRadioSimResponse as _7_android_8_hardware_5_radio_3_sim_17_IRadioSimResponse;
            }
          }
          pub mod IccIo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#IccIo {
              pub r#command: i32,
              pub r#fileId: i32,
              pub r#path: String,
              pub r#p1: i32,
              pub r#p2: i32,
              pub r#p3: i32,
              pub r#data: String,
              pub r#pin2: String,
              pub r#aid: String,
            }
            impl Default for r#IccIo {
              fn default() -> Self {
                Self {
                  r#command: 0,
                  r#fileId: 0,
                  r#path: Default::default(),
                  r#p1: 0,
                  r#p2: 0,
                  r#p3: 0,
                  r#data: Default::default(),
                  r#pin2: Default::default(),
                  r#aid: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#IccIo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#command)?;
                  subparcel.write(&self.r#fileId)?;
                  subparcel.write(&self.r#path)?;
                  subparcel.write(&self.r#p1)?;
                  subparcel.write(&self.r#p2)?;
                  subparcel.write(&self.r#p3)?;
                  subparcel.write(&self.r#data)?;
                  subparcel.write(&self.r#pin2)?;
                  subparcel.write(&self.r#aid)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#command = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#fileId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#path = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#p1 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#p2 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#p3 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#data = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#pin2 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#aid = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#IccIo);
            binder::impl_deserialize_for_parcelable!(r#IccIo);
            impl binder::binder_impl::ParcelableMetadata for r#IccIo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.sim.IccIo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#IccIo as _7_android_8_hardware_5_radio_3_sim_5_IccIo;
            }
          }
          pub mod IccIoResult {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#IccIoResult {
              pub r#sw1: i32,
              pub r#sw2: i32,
              pub r#simResponse: String,
            }
            impl Default for r#IccIoResult {
              fn default() -> Self {
                Self {
                  r#sw1: 0,
                  r#sw2: 0,
                  r#simResponse: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#IccIoResult {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#sw1)?;
                  subparcel.write(&self.r#sw2)?;
                  subparcel.write(&self.r#simResponse)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#sw1 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sw2 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#simResponse = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#IccIoResult);
            binder::impl_deserialize_for_parcelable!(r#IccIoResult);
            impl binder::binder_impl::ParcelableMetadata for r#IccIoResult {
              fn get_descriptor() -> &'static str { "android.hardware.radio.sim.IccIoResult" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#IccIoResult as _7_android_8_hardware_5_radio_3_sim_11_IccIoResult;
            }
          }
          pub mod ImsiEncryptionInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#ImsiEncryptionInfo {
              pub r#mcc: String,
              pub r#mnc: String,
              pub r#carrierKey: Vec<u8>,
              pub r#keyIdentifier: String,
              pub r#expirationTime: i64,
              pub r#keyType: i8,
            }
            pub const r#PUBLIC_KEY_TYPE_EPDG: i8 = 1;
            pub const r#PUBLIC_KEY_TYPE_WLAN: i8 = 2;
            impl Default for r#ImsiEncryptionInfo {
              fn default() -> Self {
                Self {
                  r#mcc: Default::default(),
                  r#mnc: Default::default(),
                  r#carrierKey: Default::default(),
                  r#keyIdentifier: Default::default(),
                  r#expirationTime: 0,
                  r#keyType: 0,
                }
              }
            }
            impl binder::Parcelable for r#ImsiEncryptionInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#mcc)?;
                  subparcel.write(&self.r#mnc)?;
                  subparcel.write(&self.r#carrierKey)?;
                  subparcel.write(&self.r#keyIdentifier)?;
                  subparcel.write(&self.r#expirationTime)?;
                  subparcel.write(&self.r#keyType)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#mcc = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mnc = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#carrierKey = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#keyIdentifier = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#expirationTime = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#keyType = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ImsiEncryptionInfo);
            binder::impl_deserialize_for_parcelable!(r#ImsiEncryptionInfo);
            impl binder::binder_impl::ParcelableMetadata for r#ImsiEncryptionInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.sim.ImsiEncryptionInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ImsiEncryptionInfo as _7_android_8_hardware_5_radio_3_sim_18_ImsiEncryptionInfo;
            }
          }
          pub mod PbReceivedStatus {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#PbReceivedStatus : [i8; 4] {
                r#PB_RECEIVED_OK = 1,
                r#PB_RECEIVED_ERROR = 2,
                r#PB_RECEIVED_ABORT = 3,
                r#PB_RECEIVED_FINAL = 4,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PbReceivedStatus as _7_android_8_hardware_5_radio_3_sim_16_PbReceivedStatus;
            }
          }
          pub mod PersoSubstate {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#PersoSubstate : [i32; 35] {
                r#UNKNOWN = 0,
                r#IN_PROGRESS = 1,
                r#READY = 2,
                r#SIM_NETWORK = 3,
                r#SIM_NETWORK_SUBSET = 4,
                r#SIM_CORPORATE = 5,
                r#SIM_SERVICE_PROVIDER = 6,
                r#SIM_SIM = 7,
                r#SIM_NETWORK_PUK = 8,
                r#SIM_NETWORK_SUBSET_PUK = 9,
                r#SIM_CORPORATE_PUK = 10,
                r#SIM_SERVICE_PROVIDER_PUK = 11,
                r#SIM_SIM_PUK = 12,
                r#RUIM_NETWORK1 = 13,
                r#RUIM_NETWORK2 = 14,
                r#RUIM_HRPD = 15,
                r#RUIM_CORPORATE = 16,
                r#RUIM_SERVICE_PROVIDER = 17,
                r#RUIM_RUIM = 18,
                r#RUIM_NETWORK1_PUK = 19,
                r#RUIM_NETWORK2_PUK = 20,
                r#RUIM_HRPD_PUK = 21,
                r#RUIM_CORPORATE_PUK = 22,
                r#RUIM_SERVICE_PROVIDER_PUK = 23,
                r#RUIM_RUIM_PUK = 24,
                r#SIM_SPN = 25,
                r#SIM_SPN_PUK = 26,
                r#SIM_SP_EHPLMN = 27,
                r#SIM_SP_EHPLMN_PUK = 28,
                r#SIM_ICCID = 29,
                r#SIM_ICCID_PUK = 30,
                r#SIM_IMPI = 31,
                r#SIM_IMPI_PUK = 32,
                r#SIM_NS_SP = 33,
                r#SIM_NS_SP_PUK = 34,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PersoSubstate as _7_android_8_hardware_5_radio_3_sim_13_PersoSubstate;
            }
          }
          pub mod PhonebookCapacity {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#PhonebookCapacity {
              pub r#maxAdnRecords: i32,
              pub r#usedAdnRecords: i32,
              pub r#maxEmailRecords: i32,
              pub r#usedEmailRecords: i32,
              pub r#maxAdditionalNumberRecords: i32,
              pub r#usedAdditionalNumberRecords: i32,
              pub r#maxNameLen: i32,
              pub r#maxNumberLen: i32,
              pub r#maxEmailLen: i32,
              pub r#maxAdditionalNumberLen: i32,
            }
            impl Default for r#PhonebookCapacity {
              fn default() -> Self {
                Self {
                  r#maxAdnRecords: 0,
                  r#usedAdnRecords: 0,
                  r#maxEmailRecords: 0,
                  r#usedEmailRecords: 0,
                  r#maxAdditionalNumberRecords: 0,
                  r#usedAdditionalNumberRecords: 0,
                  r#maxNameLen: 0,
                  r#maxNumberLen: 0,
                  r#maxEmailLen: 0,
                  r#maxAdditionalNumberLen: 0,
                }
              }
            }
            impl binder::Parcelable for r#PhonebookCapacity {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#maxAdnRecords)?;
                  subparcel.write(&self.r#usedAdnRecords)?;
                  subparcel.write(&self.r#maxEmailRecords)?;
                  subparcel.write(&self.r#usedEmailRecords)?;
                  subparcel.write(&self.r#maxAdditionalNumberRecords)?;
                  subparcel.write(&self.r#usedAdditionalNumberRecords)?;
                  subparcel.write(&self.r#maxNameLen)?;
                  subparcel.write(&self.r#maxNumberLen)?;
                  subparcel.write(&self.r#maxEmailLen)?;
                  subparcel.write(&self.r#maxAdditionalNumberLen)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#maxAdnRecords = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#usedAdnRecords = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxEmailRecords = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#usedEmailRecords = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxAdditionalNumberRecords = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#usedAdditionalNumberRecords = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxNameLen = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxNumberLen = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxEmailLen = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxAdditionalNumberLen = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PhonebookCapacity);
            binder::impl_deserialize_for_parcelable!(r#PhonebookCapacity);
            impl binder::binder_impl::ParcelableMetadata for r#PhonebookCapacity {
              fn get_descriptor() -> &'static str { "android.hardware.radio.sim.PhonebookCapacity" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PhonebookCapacity as _7_android_8_hardware_5_radio_3_sim_17_PhonebookCapacity;
            }
          }
          pub mod PhonebookRecordInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#PhonebookRecordInfo {
              pub r#recordId: i32,
              pub r#name: String,
              pub r#number: String,
              pub r#emails: Vec<String>,
              pub r#additionalNumbers: Vec<String>,
            }
            impl Default for r#PhonebookRecordInfo {
              fn default() -> Self {
                Self {
                  r#recordId: 0,
                  r#name: Default::default(),
                  r#number: Default::default(),
                  r#emails: Default::default(),
                  r#additionalNumbers: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#PhonebookRecordInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#recordId)?;
                  subparcel.write(&self.r#name)?;
                  subparcel.write(&self.r#number)?;
                  subparcel.write(&self.r#emails)?;
                  subparcel.write(&self.r#additionalNumbers)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#recordId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#name = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#number = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#emails = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#additionalNumbers = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PhonebookRecordInfo);
            binder::impl_deserialize_for_parcelable!(r#PhonebookRecordInfo);
            impl binder::binder_impl::ParcelableMetadata for r#PhonebookRecordInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.sim.PhonebookRecordInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PhonebookRecordInfo as _7_android_8_hardware_5_radio_3_sim_19_PhonebookRecordInfo;
            }
          }
          pub mod PinState {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#PinState : [i32; 6] {
                r#UNKNOWN = 0,
                r#ENABLED_NOT_VERIFIED = 1,
                r#ENABLED_VERIFIED = 2,
                r#DISABLED = 3,
                r#ENABLED_BLOCKED = 4,
                r#ENABLED_PERM_BLOCKED = 5,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PinState as _7_android_8_hardware_5_radio_3_sim_8_PinState;
            }
          }
          pub mod SelectUiccSub {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#SelectUiccSub {
              pub r#slot: i32,
              pub r#appIndex: i32,
              pub r#subType: i32,
              pub r#actStatus: i32,
            }
            pub const r#SUBSCRIPTION_TYPE_1: i32 = 0;
            pub const r#SUBSCRIPTION_TYPE_2: i32 = 1;
            pub const r#SUBSCRIPTION_TYPE_3: i32 = 2;
            pub const r#ACT_STATUS_DEACTIVATE: i32 = 0;
            pub const r#ACT_STATUS_ACTIVATE: i32 = 1;
            impl Default for r#SelectUiccSub {
              fn default() -> Self {
                Self {
                  r#slot: 0,
                  r#appIndex: 0,
                  r#subType: 0,
                  r#actStatus: 0,
                }
              }
            }
            impl binder::Parcelable for r#SelectUiccSub {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#slot)?;
                  subparcel.write(&self.r#appIndex)?;
                  subparcel.write(&self.r#subType)?;
                  subparcel.write(&self.r#actStatus)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#slot = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#appIndex = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#subType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#actStatus = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SelectUiccSub);
            binder::impl_deserialize_for_parcelable!(r#SelectUiccSub);
            impl binder::binder_impl::ParcelableMetadata for r#SelectUiccSub {
              fn get_descriptor() -> &'static str { "android.hardware.radio.sim.SelectUiccSub" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SelectUiccSub as _7_android_8_hardware_5_radio_3_sim_13_SelectUiccSub;
            }
          }
          pub mod SimApdu {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#SimApdu {
              pub r#sessionId: i32,
              pub r#cla: i32,
              pub r#instruction: i32,
              pub r#p1: i32,
              pub r#p2: i32,
              pub r#p3: i32,
              pub r#data: String,
            }
            impl Default for r#SimApdu {
              fn default() -> Self {
                Self {
                  r#sessionId: 0,
                  r#cla: 0,
                  r#instruction: 0,
                  r#p1: 0,
                  r#p2: 0,
                  r#p3: 0,
                  r#data: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#SimApdu {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#sessionId)?;
                  subparcel.write(&self.r#cla)?;
                  subparcel.write(&self.r#instruction)?;
                  subparcel.write(&self.r#p1)?;
                  subparcel.write(&self.r#p2)?;
                  subparcel.write(&self.r#p3)?;
                  subparcel.write(&self.r#data)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#sessionId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cla = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#instruction = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#p1 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#p2 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#p3 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#data = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SimApdu);
            binder::impl_deserialize_for_parcelable!(r#SimApdu);
            impl binder::binder_impl::ParcelableMetadata for r#SimApdu {
              fn get_descriptor() -> &'static str { "android.hardware.radio.sim.SimApdu" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SimApdu as _7_android_8_hardware_5_radio_3_sim_7_SimApdu;
            }
          }
          pub mod SimLockMultiSimPolicy {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#SimLockMultiSimPolicy : [i32; 2] {
                r#NO_MULTISIM_POLICY = 0,
                r#ONE_VALID_SIM_MUST_BE_PRESENT = 1,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#SimLockMultiSimPolicy as _7_android_8_hardware_5_radio_3_sim_21_SimLockMultiSimPolicy;
            }
          }
          pub mod SimRefreshResult {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#SimRefreshResult {
              pub r#type: i32,
              pub r#efId: i32,
              pub r#aid: String,
            }
            pub const r#TYPE_SIM_FILE_UPDATE: i32 = 0;
            pub const r#TYPE_SIM_INIT: i32 = 1;
            pub const r#TYPE_SIM_RESET: i32 = 2;
            impl Default for r#SimRefreshResult {
              fn default() -> Self {
                Self {
                  r#type: 0,
                  r#efId: 0,
                  r#aid: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#SimRefreshResult {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#type)?;
                  subparcel.write(&self.r#efId)?;
                  subparcel.write(&self.r#aid)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#type = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#efId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#aid = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SimRefreshResult);
            binder::impl_deserialize_for_parcelable!(r#SimRefreshResult);
            impl binder::binder_impl::ParcelableMetadata for r#SimRefreshResult {
              fn get_descriptor() -> &'static str { "android.hardware.radio.sim.SimRefreshResult" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SimRefreshResult as _7_android_8_hardware_5_radio_3_sim_16_SimRefreshResult;
            }
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::hardware::radio::sim::AppStatus::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::CardPowerState::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::CardStatus::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::Carrier::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::CarrierRestrictions::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::CdmaSubscriptionSource::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::IRadioSim::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::IRadioSimIndication::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::IRadioSimResponse::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::IccIo::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::IccIoResult::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::ImsiEncryptionInfo::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::PbReceivedStatus::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::PersoSubstate::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::PhonebookCapacity::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::PhonebookRecordInfo::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::PinState::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::SelectUiccSub::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::SimApdu::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::SimLockMultiSimPolicy::mangled::*;
  pub use super::aidl::android::hardware::radio::sim::SimRefreshResult::mangled::*;
  pub(crate) use android_hardware_radio::mangled::*;
  pub(crate) use android_hardware_radio_config::mangled::*;
}
