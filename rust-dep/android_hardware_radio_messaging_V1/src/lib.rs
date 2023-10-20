#![feature(custom_inner_attributes)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod hardware {
      pub mod radio {
        pub mod messaging {
          pub mod CdmaBroadcastSmsConfigInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaBroadcastSmsConfigInfo {
              pub r#serviceCategory: i32,
              pub r#language: i32,
              pub r#selected: bool,
            }
            impl Default for r#CdmaBroadcastSmsConfigInfo {
              fn default() -> Self {
                Self {
                  r#serviceCategory: 0,
                  r#language: 0,
                  r#selected: false,
                }
              }
            }
            impl binder::Parcelable for r#CdmaBroadcastSmsConfigInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#serviceCategory)?;
                  subparcel.write(&self.r#language)?;
                  subparcel.write(&self.r#selected)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#serviceCategory = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#language = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#selected = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaBroadcastSmsConfigInfo);
            binder::impl_deserialize_for_parcelable!(r#CdmaBroadcastSmsConfigInfo);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaBroadcastSmsConfigInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.messaging.CdmaBroadcastSmsConfigInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaBroadcastSmsConfigInfo as _7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo;
            }
          }
          pub mod CdmaSmsAck {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaSmsAck {
              pub r#errorClass: bool,
              pub r#smsCauseCode: i32,
            }
            impl Default for r#CdmaSmsAck {
              fn default() -> Self {
                Self {
                  r#errorClass: false,
                  r#smsCauseCode: 0,
                }
              }
            }
            impl binder::Parcelable for r#CdmaSmsAck {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#errorClass)?;
                  subparcel.write(&self.r#smsCauseCode)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#errorClass = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#smsCauseCode = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaSmsAck);
            binder::impl_deserialize_for_parcelable!(r#CdmaSmsAck);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaSmsAck {
              fn get_descriptor() -> &'static str { "android.hardware.radio.messaging.CdmaSmsAck" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaSmsAck as _7_android_8_hardware_5_radio_9_messaging_10_CdmaSmsAck;
            }
          }
          pub mod CdmaSmsAddress {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaSmsAddress {
              pub r#digitMode: i32,
              pub r#isNumberModeDataNetwork: bool,
              pub r#numberType: i32,
              pub r#numberPlan: i32,
              pub r#digits: Vec<u8>,
            }
            pub const r#DIGIT_MODE_FOUR_BIT: i32 = 0;
            pub const r#DIGIT_MODE_EIGHT_BIT: i32 = 1;
            pub const r#NUMBER_PLAN_UNKNOWN: i32 = 0;
            pub const r#NUMBER_PLAN_TELEPHONY: i32 = 1;
            pub const r#NUMBER_PLAN_RESERVED_2: i32 = 2;
            pub const r#NUMBER_PLAN_DATA: i32 = 3;
            pub const r#NUMBER_PLAN_TELEX: i32 = 4;
            pub const r#NUMBER_PLAN_RESERVED_5: i32 = 5;
            pub const r#NUMBER_PLAN_RESERVED_6: i32 = 6;
            pub const r#NUMBER_PLAN_RESERVED_7: i32 = 7;
            pub const r#NUMBER_PLAN_RESERVED_8: i32 = 8;
            pub const r#NUMBER_PLAN_PRIVATE: i32 = 9;
            pub const r#NUMBER_PLAN_RESERVED_10: i32 = 10;
            pub const r#NUMBER_PLAN_RESERVED_11: i32 = 11;
            pub const r#NUMBER_PLAN_RESERVED_12: i32 = 12;
            pub const r#NUMBER_PLAN_RESERVED_13: i32 = 13;
            pub const r#NUMBER_PLAN_RESERVED_14: i32 = 14;
            pub const r#NUMBER_PLAN_RESERVED_15: i32 = 15;
            pub const r#NUMBER_TYPE_UNKNOWN: i32 = 0;
            pub const r#NUMBER_TYPE_INTERNATIONAL_OR_DATA_IP: i32 = 1;
            pub const r#NUMBER_TYPE_NATIONAL_OR_INTERNET_MAIL: i32 = 2;
            pub const r#NUMBER_TYPE_NETWORK: i32 = 3;
            pub const r#NUMBER_TYPE_SUBSCRIBER: i32 = 4;
            pub const r#NUMBER_TYPE_ALPHANUMERIC: i32 = 5;
            pub const r#NUMBER_TYPE_ABBREVIATED: i32 = 6;
            pub const r#NUMBER_TYPE_RESERVED_7: i32 = 7;
            impl Default for r#CdmaSmsAddress {
              fn default() -> Self {
                Self {
                  r#digitMode: 0,
                  r#isNumberModeDataNetwork: false,
                  r#numberType: 0,
                  r#numberPlan: 0,
                  r#digits: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CdmaSmsAddress {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#digitMode)?;
                  subparcel.write(&self.r#isNumberModeDataNetwork)?;
                  subparcel.write(&self.r#numberType)?;
                  subparcel.write(&self.r#numberPlan)?;
                  subparcel.write(&self.r#digits)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#digitMode = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#isNumberModeDataNetwork = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#numberType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#numberPlan = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#digits = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaSmsAddress);
            binder::impl_deserialize_for_parcelable!(r#CdmaSmsAddress);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaSmsAddress {
              fn get_descriptor() -> &'static str { "android.hardware.radio.messaging.CdmaSmsAddress" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaSmsAddress as _7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsAddress;
            }
          }
          pub mod CdmaSmsMessage {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaSmsMessage {
              pub r#teleserviceId: i32,
              pub r#isServicePresent: bool,
              pub r#serviceCategory: i32,
              pub r#address: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsAddress,
              pub r#subAddress: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_17_CdmaSmsSubaddress,
              pub r#bearerData: Vec<u8>,
            }
            impl Default for r#CdmaSmsMessage {
              fn default() -> Self {
                Self {
                  r#teleserviceId: 0,
                  r#isServicePresent: false,
                  r#serviceCategory: 0,
                  r#address: Default::default(),
                  r#subAddress: Default::default(),
                  r#bearerData: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CdmaSmsMessage {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#teleserviceId)?;
                  subparcel.write(&self.r#isServicePresent)?;
                  subparcel.write(&self.r#serviceCategory)?;
                  subparcel.write(&self.r#address)?;
                  subparcel.write(&self.r#subAddress)?;
                  subparcel.write(&self.r#bearerData)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#teleserviceId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#isServicePresent = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#serviceCategory = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#address = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#subAddress = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bearerData = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaSmsMessage);
            binder::impl_deserialize_for_parcelable!(r#CdmaSmsMessage);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaSmsMessage {
              fn get_descriptor() -> &'static str { "android.hardware.radio.messaging.CdmaSmsMessage" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaSmsMessage as _7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage;
            }
          }
          pub mod CdmaSmsSubaddress {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaSmsSubaddress {
              pub r#subaddressType: i32,
              pub r#odd: bool,
              pub r#digits: Vec<u8>,
            }
            pub const r#SUBADDRESS_TYPE_NSAP: i32 = 0;
            pub const r#SUBADDRESS_TYPE_USER_SPECIFIED: i32 = 1;
            impl Default for r#CdmaSmsSubaddress {
              fn default() -> Self {
                Self {
                  r#subaddressType: 0,
                  r#odd: false,
                  r#digits: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CdmaSmsSubaddress {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#subaddressType)?;
                  subparcel.write(&self.r#odd)?;
                  subparcel.write(&self.r#digits)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#subaddressType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#odd = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#digits = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaSmsSubaddress);
            binder::impl_deserialize_for_parcelable!(r#CdmaSmsSubaddress);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaSmsSubaddress {
              fn get_descriptor() -> &'static str { "android.hardware.radio.messaging.CdmaSmsSubaddress" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaSmsSubaddress as _7_android_8_hardware_5_radio_9_messaging_17_CdmaSmsSubaddress;
            }
          }
          pub mod CdmaSmsWriteArgs {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaSmsWriteArgs {
              pub r#status: i32,
              pub r#message: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage,
            }
            pub const r#STATUS_REC_UNREAD: i32 = 0;
            pub const r#STATUS_REC_READ: i32 = 1;
            pub const r#STATUS_STO_UNSENT: i32 = 2;
            pub const r#STATUS_STO_SENT: i32 = 3;
            impl Default for r#CdmaSmsWriteArgs {
              fn default() -> Self {
                Self {
                  r#status: 0,
                  r#message: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CdmaSmsWriteArgs {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#status)?;
                  subparcel.write(&self.r#message)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#status = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#message = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaSmsWriteArgs);
            binder::impl_deserialize_for_parcelable!(r#CdmaSmsWriteArgs);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaSmsWriteArgs {
              fn get_descriptor() -> &'static str { "android.hardware.radio.messaging.CdmaSmsWriteArgs" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaSmsWriteArgs as _7_android_8_hardware_5_radio_9_messaging_16_CdmaSmsWriteArgs;
            }
          }
          pub mod GsmBroadcastSmsConfigInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#GsmBroadcastSmsConfigInfo {
              pub r#fromServiceId: i32,
              pub r#toServiceId: i32,
              pub r#fromCodeScheme: i32,
              pub r#toCodeScheme: i32,
              pub r#selected: bool,
            }
            impl Default for r#GsmBroadcastSmsConfigInfo {
              fn default() -> Self {
                Self {
                  r#fromServiceId: 0,
                  r#toServiceId: 0,
                  r#fromCodeScheme: 0,
                  r#toCodeScheme: 0,
                  r#selected: false,
                }
              }
            }
            impl binder::Parcelable for r#GsmBroadcastSmsConfigInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#fromServiceId)?;
                  subparcel.write(&self.r#toServiceId)?;
                  subparcel.write(&self.r#fromCodeScheme)?;
                  subparcel.write(&self.r#toCodeScheme)?;
                  subparcel.write(&self.r#selected)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#fromServiceId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#toServiceId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#fromCodeScheme = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#toCodeScheme = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#selected = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#GsmBroadcastSmsConfigInfo);
            binder::impl_deserialize_for_parcelable!(r#GsmBroadcastSmsConfigInfo);
            impl binder::binder_impl::ParcelableMetadata for r#GsmBroadcastSmsConfigInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.messaging.GsmBroadcastSmsConfigInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#GsmBroadcastSmsConfigInfo as _7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo;
            }
          }
          pub mod GsmSmsMessage {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#GsmSmsMessage {
              pub r#smscPdu: String,
              pub r#pdu: String,
            }
            impl Default for r#GsmSmsMessage {
              fn default() -> Self {
                Self {
                  r#smscPdu: Default::default(),
                  r#pdu: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#GsmSmsMessage {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#smscPdu)?;
                  subparcel.write(&self.r#pdu)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#smscPdu = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#pdu = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#GsmSmsMessage);
            binder::impl_deserialize_for_parcelable!(r#GsmSmsMessage);
            impl binder::binder_impl::ParcelableMetadata for r#GsmSmsMessage {
              fn get_descriptor() -> &'static str { "android.hardware.radio.messaging.GsmSmsMessage" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#GsmSmsMessage as _7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage;
            }
          }
          pub mod IRadioMessaging {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioMessaging["android.hardware.radio.messaging.IRadioMessaging"] {
                native: BnRadioMessaging(on_transact),
                proxy: BpRadioMessaging {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioMessagingAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioMessaging: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.messaging.IRadioMessaging" }
              fn r#acknowledgeIncomingGsmSmsWithPdu(&self, _arg_serial: i32, _arg_success: bool, _arg_ackPdu: &str) -> binder::Result<()>;
              fn r#acknowledgeLastIncomingCdmaSms(&self, _arg_serial: i32, _arg_smsAck: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_10_CdmaSmsAck) -> binder::Result<()>;
              fn r#acknowledgeLastIncomingGsmSms(&self, _arg_serial: i32, _arg_success: bool, _arg_cause: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_SmsAcknowledgeFailCause) -> binder::Result<()>;
              fn r#deleteSmsOnRuim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<()>;
              fn r#deleteSmsOnSim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<()>;
              fn r#getCdmaBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getGsmBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getSmscAddress(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#reportSmsMemoryStatus(&self, _arg_serial: i32, _arg_available: bool) -> binder::Result<()>;
              fn r#responseAcknowledgement(&self) -> binder::Result<()>;
              fn r#sendCdmaSms(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()>;
              fn r#sendCdmaSmsExpectMore(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()>;
              fn r#sendImsSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_ImsSmsMessage) -> binder::Result<()>;
              fn r#sendSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<()>;
              fn r#sendSmsExpectMore(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<()>;
              fn r#setCdmaBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<()>;
              fn r#setCdmaBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<()>;
              fn r#setGsmBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<()>;
              fn r#setGsmBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<()>;
              fn r#setResponseFunctions(&self, _arg_radioMessagingResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_IRadioMessagingResponse>, _arg_radioMessagingIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_IRadioMessagingIndication>) -> binder::Result<()>;
              fn r#setSmscAddress(&self, _arg_serial: i32, _arg_smsc: &str) -> binder::Result<()>;
              fn r#writeSmsToRuim(&self, _arg_serial: i32, _arg_cdmaSms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_16_CdmaSmsWriteArgs) -> binder::Result<()>;
              fn r#writeSmsToSim(&self, _arg_serial: i32, _arg_smsWriteArgs: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_12_SmsWriteArgs) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioMessagingDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioMessagingDefaultRef) -> IRadioMessagingDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioMessagingAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.messaging.IRadioMessaging" }
              fn r#acknowledgeIncomingGsmSmsWithPdu(&self, _arg_serial: i32, _arg_success: bool, _arg_ackPdu: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#acknowledgeLastIncomingCdmaSms(&self, _arg_serial: i32, _arg_smsAck: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_10_CdmaSmsAck) -> std::future::Ready<binder::Result<()>>;
              fn r#acknowledgeLastIncomingGsmSms(&self, _arg_serial: i32, _arg_success: bool, _arg_cause: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_SmsAcknowledgeFailCause) -> std::future::Ready<binder::Result<()>>;
              fn r#deleteSmsOnRuim(&self, _arg_serial: i32, _arg_index: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#deleteSmsOnSim(&self, _arg_serial: i32, _arg_index: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getCdmaBroadcastConfig(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getGsmBroadcastConfig(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getSmscAddress(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#reportSmsMemoryStatus(&self, _arg_serial: i32, _arg_available: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#responseAcknowledgement(&self) -> std::future::Ready<binder::Result<()>>;
              fn r#sendCdmaSms(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> std::future::Ready<binder::Result<()>>;
              fn r#sendCdmaSmsExpectMore(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> std::future::Ready<binder::Result<()>>;
              fn r#sendImsSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_ImsSmsMessage) -> std::future::Ready<binder::Result<()>>;
              fn r#sendSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> std::future::Ready<binder::Result<()>>;
              fn r#sendSmsExpectMore(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> std::future::Ready<binder::Result<()>>;
              fn r#setCdmaBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#setCdmaBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#setGsmBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#setGsmBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#setResponseFunctions(&self, _arg_radioMessagingResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_IRadioMessagingResponse>, _arg_radioMessagingIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_IRadioMessagingIndication>) -> std::future::Ready<binder::Result<()>>;
              fn r#setSmscAddress(&self, _arg_serial: i32, _arg_smsc: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#writeSmsToRuim(&self, _arg_serial: i32, _arg_cdmaSms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_16_CdmaSmsWriteArgs) -> std::future::Ready<binder::Result<()>>;
              fn r#writeSmsToSim(&self, _arg_serial: i32, _arg_smsWriteArgs: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_12_SmsWriteArgs) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioMessagingAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.messaging.IRadioMessaging" }
              async fn r#acknowledgeIncomingGsmSmsWithPdu(&self, _arg_serial: i32, _arg_success: bool, _arg_ackPdu: &str) -> binder::Result<()>;
              async fn r#acknowledgeLastIncomingCdmaSms(&self, _arg_serial: i32, _arg_smsAck: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_10_CdmaSmsAck) -> binder::Result<()>;
              async fn r#acknowledgeLastIncomingGsmSms(&self, _arg_serial: i32, _arg_success: bool, _arg_cause: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_SmsAcknowledgeFailCause) -> binder::Result<()>;
              async fn r#deleteSmsOnRuim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<()>;
              async fn r#deleteSmsOnSim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<()>;
              async fn r#getCdmaBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getGsmBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getSmscAddress(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#reportSmsMemoryStatus(&self, _arg_serial: i32, _arg_available: bool) -> binder::Result<()>;
              async fn r#responseAcknowledgement(&self) -> binder::Result<()>;
              async fn r#sendCdmaSms(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()>;
              async fn r#sendCdmaSmsExpectMore(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()>;
              async fn r#sendImsSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_ImsSmsMessage) -> binder::Result<()>;
              async fn r#sendSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<()>;
              async fn r#sendSmsExpectMore(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<()>;
              async fn r#setCdmaBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<()>;
              async fn r#setCdmaBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<()>;
              async fn r#setGsmBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<()>;
              async fn r#setGsmBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<()>;
              async fn r#setResponseFunctions(&self, _arg_radioMessagingResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_IRadioMessagingResponse>, _arg_radioMessagingIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_IRadioMessagingIndication>) -> binder::Result<()>;
              async fn r#setSmscAddress(&self, _arg_serial: i32, _arg_smsc: &str) -> binder::Result<()>;
              async fn r#writeSmsToRuim(&self, _arg_serial: i32, _arg_cdmaSms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_16_CdmaSmsWriteArgs) -> binder::Result<()>;
              async fn r#writeSmsToSim(&self, _arg_serial: i32, _arg_smsWriteArgs: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_12_SmsWriteArgs) -> binder::Result<()>;
            }
            impl BnRadioMessaging {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioMessaging>
              where
                T: IRadioMessagingAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioMessaging for Wrapper<T, R>
                where
                  T: IRadioMessagingAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#acknowledgeIncomingGsmSmsWithPdu(&self, _arg_serial: i32, _arg_success: bool, _arg_ackPdu: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acknowledgeIncomingGsmSmsWithPdu(_arg_serial, _arg_success, _arg_ackPdu))
                  }
                  fn r#acknowledgeLastIncomingCdmaSms(&self, _arg_serial: i32, _arg_smsAck: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_10_CdmaSmsAck) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acknowledgeLastIncomingCdmaSms(_arg_serial, _arg_smsAck))
                  }
                  fn r#acknowledgeLastIncomingGsmSms(&self, _arg_serial: i32, _arg_success: bool, _arg_cause: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_SmsAcknowledgeFailCause) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acknowledgeLastIncomingGsmSms(_arg_serial, _arg_success, _arg_cause))
                  }
                  fn r#deleteSmsOnRuim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#deleteSmsOnRuim(_arg_serial, _arg_index))
                  }
                  fn r#deleteSmsOnSim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#deleteSmsOnSim(_arg_serial, _arg_index))
                  }
                  fn r#getCdmaBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCdmaBroadcastConfig(_arg_serial))
                  }
                  fn r#getGsmBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getGsmBroadcastConfig(_arg_serial))
                  }
                  fn r#getSmscAddress(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSmscAddress(_arg_serial))
                  }
                  fn r#reportSmsMemoryStatus(&self, _arg_serial: i32, _arg_available: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#reportSmsMemoryStatus(_arg_serial, _arg_available))
                  }
                  fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#responseAcknowledgement())
                  }
                  fn r#sendCdmaSms(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendCdmaSms(_arg_serial, _arg_sms))
                  }
                  fn r#sendCdmaSmsExpectMore(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendCdmaSmsExpectMore(_arg_serial, _arg_sms))
                  }
                  fn r#sendImsSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_ImsSmsMessage) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendImsSms(_arg_serial, _arg_message))
                  }
                  fn r#sendSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendSms(_arg_serial, _arg_message))
                  }
                  fn r#sendSmsExpectMore(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendSmsExpectMore(_arg_serial, _arg_message))
                  }
                  fn r#setCdmaBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCdmaBroadcastActivation(_arg_serial, _arg_activate))
                  }
                  fn r#setCdmaBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCdmaBroadcastConfig(_arg_serial, _arg_configInfo))
                  }
                  fn r#setGsmBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setGsmBroadcastActivation(_arg_serial, _arg_activate))
                  }
                  fn r#setGsmBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setGsmBroadcastConfig(_arg_serial, _arg_configInfo))
                  }
                  fn r#setResponseFunctions(&self, _arg_radioMessagingResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_IRadioMessagingResponse>, _arg_radioMessagingIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_IRadioMessagingIndication>) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setResponseFunctions(_arg_radioMessagingResponse, _arg_radioMessagingIndication))
                  }
                  fn r#setSmscAddress(&self, _arg_serial: i32, _arg_smsc: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setSmscAddress(_arg_serial, _arg_smsc))
                  }
                  fn r#writeSmsToRuim(&self, _arg_serial: i32, _arg_cdmaSms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_16_CdmaSmsWriteArgs) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#writeSmsToRuim(_arg_serial, _arg_cdmaSms))
                  }
                  fn r#writeSmsToSim(&self, _arg_serial: i32, _arg_smsWriteArgs: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_12_SmsWriteArgs) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#writeSmsToSim(_arg_serial, _arg_smsWriteArgs))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioMessagingDefault: Send + Sync {
              fn r#acknowledgeIncomingGsmSmsWithPdu(&self, _arg_serial: i32, _arg_success: bool, _arg_ackPdu: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#acknowledgeLastIncomingCdmaSms(&self, _arg_serial: i32, _arg_smsAck: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_10_CdmaSmsAck) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#acknowledgeLastIncomingGsmSms(&self, _arg_serial: i32, _arg_success: bool, _arg_cause: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_SmsAcknowledgeFailCause) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#deleteSmsOnRuim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#deleteSmsOnSim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCdmaBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getGsmBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSmscAddress(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#reportSmsMemoryStatus(&self, _arg_serial: i32, _arg_available: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendCdmaSms(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendCdmaSmsExpectMore(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendImsSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_ImsSmsMessage) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendSmsExpectMore(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCdmaBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCdmaBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setGsmBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setGsmBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setResponseFunctions(&self, _arg_radioMessagingResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_IRadioMessagingResponse>, _arg_radioMessagingIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_IRadioMessagingIndication>) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setSmscAddress(&self, _arg_serial: i32, _arg_smsc: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#writeSmsToRuim(&self, _arg_serial: i32, _arg_cdmaSms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_16_CdmaSmsWriteArgs) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#writeSmsToSim(&self, _arg_serial: i32, _arg_smsWriteArgs: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_12_SmsWriteArgs) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#acknowledgeIncomingGsmSmsWithPdu: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#acknowledgeLastIncomingCdmaSms: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#acknowledgeLastIncomingGsmSms: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#deleteSmsOnRuim: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#deleteSmsOnSim: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#getCdmaBroadcastConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#getGsmBroadcastConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#getSmscAddress: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#reportSmsMemoryStatus: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#responseAcknowledgement: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#sendCdmaSms: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#sendCdmaSmsExpectMore: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#sendImsSms: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#sendSms: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#sendSmsExpectMore: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#setCdmaBroadcastActivation: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#setCdmaBroadcastConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
              pub const r#setGsmBroadcastActivation: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
              pub const r#setGsmBroadcastConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
              pub const r#setResponseFunctions: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
              pub const r#setSmscAddress: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 20;
              pub const r#writeSmsToRuim: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 21;
              pub const r#writeSmsToSim: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 22;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioMessagingDefaultRef = Option<std::sync::Arc<dyn IRadioMessagingDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioMessagingDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "5237ec5f500627b6b844b155e356e603157f9ba6";
            impl BpRadioMessaging {
              fn build_parcel_acknowledgeIncomingGsmSmsWithPdu(&self, _arg_serial: i32, _arg_success: bool, _arg_ackPdu: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_success)?;
                aidl_data.write(_arg_ackPdu)?;
                Ok(aidl_data)
              }
              fn read_response_acknowledgeIncomingGsmSmsWithPdu(&self, _arg_serial: i32, _arg_success: bool, _arg_ackPdu: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#acknowledgeIncomingGsmSmsWithPdu(_arg_serial, _arg_success, _arg_ackPdu);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_acknowledgeLastIncomingCdmaSms(&self, _arg_serial: i32, _arg_smsAck: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_10_CdmaSmsAck) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_smsAck)?;
                Ok(aidl_data)
              }
              fn read_response_acknowledgeLastIncomingCdmaSms(&self, _arg_serial: i32, _arg_smsAck: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_10_CdmaSmsAck, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#acknowledgeLastIncomingCdmaSms(_arg_serial, _arg_smsAck);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_acknowledgeLastIncomingGsmSms(&self, _arg_serial: i32, _arg_success: bool, _arg_cause: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_SmsAcknowledgeFailCause) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_success)?;
                aidl_data.write(&_arg_cause)?;
                Ok(aidl_data)
              }
              fn read_response_acknowledgeLastIncomingGsmSms(&self, _arg_serial: i32, _arg_success: bool, _arg_cause: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_SmsAcknowledgeFailCause, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#acknowledgeLastIncomingGsmSms(_arg_serial, _arg_success, _arg_cause);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_deleteSmsOnRuim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_index)?;
                Ok(aidl_data)
              }
              fn read_response_deleteSmsOnRuim(&self, _arg_serial: i32, _arg_index: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#deleteSmsOnRuim(_arg_serial, _arg_index);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_deleteSmsOnSim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_index)?;
                Ok(aidl_data)
              }
              fn read_response_deleteSmsOnSim(&self, _arg_serial: i32, _arg_index: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#deleteSmsOnSim(_arg_serial, _arg_index);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCdmaBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getCdmaBroadcastConfig(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCdmaBroadcastConfig(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getGsmBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getGsmBroadcastConfig(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#getGsmBroadcastConfig(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSmscAddress(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getSmscAddress(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSmscAddress(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_reportSmsMemoryStatus(&self, _arg_serial: i32, _arg_available: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_available)?;
                Ok(aidl_data)
              }
              fn read_response_reportSmsMemoryStatus(&self, _arg_serial: i32, _arg_available: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#reportSmsMemoryStatus(_arg_serial, _arg_available);
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
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#responseAcknowledgement();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendCdmaSms(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_sms)?;
                Ok(aidl_data)
              }
              fn read_response_sendCdmaSms(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendCdmaSms(_arg_serial, _arg_sms);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendCdmaSmsExpectMore(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_sms)?;
                Ok(aidl_data)
              }
              fn read_response_sendCdmaSmsExpectMore(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendCdmaSmsExpectMore(_arg_serial, _arg_sms);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendImsSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_ImsSmsMessage) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_message)?;
                Ok(aidl_data)
              }
              fn read_response_sendImsSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_ImsSmsMessage, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendImsSms(_arg_serial, _arg_message);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_message)?;
                Ok(aidl_data)
              }
              fn read_response_sendSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendSms(_arg_serial, _arg_message);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendSmsExpectMore(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_message)?;
                Ok(aidl_data)
              }
              fn read_response_sendSmsExpectMore(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendSmsExpectMore(_arg_serial, _arg_message);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCdmaBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_activate)?;
                Ok(aidl_data)
              }
              fn read_response_setCdmaBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCdmaBroadcastActivation(_arg_serial, _arg_activate);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCdmaBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_configInfo)?;
                Ok(aidl_data)
              }
              fn read_response_setCdmaBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCdmaBroadcastConfig(_arg_serial, _arg_configInfo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setGsmBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_activate)?;
                Ok(aidl_data)
              }
              fn read_response_setGsmBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#setGsmBroadcastActivation(_arg_serial, _arg_activate);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setGsmBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_configInfo)?;
                Ok(aidl_data)
              }
              fn read_response_setGsmBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#setGsmBroadcastConfig(_arg_serial, _arg_configInfo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setResponseFunctions(&self, _arg_radioMessagingResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_IRadioMessagingResponse>, _arg_radioMessagingIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_IRadioMessagingIndication>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_radioMessagingResponse)?;
                aidl_data.write(_arg_radioMessagingIndication)?;
                Ok(aidl_data)
              }
              fn read_response_setResponseFunctions(&self, _arg_radioMessagingResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_IRadioMessagingResponse>, _arg_radioMessagingIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_IRadioMessagingIndication>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#setResponseFunctions(_arg_radioMessagingResponse, _arg_radioMessagingIndication);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setSmscAddress(&self, _arg_serial: i32, _arg_smsc: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_smsc)?;
                Ok(aidl_data)
              }
              fn read_response_setSmscAddress(&self, _arg_serial: i32, _arg_smsc: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#setSmscAddress(_arg_serial, _arg_smsc);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_writeSmsToRuim(&self, _arg_serial: i32, _arg_cdmaSms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_16_CdmaSmsWriteArgs) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_cdmaSms)?;
                Ok(aidl_data)
              }
              fn read_response_writeSmsToRuim(&self, _arg_serial: i32, _arg_cdmaSms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_16_CdmaSmsWriteArgs, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#writeSmsToRuim(_arg_serial, _arg_cdmaSms);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_writeSmsToSim(&self, _arg_serial: i32, _arg_smsWriteArgs: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_12_SmsWriteArgs) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_smsWriteArgs)?;
                Ok(aidl_data)
              }
              fn read_response_writeSmsToSim(&self, _arg_serial: i32, _arg_smsWriteArgs: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_12_SmsWriteArgs, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessaging>::getDefaultImpl() {
                    return _aidl_default_impl.r#writeSmsToSim(_arg_serial, _arg_smsWriteArgs);
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
            impl IRadioMessaging for BpRadioMessaging {
              fn r#acknowledgeIncomingGsmSmsWithPdu(&self, _arg_serial: i32, _arg_success: bool, _arg_ackPdu: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acknowledgeIncomingGsmSmsWithPdu(_arg_serial, _arg_success, _arg_ackPdu)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeIncomingGsmSmsWithPdu, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acknowledgeIncomingGsmSmsWithPdu(_arg_serial, _arg_success, _arg_ackPdu, _aidl_reply)
              }
              fn r#acknowledgeLastIncomingCdmaSms(&self, _arg_serial: i32, _arg_smsAck: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_10_CdmaSmsAck) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acknowledgeLastIncomingCdmaSms(_arg_serial, _arg_smsAck)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeLastIncomingCdmaSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acknowledgeLastIncomingCdmaSms(_arg_serial, _arg_smsAck, _aidl_reply)
              }
              fn r#acknowledgeLastIncomingGsmSms(&self, _arg_serial: i32, _arg_success: bool, _arg_cause: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_SmsAcknowledgeFailCause) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acknowledgeLastIncomingGsmSms(_arg_serial, _arg_success, _arg_cause)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeLastIncomingGsmSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acknowledgeLastIncomingGsmSms(_arg_serial, _arg_success, _arg_cause, _aidl_reply)
              }
              fn r#deleteSmsOnRuim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_deleteSmsOnRuim(_arg_serial, _arg_index)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#deleteSmsOnRuim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_deleteSmsOnRuim(_arg_serial, _arg_index, _aidl_reply)
              }
              fn r#deleteSmsOnSim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_deleteSmsOnSim(_arg_serial, _arg_index)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#deleteSmsOnSim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_deleteSmsOnSim(_arg_serial, _arg_index, _aidl_reply)
              }
              fn r#getCdmaBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCdmaBroadcastConfig(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaBroadcastConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCdmaBroadcastConfig(_arg_serial, _aidl_reply)
              }
              fn r#getGsmBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getGsmBroadcastConfig(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getGsmBroadcastConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getGsmBroadcastConfig(_arg_serial, _aidl_reply)
              }
              fn r#getSmscAddress(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSmscAddress(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSmscAddress, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSmscAddress(_arg_serial, _aidl_reply)
              }
              fn r#reportSmsMemoryStatus(&self, _arg_serial: i32, _arg_available: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_reportSmsMemoryStatus(_arg_serial, _arg_available)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#reportSmsMemoryStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_reportSmsMemoryStatus(_arg_serial, _arg_available, _aidl_reply)
              }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_responseAcknowledgement()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#responseAcknowledgement, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_responseAcknowledgement(_aidl_reply)
              }
              fn r#sendCdmaSms(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendCdmaSms(_arg_serial, _arg_sms)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendCdmaSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendCdmaSms(_arg_serial, _arg_sms, _aidl_reply)
              }
              fn r#sendCdmaSmsExpectMore(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendCdmaSmsExpectMore(_arg_serial, _arg_sms)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendCdmaSmsExpectMore, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendCdmaSmsExpectMore(_arg_serial, _arg_sms, _aidl_reply)
              }
              fn r#sendImsSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_ImsSmsMessage) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendImsSms(_arg_serial, _arg_message)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendImsSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendImsSms(_arg_serial, _arg_message, _aidl_reply)
              }
              fn r#sendSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendSms(_arg_serial, _arg_message)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendSms(_arg_serial, _arg_message, _aidl_reply)
              }
              fn r#sendSmsExpectMore(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendSmsExpectMore(_arg_serial, _arg_message)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendSmsExpectMore, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendSmsExpectMore(_arg_serial, _arg_message, _aidl_reply)
              }
              fn r#setCdmaBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCdmaBroadcastActivation(_arg_serial, _arg_activate)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaBroadcastActivation, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCdmaBroadcastActivation(_arg_serial, _arg_activate, _aidl_reply)
              }
              fn r#setCdmaBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCdmaBroadcastConfig(_arg_serial, _arg_configInfo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaBroadcastConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCdmaBroadcastConfig(_arg_serial, _arg_configInfo, _aidl_reply)
              }
              fn r#setGsmBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setGsmBroadcastActivation(_arg_serial, _arg_activate)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setGsmBroadcastActivation, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setGsmBroadcastActivation(_arg_serial, _arg_activate, _aidl_reply)
              }
              fn r#setGsmBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setGsmBroadcastConfig(_arg_serial, _arg_configInfo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setGsmBroadcastConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setGsmBroadcastConfig(_arg_serial, _arg_configInfo, _aidl_reply)
              }
              fn r#setResponseFunctions(&self, _arg_radioMessagingResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_IRadioMessagingResponse>, _arg_radioMessagingIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_IRadioMessagingIndication>) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setResponseFunctions(_arg_radioMessagingResponse, _arg_radioMessagingIndication)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setResponseFunctions(_arg_radioMessagingResponse, _arg_radioMessagingIndication, _aidl_reply)
              }
              fn r#setSmscAddress(&self, _arg_serial: i32, _arg_smsc: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setSmscAddress(_arg_serial, _arg_smsc)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSmscAddress, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setSmscAddress(_arg_serial, _arg_smsc, _aidl_reply)
              }
              fn r#writeSmsToRuim(&self, _arg_serial: i32, _arg_cdmaSms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_16_CdmaSmsWriteArgs) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_writeSmsToRuim(_arg_serial, _arg_cdmaSms)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#writeSmsToRuim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_writeSmsToRuim(_arg_serial, _arg_cdmaSms, _aidl_reply)
              }
              fn r#writeSmsToSim(&self, _arg_serial: i32, _arg_smsWriteArgs: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_12_SmsWriteArgs) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_writeSmsToSim(_arg_serial, _arg_smsWriteArgs)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#writeSmsToSim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_writeSmsToSim(_arg_serial, _arg_smsWriteArgs, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioMessagingAsync<P> for BpRadioMessaging {
              fn r#acknowledgeIncomingGsmSmsWithPdu(&self, _arg_serial: i32, _arg_success: bool, _arg_ackPdu: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acknowledgeIncomingGsmSmsWithPdu(_arg_serial, _arg_success, _arg_ackPdu) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeIncomingGsmSmsWithPdu, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acknowledgeIncomingGsmSmsWithPdu(_arg_serial, _arg_success, _arg_ackPdu, _aidl_reply))
              }
              fn r#acknowledgeLastIncomingCdmaSms(&self, _arg_serial: i32, _arg_smsAck: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_10_CdmaSmsAck) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acknowledgeLastIncomingCdmaSms(_arg_serial, _arg_smsAck) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeLastIncomingCdmaSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acknowledgeLastIncomingCdmaSms(_arg_serial, _arg_smsAck, _aidl_reply))
              }
              fn r#acknowledgeLastIncomingGsmSms(&self, _arg_serial: i32, _arg_success: bool, _arg_cause: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_SmsAcknowledgeFailCause) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acknowledgeLastIncomingGsmSms(_arg_serial, _arg_success, _arg_cause) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeLastIncomingGsmSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acknowledgeLastIncomingGsmSms(_arg_serial, _arg_success, _arg_cause, _aidl_reply))
              }
              fn r#deleteSmsOnRuim(&self, _arg_serial: i32, _arg_index: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_deleteSmsOnRuim(_arg_serial, _arg_index) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#deleteSmsOnRuim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_deleteSmsOnRuim(_arg_serial, _arg_index, _aidl_reply))
              }
              fn r#deleteSmsOnSim(&self, _arg_serial: i32, _arg_index: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_deleteSmsOnSim(_arg_serial, _arg_index) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#deleteSmsOnSim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_deleteSmsOnSim(_arg_serial, _arg_index, _aidl_reply))
              }
              fn r#getCdmaBroadcastConfig(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCdmaBroadcastConfig(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaBroadcastConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCdmaBroadcastConfig(_arg_serial, _aidl_reply))
              }
              fn r#getGsmBroadcastConfig(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getGsmBroadcastConfig(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getGsmBroadcastConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getGsmBroadcastConfig(_arg_serial, _aidl_reply))
              }
              fn r#getSmscAddress(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSmscAddress(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSmscAddress, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSmscAddress(_arg_serial, _aidl_reply))
              }
              fn r#reportSmsMemoryStatus(&self, _arg_serial: i32, _arg_available: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_reportSmsMemoryStatus(_arg_serial, _arg_available) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#reportSmsMemoryStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_reportSmsMemoryStatus(_arg_serial, _arg_available, _aidl_reply))
              }
              fn r#responseAcknowledgement(&self) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_responseAcknowledgement() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#responseAcknowledgement, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_responseAcknowledgement(_aidl_reply))
              }
              fn r#sendCdmaSms(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendCdmaSms(_arg_serial, _arg_sms) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendCdmaSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendCdmaSms(_arg_serial, _arg_sms, _aidl_reply))
              }
              fn r#sendCdmaSmsExpectMore(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendCdmaSmsExpectMore(_arg_serial, _arg_sms) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendCdmaSmsExpectMore, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendCdmaSmsExpectMore(_arg_serial, _arg_sms, _aidl_reply))
              }
              fn r#sendImsSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_ImsSmsMessage) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendImsSms(_arg_serial, _arg_message) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendImsSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendImsSms(_arg_serial, _arg_message, _aidl_reply))
              }
              fn r#sendSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendSms(_arg_serial, _arg_message) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendSms(_arg_serial, _arg_message, _aidl_reply))
              }
              fn r#sendSmsExpectMore(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendSmsExpectMore(_arg_serial, _arg_message) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendSmsExpectMore, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendSmsExpectMore(_arg_serial, _arg_message, _aidl_reply))
              }
              fn r#setCdmaBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCdmaBroadcastActivation(_arg_serial, _arg_activate) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaBroadcastActivation, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCdmaBroadcastActivation(_arg_serial, _arg_activate, _aidl_reply))
              }
              fn r#setCdmaBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCdmaBroadcastConfig(_arg_serial, _arg_configInfo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaBroadcastConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCdmaBroadcastConfig(_arg_serial, _arg_configInfo, _aidl_reply))
              }
              fn r#setGsmBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setGsmBroadcastActivation(_arg_serial, _arg_activate) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setGsmBroadcastActivation, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setGsmBroadcastActivation(_arg_serial, _arg_activate, _aidl_reply))
              }
              fn r#setGsmBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setGsmBroadcastConfig(_arg_serial, _arg_configInfo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setGsmBroadcastConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setGsmBroadcastConfig(_arg_serial, _arg_configInfo, _aidl_reply))
              }
              fn r#setResponseFunctions(&self, _arg_radioMessagingResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_IRadioMessagingResponse>, _arg_radioMessagingIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_IRadioMessagingIndication>) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setResponseFunctions(_arg_radioMessagingResponse, _arg_radioMessagingIndication) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setResponseFunctions(_arg_radioMessagingResponse, _arg_radioMessagingIndication, _aidl_reply))
              }
              fn r#setSmscAddress(&self, _arg_serial: i32, _arg_smsc: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setSmscAddress(_arg_serial, _arg_smsc) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSmscAddress, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setSmscAddress(_arg_serial, _arg_smsc, _aidl_reply))
              }
              fn r#writeSmsToRuim(&self, _arg_serial: i32, _arg_cdmaSms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_16_CdmaSmsWriteArgs) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_writeSmsToRuim(_arg_serial, _arg_cdmaSms) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#writeSmsToRuim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_writeSmsToRuim(_arg_serial, _arg_cdmaSms, _aidl_reply))
              }
              fn r#writeSmsToSim(&self, _arg_serial: i32, _arg_smsWriteArgs: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_12_SmsWriteArgs) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_writeSmsToSim(_arg_serial, _arg_smsWriteArgs) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#writeSmsToSim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_writeSmsToSim(_arg_serial, _arg_smsWriteArgs, _aidl_reply))
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
            impl IRadioMessaging for binder::binder_impl::Binder<BnRadioMessaging> {
              fn r#acknowledgeIncomingGsmSmsWithPdu(&self, _arg_serial: i32, _arg_success: bool, _arg_ackPdu: &str) -> binder::Result<()> { self.0.r#acknowledgeIncomingGsmSmsWithPdu(_arg_serial, _arg_success, _arg_ackPdu) }
              fn r#acknowledgeLastIncomingCdmaSms(&self, _arg_serial: i32, _arg_smsAck: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_10_CdmaSmsAck) -> binder::Result<()> { self.0.r#acknowledgeLastIncomingCdmaSms(_arg_serial, _arg_smsAck) }
              fn r#acknowledgeLastIncomingGsmSms(&self, _arg_serial: i32, _arg_success: bool, _arg_cause: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_SmsAcknowledgeFailCause) -> binder::Result<()> { self.0.r#acknowledgeLastIncomingGsmSms(_arg_serial, _arg_success, _arg_cause) }
              fn r#deleteSmsOnRuim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<()> { self.0.r#deleteSmsOnRuim(_arg_serial, _arg_index) }
              fn r#deleteSmsOnSim(&self, _arg_serial: i32, _arg_index: i32) -> binder::Result<()> { self.0.r#deleteSmsOnSim(_arg_serial, _arg_index) }
              fn r#getCdmaBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getCdmaBroadcastConfig(_arg_serial) }
              fn r#getGsmBroadcastConfig(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getGsmBroadcastConfig(_arg_serial) }
              fn r#getSmscAddress(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getSmscAddress(_arg_serial) }
              fn r#reportSmsMemoryStatus(&self, _arg_serial: i32, _arg_available: bool) -> binder::Result<()> { self.0.r#reportSmsMemoryStatus(_arg_serial, _arg_available) }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> { self.0.r#responseAcknowledgement() }
              fn r#sendCdmaSms(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()> { self.0.r#sendCdmaSms(_arg_serial, _arg_sms) }
              fn r#sendCdmaSmsExpectMore(&self, _arg_serial: i32, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()> { self.0.r#sendCdmaSmsExpectMore(_arg_serial, _arg_sms) }
              fn r#sendImsSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_ImsSmsMessage) -> binder::Result<()> { self.0.r#sendImsSms(_arg_serial, _arg_message) }
              fn r#sendSms(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<()> { self.0.r#sendSms(_arg_serial, _arg_message) }
              fn r#sendSmsExpectMore(&self, _arg_serial: i32, _arg_message: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage) -> binder::Result<()> { self.0.r#sendSmsExpectMore(_arg_serial, _arg_message) }
              fn r#setCdmaBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<()> { self.0.r#setCdmaBroadcastActivation(_arg_serial, _arg_activate) }
              fn r#setCdmaBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<()> { self.0.r#setCdmaBroadcastConfig(_arg_serial, _arg_configInfo) }
              fn r#setGsmBroadcastActivation(&self, _arg_serial: i32, _arg_activate: bool) -> binder::Result<()> { self.0.r#setGsmBroadcastActivation(_arg_serial, _arg_activate) }
              fn r#setGsmBroadcastConfig(&self, _arg_serial: i32, _arg_configInfo: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<()> { self.0.r#setGsmBroadcastConfig(_arg_serial, _arg_configInfo) }
              fn r#setResponseFunctions(&self, _arg_radioMessagingResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_IRadioMessagingResponse>, _arg_radioMessagingIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_IRadioMessagingIndication>) -> binder::Result<()> { self.0.r#setResponseFunctions(_arg_radioMessagingResponse, _arg_radioMessagingIndication) }
              fn r#setSmscAddress(&self, _arg_serial: i32, _arg_smsc: &str) -> binder::Result<()> { self.0.r#setSmscAddress(_arg_serial, _arg_smsc) }
              fn r#writeSmsToRuim(&self, _arg_serial: i32, _arg_cdmaSms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_16_CdmaSmsWriteArgs) -> binder::Result<()> { self.0.r#writeSmsToRuim(_arg_serial, _arg_cdmaSms) }
              fn r#writeSmsToSim(&self, _arg_serial: i32, _arg_smsWriteArgs: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_12_SmsWriteArgs) -> binder::Result<()> { self.0.r#writeSmsToSim(_arg_serial, _arg_smsWriteArgs) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioMessaging, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#acknowledgeIncomingGsmSmsWithPdu => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_success: bool = _aidl_data.read()?;
                  let _arg_ackPdu: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acknowledgeIncomingGsmSmsWithPdu(_arg_serial, _arg_success, &_arg_ackPdu);
                  Ok(())
                }
                transactions::r#acknowledgeLastIncomingCdmaSms => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_smsAck: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_10_CdmaSmsAck = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acknowledgeLastIncomingCdmaSms(_arg_serial, &_arg_smsAck);
                  Ok(())
                }
                transactions::r#acknowledgeLastIncomingGsmSms => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_success: bool = _aidl_data.read()?;
                  let _arg_cause: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_SmsAcknowledgeFailCause = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acknowledgeLastIncomingGsmSms(_arg_serial, _arg_success, _arg_cause);
                  Ok(())
                }
                transactions::r#deleteSmsOnRuim => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_index: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#deleteSmsOnRuim(_arg_serial, _arg_index);
                  Ok(())
                }
                transactions::r#deleteSmsOnSim => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_index: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#deleteSmsOnSim(_arg_serial, _arg_index);
                  Ok(())
                }
                transactions::r#getCdmaBroadcastConfig => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCdmaBroadcastConfig(_arg_serial);
                  Ok(())
                }
                transactions::r#getGsmBroadcastConfig => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getGsmBroadcastConfig(_arg_serial);
                  Ok(())
                }
                transactions::r#getSmscAddress => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSmscAddress(_arg_serial);
                  Ok(())
                }
                transactions::r#reportSmsMemoryStatus => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_available: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#reportSmsMemoryStatus(_arg_serial, _arg_available);
                  Ok(())
                }
                transactions::r#responseAcknowledgement => {
                  let _aidl_return = _aidl_service.r#responseAcknowledgement();
                  Ok(())
                }
                transactions::r#sendCdmaSms => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_sms: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendCdmaSms(_arg_serial, &_arg_sms);
                  Ok(())
                }
                transactions::r#sendCdmaSmsExpectMore => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_sms: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendCdmaSmsExpectMore(_arg_serial, &_arg_sms);
                  Ok(())
                }
                transactions::r#sendImsSms => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_message: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_ImsSmsMessage = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendImsSms(_arg_serial, &_arg_message);
                  Ok(())
                }
                transactions::r#sendSms => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_message: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendSms(_arg_serial, &_arg_message);
                  Ok(())
                }
                transactions::r#sendSmsExpectMore => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_message: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendSmsExpectMore(_arg_serial, &_arg_message);
                  Ok(())
                }
                transactions::r#setCdmaBroadcastActivation => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_activate: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCdmaBroadcastActivation(_arg_serial, _arg_activate);
                  Ok(())
                }
                transactions::r#setCdmaBroadcastConfig => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_configInfo: Vec<crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCdmaBroadcastConfig(_arg_serial, &_arg_configInfo);
                  Ok(())
                }
                transactions::r#setGsmBroadcastActivation => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_activate: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setGsmBroadcastActivation(_arg_serial, _arg_activate);
                  Ok(())
                }
                transactions::r#setGsmBroadcastConfig => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_configInfo: Vec<crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setGsmBroadcastConfig(_arg_serial, &_arg_configInfo);
                  Ok(())
                }
                transactions::r#setResponseFunctions => {
                  let _arg_radioMessagingResponse: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_23_IRadioMessagingResponse> = _aidl_data.read()?;
                  let _arg_radioMessagingIndication: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_IRadioMessagingIndication> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setResponseFunctions(&_arg_radioMessagingResponse, &_arg_radioMessagingIndication);
                  Ok(())
                }
                transactions::r#setSmscAddress => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_smsc: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setSmscAddress(_arg_serial, &_arg_smsc);
                  Ok(())
                }
                transactions::r#writeSmsToRuim => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_cdmaSms: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_16_CdmaSmsWriteArgs = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#writeSmsToRuim(_arg_serial, &_arg_cdmaSms);
                  Ok(())
                }
                transactions::r#writeSmsToSim => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_smsWriteArgs: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_12_SmsWriteArgs = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#writeSmsToSim(_arg_serial, &_arg_smsWriteArgs);
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
             pub use super::r#IRadioMessaging as _7_android_8_hardware_5_radio_9_messaging_15_IRadioMessaging;
            }
          }
          pub mod IRadioMessagingIndication {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioMessagingIndication["android.hardware.radio.messaging.IRadioMessagingIndication"] {
                native: BnRadioMessagingIndication(on_transact),
                proxy: BpRadioMessagingIndication {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioMessagingIndicationAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioMessagingIndication: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.messaging.IRadioMessagingIndication" }
              fn r#cdmaNewSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_msg: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()>;
              fn r#cdmaRuimSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#newBroadcastSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_data: &[u8]) -> binder::Result<()>;
              fn r#newSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<()>;
              fn r#newSmsOnSim(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_recordNumber: i32) -> binder::Result<()>;
              fn r#newSmsStatusReport(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<()>;
              fn r#simSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioMessagingIndicationDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioMessagingIndicationDefaultRef) -> IRadioMessagingIndicationDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioMessagingIndicationAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.messaging.IRadioMessagingIndication" }
              fn r#cdmaNewSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_msg: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> std::future::Ready<binder::Result<()>>;
              fn r#cdmaRuimSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#newBroadcastSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_data: &[u8]) -> std::future::Ready<binder::Result<()>>;
              fn r#newSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> std::future::Ready<binder::Result<()>>;
              fn r#newSmsOnSim(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_recordNumber: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#newSmsStatusReport(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> std::future::Ready<binder::Result<()>>;
              fn r#simSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioMessagingIndicationAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.messaging.IRadioMessagingIndication" }
              async fn r#cdmaNewSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_msg: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()>;
              async fn r#cdmaRuimSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              async fn r#newBroadcastSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_data: &[u8]) -> binder::Result<()>;
              async fn r#newSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<()>;
              async fn r#newSmsOnSim(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_recordNumber: i32) -> binder::Result<()>;
              async fn r#newSmsStatusReport(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<()>;
              async fn r#simSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
            }
            impl BnRadioMessagingIndication {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioMessagingIndication>
              where
                T: IRadioMessagingIndicationAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioMessagingIndication for Wrapper<T, R>
                where
                  T: IRadioMessagingIndicationAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#cdmaNewSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_msg: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#cdmaNewSms(_arg_type, _arg_msg))
                  }
                  fn r#cdmaRuimSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#cdmaRuimSmsStorageFull(_arg_type))
                  }
                  fn r#newBroadcastSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_data: &[u8]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#newBroadcastSms(_arg_type, _arg_data))
                  }
                  fn r#newSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#newSms(_arg_type, _arg_pdu))
                  }
                  fn r#newSmsOnSim(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_recordNumber: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#newSmsOnSim(_arg_type, _arg_recordNumber))
                  }
                  fn r#newSmsStatusReport(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#newSmsStatusReport(_arg_type, _arg_pdu))
                  }
                  fn r#simSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#simSmsStorageFull(_arg_type))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioMessagingIndicationDefault: Send + Sync {
              fn r#cdmaNewSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_msg: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#cdmaRuimSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#newBroadcastSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_data: &[u8]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#newSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#newSmsOnSim(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_recordNumber: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#newSmsStatusReport(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#simSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#cdmaNewSms: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#cdmaRuimSmsStorageFull: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#newBroadcastSms: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#newSms: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#newSmsOnSim: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#newSmsStatusReport: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#simSmsStorageFull: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioMessagingIndicationDefaultRef = Option<std::sync::Arc<dyn IRadioMessagingIndicationDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioMessagingIndicationDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "5237ec5f500627b6b844b155e356e603157f9ba6";
            impl BpRadioMessagingIndication {
              fn build_parcel_cdmaNewSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_msg: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_msg)?;
                Ok(aidl_data)
              }
              fn read_response_cdmaNewSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_msg: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#cdmaNewSms(_arg_type, _arg_msg);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_cdmaRuimSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_cdmaRuimSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#cdmaRuimSmsStorageFull(_arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_newBroadcastSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_data: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_data)?;
                Ok(aidl_data)
              }
              fn read_response_newBroadcastSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_data: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#newBroadcastSms(_arg_type, _arg_data);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_newSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_pdu)?;
                Ok(aidl_data)
              }
              fn read_response_newSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#newSms(_arg_type, _arg_pdu);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_newSmsOnSim(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_recordNumber: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_recordNumber)?;
                Ok(aidl_data)
              }
              fn read_response_newSmsOnSim(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_recordNumber: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#newSmsOnSim(_arg_type, _arg_recordNumber);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_newSmsStatusReport(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_pdu)?;
                Ok(aidl_data)
              }
              fn read_response_newSmsStatusReport(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#newSmsStatusReport(_arg_type, _arg_pdu);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_simSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_simSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#simSmsStorageFull(_arg_type);
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
            impl IRadioMessagingIndication for BpRadioMessagingIndication {
              fn r#cdmaNewSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_msg: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_cdmaNewSms(_arg_type, _arg_msg)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaNewSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_cdmaNewSms(_arg_type, _arg_msg, _aidl_reply)
              }
              fn r#cdmaRuimSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_cdmaRuimSmsStorageFull(_arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaRuimSmsStorageFull, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_cdmaRuimSmsStorageFull(_arg_type, _aidl_reply)
              }
              fn r#newBroadcastSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_data: &[u8]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_newBroadcastSms(_arg_type, _arg_data)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#newBroadcastSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_newBroadcastSms(_arg_type, _arg_data, _aidl_reply)
              }
              fn r#newSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_newSms(_arg_type, _arg_pdu)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#newSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_newSms(_arg_type, _arg_pdu, _aidl_reply)
              }
              fn r#newSmsOnSim(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_recordNumber: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_newSmsOnSim(_arg_type, _arg_recordNumber)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#newSmsOnSim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_newSmsOnSim(_arg_type, _arg_recordNumber, _aidl_reply)
              }
              fn r#newSmsStatusReport(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_newSmsStatusReport(_arg_type, _arg_pdu)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#newSmsStatusReport, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_newSmsStatusReport(_arg_type, _arg_pdu, _aidl_reply)
              }
              fn r#simSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_simSmsStorageFull(_arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#simSmsStorageFull, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_simSmsStorageFull(_arg_type, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioMessagingIndicationAsync<P> for BpRadioMessagingIndication {
              fn r#cdmaNewSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_msg: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_cdmaNewSms(_arg_type, _arg_msg) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaNewSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_cdmaNewSms(_arg_type, _arg_msg, _aidl_reply))
              }
              fn r#cdmaRuimSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_cdmaRuimSmsStorageFull(_arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaRuimSmsStorageFull, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_cdmaRuimSmsStorageFull(_arg_type, _aidl_reply))
              }
              fn r#newBroadcastSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_data: &[u8]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_newBroadcastSms(_arg_type, _arg_data) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#newBroadcastSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_newBroadcastSms(_arg_type, _arg_data, _aidl_reply))
              }
              fn r#newSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_newSms(_arg_type, _arg_pdu) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#newSms, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_newSms(_arg_type, _arg_pdu, _aidl_reply))
              }
              fn r#newSmsOnSim(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_recordNumber: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_newSmsOnSim(_arg_type, _arg_recordNumber) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#newSmsOnSim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_newSmsOnSim(_arg_type, _arg_recordNumber, _aidl_reply))
              }
              fn r#newSmsStatusReport(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_newSmsStatusReport(_arg_type, _arg_pdu) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#newSmsStatusReport, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_newSmsStatusReport(_arg_type, _arg_pdu, _aidl_reply))
              }
              fn r#simSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_simSmsStorageFull(_arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#simSmsStorageFull, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_simSmsStorageFull(_arg_type, _aidl_reply))
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
            impl IRadioMessagingIndication for binder::binder_impl::Binder<BnRadioMessagingIndication> {
              fn r#cdmaNewSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_msg: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage) -> binder::Result<()> { self.0.r#cdmaNewSms(_arg_type, _arg_msg) }
              fn r#cdmaRuimSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#cdmaRuimSmsStorageFull(_arg_type) }
              fn r#newBroadcastSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_data: &[u8]) -> binder::Result<()> { self.0.r#newBroadcastSms(_arg_type, _arg_data) }
              fn r#newSms(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<()> { self.0.r#newSms(_arg_type, _arg_pdu) }
              fn r#newSmsOnSim(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_recordNumber: i32) -> binder::Result<()> { self.0.r#newSmsOnSim(_arg_type, _arg_recordNumber) }
              fn r#newSmsStatusReport(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pdu: &[u8]) -> binder::Result<()> { self.0.r#newSmsStatusReport(_arg_type, _arg_pdu) }
              fn r#simSmsStorageFull(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#simSmsStorageFull(_arg_type) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioMessagingIndication, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#cdmaNewSms => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_msg: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#cdmaNewSms(_arg_type, &_arg_msg);
                  Ok(())
                }
                transactions::r#cdmaRuimSmsStorageFull => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#cdmaRuimSmsStorageFull(_arg_type);
                  Ok(())
                }
                transactions::r#newBroadcastSms => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_data: Vec<u8> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#newBroadcastSms(_arg_type, &_arg_data);
                  Ok(())
                }
                transactions::r#newSms => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_pdu: Vec<u8> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#newSms(_arg_type, &_arg_pdu);
                  Ok(())
                }
                transactions::r#newSmsOnSim => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_recordNumber: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#newSmsOnSim(_arg_type, _arg_recordNumber);
                  Ok(())
                }
                transactions::r#newSmsStatusReport => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_pdu: Vec<u8> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#newSmsStatusReport(_arg_type, &_arg_pdu);
                  Ok(())
                }
                transactions::r#simSmsStorageFull => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#simSmsStorageFull(_arg_type);
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
             pub use super::r#IRadioMessagingIndication as _7_android_8_hardware_5_radio_9_messaging_25_IRadioMessagingIndication;
            }
          }
          pub mod IRadioMessagingResponse {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioMessagingResponse["android.hardware.radio.messaging.IRadioMessagingResponse"] {
                native: BnRadioMessagingResponse(on_transact),
                proxy: BpRadioMessagingResponse {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioMessagingResponseAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioMessagingResponse: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.messaging.IRadioMessagingResponse" }
              fn r#acknowledgeIncomingGsmSmsWithPduResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#acknowledgeLastIncomingCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#acknowledgeLastIncomingGsmSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#deleteSmsOnRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#deleteSmsOnSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#getCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<()>;
              fn r#getGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<()>;
              fn r#getSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_smsc: &str) -> binder::Result<()>;
              fn r#reportSmsMemoryStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#sendCdmaSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()>;
              fn r#sendCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()>;
              fn r#sendImsSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()>;
              fn r#sendSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()>;
              fn r#sendSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()>;
              fn r#setCdmaBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setGsmBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#writeSmsToRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<()>;
              fn r#writeSmsToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioMessagingResponseDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioMessagingResponseDefaultRef) -> IRadioMessagingResponseDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioMessagingResponseAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.messaging.IRadioMessagingResponse" }
              fn r#acknowledgeIncomingGsmSmsWithPduResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#acknowledgeLastIncomingCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#acknowledgeLastIncomingGsmSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#deleteSmsOnRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#deleteSmsOnSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#getGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#getSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_smsc: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#reportSmsMemoryStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#sendCdmaSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> std::future::Ready<binder::Result<()>>;
              fn r#sendCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> std::future::Ready<binder::Result<()>>;
              fn r#sendImsSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> std::future::Ready<binder::Result<()>>;
              fn r#sendSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> std::future::Ready<binder::Result<()>>;
              fn r#sendSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> std::future::Ready<binder::Result<()>>;
              fn r#setCdmaBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setGsmBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#writeSmsToRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#writeSmsToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioMessagingResponseAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.messaging.IRadioMessagingResponse" }
              async fn r#acknowledgeIncomingGsmSmsWithPduResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#acknowledgeLastIncomingCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#acknowledgeLastIncomingGsmSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#deleteSmsOnRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#deleteSmsOnSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#getCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<()>;
              async fn r#getGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<()>;
              async fn r#getSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_smsc: &str) -> binder::Result<()>;
              async fn r#reportSmsMemoryStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#sendCdmaSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()>;
              async fn r#sendCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()>;
              async fn r#sendImsSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()>;
              async fn r#sendSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()>;
              async fn r#sendSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()>;
              async fn r#setCdmaBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setGsmBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#writeSmsToRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<()>;
              async fn r#writeSmsToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<()>;
            }
            impl BnRadioMessagingResponse {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioMessagingResponse>
              where
                T: IRadioMessagingResponseAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioMessagingResponse for Wrapper<T, R>
                where
                  T: IRadioMessagingResponseAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#acknowledgeIncomingGsmSmsWithPduResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acknowledgeIncomingGsmSmsWithPduResponse(_arg_info))
                  }
                  fn r#acknowledgeLastIncomingCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acknowledgeLastIncomingCdmaSmsResponse(_arg_info))
                  }
                  fn r#acknowledgeLastIncomingGsmSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acknowledgeLastIncomingGsmSmsResponse(_arg_info))
                  }
                  fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acknowledgeRequest(_arg_serial))
                  }
                  fn r#deleteSmsOnRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#deleteSmsOnRuimResponse(_arg_info))
                  }
                  fn r#deleteSmsOnSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#deleteSmsOnSimResponse(_arg_info))
                  }
                  fn r#getCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCdmaBroadcastConfigResponse(_arg_info, _arg_configs))
                  }
                  fn r#getGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getGsmBroadcastConfigResponse(_arg_info, _arg_configs))
                  }
                  fn r#getSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_smsc: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSmscAddressResponse(_arg_info, _arg_smsc))
                  }
                  fn r#reportSmsMemoryStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#reportSmsMemoryStatusResponse(_arg_info))
                  }
                  fn r#sendCdmaSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendCdmaSmsExpectMoreResponse(_arg_info, _arg_sms))
                  }
                  fn r#sendCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendCdmaSmsResponse(_arg_info, _arg_sms))
                  }
                  fn r#sendImsSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendImsSmsResponse(_arg_info, _arg_sms))
                  }
                  fn r#sendSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendSmsExpectMoreResponse(_arg_info, _arg_sms))
                  }
                  fn r#sendSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendSmsResponse(_arg_info, _arg_sms))
                  }
                  fn r#setCdmaBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCdmaBroadcastActivationResponse(_arg_info))
                  }
                  fn r#setCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCdmaBroadcastConfigResponse(_arg_info))
                  }
                  fn r#setGsmBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setGsmBroadcastActivationResponse(_arg_info))
                  }
                  fn r#setGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setGsmBroadcastConfigResponse(_arg_info))
                  }
                  fn r#setSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setSmscAddressResponse(_arg_info))
                  }
                  fn r#writeSmsToRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#writeSmsToRuimResponse(_arg_info, _arg_index))
                  }
                  fn r#writeSmsToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#writeSmsToSimResponse(_arg_info, _arg_index))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioMessagingResponseDefault: Send + Sync {
              fn r#acknowledgeIncomingGsmSmsWithPduResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#acknowledgeLastIncomingCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#acknowledgeLastIncomingGsmSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#deleteSmsOnRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#deleteSmsOnSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_smsc: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#reportSmsMemoryStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendCdmaSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendImsSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCdmaBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setGsmBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#writeSmsToRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#writeSmsToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#acknowledgeIncomingGsmSmsWithPduResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#acknowledgeLastIncomingCdmaSmsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#acknowledgeLastIncomingGsmSmsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#acknowledgeRequest: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#deleteSmsOnRuimResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#deleteSmsOnSimResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#getCdmaBroadcastConfigResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#getGsmBroadcastConfigResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#getSmscAddressResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#reportSmsMemoryStatusResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#sendCdmaSmsExpectMoreResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#sendCdmaSmsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#sendImsSmsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#sendSmsExpectMoreResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#sendSmsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#setCdmaBroadcastActivationResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#setCdmaBroadcastConfigResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
              pub const r#setGsmBroadcastActivationResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
              pub const r#setGsmBroadcastConfigResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
              pub const r#setSmscAddressResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
              pub const r#writeSmsToRuimResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 20;
              pub const r#writeSmsToSimResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 21;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioMessagingResponseDefaultRef = Option<std::sync::Arc<dyn IRadioMessagingResponseDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioMessagingResponseDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "5237ec5f500627b6b844b155e356e603157f9ba6";
            impl BpRadioMessagingResponse {
              fn build_parcel_acknowledgeIncomingGsmSmsWithPduResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_acknowledgeIncomingGsmSmsWithPduResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#acknowledgeIncomingGsmSmsWithPduResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_acknowledgeLastIncomingCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_acknowledgeLastIncomingCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#acknowledgeLastIncomingCdmaSmsResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_acknowledgeLastIncomingGsmSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_acknowledgeLastIncomingGsmSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#acknowledgeLastIncomingGsmSmsResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_acknowledgeRequest(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#acknowledgeRequest(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_deleteSmsOnRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_deleteSmsOnRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#deleteSmsOnRuimResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_deleteSmsOnSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_deleteSmsOnSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#deleteSmsOnSimResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_configs)?;
                Ok(aidl_data)
              }
              fn read_response_getCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCdmaBroadcastConfigResponse(_arg_info, _arg_configs);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_configs)?;
                Ok(aidl_data)
              }
              fn read_response_getGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getGsmBroadcastConfigResponse(_arg_info, _arg_configs);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_smsc: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_smsc)?;
                Ok(aidl_data)
              }
              fn read_response_getSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_smsc: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSmscAddressResponse(_arg_info, _arg_smsc);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_reportSmsMemoryStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_reportSmsMemoryStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#reportSmsMemoryStatusResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendCdmaSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_sms)?;
                Ok(aidl_data)
              }
              fn read_response_sendCdmaSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendCdmaSmsExpectMoreResponse(_arg_info, _arg_sms);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_sms)?;
                Ok(aidl_data)
              }
              fn read_response_sendCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendCdmaSmsResponse(_arg_info, _arg_sms);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendImsSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_sms)?;
                Ok(aidl_data)
              }
              fn read_response_sendImsSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendImsSmsResponse(_arg_info, _arg_sms);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_sms)?;
                Ok(aidl_data)
              }
              fn read_response_sendSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendSmsExpectMoreResponse(_arg_info, _arg_sms);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_sms)?;
                Ok(aidl_data)
              }
              fn read_response_sendSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendSmsResponse(_arg_info, _arg_sms);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCdmaBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setCdmaBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCdmaBroadcastActivationResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCdmaBroadcastConfigResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setGsmBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setGsmBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setGsmBroadcastActivationResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setGsmBroadcastConfigResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setSmscAddressResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_writeSmsToRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_index)?;
                Ok(aidl_data)
              }
              fn read_response_writeSmsToRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#writeSmsToRuimResponse(_arg_info, _arg_index);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_writeSmsToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_index)?;
                Ok(aidl_data)
              }
              fn read_response_writeSmsToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioMessagingResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#writeSmsToSimResponse(_arg_info, _arg_index);
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
            impl IRadioMessagingResponse for BpRadioMessagingResponse {
              fn r#acknowledgeIncomingGsmSmsWithPduResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acknowledgeIncomingGsmSmsWithPduResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeIncomingGsmSmsWithPduResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acknowledgeIncomingGsmSmsWithPduResponse(_arg_info, _aidl_reply)
              }
              fn r#acknowledgeLastIncomingCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acknowledgeLastIncomingCdmaSmsResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeLastIncomingCdmaSmsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acknowledgeLastIncomingCdmaSmsResponse(_arg_info, _aidl_reply)
              }
              fn r#acknowledgeLastIncomingGsmSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acknowledgeLastIncomingGsmSmsResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeLastIncomingGsmSmsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acknowledgeLastIncomingGsmSmsResponse(_arg_info, _aidl_reply)
              }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acknowledgeRequest(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeRequest, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acknowledgeRequest(_arg_serial, _aidl_reply)
              }
              fn r#deleteSmsOnRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_deleteSmsOnRuimResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#deleteSmsOnRuimResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_deleteSmsOnRuimResponse(_arg_info, _aidl_reply)
              }
              fn r#deleteSmsOnSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_deleteSmsOnSimResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#deleteSmsOnSimResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_deleteSmsOnSimResponse(_arg_info, _aidl_reply)
              }
              fn r#getCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCdmaBroadcastConfigResponse(_arg_info, _arg_configs)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaBroadcastConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCdmaBroadcastConfigResponse(_arg_info, _arg_configs, _aidl_reply)
              }
              fn r#getGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getGsmBroadcastConfigResponse(_arg_info, _arg_configs)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getGsmBroadcastConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getGsmBroadcastConfigResponse(_arg_info, _arg_configs, _aidl_reply)
              }
              fn r#getSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_smsc: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSmscAddressResponse(_arg_info, _arg_smsc)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSmscAddressResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSmscAddressResponse(_arg_info, _arg_smsc, _aidl_reply)
              }
              fn r#reportSmsMemoryStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_reportSmsMemoryStatusResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#reportSmsMemoryStatusResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_reportSmsMemoryStatusResponse(_arg_info, _aidl_reply)
              }
              fn r#sendCdmaSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendCdmaSmsExpectMoreResponse(_arg_info, _arg_sms)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendCdmaSmsExpectMoreResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendCdmaSmsExpectMoreResponse(_arg_info, _arg_sms, _aidl_reply)
              }
              fn r#sendCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendCdmaSmsResponse(_arg_info, _arg_sms)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendCdmaSmsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendCdmaSmsResponse(_arg_info, _arg_sms, _aidl_reply)
              }
              fn r#sendImsSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendImsSmsResponse(_arg_info, _arg_sms)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendImsSmsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendImsSmsResponse(_arg_info, _arg_sms, _aidl_reply)
              }
              fn r#sendSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendSmsExpectMoreResponse(_arg_info, _arg_sms)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendSmsExpectMoreResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendSmsExpectMoreResponse(_arg_info, _arg_sms, _aidl_reply)
              }
              fn r#sendSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendSmsResponse(_arg_info, _arg_sms)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendSmsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendSmsResponse(_arg_info, _arg_sms, _aidl_reply)
              }
              fn r#setCdmaBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCdmaBroadcastActivationResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaBroadcastActivationResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCdmaBroadcastActivationResponse(_arg_info, _aidl_reply)
              }
              fn r#setCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCdmaBroadcastConfigResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaBroadcastConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCdmaBroadcastConfigResponse(_arg_info, _aidl_reply)
              }
              fn r#setGsmBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setGsmBroadcastActivationResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setGsmBroadcastActivationResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setGsmBroadcastActivationResponse(_arg_info, _aidl_reply)
              }
              fn r#setGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setGsmBroadcastConfigResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setGsmBroadcastConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setGsmBroadcastConfigResponse(_arg_info, _aidl_reply)
              }
              fn r#setSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setSmscAddressResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSmscAddressResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setSmscAddressResponse(_arg_info, _aidl_reply)
              }
              fn r#writeSmsToRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_writeSmsToRuimResponse(_arg_info, _arg_index)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#writeSmsToRuimResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_writeSmsToRuimResponse(_arg_info, _arg_index, _aidl_reply)
              }
              fn r#writeSmsToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_writeSmsToSimResponse(_arg_info, _arg_index)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#writeSmsToSimResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_writeSmsToSimResponse(_arg_info, _arg_index, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioMessagingResponseAsync<P> for BpRadioMessagingResponse {
              fn r#acknowledgeIncomingGsmSmsWithPduResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acknowledgeIncomingGsmSmsWithPduResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeIncomingGsmSmsWithPduResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acknowledgeIncomingGsmSmsWithPduResponse(_arg_info, _aidl_reply))
              }
              fn r#acknowledgeLastIncomingCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acknowledgeLastIncomingCdmaSmsResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeLastIncomingCdmaSmsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acknowledgeLastIncomingCdmaSmsResponse(_arg_info, _aidl_reply))
              }
              fn r#acknowledgeLastIncomingGsmSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acknowledgeLastIncomingGsmSmsResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeLastIncomingGsmSmsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acknowledgeLastIncomingGsmSmsResponse(_arg_info, _aidl_reply))
              }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acknowledgeRequest(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeRequest, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acknowledgeRequest(_arg_serial, _aidl_reply))
              }
              fn r#deleteSmsOnRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_deleteSmsOnRuimResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#deleteSmsOnRuimResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_deleteSmsOnRuimResponse(_arg_info, _aidl_reply))
              }
              fn r#deleteSmsOnSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_deleteSmsOnSimResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#deleteSmsOnSimResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_deleteSmsOnSimResponse(_arg_info, _aidl_reply))
              }
              fn r#getCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCdmaBroadcastConfigResponse(_arg_info, _arg_configs) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaBroadcastConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCdmaBroadcastConfigResponse(_arg_info, _arg_configs, _aidl_reply))
              }
              fn r#getGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getGsmBroadcastConfigResponse(_arg_info, _arg_configs) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getGsmBroadcastConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getGsmBroadcastConfigResponse(_arg_info, _arg_configs, _aidl_reply))
              }
              fn r#getSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_smsc: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSmscAddressResponse(_arg_info, _arg_smsc) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSmscAddressResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSmscAddressResponse(_arg_info, _arg_smsc, _aidl_reply))
              }
              fn r#reportSmsMemoryStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_reportSmsMemoryStatusResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#reportSmsMemoryStatusResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_reportSmsMemoryStatusResponse(_arg_info, _aidl_reply))
              }
              fn r#sendCdmaSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendCdmaSmsExpectMoreResponse(_arg_info, _arg_sms) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendCdmaSmsExpectMoreResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendCdmaSmsExpectMoreResponse(_arg_info, _arg_sms, _aidl_reply))
              }
              fn r#sendCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendCdmaSmsResponse(_arg_info, _arg_sms) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendCdmaSmsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendCdmaSmsResponse(_arg_info, _arg_sms, _aidl_reply))
              }
              fn r#sendImsSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendImsSmsResponse(_arg_info, _arg_sms) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendImsSmsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendImsSmsResponse(_arg_info, _arg_sms, _aidl_reply))
              }
              fn r#sendSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendSmsExpectMoreResponse(_arg_info, _arg_sms) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendSmsExpectMoreResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendSmsExpectMoreResponse(_arg_info, _arg_sms, _aidl_reply))
              }
              fn r#sendSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendSmsResponse(_arg_info, _arg_sms) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendSmsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendSmsResponse(_arg_info, _arg_sms, _aidl_reply))
              }
              fn r#setCdmaBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCdmaBroadcastActivationResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaBroadcastActivationResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCdmaBroadcastActivationResponse(_arg_info, _aidl_reply))
              }
              fn r#setCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCdmaBroadcastConfigResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaBroadcastConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCdmaBroadcastConfigResponse(_arg_info, _aidl_reply))
              }
              fn r#setGsmBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setGsmBroadcastActivationResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setGsmBroadcastActivationResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setGsmBroadcastActivationResponse(_arg_info, _aidl_reply))
              }
              fn r#setGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setGsmBroadcastConfigResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setGsmBroadcastConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setGsmBroadcastConfigResponse(_arg_info, _aidl_reply))
              }
              fn r#setSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setSmscAddressResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSmscAddressResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setSmscAddressResponse(_arg_info, _aidl_reply))
              }
              fn r#writeSmsToRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_writeSmsToRuimResponse(_arg_info, _arg_index) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#writeSmsToRuimResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_writeSmsToRuimResponse(_arg_info, _arg_index, _aidl_reply))
              }
              fn r#writeSmsToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_writeSmsToSimResponse(_arg_info, _arg_index) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#writeSmsToSimResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_writeSmsToSimResponse(_arg_info, _arg_index, _aidl_reply))
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
            impl IRadioMessagingResponse for binder::binder_impl::Binder<BnRadioMessagingResponse> {
              fn r#acknowledgeIncomingGsmSmsWithPduResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#acknowledgeIncomingGsmSmsWithPduResponse(_arg_info) }
              fn r#acknowledgeLastIncomingCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#acknowledgeLastIncomingCdmaSmsResponse(_arg_info) }
              fn r#acknowledgeLastIncomingGsmSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#acknowledgeLastIncomingGsmSmsResponse(_arg_info) }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#acknowledgeRequest(_arg_serial) }
              fn r#deleteSmsOnRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#deleteSmsOnRuimResponse(_arg_info) }
              fn r#deleteSmsOnSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#deleteSmsOnSimResponse(_arg_info) }
              fn r#getCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo]) -> binder::Result<()> { self.0.r#getCdmaBroadcastConfigResponse(_arg_info, _arg_configs) }
              fn r#getGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo]) -> binder::Result<()> { self.0.r#getGsmBroadcastConfigResponse(_arg_info, _arg_configs) }
              fn r#getSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_smsc: &str) -> binder::Result<()> { self.0.r#getSmscAddressResponse(_arg_info, _arg_smsc) }
              fn r#reportSmsMemoryStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#reportSmsMemoryStatusResponse(_arg_info) }
              fn r#sendCdmaSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> { self.0.r#sendCdmaSmsExpectMoreResponse(_arg_info, _arg_sms) }
              fn r#sendCdmaSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> { self.0.r#sendCdmaSmsResponse(_arg_info, _arg_sms) }
              fn r#sendImsSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> { self.0.r#sendImsSmsResponse(_arg_info, _arg_sms) }
              fn r#sendSmsExpectMoreResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> { self.0.r#sendSmsExpectMoreResponse(_arg_info, _arg_sms) }
              fn r#sendSmsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_sms: &crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult) -> binder::Result<()> { self.0.r#sendSmsResponse(_arg_info, _arg_sms) }
              fn r#setCdmaBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setCdmaBroadcastActivationResponse(_arg_info) }
              fn r#setCdmaBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setCdmaBroadcastConfigResponse(_arg_info) }
              fn r#setGsmBroadcastActivationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setGsmBroadcastActivationResponse(_arg_info) }
              fn r#setGsmBroadcastConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setGsmBroadcastConfigResponse(_arg_info) }
              fn r#setSmscAddressResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setSmscAddressResponse(_arg_info) }
              fn r#writeSmsToRuimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<()> { self.0.r#writeSmsToRuimResponse(_arg_info, _arg_index) }
              fn r#writeSmsToSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_index: i32) -> binder::Result<()> { self.0.r#writeSmsToSimResponse(_arg_info, _arg_index) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioMessagingResponse, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#acknowledgeIncomingGsmSmsWithPduResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acknowledgeIncomingGsmSmsWithPduResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#acknowledgeLastIncomingCdmaSmsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acknowledgeLastIncomingCdmaSmsResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#acknowledgeLastIncomingGsmSmsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acknowledgeLastIncomingGsmSmsResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#acknowledgeRequest => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acknowledgeRequest(_arg_serial);
                  Ok(())
                }
                transactions::r#deleteSmsOnRuimResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#deleteSmsOnRuimResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#deleteSmsOnSimResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#deleteSmsOnSimResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#getCdmaBroadcastConfigResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_configs: Vec<crate::mangled::_7_android_8_hardware_5_radio_9_messaging_26_CdmaBroadcastSmsConfigInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCdmaBroadcastConfigResponse(&_arg_info, &_arg_configs);
                  Ok(())
                }
                transactions::r#getGsmBroadcastConfigResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_configs: Vec<crate::mangled::_7_android_8_hardware_5_radio_9_messaging_25_GsmBroadcastSmsConfigInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getGsmBroadcastConfigResponse(&_arg_info, &_arg_configs);
                  Ok(())
                }
                transactions::r#getSmscAddressResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_smsc: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSmscAddressResponse(&_arg_info, &_arg_smsc);
                  Ok(())
                }
                transactions::r#reportSmsMemoryStatusResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#reportSmsMemoryStatusResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#sendCdmaSmsExpectMoreResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_sms: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendCdmaSmsExpectMoreResponse(&_arg_info, &_arg_sms);
                  Ok(())
                }
                transactions::r#sendCdmaSmsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_sms: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendCdmaSmsResponse(&_arg_info, &_arg_sms);
                  Ok(())
                }
                transactions::r#sendImsSmsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_sms: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendImsSmsResponse(&_arg_info, &_arg_sms);
                  Ok(())
                }
                transactions::r#sendSmsExpectMoreResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_sms: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendSmsExpectMoreResponse(&_arg_info, &_arg_sms);
                  Ok(())
                }
                transactions::r#sendSmsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_sms: crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendSmsResponse(&_arg_info, &_arg_sms);
                  Ok(())
                }
                transactions::r#setCdmaBroadcastActivationResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCdmaBroadcastActivationResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setCdmaBroadcastConfigResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCdmaBroadcastConfigResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setGsmBroadcastActivationResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setGsmBroadcastActivationResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setGsmBroadcastConfigResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setGsmBroadcastConfigResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setSmscAddressResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setSmscAddressResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#writeSmsToRuimResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_index: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#writeSmsToRuimResponse(&_arg_info, _arg_index);
                  Ok(())
                }
                transactions::r#writeSmsToSimResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_index: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#writeSmsToSimResponse(&_arg_info, _arg_index);
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
             pub use super::r#IRadioMessagingResponse as _7_android_8_hardware_5_radio_9_messaging_23_IRadioMessagingResponse;
            }
          }
          pub mod ImsSmsMessage {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#ImsSmsMessage {
              pub r#tech: crate::mangled::_7_android_8_hardware_5_radio_21_RadioTechnologyFamily,
              pub r#retry: bool,
              pub r#messageRef: i32,
              pub r#cdmaMessage: Vec<crate::mangled::_7_android_8_hardware_5_radio_9_messaging_14_CdmaSmsMessage>,
              pub r#gsmMessage: Vec<crate::mangled::_7_android_8_hardware_5_radio_9_messaging_13_GsmSmsMessage>,
            }
            impl Default for r#ImsSmsMessage {
              fn default() -> Self {
                Self {
                  r#tech: Default::default(),
                  r#retry: false,
                  r#messageRef: 0,
                  r#cdmaMessage: Default::default(),
                  r#gsmMessage: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#ImsSmsMessage {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#tech)?;
                  subparcel.write(&self.r#retry)?;
                  subparcel.write(&self.r#messageRef)?;
                  subparcel.write(&self.r#cdmaMessage)?;
                  subparcel.write(&self.r#gsmMessage)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#tech = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#retry = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#messageRef = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cdmaMessage = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#gsmMessage = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ImsSmsMessage);
            binder::impl_deserialize_for_parcelable!(r#ImsSmsMessage);
            impl binder::binder_impl::ParcelableMetadata for r#ImsSmsMessage {
              fn get_descriptor() -> &'static str { "android.hardware.radio.messaging.ImsSmsMessage" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ImsSmsMessage as _7_android_8_hardware_5_radio_9_messaging_13_ImsSmsMessage;
            }
          }
          pub mod SendSmsResult {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#SendSmsResult {
              pub r#messageRef: i32,
              pub r#ackPDU: String,
              pub r#errorCode: i32,
            }
            impl Default for r#SendSmsResult {
              fn default() -> Self {
                Self {
                  r#messageRef: 0,
                  r#ackPDU: Default::default(),
                  r#errorCode: 0,
                }
              }
            }
            impl binder::Parcelable for r#SendSmsResult {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#messageRef)?;
                  subparcel.write(&self.r#ackPDU)?;
                  subparcel.write(&self.r#errorCode)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#messageRef = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#ackPDU = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#errorCode = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SendSmsResult);
            binder::impl_deserialize_for_parcelable!(r#SendSmsResult);
            impl binder::binder_impl::ParcelableMetadata for r#SendSmsResult {
              fn get_descriptor() -> &'static str { "android.hardware.radio.messaging.SendSmsResult" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SendSmsResult as _7_android_8_hardware_5_radio_9_messaging_13_SendSmsResult;
            }
          }
          pub mod SmsAcknowledgeFailCause {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#SmsAcknowledgeFailCause : [i32; 2] {
                r#MEMORY_CAPACITY_EXCEEDED = 211,
                r#UNSPECIFIED_ERROR = 255,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#SmsAcknowledgeFailCause as _7_android_8_hardware_5_radio_9_messaging_23_SmsAcknowledgeFailCause;
            }
          }
          pub mod SmsWriteArgs {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#SmsWriteArgs {
              pub r#status: i32,
              pub r#pdu: String,
              pub r#smsc: String,
            }
            pub const r#STATUS_REC_UNREAD: i32 = 0;
            pub const r#STATUS_REC_READ: i32 = 1;
            pub const r#STATUS_STO_UNSENT: i32 = 2;
            pub const r#STATUS_STO_SENT: i32 = 3;
            impl Default for r#SmsWriteArgs {
              fn default() -> Self {
                Self {
                  r#status: 0,
                  r#pdu: Default::default(),
                  r#smsc: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#SmsWriteArgs {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#status)?;
                  subparcel.write(&self.r#pdu)?;
                  subparcel.write(&self.r#smsc)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#status = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#pdu = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#smsc = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SmsWriteArgs);
            binder::impl_deserialize_for_parcelable!(r#SmsWriteArgs);
            impl binder::binder_impl::ParcelableMetadata for r#SmsWriteArgs {
              fn get_descriptor() -> &'static str { "android.hardware.radio.messaging.SmsWriteArgs" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SmsWriteArgs as _7_android_8_hardware_5_radio_9_messaging_12_SmsWriteArgs;
            }
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::hardware::radio::messaging::CdmaBroadcastSmsConfigInfo::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::CdmaSmsAck::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::CdmaSmsAddress::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::CdmaSmsMessage::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::CdmaSmsSubaddress::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::CdmaSmsWriteArgs::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::GsmBroadcastSmsConfigInfo::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::GsmSmsMessage::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::IRadioMessaging::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::IRadioMessagingIndication::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::IRadioMessagingResponse::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::ImsSmsMessage::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::SendSmsResult::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::SmsAcknowledgeFailCause::mangled::*;
  pub use super::aidl::android::hardware::radio::messaging::SmsWriteArgs::mangled::*;
  pub(crate) use android_hardware_radio::mangled::*;
}
