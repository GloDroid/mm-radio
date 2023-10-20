#![feature(custom_inner_attributes)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod hardware {
      pub mod radio {
        pub mod modem {
          pub mod ActivityStatsInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#ActivityStatsInfo {
              pub r#sleepModeTimeMs: i32,
              pub r#idleModeTimeMs: i32,
              pub r#techSpecificInfo: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_modem_29_ActivityStatsTechSpecificInfo>,
            }
            impl Default for r#ActivityStatsInfo {
              fn default() -> Self {
                Self {
                  r#sleepModeTimeMs: 0,
                  r#idleModeTimeMs: 0,
                  r#techSpecificInfo: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#ActivityStatsInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#sleepModeTimeMs)?;
                  subparcel.write(&self.r#idleModeTimeMs)?;
                  subparcel.write(&self.r#techSpecificInfo)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#sleepModeTimeMs = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#idleModeTimeMs = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#techSpecificInfo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ActivityStatsInfo);
            binder::impl_deserialize_for_parcelable!(r#ActivityStatsInfo);
            impl binder::binder_impl::ParcelableMetadata for r#ActivityStatsInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.modem.ActivityStatsInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ActivityStatsInfo as _7_android_8_hardware_5_radio_5_modem_17_ActivityStatsInfo;
            }
          }
          pub mod ActivityStatsTechSpecificInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#ActivityStatsTechSpecificInfo {
              pub r#rat: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork,
              pub r#frequencyRange: i32,
              pub r#txmModetimeMs: Vec<i32>,
              pub r#rxModeTimeMs: i32,
            }
            pub const r#FREQUENCY_RANGE_UNKNOWN: i32 = 0;
            pub const r#FREQUENCY_RANGE_LOW: i32 = 1;
            pub const r#FREQUENCY_RANGE_MID: i32 = 2;
            pub const r#FREQUENCY_RANGE_HIGH: i32 = 3;
            pub const r#FREQUENCY_RANGE_MMWAVE: i32 = 4;
            impl Default for r#ActivityStatsTechSpecificInfo {
              fn default() -> Self {
                Self {
                  r#rat: Default::default(),
                  r#frequencyRange: 0,
                  r#txmModetimeMs: Default::default(),
                  r#rxModeTimeMs: 0,
                }
              }
            }
            impl binder::Parcelable for r#ActivityStatsTechSpecificInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#rat)?;
                  subparcel.write(&self.r#frequencyRange)?;
                  subparcel.write(&self.r#txmModetimeMs)?;
                  subparcel.write(&self.r#rxModeTimeMs)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#rat = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#frequencyRange = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#txmModetimeMs = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#rxModeTimeMs = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ActivityStatsTechSpecificInfo);
            binder::impl_deserialize_for_parcelable!(r#ActivityStatsTechSpecificInfo);
            impl binder::binder_impl::ParcelableMetadata for r#ActivityStatsTechSpecificInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.modem.ActivityStatsTechSpecificInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ActivityStatsTechSpecificInfo as _7_android_8_hardware_5_radio_5_modem_29_ActivityStatsTechSpecificInfo;
            }
          }
          pub mod DeviceStateType {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#DeviceStateType : [i32; 3] {
                r#POWER_SAVE_MODE = 0,
                r#CHARGING_STATE = 1,
                r#LOW_DATA_EXPECTED = 2,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#DeviceStateType as _7_android_8_hardware_5_radio_5_modem_15_DeviceStateType;
            }
          }
          pub mod HardwareConfig {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#HardwareConfig {
              pub r#type: i32,
              pub r#uuid: String,
              pub r#state: i32,
              pub r#modem: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_modem_19_HardwareConfigModem>,
              pub r#sim: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_modem_17_HardwareConfigSim>,
            }
            pub const r#STATE_ENABLED: i32 = 0;
            pub const r#STATE_STANDBY: i32 = 1;
            pub const r#STATE_DISABLED: i32 = 2;
            pub const r#TYPE_MODEM: i32 = 0;
            pub const r#TYPE_SIM: i32 = 1;
            impl Default for r#HardwareConfig {
              fn default() -> Self {
                Self {
                  r#type: 0,
                  r#uuid: Default::default(),
                  r#state: 0,
                  r#modem: Default::default(),
                  r#sim: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#HardwareConfig {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#type)?;
                  subparcel.write(&self.r#uuid)?;
                  subparcel.write(&self.r#state)?;
                  subparcel.write(&self.r#modem)?;
                  subparcel.write(&self.r#sim)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#type = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#uuid = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#state = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#modem = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sim = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#HardwareConfig);
            binder::impl_deserialize_for_parcelable!(r#HardwareConfig);
            impl binder::binder_impl::ParcelableMetadata for r#HardwareConfig {
              fn get_descriptor() -> &'static str { "android.hardware.radio.modem.HardwareConfig" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#HardwareConfig as _7_android_8_hardware_5_radio_5_modem_14_HardwareConfig;
            }
          }
          pub mod HardwareConfigModem {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#HardwareConfigModem {
              pub r#rilModel: i32,
              pub r#rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology,
              pub r#maxVoiceCalls: i32,
              pub r#maxDataCalls: i32,
              pub r#maxStandby: i32,
            }
            impl Default for r#HardwareConfigModem {
              fn default() -> Self {
                Self {
                  r#rilModel: 0,
                  r#rat: Default::default(),
                  r#maxVoiceCalls: 0,
                  r#maxDataCalls: 0,
                  r#maxStandby: 0,
                }
              }
            }
            impl binder::Parcelable for r#HardwareConfigModem {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#rilModel)?;
                  subparcel.write(&self.r#rat)?;
                  subparcel.write(&self.r#maxVoiceCalls)?;
                  subparcel.write(&self.r#maxDataCalls)?;
                  subparcel.write(&self.r#maxStandby)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#rilModel = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#rat = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxVoiceCalls = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxDataCalls = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxStandby = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#HardwareConfigModem);
            binder::impl_deserialize_for_parcelable!(r#HardwareConfigModem);
            impl binder::binder_impl::ParcelableMetadata for r#HardwareConfigModem {
              fn get_descriptor() -> &'static str { "android.hardware.radio.modem.HardwareConfigModem" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#HardwareConfigModem as _7_android_8_hardware_5_radio_5_modem_19_HardwareConfigModem;
            }
          }
          pub mod HardwareConfigSim {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#HardwareConfigSim {
              pub r#modemUuid: String,
            }
            impl Default for r#HardwareConfigSim {
              fn default() -> Self {
                Self {
                  r#modemUuid: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#HardwareConfigSim {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#modemUuid)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#modemUuid = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#HardwareConfigSim);
            binder::impl_deserialize_for_parcelable!(r#HardwareConfigSim);
            impl binder::binder_impl::ParcelableMetadata for r#HardwareConfigSim {
              fn get_descriptor() -> &'static str { "android.hardware.radio.modem.HardwareConfigSim" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#HardwareConfigSim as _7_android_8_hardware_5_radio_5_modem_17_HardwareConfigSim;
            }
          }
          pub mod IRadioModem {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioModem["android.hardware.radio.modem.IRadioModem"] {
                native: BnRadioModem(on_transact),
                proxy: BpRadioModem {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioModemAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioModem: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.modem.IRadioModem" }
              fn r#enableModem(&self, _arg_serial: i32, _arg_on: bool) -> binder::Result<()>;
              fn r#getBasebandVersion(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getDeviceIdentity(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getHardwareConfig(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getModemActivityInfo(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getModemStackStatus(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getRadioCapability(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#nvReadItem(&self, _arg_serial: i32, _arg_itemId: crate::mangled::_7_android_8_hardware_5_radio_5_modem_6_NvItem) -> binder::Result<()>;
              fn r#nvResetConfig(&self, _arg_serial: i32, _arg_resetType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_ResetNvType) -> binder::Result<()>;
              fn r#nvWriteCdmaPrl(&self, _arg_serial: i32, _arg_prl: &[u8]) -> binder::Result<()>;
              fn r#nvWriteItem(&self, _arg_serial: i32, _arg_item: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_NvWriteItem) -> binder::Result<()>;
              fn r#requestShutdown(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#responseAcknowledgement(&self) -> binder::Result<()>;
              fn r#sendDeviceState(&self, _arg_serial: i32, _arg_deviceStateType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_DeviceStateType, _arg_state: bool) -> binder::Result<()>;
              fn r#setRadioCapability(&self, _arg_serial: i32, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()>;
              fn r#setRadioPower(&self, _arg_serial: i32, _arg_powerOn: bool, _arg_forEmergencyCall: bool, _arg_preferredForEmergencyCall: bool) -> binder::Result<()>;
              fn r#setResponseFunctions(&self, _arg_radioModemResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_19_IRadioModemResponse>, _arg_radioModemIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_21_IRadioModemIndication>) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioModemDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioModemDefaultRef) -> IRadioModemDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioModemAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.modem.IRadioModem" }
              fn r#enableModem(&self, _arg_serial: i32, _arg_on: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#getBasebandVersion(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getDeviceIdentity(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getHardwareConfig(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getModemActivityInfo(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getModemStackStatus(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getRadioCapability(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#nvReadItem(&self, _arg_serial: i32, _arg_itemId: crate::mangled::_7_android_8_hardware_5_radio_5_modem_6_NvItem) -> std::future::Ready<binder::Result<()>>;
              fn r#nvResetConfig(&self, _arg_serial: i32, _arg_resetType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_ResetNvType) -> std::future::Ready<binder::Result<()>>;
              fn r#nvWriteCdmaPrl(&self, _arg_serial: i32, _arg_prl: &[u8]) -> std::future::Ready<binder::Result<()>>;
              fn r#nvWriteItem(&self, _arg_serial: i32, _arg_item: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_NvWriteItem) -> std::future::Ready<binder::Result<()>>;
              fn r#requestShutdown(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#responseAcknowledgement(&self) -> std::future::Ready<binder::Result<()>>;
              fn r#sendDeviceState(&self, _arg_serial: i32, _arg_deviceStateType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_DeviceStateType, _arg_state: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#setRadioCapability(&self, _arg_serial: i32, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> std::future::Ready<binder::Result<()>>;
              fn r#setRadioPower(&self, _arg_serial: i32, _arg_powerOn: bool, _arg_forEmergencyCall: bool, _arg_preferredForEmergencyCall: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#setResponseFunctions(&self, _arg_radioModemResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_19_IRadioModemResponse>, _arg_radioModemIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_21_IRadioModemIndication>) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioModemAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.modem.IRadioModem" }
              async fn r#enableModem(&self, _arg_serial: i32, _arg_on: bool) -> binder::Result<()>;
              async fn r#getBasebandVersion(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getDeviceIdentity(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getHardwareConfig(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getModemActivityInfo(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getModemStackStatus(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getRadioCapability(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#nvReadItem(&self, _arg_serial: i32, _arg_itemId: crate::mangled::_7_android_8_hardware_5_radio_5_modem_6_NvItem) -> binder::Result<()>;
              async fn r#nvResetConfig(&self, _arg_serial: i32, _arg_resetType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_ResetNvType) -> binder::Result<()>;
              async fn r#nvWriteCdmaPrl(&self, _arg_serial: i32, _arg_prl: &[u8]) -> binder::Result<()>;
              async fn r#nvWriteItem(&self, _arg_serial: i32, _arg_item: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_NvWriteItem) -> binder::Result<()>;
              async fn r#requestShutdown(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#responseAcknowledgement(&self) -> binder::Result<()>;
              async fn r#sendDeviceState(&self, _arg_serial: i32, _arg_deviceStateType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_DeviceStateType, _arg_state: bool) -> binder::Result<()>;
              async fn r#setRadioCapability(&self, _arg_serial: i32, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()>;
              async fn r#setRadioPower(&self, _arg_serial: i32, _arg_powerOn: bool, _arg_forEmergencyCall: bool, _arg_preferredForEmergencyCall: bool) -> binder::Result<()>;
              async fn r#setResponseFunctions(&self, _arg_radioModemResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_19_IRadioModemResponse>, _arg_radioModemIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_21_IRadioModemIndication>) -> binder::Result<()>;
            }
            impl BnRadioModem {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioModem>
              where
                T: IRadioModemAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioModem for Wrapper<T, R>
                where
                  T: IRadioModemAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#enableModem(&self, _arg_serial: i32, _arg_on: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#enableModem(_arg_serial, _arg_on))
                  }
                  fn r#getBasebandVersion(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getBasebandVersion(_arg_serial))
                  }
                  fn r#getDeviceIdentity(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getDeviceIdentity(_arg_serial))
                  }
                  fn r#getHardwareConfig(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getHardwareConfig(_arg_serial))
                  }
                  fn r#getModemActivityInfo(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getModemActivityInfo(_arg_serial))
                  }
                  fn r#getModemStackStatus(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getModemStackStatus(_arg_serial))
                  }
                  fn r#getRadioCapability(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getRadioCapability(_arg_serial))
                  }
                  fn r#nvReadItem(&self, _arg_serial: i32, _arg_itemId: crate::mangled::_7_android_8_hardware_5_radio_5_modem_6_NvItem) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#nvReadItem(_arg_serial, _arg_itemId))
                  }
                  fn r#nvResetConfig(&self, _arg_serial: i32, _arg_resetType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_ResetNvType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#nvResetConfig(_arg_serial, _arg_resetType))
                  }
                  fn r#nvWriteCdmaPrl(&self, _arg_serial: i32, _arg_prl: &[u8]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#nvWriteCdmaPrl(_arg_serial, _arg_prl))
                  }
                  fn r#nvWriteItem(&self, _arg_serial: i32, _arg_item: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_NvWriteItem) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#nvWriteItem(_arg_serial, _arg_item))
                  }
                  fn r#requestShutdown(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#requestShutdown(_arg_serial))
                  }
                  fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#responseAcknowledgement())
                  }
                  fn r#sendDeviceState(&self, _arg_serial: i32, _arg_deviceStateType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_DeviceStateType, _arg_state: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendDeviceState(_arg_serial, _arg_deviceStateType, _arg_state))
                  }
                  fn r#setRadioCapability(&self, _arg_serial: i32, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setRadioCapability(_arg_serial, _arg_rc))
                  }
                  fn r#setRadioPower(&self, _arg_serial: i32, _arg_powerOn: bool, _arg_forEmergencyCall: bool, _arg_preferredForEmergencyCall: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setRadioPower(_arg_serial, _arg_powerOn, _arg_forEmergencyCall, _arg_preferredForEmergencyCall))
                  }
                  fn r#setResponseFunctions(&self, _arg_radioModemResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_19_IRadioModemResponse>, _arg_radioModemIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_21_IRadioModemIndication>) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setResponseFunctions(_arg_radioModemResponse, _arg_radioModemIndication))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioModemDefault: Send + Sync {
              fn r#enableModem(&self, _arg_serial: i32, _arg_on: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getBasebandVersion(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDeviceIdentity(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getHardwareConfig(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getModemActivityInfo(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getModemStackStatus(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getRadioCapability(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#nvReadItem(&self, _arg_serial: i32, _arg_itemId: crate::mangled::_7_android_8_hardware_5_radio_5_modem_6_NvItem) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#nvResetConfig(&self, _arg_serial: i32, _arg_resetType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_ResetNvType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#nvWriteCdmaPrl(&self, _arg_serial: i32, _arg_prl: &[u8]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#nvWriteItem(&self, _arg_serial: i32, _arg_item: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_NvWriteItem) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#requestShutdown(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendDeviceState(&self, _arg_serial: i32, _arg_deviceStateType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_DeviceStateType, _arg_state: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setRadioCapability(&self, _arg_serial: i32, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setRadioPower(&self, _arg_serial: i32, _arg_powerOn: bool, _arg_forEmergencyCall: bool, _arg_preferredForEmergencyCall: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setResponseFunctions(&self, _arg_radioModemResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_19_IRadioModemResponse>, _arg_radioModemIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_21_IRadioModemIndication>) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#enableModem: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#getBasebandVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#getDeviceIdentity: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#getHardwareConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#getModemActivityInfo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#getModemStackStatus: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#getRadioCapability: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#nvReadItem: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#nvResetConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#nvWriteCdmaPrl: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#nvWriteItem: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#requestShutdown: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#responseAcknowledgement: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#sendDeviceState: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#setRadioCapability: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#setRadioPower: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#setResponseFunctions: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioModemDefaultRef = Option<std::sync::Arc<dyn IRadioModemDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioModemDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "9dee2319b599d654955c05268c1eed6ca4373b58";
            impl BpRadioModem {
              fn build_parcel_enableModem(&self, _arg_serial: i32, _arg_on: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_on)?;
                Ok(aidl_data)
              }
              fn read_response_enableModem(&self, _arg_serial: i32, _arg_on: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#enableModem(_arg_serial, _arg_on);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getBasebandVersion(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getBasebandVersion(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#getBasebandVersion(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getDeviceIdentity(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getDeviceIdentity(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDeviceIdentity(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getHardwareConfig(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getHardwareConfig(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#getHardwareConfig(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getModemActivityInfo(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getModemActivityInfo(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#getModemActivityInfo(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getModemStackStatus(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getModemStackStatus(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#getModemStackStatus(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getRadioCapability(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getRadioCapability(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#getRadioCapability(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_nvReadItem(&self, _arg_serial: i32, _arg_itemId: crate::mangled::_7_android_8_hardware_5_radio_5_modem_6_NvItem) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_itemId)?;
                Ok(aidl_data)
              }
              fn read_response_nvReadItem(&self, _arg_serial: i32, _arg_itemId: crate::mangled::_7_android_8_hardware_5_radio_5_modem_6_NvItem, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#nvReadItem(_arg_serial, _arg_itemId);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_nvResetConfig(&self, _arg_serial: i32, _arg_resetType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_ResetNvType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_resetType)?;
                Ok(aidl_data)
              }
              fn read_response_nvResetConfig(&self, _arg_serial: i32, _arg_resetType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_ResetNvType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#nvResetConfig(_arg_serial, _arg_resetType);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_nvWriteCdmaPrl(&self, _arg_serial: i32, _arg_prl: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_prl)?;
                Ok(aidl_data)
              }
              fn read_response_nvWriteCdmaPrl(&self, _arg_serial: i32, _arg_prl: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#nvWriteCdmaPrl(_arg_serial, _arg_prl);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_nvWriteItem(&self, _arg_serial: i32, _arg_item: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_NvWriteItem) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_item)?;
                Ok(aidl_data)
              }
              fn read_response_nvWriteItem(&self, _arg_serial: i32, _arg_item: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_NvWriteItem, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#nvWriteItem(_arg_serial, _arg_item);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_requestShutdown(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_requestShutdown(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#requestShutdown(_arg_serial);
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
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#responseAcknowledgement();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendDeviceState(&self, _arg_serial: i32, _arg_deviceStateType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_DeviceStateType, _arg_state: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_deviceStateType)?;
                aidl_data.write(&_arg_state)?;
                Ok(aidl_data)
              }
              fn read_response_sendDeviceState(&self, _arg_serial: i32, _arg_deviceStateType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_DeviceStateType, _arg_state: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendDeviceState(_arg_serial, _arg_deviceStateType, _arg_state);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setRadioCapability(&self, _arg_serial: i32, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_rc)?;
                Ok(aidl_data)
              }
              fn read_response_setRadioCapability(&self, _arg_serial: i32, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#setRadioCapability(_arg_serial, _arg_rc);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setRadioPower(&self, _arg_serial: i32, _arg_powerOn: bool, _arg_forEmergencyCall: bool, _arg_preferredForEmergencyCall: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_powerOn)?;
                aidl_data.write(&_arg_forEmergencyCall)?;
                aidl_data.write(&_arg_preferredForEmergencyCall)?;
                Ok(aidl_data)
              }
              fn read_response_setRadioPower(&self, _arg_serial: i32, _arg_powerOn: bool, _arg_forEmergencyCall: bool, _arg_preferredForEmergencyCall: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#setRadioPower(_arg_serial, _arg_powerOn, _arg_forEmergencyCall, _arg_preferredForEmergencyCall);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setResponseFunctions(&self, _arg_radioModemResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_19_IRadioModemResponse>, _arg_radioModemIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_21_IRadioModemIndication>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_radioModemResponse)?;
                aidl_data.write(_arg_radioModemIndication)?;
                Ok(aidl_data)
              }
              fn read_response_setResponseFunctions(&self, _arg_radioModemResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_19_IRadioModemResponse>, _arg_radioModemIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_21_IRadioModemIndication>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModem>::getDefaultImpl() {
                    return _aidl_default_impl.r#setResponseFunctions(_arg_radioModemResponse, _arg_radioModemIndication);
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
            impl IRadioModem for BpRadioModem {
              fn r#enableModem(&self, _arg_serial: i32, _arg_on: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_enableModem(_arg_serial, _arg_on)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#enableModem, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_enableModem(_arg_serial, _arg_on, _aidl_reply)
              }
              fn r#getBasebandVersion(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getBasebandVersion(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getBasebandVersion, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getBasebandVersion(_arg_serial, _aidl_reply)
              }
              fn r#getDeviceIdentity(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getDeviceIdentity(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDeviceIdentity, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDeviceIdentity(_arg_serial, _aidl_reply)
              }
              fn r#getHardwareConfig(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getHardwareConfig(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getHardwareConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getHardwareConfig(_arg_serial, _aidl_reply)
              }
              fn r#getModemActivityInfo(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getModemActivityInfo(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getModemActivityInfo, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getModemActivityInfo(_arg_serial, _aidl_reply)
              }
              fn r#getModemStackStatus(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getModemStackStatus(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getModemStackStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getModemStackStatus(_arg_serial, _aidl_reply)
              }
              fn r#getRadioCapability(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getRadioCapability(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getRadioCapability, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getRadioCapability(_arg_serial, _aidl_reply)
              }
              fn r#nvReadItem(&self, _arg_serial: i32, _arg_itemId: crate::mangled::_7_android_8_hardware_5_radio_5_modem_6_NvItem) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_nvReadItem(_arg_serial, _arg_itemId)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvReadItem, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_nvReadItem(_arg_serial, _arg_itemId, _aidl_reply)
              }
              fn r#nvResetConfig(&self, _arg_serial: i32, _arg_resetType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_ResetNvType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_nvResetConfig(_arg_serial, _arg_resetType)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvResetConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_nvResetConfig(_arg_serial, _arg_resetType, _aidl_reply)
              }
              fn r#nvWriteCdmaPrl(&self, _arg_serial: i32, _arg_prl: &[u8]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_nvWriteCdmaPrl(_arg_serial, _arg_prl)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvWriteCdmaPrl, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_nvWriteCdmaPrl(_arg_serial, _arg_prl, _aidl_reply)
              }
              fn r#nvWriteItem(&self, _arg_serial: i32, _arg_item: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_NvWriteItem) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_nvWriteItem(_arg_serial, _arg_item)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvWriteItem, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_nvWriteItem(_arg_serial, _arg_item, _aidl_reply)
              }
              fn r#requestShutdown(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_requestShutdown(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#requestShutdown, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_requestShutdown(_arg_serial, _aidl_reply)
              }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_responseAcknowledgement()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#responseAcknowledgement, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_responseAcknowledgement(_aidl_reply)
              }
              fn r#sendDeviceState(&self, _arg_serial: i32, _arg_deviceStateType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_DeviceStateType, _arg_state: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendDeviceState(_arg_serial, _arg_deviceStateType, _arg_state)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendDeviceState, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendDeviceState(_arg_serial, _arg_deviceStateType, _arg_state, _aidl_reply)
              }
              fn r#setRadioCapability(&self, _arg_serial: i32, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setRadioCapability(_arg_serial, _arg_rc)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setRadioCapability, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setRadioCapability(_arg_serial, _arg_rc, _aidl_reply)
              }
              fn r#setRadioPower(&self, _arg_serial: i32, _arg_powerOn: bool, _arg_forEmergencyCall: bool, _arg_preferredForEmergencyCall: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setRadioPower(_arg_serial, _arg_powerOn, _arg_forEmergencyCall, _arg_preferredForEmergencyCall)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setRadioPower, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setRadioPower(_arg_serial, _arg_powerOn, _arg_forEmergencyCall, _arg_preferredForEmergencyCall, _aidl_reply)
              }
              fn r#setResponseFunctions(&self, _arg_radioModemResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_19_IRadioModemResponse>, _arg_radioModemIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_21_IRadioModemIndication>) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setResponseFunctions(_arg_radioModemResponse, _arg_radioModemIndication)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setResponseFunctions(_arg_radioModemResponse, _arg_radioModemIndication, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioModemAsync<P> for BpRadioModem {
              fn r#enableModem(&self, _arg_serial: i32, _arg_on: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_enableModem(_arg_serial, _arg_on) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#enableModem, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_enableModem(_arg_serial, _arg_on, _aidl_reply))
              }
              fn r#getBasebandVersion(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getBasebandVersion(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getBasebandVersion, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getBasebandVersion(_arg_serial, _aidl_reply))
              }
              fn r#getDeviceIdentity(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getDeviceIdentity(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDeviceIdentity, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getDeviceIdentity(_arg_serial, _aidl_reply))
              }
              fn r#getHardwareConfig(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getHardwareConfig(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getHardwareConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getHardwareConfig(_arg_serial, _aidl_reply))
              }
              fn r#getModemActivityInfo(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getModemActivityInfo(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getModemActivityInfo, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getModemActivityInfo(_arg_serial, _aidl_reply))
              }
              fn r#getModemStackStatus(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getModemStackStatus(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getModemStackStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getModemStackStatus(_arg_serial, _aidl_reply))
              }
              fn r#getRadioCapability(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getRadioCapability(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getRadioCapability, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getRadioCapability(_arg_serial, _aidl_reply))
              }
              fn r#nvReadItem(&self, _arg_serial: i32, _arg_itemId: crate::mangled::_7_android_8_hardware_5_radio_5_modem_6_NvItem) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_nvReadItem(_arg_serial, _arg_itemId) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvReadItem, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_nvReadItem(_arg_serial, _arg_itemId, _aidl_reply))
              }
              fn r#nvResetConfig(&self, _arg_serial: i32, _arg_resetType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_ResetNvType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_nvResetConfig(_arg_serial, _arg_resetType) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvResetConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_nvResetConfig(_arg_serial, _arg_resetType, _aidl_reply))
              }
              fn r#nvWriteCdmaPrl(&self, _arg_serial: i32, _arg_prl: &[u8]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_nvWriteCdmaPrl(_arg_serial, _arg_prl) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvWriteCdmaPrl, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_nvWriteCdmaPrl(_arg_serial, _arg_prl, _aidl_reply))
              }
              fn r#nvWriteItem(&self, _arg_serial: i32, _arg_item: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_NvWriteItem) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_nvWriteItem(_arg_serial, _arg_item) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvWriteItem, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_nvWriteItem(_arg_serial, _arg_item, _aidl_reply))
              }
              fn r#requestShutdown(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_requestShutdown(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#requestShutdown, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_requestShutdown(_arg_serial, _aidl_reply))
              }
              fn r#responseAcknowledgement(&self) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_responseAcknowledgement() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#responseAcknowledgement, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_responseAcknowledgement(_aidl_reply))
              }
              fn r#sendDeviceState(&self, _arg_serial: i32, _arg_deviceStateType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_DeviceStateType, _arg_state: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendDeviceState(_arg_serial, _arg_deviceStateType, _arg_state) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendDeviceState, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendDeviceState(_arg_serial, _arg_deviceStateType, _arg_state, _aidl_reply))
              }
              fn r#setRadioCapability(&self, _arg_serial: i32, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setRadioCapability(_arg_serial, _arg_rc) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setRadioCapability, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setRadioCapability(_arg_serial, _arg_rc, _aidl_reply))
              }
              fn r#setRadioPower(&self, _arg_serial: i32, _arg_powerOn: bool, _arg_forEmergencyCall: bool, _arg_preferredForEmergencyCall: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setRadioPower(_arg_serial, _arg_powerOn, _arg_forEmergencyCall, _arg_preferredForEmergencyCall) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setRadioPower, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setRadioPower(_arg_serial, _arg_powerOn, _arg_forEmergencyCall, _arg_preferredForEmergencyCall, _aidl_reply))
              }
              fn r#setResponseFunctions(&self, _arg_radioModemResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_19_IRadioModemResponse>, _arg_radioModemIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_21_IRadioModemIndication>) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setResponseFunctions(_arg_radioModemResponse, _arg_radioModemIndication) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setResponseFunctions(_arg_radioModemResponse, _arg_radioModemIndication, _aidl_reply))
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
            impl IRadioModem for binder::binder_impl::Binder<BnRadioModem> {
              fn r#enableModem(&self, _arg_serial: i32, _arg_on: bool) -> binder::Result<()> { self.0.r#enableModem(_arg_serial, _arg_on) }
              fn r#getBasebandVersion(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getBasebandVersion(_arg_serial) }
              fn r#getDeviceIdentity(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getDeviceIdentity(_arg_serial) }
              fn r#getHardwareConfig(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getHardwareConfig(_arg_serial) }
              fn r#getModemActivityInfo(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getModemActivityInfo(_arg_serial) }
              fn r#getModemStackStatus(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getModemStackStatus(_arg_serial) }
              fn r#getRadioCapability(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getRadioCapability(_arg_serial) }
              fn r#nvReadItem(&self, _arg_serial: i32, _arg_itemId: crate::mangled::_7_android_8_hardware_5_radio_5_modem_6_NvItem) -> binder::Result<()> { self.0.r#nvReadItem(_arg_serial, _arg_itemId) }
              fn r#nvResetConfig(&self, _arg_serial: i32, _arg_resetType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_ResetNvType) -> binder::Result<()> { self.0.r#nvResetConfig(_arg_serial, _arg_resetType) }
              fn r#nvWriteCdmaPrl(&self, _arg_serial: i32, _arg_prl: &[u8]) -> binder::Result<()> { self.0.r#nvWriteCdmaPrl(_arg_serial, _arg_prl) }
              fn r#nvWriteItem(&self, _arg_serial: i32, _arg_item: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_NvWriteItem) -> binder::Result<()> { self.0.r#nvWriteItem(_arg_serial, _arg_item) }
              fn r#requestShutdown(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#requestShutdown(_arg_serial) }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> { self.0.r#responseAcknowledgement() }
              fn r#sendDeviceState(&self, _arg_serial: i32, _arg_deviceStateType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_DeviceStateType, _arg_state: bool) -> binder::Result<()> { self.0.r#sendDeviceState(_arg_serial, _arg_deviceStateType, _arg_state) }
              fn r#setRadioCapability(&self, _arg_serial: i32, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> { self.0.r#setRadioCapability(_arg_serial, _arg_rc) }
              fn r#setRadioPower(&self, _arg_serial: i32, _arg_powerOn: bool, _arg_forEmergencyCall: bool, _arg_preferredForEmergencyCall: bool) -> binder::Result<()> { self.0.r#setRadioPower(_arg_serial, _arg_powerOn, _arg_forEmergencyCall, _arg_preferredForEmergencyCall) }
              fn r#setResponseFunctions(&self, _arg_radioModemResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_19_IRadioModemResponse>, _arg_radioModemIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_21_IRadioModemIndication>) -> binder::Result<()> { self.0.r#setResponseFunctions(_arg_radioModemResponse, _arg_radioModemIndication) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioModem, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#enableModem => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_on: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#enableModem(_arg_serial, _arg_on);
                  Ok(())
                }
                transactions::r#getBasebandVersion => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getBasebandVersion(_arg_serial);
                  Ok(())
                }
                transactions::r#getDeviceIdentity => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDeviceIdentity(_arg_serial);
                  Ok(())
                }
                transactions::r#getHardwareConfig => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getHardwareConfig(_arg_serial);
                  Ok(())
                }
                transactions::r#getModemActivityInfo => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getModemActivityInfo(_arg_serial);
                  Ok(())
                }
                transactions::r#getModemStackStatus => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getModemStackStatus(_arg_serial);
                  Ok(())
                }
                transactions::r#getRadioCapability => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getRadioCapability(_arg_serial);
                  Ok(())
                }
                transactions::r#nvReadItem => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_itemId: crate::mangled::_7_android_8_hardware_5_radio_5_modem_6_NvItem = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#nvReadItem(_arg_serial, _arg_itemId);
                  Ok(())
                }
                transactions::r#nvResetConfig => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_resetType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_ResetNvType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#nvResetConfig(_arg_serial, _arg_resetType);
                  Ok(())
                }
                transactions::r#nvWriteCdmaPrl => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_prl: Vec<u8> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#nvWriteCdmaPrl(_arg_serial, &_arg_prl);
                  Ok(())
                }
                transactions::r#nvWriteItem => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_item: crate::mangled::_7_android_8_hardware_5_radio_5_modem_11_NvWriteItem = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#nvWriteItem(_arg_serial, &_arg_item);
                  Ok(())
                }
                transactions::r#requestShutdown => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#requestShutdown(_arg_serial);
                  Ok(())
                }
                transactions::r#responseAcknowledgement => {
                  let _aidl_return = _aidl_service.r#responseAcknowledgement();
                  Ok(())
                }
                transactions::r#sendDeviceState => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_deviceStateType: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_DeviceStateType = _aidl_data.read()?;
                  let _arg_state: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendDeviceState(_arg_serial, _arg_deviceStateType, _arg_state);
                  Ok(())
                }
                transactions::r#setRadioCapability => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_rc: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setRadioCapability(_arg_serial, &_arg_rc);
                  Ok(())
                }
                transactions::r#setRadioPower => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_powerOn: bool = _aidl_data.read()?;
                  let _arg_forEmergencyCall: bool = _aidl_data.read()?;
                  let _arg_preferredForEmergencyCall: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setRadioPower(_arg_serial, _arg_powerOn, _arg_forEmergencyCall, _arg_preferredForEmergencyCall);
                  Ok(())
                }
                transactions::r#setResponseFunctions => {
                  let _arg_radioModemResponse: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_19_IRadioModemResponse> = _aidl_data.read()?;
                  let _arg_radioModemIndication: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_modem_21_IRadioModemIndication> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setResponseFunctions(&_arg_radioModemResponse, &_arg_radioModemIndication);
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
             pub use super::r#IRadioModem as _7_android_8_hardware_5_radio_5_modem_11_IRadioModem;
            }
          }
          pub mod IRadioModemIndication {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioModemIndication["android.hardware.radio.modem.IRadioModemIndication"] {
                native: BnRadioModemIndication(on_transact),
                proxy: BpRadioModemIndication {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioModemIndicationAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioModemIndication: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.modem.IRadioModemIndication" }
              fn r#hardwareConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<()>;
              fn r#modemReset(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_reason: &str) -> binder::Result<()>;
              fn r#radioCapabilityIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()>;
              fn r#radioStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_radioState: crate::mangled::_7_android_8_hardware_5_radio_5_modem_10_RadioState) -> binder::Result<()>;
              fn r#rilConnected(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioModemIndicationDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioModemIndicationDefaultRef) -> IRadioModemIndicationDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioModemIndicationAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.modem.IRadioModemIndication" }
              fn r#hardwareConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> std::future::Ready<binder::Result<()>>;
              fn r#modemReset(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_reason: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#radioCapabilityIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> std::future::Ready<binder::Result<()>>;
              fn r#radioStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_radioState: crate::mangled::_7_android_8_hardware_5_radio_5_modem_10_RadioState) -> std::future::Ready<binder::Result<()>>;
              fn r#rilConnected(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioModemIndicationAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.modem.IRadioModemIndication" }
              async fn r#hardwareConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<()>;
              async fn r#modemReset(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_reason: &str) -> binder::Result<()>;
              async fn r#radioCapabilityIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()>;
              async fn r#radioStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_radioState: crate::mangled::_7_android_8_hardware_5_radio_5_modem_10_RadioState) -> binder::Result<()>;
              async fn r#rilConnected(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
            }
            impl BnRadioModemIndication {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioModemIndication>
              where
                T: IRadioModemIndicationAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioModemIndication for Wrapper<T, R>
                where
                  T: IRadioModemIndicationAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#hardwareConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#hardwareConfigChanged(_arg_type, _arg_configs))
                  }
                  fn r#modemReset(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_reason: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#modemReset(_arg_type, _arg_reason))
                  }
                  fn r#radioCapabilityIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#radioCapabilityIndication(_arg_type, _arg_rc))
                  }
                  fn r#radioStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_radioState: crate::mangled::_7_android_8_hardware_5_radio_5_modem_10_RadioState) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#radioStateChanged(_arg_type, _arg_radioState))
                  }
                  fn r#rilConnected(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#rilConnected(_arg_type))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioModemIndicationDefault: Send + Sync {
              fn r#hardwareConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#modemReset(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_reason: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#radioCapabilityIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#radioStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_radioState: crate::mangled::_7_android_8_hardware_5_radio_5_modem_10_RadioState) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#rilConnected(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#hardwareConfigChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#modemReset: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#radioCapabilityIndication: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#radioStateChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#rilConnected: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioModemIndicationDefaultRef = Option<std::sync::Arc<dyn IRadioModemIndicationDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioModemIndicationDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "9dee2319b599d654955c05268c1eed6ca4373b58";
            impl BpRadioModemIndication {
              fn build_parcel_hardwareConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_configs)?;
                Ok(aidl_data)
              }
              fn read_response_hardwareConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#hardwareConfigChanged(_arg_type, _arg_configs);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_modemReset(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_reason: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_reason)?;
                Ok(aidl_data)
              }
              fn read_response_modemReset(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_reason: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#modemReset(_arg_type, _arg_reason);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_radioCapabilityIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_rc)?;
                Ok(aidl_data)
              }
              fn read_response_radioCapabilityIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#radioCapabilityIndication(_arg_type, _arg_rc);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_radioStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_radioState: crate::mangled::_7_android_8_hardware_5_radio_5_modem_10_RadioState) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_radioState)?;
                Ok(aidl_data)
              }
              fn read_response_radioStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_radioState: crate::mangled::_7_android_8_hardware_5_radio_5_modem_10_RadioState, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#radioStateChanged(_arg_type, _arg_radioState);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_rilConnected(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_rilConnected(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#rilConnected(_arg_type);
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
            impl IRadioModemIndication for BpRadioModemIndication {
              fn r#hardwareConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_hardwareConfigChanged(_arg_type, _arg_configs)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#hardwareConfigChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_hardwareConfigChanged(_arg_type, _arg_configs, _aidl_reply)
              }
              fn r#modemReset(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_reason: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_modemReset(_arg_type, _arg_reason)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#modemReset, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_modemReset(_arg_type, _arg_reason, _aidl_reply)
              }
              fn r#radioCapabilityIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_radioCapabilityIndication(_arg_type, _arg_rc)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#radioCapabilityIndication, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_radioCapabilityIndication(_arg_type, _arg_rc, _aidl_reply)
              }
              fn r#radioStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_radioState: crate::mangled::_7_android_8_hardware_5_radio_5_modem_10_RadioState) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_radioStateChanged(_arg_type, _arg_radioState)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#radioStateChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_radioStateChanged(_arg_type, _arg_radioState, _aidl_reply)
              }
              fn r#rilConnected(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_rilConnected(_arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#rilConnected, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_rilConnected(_arg_type, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioModemIndicationAsync<P> for BpRadioModemIndication {
              fn r#hardwareConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_hardwareConfigChanged(_arg_type, _arg_configs) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#hardwareConfigChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_hardwareConfigChanged(_arg_type, _arg_configs, _aidl_reply))
              }
              fn r#modemReset(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_reason: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_modemReset(_arg_type, _arg_reason) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#modemReset, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_modemReset(_arg_type, _arg_reason, _aidl_reply))
              }
              fn r#radioCapabilityIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_radioCapabilityIndication(_arg_type, _arg_rc) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#radioCapabilityIndication, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_radioCapabilityIndication(_arg_type, _arg_rc, _aidl_reply))
              }
              fn r#radioStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_radioState: crate::mangled::_7_android_8_hardware_5_radio_5_modem_10_RadioState) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_radioStateChanged(_arg_type, _arg_radioState) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#radioStateChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_radioStateChanged(_arg_type, _arg_radioState, _aidl_reply))
              }
              fn r#rilConnected(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_rilConnected(_arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#rilConnected, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_rilConnected(_arg_type, _aidl_reply))
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
            impl IRadioModemIndication for binder::binder_impl::Binder<BnRadioModemIndication> {
              fn r#hardwareConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<()> { self.0.r#hardwareConfigChanged(_arg_type, _arg_configs) }
              fn r#modemReset(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_reason: &str) -> binder::Result<()> { self.0.r#modemReset(_arg_type, _arg_reason) }
              fn r#radioCapabilityIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> { self.0.r#radioCapabilityIndication(_arg_type, _arg_rc) }
              fn r#radioStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_radioState: crate::mangled::_7_android_8_hardware_5_radio_5_modem_10_RadioState) -> binder::Result<()> { self.0.r#radioStateChanged(_arg_type, _arg_radioState) }
              fn r#rilConnected(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#rilConnected(_arg_type) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioModemIndication, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#hardwareConfigChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_configs: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#hardwareConfigChanged(_arg_type, &_arg_configs);
                  Ok(())
                }
                transactions::r#modemReset => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_reason: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#modemReset(_arg_type, &_arg_reason);
                  Ok(())
                }
                transactions::r#radioCapabilityIndication => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_rc: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#radioCapabilityIndication(_arg_type, &_arg_rc);
                  Ok(())
                }
                transactions::r#radioStateChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_radioState: crate::mangled::_7_android_8_hardware_5_radio_5_modem_10_RadioState = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#radioStateChanged(_arg_type, _arg_radioState);
                  Ok(())
                }
                transactions::r#rilConnected => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#rilConnected(_arg_type);
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
             pub use super::r#IRadioModemIndication as _7_android_8_hardware_5_radio_5_modem_21_IRadioModemIndication;
            }
          }
          pub mod IRadioModemResponse {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioModemResponse["android.hardware.radio.modem.IRadioModemResponse"] {
                native: BnRadioModemResponse(on_transact),
                proxy: BpRadioModemResponse {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioModemResponseAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioModemResponse: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.modem.IRadioModemResponse" }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#enableModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#getBasebandVersionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_version: &str) -> binder::Result<()>;
              fn r#getDeviceIdentityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imei: &str, _arg_imeisv: &str, _arg_esn: &str, _arg_meid: &str) -> binder::Result<()>;
              fn r#getHardwareConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_config: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<()>;
              fn r#getModemActivityInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_activityInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_17_ActivityStatsInfo) -> binder::Result<()>;
              fn r#getModemStackStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<()>;
              fn r#getRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()>;
              fn r#nvReadItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &str) -> binder::Result<()>;
              fn r#nvResetConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#nvWriteCdmaPrlResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#nvWriteItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#requestShutdownResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#sendDeviceStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()>;
              fn r#setRadioPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioModemResponseDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioModemResponseDefaultRef) -> IRadioModemResponseDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioModemResponseAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.modem.IRadioModemResponse" }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#enableModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getBasebandVersionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_version: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#getDeviceIdentityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imei: &str, _arg_imeisv: &str, _arg_esn: &str, _arg_meid: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#getHardwareConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_config: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> std::future::Ready<binder::Result<()>>;
              fn r#getModemActivityInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_activityInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_17_ActivityStatsInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getModemStackStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#getRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> std::future::Ready<binder::Result<()>>;
              fn r#nvReadItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#nvResetConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#nvWriteCdmaPrlResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#nvWriteItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#requestShutdownResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#sendDeviceStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> std::future::Ready<binder::Result<()>>;
              fn r#setRadioPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioModemResponseAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.modem.IRadioModemResponse" }
              async fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#enableModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#getBasebandVersionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_version: &str) -> binder::Result<()>;
              async fn r#getDeviceIdentityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imei: &str, _arg_imeisv: &str, _arg_esn: &str, _arg_meid: &str) -> binder::Result<()>;
              async fn r#getHardwareConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_config: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<()>;
              async fn r#getModemActivityInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_activityInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_17_ActivityStatsInfo) -> binder::Result<()>;
              async fn r#getModemStackStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<()>;
              async fn r#getRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()>;
              async fn r#nvReadItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &str) -> binder::Result<()>;
              async fn r#nvResetConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#nvWriteCdmaPrlResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#nvWriteItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#requestShutdownResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#sendDeviceStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()>;
              async fn r#setRadioPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
            }
            impl BnRadioModemResponse {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioModemResponse>
              where
                T: IRadioModemResponseAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioModemResponse for Wrapper<T, R>
                where
                  T: IRadioModemResponseAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acknowledgeRequest(_arg_serial))
                  }
                  fn r#enableModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#enableModemResponse(_arg_info))
                  }
                  fn r#getBasebandVersionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_version: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getBasebandVersionResponse(_arg_info, _arg_version))
                  }
                  fn r#getDeviceIdentityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imei: &str, _arg_imeisv: &str, _arg_esn: &str, _arg_meid: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getDeviceIdentityResponse(_arg_info, _arg_imei, _arg_imeisv, _arg_esn, _arg_meid))
                  }
                  fn r#getHardwareConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_config: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getHardwareConfigResponse(_arg_info, _arg_config))
                  }
                  fn r#getModemActivityInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_activityInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_17_ActivityStatsInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getModemActivityInfoResponse(_arg_info, _arg_activityInfo))
                  }
                  fn r#getModemStackStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getModemStackStatusResponse(_arg_info, _arg_isEnabled))
                  }
                  fn r#getRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getRadioCapabilityResponse(_arg_info, _arg_rc))
                  }
                  fn r#nvReadItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#nvReadItemResponse(_arg_info, _arg_result))
                  }
                  fn r#nvResetConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#nvResetConfigResponse(_arg_info))
                  }
                  fn r#nvWriteCdmaPrlResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#nvWriteCdmaPrlResponse(_arg_info))
                  }
                  fn r#nvWriteItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#nvWriteItemResponse(_arg_info))
                  }
                  fn r#requestShutdownResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#requestShutdownResponse(_arg_info))
                  }
                  fn r#sendDeviceStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendDeviceStateResponse(_arg_info))
                  }
                  fn r#setRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setRadioCapabilityResponse(_arg_info, _arg_rc))
                  }
                  fn r#setRadioPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setRadioPowerResponse(_arg_info))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioModemResponseDefault: Send + Sync {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#enableModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getBasebandVersionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_version: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDeviceIdentityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imei: &str, _arg_imeisv: &str, _arg_esn: &str, _arg_meid: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getHardwareConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_config: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getModemActivityInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_activityInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_17_ActivityStatsInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getModemStackStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#nvReadItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#nvResetConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#nvWriteCdmaPrlResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#nvWriteItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#requestShutdownResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendDeviceStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setRadioPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#acknowledgeRequest: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#enableModemResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#getBasebandVersionResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#getDeviceIdentityResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#getHardwareConfigResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#getModemActivityInfoResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#getModemStackStatusResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#getRadioCapabilityResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#nvReadItemResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#nvResetConfigResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#nvWriteCdmaPrlResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#nvWriteItemResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#requestShutdownResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#sendDeviceStateResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#setRadioCapabilityResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#setRadioPowerResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioModemResponseDefaultRef = Option<std::sync::Arc<dyn IRadioModemResponseDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioModemResponseDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "9dee2319b599d654955c05268c1eed6ca4373b58";
            impl BpRadioModemResponse {
              fn build_parcel_acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_acknowledgeRequest(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#acknowledgeRequest(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_enableModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_enableModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#enableModemResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getBasebandVersionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_version: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_version)?;
                Ok(aidl_data)
              }
              fn read_response_getBasebandVersionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_version: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getBasebandVersionResponse(_arg_info, _arg_version);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getDeviceIdentityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imei: &str, _arg_imeisv: &str, _arg_esn: &str, _arg_meid: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_imei)?;
                aidl_data.write(_arg_imeisv)?;
                aidl_data.write(_arg_esn)?;
                aidl_data.write(_arg_meid)?;
                Ok(aidl_data)
              }
              fn read_response_getDeviceIdentityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imei: &str, _arg_imeisv: &str, _arg_esn: &str, _arg_meid: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDeviceIdentityResponse(_arg_info, _arg_imei, _arg_imeisv, _arg_esn, _arg_meid);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getHardwareConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_config: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_config)?;
                Ok(aidl_data)
              }
              fn read_response_getHardwareConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_config: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getHardwareConfigResponse(_arg_info, _arg_config);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getModemActivityInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_activityInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_17_ActivityStatsInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_activityInfo)?;
                Ok(aidl_data)
              }
              fn read_response_getModemActivityInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_activityInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_17_ActivityStatsInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getModemActivityInfoResponse(_arg_info, _arg_activityInfo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getModemStackStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_isEnabled)?;
                Ok(aidl_data)
              }
              fn read_response_getModemStackStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getModemStackStatusResponse(_arg_info, _arg_isEnabled);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_rc)?;
                Ok(aidl_data)
              }
              fn read_response_getRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getRadioCapabilityResponse(_arg_info, _arg_rc);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_nvReadItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_result)?;
                Ok(aidl_data)
              }
              fn read_response_nvReadItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#nvReadItemResponse(_arg_info, _arg_result);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_nvResetConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_nvResetConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#nvResetConfigResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_nvWriteCdmaPrlResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_nvWriteCdmaPrlResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#nvWriteCdmaPrlResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_nvWriteItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_nvWriteItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#nvWriteItemResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_requestShutdownResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_requestShutdownResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#requestShutdownResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendDeviceStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_sendDeviceStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendDeviceStateResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_rc)?;
                Ok(aidl_data)
              }
              fn read_response_setRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setRadioCapabilityResponse(_arg_info, _arg_rc);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setRadioPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setRadioPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioModemResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setRadioPowerResponse(_arg_info);
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
            impl IRadioModemResponse for BpRadioModemResponse {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acknowledgeRequest(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeRequest, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acknowledgeRequest(_arg_serial, _aidl_reply)
              }
              fn r#enableModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_enableModemResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#enableModemResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_enableModemResponse(_arg_info, _aidl_reply)
              }
              fn r#getBasebandVersionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_version: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getBasebandVersionResponse(_arg_info, _arg_version)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getBasebandVersionResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getBasebandVersionResponse(_arg_info, _arg_version, _aidl_reply)
              }
              fn r#getDeviceIdentityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imei: &str, _arg_imeisv: &str, _arg_esn: &str, _arg_meid: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getDeviceIdentityResponse(_arg_info, _arg_imei, _arg_imeisv, _arg_esn, _arg_meid)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDeviceIdentityResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDeviceIdentityResponse(_arg_info, _arg_imei, _arg_imeisv, _arg_esn, _arg_meid, _aidl_reply)
              }
              fn r#getHardwareConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_config: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getHardwareConfigResponse(_arg_info, _arg_config)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getHardwareConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getHardwareConfigResponse(_arg_info, _arg_config, _aidl_reply)
              }
              fn r#getModemActivityInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_activityInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_17_ActivityStatsInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getModemActivityInfoResponse(_arg_info, _arg_activityInfo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getModemActivityInfoResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getModemActivityInfoResponse(_arg_info, _arg_activityInfo, _aidl_reply)
              }
              fn r#getModemStackStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getModemStackStatusResponse(_arg_info, _arg_isEnabled)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getModemStackStatusResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getModemStackStatusResponse(_arg_info, _arg_isEnabled, _aidl_reply)
              }
              fn r#getRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getRadioCapabilityResponse(_arg_info, _arg_rc)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getRadioCapabilityResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getRadioCapabilityResponse(_arg_info, _arg_rc, _aidl_reply)
              }
              fn r#nvReadItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_nvReadItemResponse(_arg_info, _arg_result)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvReadItemResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_nvReadItemResponse(_arg_info, _arg_result, _aidl_reply)
              }
              fn r#nvResetConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_nvResetConfigResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvResetConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_nvResetConfigResponse(_arg_info, _aidl_reply)
              }
              fn r#nvWriteCdmaPrlResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_nvWriteCdmaPrlResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvWriteCdmaPrlResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_nvWriteCdmaPrlResponse(_arg_info, _aidl_reply)
              }
              fn r#nvWriteItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_nvWriteItemResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvWriteItemResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_nvWriteItemResponse(_arg_info, _aidl_reply)
              }
              fn r#requestShutdownResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_requestShutdownResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#requestShutdownResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_requestShutdownResponse(_arg_info, _aidl_reply)
              }
              fn r#sendDeviceStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendDeviceStateResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendDeviceStateResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendDeviceStateResponse(_arg_info, _aidl_reply)
              }
              fn r#setRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setRadioCapabilityResponse(_arg_info, _arg_rc)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setRadioCapabilityResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setRadioCapabilityResponse(_arg_info, _arg_rc, _aidl_reply)
              }
              fn r#setRadioPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setRadioPowerResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setRadioPowerResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setRadioPowerResponse(_arg_info, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioModemResponseAsync<P> for BpRadioModemResponse {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acknowledgeRequest(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeRequest, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acknowledgeRequest(_arg_serial, _aidl_reply))
              }
              fn r#enableModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_enableModemResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#enableModemResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_enableModemResponse(_arg_info, _aidl_reply))
              }
              fn r#getBasebandVersionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_version: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getBasebandVersionResponse(_arg_info, _arg_version) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getBasebandVersionResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getBasebandVersionResponse(_arg_info, _arg_version, _aidl_reply))
              }
              fn r#getDeviceIdentityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imei: &str, _arg_imeisv: &str, _arg_esn: &str, _arg_meid: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getDeviceIdentityResponse(_arg_info, _arg_imei, _arg_imeisv, _arg_esn, _arg_meid) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDeviceIdentityResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getDeviceIdentityResponse(_arg_info, _arg_imei, _arg_imeisv, _arg_esn, _arg_meid, _aidl_reply))
              }
              fn r#getHardwareConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_config: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getHardwareConfigResponse(_arg_info, _arg_config) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getHardwareConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getHardwareConfigResponse(_arg_info, _arg_config, _aidl_reply))
              }
              fn r#getModemActivityInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_activityInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_17_ActivityStatsInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getModemActivityInfoResponse(_arg_info, _arg_activityInfo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getModemActivityInfoResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getModemActivityInfoResponse(_arg_info, _arg_activityInfo, _aidl_reply))
              }
              fn r#getModemStackStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getModemStackStatusResponse(_arg_info, _arg_isEnabled) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getModemStackStatusResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getModemStackStatusResponse(_arg_info, _arg_isEnabled, _aidl_reply))
              }
              fn r#getRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getRadioCapabilityResponse(_arg_info, _arg_rc) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getRadioCapabilityResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getRadioCapabilityResponse(_arg_info, _arg_rc, _aidl_reply))
              }
              fn r#nvReadItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_nvReadItemResponse(_arg_info, _arg_result) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvReadItemResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_nvReadItemResponse(_arg_info, _arg_result, _aidl_reply))
              }
              fn r#nvResetConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_nvResetConfigResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvResetConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_nvResetConfigResponse(_arg_info, _aidl_reply))
              }
              fn r#nvWriteCdmaPrlResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_nvWriteCdmaPrlResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvWriteCdmaPrlResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_nvWriteCdmaPrlResponse(_arg_info, _aidl_reply))
              }
              fn r#nvWriteItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_nvWriteItemResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#nvWriteItemResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_nvWriteItemResponse(_arg_info, _aidl_reply))
              }
              fn r#requestShutdownResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_requestShutdownResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#requestShutdownResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_requestShutdownResponse(_arg_info, _aidl_reply))
              }
              fn r#sendDeviceStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendDeviceStateResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendDeviceStateResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendDeviceStateResponse(_arg_info, _aidl_reply))
              }
              fn r#setRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setRadioCapabilityResponse(_arg_info, _arg_rc) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setRadioCapabilityResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setRadioCapabilityResponse(_arg_info, _arg_rc, _aidl_reply))
              }
              fn r#setRadioPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setRadioPowerResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setRadioPowerResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setRadioPowerResponse(_arg_info, _aidl_reply))
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
            impl IRadioModemResponse for binder::binder_impl::Binder<BnRadioModemResponse> {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#acknowledgeRequest(_arg_serial) }
              fn r#enableModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#enableModemResponse(_arg_info) }
              fn r#getBasebandVersionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_version: &str) -> binder::Result<()> { self.0.r#getBasebandVersionResponse(_arg_info, _arg_version) }
              fn r#getDeviceIdentityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_imei: &str, _arg_imeisv: &str, _arg_esn: &str, _arg_meid: &str) -> binder::Result<()> { self.0.r#getDeviceIdentityResponse(_arg_info, _arg_imei, _arg_imeisv, _arg_esn, _arg_meid) }
              fn r#getHardwareConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_config: &[crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig]) -> binder::Result<()> { self.0.r#getHardwareConfigResponse(_arg_info, _arg_config) }
              fn r#getModemActivityInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_activityInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_17_ActivityStatsInfo) -> binder::Result<()> { self.0.r#getModemActivityInfoResponse(_arg_info, _arg_activityInfo) }
              fn r#getModemStackStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<()> { self.0.r#getModemStackStatusResponse(_arg_info, _arg_isEnabled) }
              fn r#getRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> { self.0.r#getRadioCapabilityResponse(_arg_info, _arg_rc) }
              fn r#nvReadItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_result: &str) -> binder::Result<()> { self.0.r#nvReadItemResponse(_arg_info, _arg_result) }
              fn r#nvResetConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#nvResetConfigResponse(_arg_info) }
              fn r#nvWriteCdmaPrlResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#nvWriteCdmaPrlResponse(_arg_info) }
              fn r#nvWriteItemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#nvWriteItemResponse(_arg_info) }
              fn r#requestShutdownResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#requestShutdownResponse(_arg_info) }
              fn r#sendDeviceStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#sendDeviceStateResponse(_arg_info) }
              fn r#setRadioCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rc: &crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability) -> binder::Result<()> { self.0.r#setRadioCapabilityResponse(_arg_info, _arg_rc) }
              fn r#setRadioPowerResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setRadioPowerResponse(_arg_info) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioModemResponse, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#acknowledgeRequest => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acknowledgeRequest(_arg_serial);
                  Ok(())
                }
                transactions::r#enableModemResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#enableModemResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#getBasebandVersionResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_version: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getBasebandVersionResponse(&_arg_info, &_arg_version);
                  Ok(())
                }
                transactions::r#getDeviceIdentityResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_imei: String = _aidl_data.read()?;
                  let _arg_imeisv: String = _aidl_data.read()?;
                  let _arg_esn: String = _aidl_data.read()?;
                  let _arg_meid: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDeviceIdentityResponse(&_arg_info, &_arg_imei, &_arg_imeisv, &_arg_esn, &_arg_meid);
                  Ok(())
                }
                transactions::r#getHardwareConfigResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_config: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_modem_14_HardwareConfig> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getHardwareConfigResponse(&_arg_info, &_arg_config);
                  Ok(())
                }
                transactions::r#getModemActivityInfoResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_activityInfo: crate::mangled::_7_android_8_hardware_5_radio_5_modem_17_ActivityStatsInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getModemActivityInfoResponse(&_arg_info, &_arg_activityInfo);
                  Ok(())
                }
                transactions::r#getModemStackStatusResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_isEnabled: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getModemStackStatusResponse(&_arg_info, _arg_isEnabled);
                  Ok(())
                }
                transactions::r#getRadioCapabilityResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_rc: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getRadioCapabilityResponse(&_arg_info, &_arg_rc);
                  Ok(())
                }
                transactions::r#nvReadItemResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_result: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#nvReadItemResponse(&_arg_info, &_arg_result);
                  Ok(())
                }
                transactions::r#nvResetConfigResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#nvResetConfigResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#nvWriteCdmaPrlResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#nvWriteCdmaPrlResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#nvWriteItemResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#nvWriteItemResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#requestShutdownResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#requestShutdownResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#sendDeviceStateResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendDeviceStateResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setRadioCapabilityResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_rc: crate::mangled::_7_android_8_hardware_5_radio_5_modem_15_RadioCapability = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setRadioCapabilityResponse(&_arg_info, &_arg_rc);
                  Ok(())
                }
                transactions::r#setRadioPowerResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setRadioPowerResponse(&_arg_info);
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
             pub use super::r#IRadioModemResponse as _7_android_8_hardware_5_radio_5_modem_19_IRadioModemResponse;
            }
          }
          pub mod NvItem {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#NvItem : [i32; 41] {
                r#CDMA_MEID = 1,
                r#CDMA_MIN = 2,
                r#CDMA_MDN = 3,
                r#CDMA_ACCOLC = 4,
                r#DEVICE_MSL = 11,
                r#RTN_RECONDITIONED_STATUS = 12,
                r#RTN_ACTIVATION_DATE = 13,
                r#RTN_LIFE_TIMER = 14,
                r#RTN_LIFE_CALLS = 15,
                r#RTN_LIFE_DATA_TX = 16,
                r#RTN_LIFE_DATA_RX = 17,
                r#OMADM_HFA_LEVEL = 18,
                r#MIP_PROFILE_NAI = 31,
                r#MIP_PROFILE_HOME_ADDRESS = 32,
                r#MIP_PROFILE_AAA_AUTH = 33,
                r#MIP_PROFILE_HA_AUTH = 34,
                r#MIP_PROFILE_PRI_HA_ADDR = 35,
                r#MIP_PROFILE_SEC_HA_ADDR = 36,
                r#MIP_PROFILE_REV_TUN_PREF = 37,
                r#MIP_PROFILE_HA_SPI = 38,
                r#MIP_PROFILE_AAA_SPI = 39,
                r#MIP_PROFILE_MN_HA_SS = 40,
                r#MIP_PROFILE_MN_AAA_SS = 41,
                r#CDMA_PRL_VERSION = 51,
                r#CDMA_BC10 = 52,
                r#CDMA_BC14 = 53,
                r#CDMA_SO68 = 54,
                r#CDMA_SO73_COP0 = 55,
                r#CDMA_SO73_COP1TO7 = 56,
                r#CDMA_1X_ADVANCED_ENABLED = 57,
                r#CDMA_EHRPD_ENABLED = 58,
                r#CDMA_EHRPD_FORCED = 59,
                r#LTE_BAND_ENABLE_25 = 71,
                r#LTE_BAND_ENABLE_26 = 72,
                r#LTE_BAND_ENABLE_41 = 73,
                r#LTE_SCAN_PRIORITY_25 = 74,
                r#LTE_SCAN_PRIORITY_26 = 75,
                r#LTE_SCAN_PRIORITY_41 = 76,
                r#LTE_HIDDEN_BAND_PRIORITY_25 = 77,
                r#LTE_HIDDEN_BAND_PRIORITY_26 = 78,
                r#LTE_HIDDEN_BAND_PRIORITY_41 = 79,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#NvItem as _7_android_8_hardware_5_radio_5_modem_6_NvItem;
            }
          }
          pub mod NvWriteItem {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#NvWriteItem {
              pub r#itemId: crate::mangled::_7_android_8_hardware_5_radio_5_modem_6_NvItem,
              pub r#value: String,
            }
            impl Default for r#NvWriteItem {
              fn default() -> Self {
                Self {
                  r#itemId: Default::default(),
                  r#value: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#NvWriteItem {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#itemId)?;
                  subparcel.write(&self.r#value)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#itemId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#value = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#NvWriteItem);
            binder::impl_deserialize_for_parcelable!(r#NvWriteItem);
            impl binder::binder_impl::ParcelableMetadata for r#NvWriteItem {
              fn get_descriptor() -> &'static str { "android.hardware.radio.modem.NvWriteItem" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#NvWriteItem as _7_android_8_hardware_5_radio_5_modem_11_NvWriteItem;
            }
          }
          pub mod RadioCapability {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug, Clone)]
            pub struct r#RadioCapability {
              pub r#session: i32,
              pub r#phase: i32,
              pub r#raf: i32,
              pub r#logicalModemUuid: String,
              pub r#status: i32,
            }
            pub const r#PHASE_CONFIGURED: i32 = 0;
            pub const r#PHASE_START: i32 = 1;
            pub const r#PHASE_APPLY: i32 = 2;
            pub const r#PHASE_UNSOL_RSP: i32 = 3;
            pub const r#PHASE_FINISH: i32 = 4;
            pub const r#STATUS_NONE: i32 = 0;
            pub const r#STATUS_SUCCESS: i32 = 1;
            pub const r#STATUS_FAIL: i32 = 2;
            impl Default for r#RadioCapability {
              fn default() -> Self {
                Self {
                  r#session: 0,
                  r#phase: 0,
                  r#raf: 0,
                  r#logicalModemUuid: Default::default(),
                  r#status: 0,
                }
              }
            }
            impl binder::Parcelable for r#RadioCapability {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#session)?;
                  subparcel.write(&self.r#phase)?;
                  subparcel.write(&self.r#raf)?;
                  subparcel.write(&self.r#logicalModemUuid)?;
                  subparcel.write(&self.r#status)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#session = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#phase = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#raf = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#logicalModemUuid = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#status = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#RadioCapability);
            binder::impl_deserialize_for_parcelable!(r#RadioCapability);
            impl binder::binder_impl::ParcelableMetadata for r#RadioCapability {
              fn get_descriptor() -> &'static str { "android.hardware.radio.modem.RadioCapability" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#RadioCapability as _7_android_8_hardware_5_radio_5_modem_15_RadioCapability;
            }
          }
          pub mod RadioState {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#RadioState : [i32; 3] {
                r#OFF = 0,
                r#UNAVAILABLE = 1,
                r#ON = 10,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#RadioState as _7_android_8_hardware_5_radio_5_modem_10_RadioState;
            }
          }
          pub mod ResetNvType {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#ResetNvType : [i32; 3] {
                r#RELOAD = 0,
                r#ERASE = 1,
                r#FACTORY_RESET = 2,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#ResetNvType as _7_android_8_hardware_5_radio_5_modem_11_ResetNvType;
            }
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::hardware::radio::modem::ActivityStatsInfo::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::ActivityStatsTechSpecificInfo::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::DeviceStateType::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::HardwareConfig::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::HardwareConfigModem::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::HardwareConfigSim::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::IRadioModem::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::IRadioModemIndication::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::IRadioModemResponse::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::NvItem::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::NvWriteItem::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::RadioCapability::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::RadioState::mangled::*;
  pub use super::aidl::android::hardware::radio::modem::ResetNvType::mangled::*;
  pub(crate) use android_hardware_radio::mangled::*;
}
