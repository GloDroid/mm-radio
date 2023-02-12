#![feature(custom_inner_attributes)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
pub use binder;
pub mod aidl {
    pub mod android {
        pub mod hardware {
            pub mod radio {
                pub mod voice {
                    pub mod AudioQuality {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#AudioQuality : [i32; 10] {
                r#UNSPECIFIED = 0,
                r#AMR = 1,
                r#AMR_WB = 2,
                r#GSM_EFR = 3,
                r#GSM_FR = 4,
                r#GSM_HR = 5,
                r#EVRC = 6,
                r#EVRC_B = 7,
                r#EVRC_WB = 8,
                r#EVRC_NW = 9,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#AudioQuality as _7_android_8_hardware_5_radio_5_voice_12_AudioQuality;
            }
          }
                    pub mod Call {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#Call {
              pub r#state: i32,
              pub r#index: i32,
              pub r#toa: i32,
              pub r#isMpty: bool,
              pub r#isMT: bool,
              pub r#als: i8,
              pub r#isVoice: bool,
              pub r#isVoicePrivacy: bool,
              pub r#number: String,
              pub r#numberPresentation: i32,
              pub r#name: String,
              pub r#namePresentation: i32,
              pub r#uusInfo: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_UusInfo>,
              pub r#audioQuality: crate::mangled::_7_android_8_hardware_5_radio_5_voice_12_AudioQuality,
              pub r#forwardedNumber: String,
            }
            pub const r#PRESENTATION_ALLOWED: i32 = 0;
            pub const r#PRESENTATION_RESTRICTED: i32 = 1;
            pub const r#PRESENTATION_UNKNOWN: i32 = 2;
            pub const r#PRESENTATION_PAYPHONE: i32 = 3;
            pub const r#STATE_ACTIVE: i32 = 0;
            pub const r#STATE_HOLDING: i32 = 1;
            pub const r#STATE_DIALING: i32 = 2;
            pub const r#STATE_ALERTING: i32 = 3;
            pub const r#STATE_INCOMING: i32 = 4;
            pub const r#STATE_WAITING: i32 = 5;
            impl Default for r#Call {
              fn default() -> Self {
                Self {
                  r#state: 0,
                  r#index: 0,
                  r#toa: 0,
                  r#isMpty: false,
                  r#isMT: false,
                  r#als: 0,
                  r#isVoice: false,
                  r#isVoicePrivacy: false,
                  r#number: Default::default(),
                  r#numberPresentation: 0,
                  r#name: Default::default(),
                  r#namePresentation: 0,
                  r#uusInfo: Default::default(),
                  r#audioQuality: Default::default(),
                  r#forwardedNumber: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#Call {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#state)?;
                  subparcel.write(&self.r#index)?;
                  subparcel.write(&self.r#toa)?;
                  subparcel.write(&self.r#isMpty)?;
                  subparcel.write(&self.r#isMT)?;
                  subparcel.write(&self.r#als)?;
                  subparcel.write(&self.r#isVoice)?;
                  subparcel.write(&self.r#isVoicePrivacy)?;
                  subparcel.write(&self.r#number)?;
                  subparcel.write(&self.r#numberPresentation)?;
                  subparcel.write(&self.r#name)?;
                  subparcel.write(&self.r#namePresentation)?;
                  subparcel.write(&self.r#uusInfo)?;
                  subparcel.write(&self.r#audioQuality)?;
                  subparcel.write(&self.r#forwardedNumber)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#state = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#index = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#toa = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#isMpty = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#isMT = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#als = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#isVoice = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#isVoicePrivacy = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#number = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#numberPresentation = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#name = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#namePresentation = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#uusInfo = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#audioQuality = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#forwardedNumber = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Call);
            binder::impl_deserialize_for_parcelable!(r#Call);
            impl binder::binder_impl::ParcelableMetadata for r#Call {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.Call" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Call as _7_android_8_hardware_5_radio_5_voice_4_Call;
            }
          }
                    pub mod CallForwardInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CallForwardInfo {
              pub r#status: i32,
              pub r#reason: i32,
              pub r#serviceClass: i32,
              pub r#toa: i32,
              pub r#number: String,
              pub r#timeSeconds: i32,
            }
            pub const r#STATUS_DISABLE: i32 = 0;
            pub const r#STATUS_ENABLE: i32 = 1;
            pub const r#STATUS_INTERROGATE: i32 = 2;
            pub const r#STATUS_REGISTRATION: i32 = 3;
            pub const r#STATUS_ERASURE: i32 = 4;
            impl Default for r#CallForwardInfo {
              fn default() -> Self {
                Self {
                  r#status: 0,
                  r#reason: 0,
                  r#serviceClass: 0,
                  r#toa: 0,
                  r#number: Default::default(),
                  r#timeSeconds: 0,
                }
              }
            }
            impl binder::Parcelable for r#CallForwardInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#status)?;
                  subparcel.write(&self.r#reason)?;
                  subparcel.write(&self.r#serviceClass)?;
                  subparcel.write(&self.r#toa)?;
                  subparcel.write(&self.r#number)?;
                  subparcel.write(&self.r#timeSeconds)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#status = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#reason = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#serviceClass = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#toa = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#number = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#timeSeconds = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CallForwardInfo);
            binder::impl_deserialize_for_parcelable!(r#CallForwardInfo);
            impl binder::binder_impl::ParcelableMetadata for r#CallForwardInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.CallForwardInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CallForwardInfo as _7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo;
            }
          }
                    pub mod CdmaCallWaiting {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaCallWaiting {
              pub r#number: String,
              pub r#numberPresentation: i32,
              pub r#name: String,
              pub r#signalInfoRecord: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord,
              pub r#numberType: i32,
              pub r#numberPlan: i32,
            }
            pub const r#NUMBER_PLAN_UNKNOWN: i32 = 0;
            pub const r#NUMBER_PLAN_ISDN: i32 = 1;
            pub const r#NUMBER_PLAN_DATA: i32 = 3;
            pub const r#NUMBER_PLAN_TELEX: i32 = 4;
            pub const r#NUMBER_PLAN_NATIONAL: i32 = 8;
            pub const r#NUMBER_PLAN_PRIVATE: i32 = 9;
            pub const r#NUMBER_PRESENTATION_ALLOWED: i32 = 0;
            pub const r#NUMBER_PRESENTATION_RESTRICTED: i32 = 1;
            pub const r#NUMBER_PRESENTATION_UNKNOWN: i32 = 2;
            pub const r#NUMBER_TYPE_UNKNOWN: i32 = 0;
            pub const r#NUMBER_TYPE_INTERNATIONAL: i32 = 1;
            pub const r#NUMBER_TYPE_NATIONAL: i32 = 2;
            pub const r#NUMBER_TYPE_NETWORK_SPECIFIC: i32 = 3;
            pub const r#NUMBER_TYPE_SUBSCRIBER: i32 = 4;
            impl Default for r#CdmaCallWaiting {
              fn default() -> Self {
                Self {
                  r#number: Default::default(),
                  r#numberPresentation: 0,
                  r#name: Default::default(),
                  r#signalInfoRecord: Default::default(),
                  r#numberType: 0,
                  r#numberPlan: 0,
                }
              }
            }
            impl binder::Parcelable for r#CdmaCallWaiting {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#number)?;
                  subparcel.write(&self.r#numberPresentation)?;
                  subparcel.write(&self.r#name)?;
                  subparcel.write(&self.r#signalInfoRecord)?;
                  subparcel.write(&self.r#numberType)?;
                  subparcel.write(&self.r#numberPlan)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#number = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#numberPresentation = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#name = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signalInfoRecord = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#numberType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#numberPlan = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaCallWaiting);
            binder::impl_deserialize_for_parcelable!(r#CdmaCallWaiting);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaCallWaiting {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.CdmaCallWaiting" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaCallWaiting as _7_android_8_hardware_5_radio_5_voice_15_CdmaCallWaiting;
            }
          }
                    pub mod CdmaDisplayInfoRecord {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaDisplayInfoRecord {
              pub r#alphaBuf: String,
            }
            pub const r#CDMA_ALPHA_INFO_BUFFER_LENGTH: i32 = 64;
            impl Default for r#CdmaDisplayInfoRecord {
              fn default() -> Self {
                Self {
                  r#alphaBuf: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CdmaDisplayInfoRecord {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#alphaBuf)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#alphaBuf = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaDisplayInfoRecord);
            binder::impl_deserialize_for_parcelable!(r#CdmaDisplayInfoRecord);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaDisplayInfoRecord {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.CdmaDisplayInfoRecord" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaDisplayInfoRecord as _7_android_8_hardware_5_radio_5_voice_21_CdmaDisplayInfoRecord;
            }
          }
                    pub mod CdmaInformationRecord {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaInformationRecord {
              pub r#name: i32,
              pub r#display: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaDisplayInfoRecord>,
              pub r#number: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaNumberInfoRecord>,
              pub r#signal: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord>,
              pub r#redir: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_31_CdmaRedirectingNumberInfoRecord>,
              pub r#lineCtrl: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_25_CdmaLineControlInfoRecord>,
              pub r#clir: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaT53ClirInfoRecord>,
              pub r#audioCtrl: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_29_CdmaT53AudioControlInfoRecord>,
            }
            pub const r#CDMA_MAX_NUMBER_OF_INFO_RECS: i32 = 10;
            pub const r#NAME_DISPLAY: i32 = 0;
            pub const r#NAME_CALLED_PARTY_NUMBER: i32 = 1;
            pub const r#NAME_CALLING_PARTY_NUMBER: i32 = 2;
            pub const r#NAME_CONNECTED_NUMBER: i32 = 3;
            pub const r#NAME_SIGNAL: i32 = 4;
            pub const r#NAME_REDIRECTING_NUMBER: i32 = 5;
            pub const r#NAME_LINE_CONTROL: i32 = 6;
            pub const r#NAME_EXTENDED_DISPLAY: i32 = 7;
            pub const r#NAME_T53_CLIR: i32 = 8;
            pub const r#NAME_T53_RELEASE: i32 = 9;
            pub const r#NAME_T53_AUDIO_CONTROL: i32 = 10;
            impl Default for r#CdmaInformationRecord {
              fn default() -> Self {
                Self {
                  r#name: 0,
                  r#display: Default::default(),
                  r#number: Default::default(),
                  r#signal: Default::default(),
                  r#redir: Default::default(),
                  r#lineCtrl: Default::default(),
                  r#clir: Default::default(),
                  r#audioCtrl: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CdmaInformationRecord {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#name)?;
                  subparcel.write(&self.r#display)?;
                  subparcel.write(&self.r#number)?;
                  subparcel.write(&self.r#signal)?;
                  subparcel.write(&self.r#redir)?;
                  subparcel.write(&self.r#lineCtrl)?;
                  subparcel.write(&self.r#clir)?;
                  subparcel.write(&self.r#audioCtrl)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#name = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#display = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#number = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signal = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#redir = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#lineCtrl = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#clir = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#audioCtrl = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaInformationRecord);
            binder::impl_deserialize_for_parcelable!(r#CdmaInformationRecord);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaInformationRecord {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.CdmaInformationRecord" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaInformationRecord as _7_android_8_hardware_5_radio_5_voice_21_CdmaInformationRecord;
            }
          }
                    pub mod CdmaLineControlInfoRecord {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaLineControlInfoRecord {
              pub r#lineCtrlPolarityIncluded: i8,
              pub r#lineCtrlToggle: i8,
              pub r#lineCtrlReverse: i8,
              pub r#lineCtrlPowerDenial: i8,
            }
            impl Default for r#CdmaLineControlInfoRecord {
              fn default() -> Self {
                Self {
                  r#lineCtrlPolarityIncluded: 0,
                  r#lineCtrlToggle: 0,
                  r#lineCtrlReverse: 0,
                  r#lineCtrlPowerDenial: 0,
                }
              }
            }
            impl binder::Parcelable for r#CdmaLineControlInfoRecord {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#lineCtrlPolarityIncluded)?;
                  subparcel.write(&self.r#lineCtrlToggle)?;
                  subparcel.write(&self.r#lineCtrlReverse)?;
                  subparcel.write(&self.r#lineCtrlPowerDenial)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#lineCtrlPolarityIncluded = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#lineCtrlToggle = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#lineCtrlReverse = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#lineCtrlPowerDenial = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaLineControlInfoRecord);
            binder::impl_deserialize_for_parcelable!(r#CdmaLineControlInfoRecord);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaLineControlInfoRecord {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.CdmaLineControlInfoRecord" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaLineControlInfoRecord as _7_android_8_hardware_5_radio_5_voice_25_CdmaLineControlInfoRecord;
            }
          }
                    pub mod CdmaNumberInfoRecord {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaNumberInfoRecord {
              pub r#number: String,
              pub r#numberType: i8,
              pub r#numberPlan: i8,
              pub r#pi: i8,
              pub r#si: i8,
            }
            pub const r#CDMA_NUMBER_INFO_BUFFER_LENGTH: i32 = 81;
            impl Default for r#CdmaNumberInfoRecord {
              fn default() -> Self {
                Self {
                  r#number: Default::default(),
                  r#numberType: 0,
                  r#numberPlan: 0,
                  r#pi: 0,
                  r#si: 0,
                }
              }
            }
            impl binder::Parcelable for r#CdmaNumberInfoRecord {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#number)?;
                  subparcel.write(&self.r#numberType)?;
                  subparcel.write(&self.r#numberPlan)?;
                  subparcel.write(&self.r#pi)?;
                  subparcel.write(&self.r#si)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#number = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#numberType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#numberPlan = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#pi = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#si = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaNumberInfoRecord);
            binder::impl_deserialize_for_parcelable!(r#CdmaNumberInfoRecord);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaNumberInfoRecord {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.CdmaNumberInfoRecord" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaNumberInfoRecord as _7_android_8_hardware_5_radio_5_voice_20_CdmaNumberInfoRecord;
            }
          }
                    pub mod CdmaOtaProvisionStatus {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#CdmaOtaProvisionStatus : [i32; 12] {
                r#SPL_UNLOCKED = 0,
                r#SPC_RETRIES_EXCEEDED = 1,
                r#A_KEY_EXCHANGED = 2,
                r#SSD_UPDATED = 3,
                r#NAM_DOWNLOADED = 4,
                r#MDN_DOWNLOADED = 5,
                r#IMSI_DOWNLOADED = 6,
                r#PRL_DOWNLOADED = 7,
                r#COMMITTED = 8,
                r#OTAPA_STARTED = 9,
                r#OTAPA_STOPPED = 10,
                r#OTAPA_ABORTED = 11,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaOtaProvisionStatus as _7_android_8_hardware_5_radio_5_voice_22_CdmaOtaProvisionStatus;
            }
          }
                    pub mod CdmaRedirectingNumberInfoRecord {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaRedirectingNumberInfoRecord {
              pub r#redirectingNumber: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaNumberInfoRecord,
              pub r#redirectingReason: i32,
            }
            pub const r#REDIRECTING_REASON_UNKNOWN: i32 = 0;
            pub const r#REDIRECTING_REASON_CALL_FORWARDING_BUSY: i32 = 1;
            pub const r#REDIRECTING_REASON_CALL_FORWARDING_NO_REPLY: i32 = 2;
            pub const r#REDIRECTING_REASON_CALLED_DTE_OUT_OF_ORDER: i32 = 9;
            pub const r#REDIRECTING_REASON_CALL_FORWARDING_BY_THE_CALLED_DTE: i32 = 10;
            pub const r#REDIRECTING_REASON_CALL_FORWARDING_UNCONDITIONAL: i32 = 15;
            pub const r#REDIRECTING_REASON_RESERVED: i32 = 16;
            impl Default for r#CdmaRedirectingNumberInfoRecord {
              fn default() -> Self {
                Self {
                  r#redirectingNumber: Default::default(),
                  r#redirectingReason: 0,
                }
              }
            }
            impl binder::Parcelable for r#CdmaRedirectingNumberInfoRecord {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#redirectingNumber)?;
                  subparcel.write(&self.r#redirectingReason)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#redirectingNumber = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#redirectingReason = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaRedirectingNumberInfoRecord);
            binder::impl_deserialize_for_parcelable!(r#CdmaRedirectingNumberInfoRecord);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaRedirectingNumberInfoRecord {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.CdmaRedirectingNumberInfoRecord" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaRedirectingNumberInfoRecord as _7_android_8_hardware_5_radio_5_voice_31_CdmaRedirectingNumberInfoRecord;
            }
          }
                    pub mod CdmaSignalInfoRecord {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaSignalInfoRecord {
              pub r#isPresent: bool,
              pub r#signalType: i8,
              pub r#alertPitch: i8,
              pub r#signal: i8,
            }
            impl Default for r#CdmaSignalInfoRecord {
              fn default() -> Self {
                Self {
                  r#isPresent: false,
                  r#signalType: 0,
                  r#alertPitch: 0,
                  r#signal: 0,
                }
              }
            }
            impl binder::Parcelable for r#CdmaSignalInfoRecord {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#isPresent)?;
                  subparcel.write(&self.r#signalType)?;
                  subparcel.write(&self.r#alertPitch)?;
                  subparcel.write(&self.r#signal)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#isPresent = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signalType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#alertPitch = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signal = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaSignalInfoRecord);
            binder::impl_deserialize_for_parcelable!(r#CdmaSignalInfoRecord);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaSignalInfoRecord {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.CdmaSignalInfoRecord" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaSignalInfoRecord as _7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord;
            }
          }
                    pub mod CdmaT53AudioControlInfoRecord {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaT53AudioControlInfoRecord {
              pub r#upLink: i8,
              pub r#downLink: i8,
            }
            impl Default for r#CdmaT53AudioControlInfoRecord {
              fn default() -> Self {
                Self {
                  r#upLink: 0,
                  r#downLink: 0,
                }
              }
            }
            impl binder::Parcelable for r#CdmaT53AudioControlInfoRecord {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#upLink)?;
                  subparcel.write(&self.r#downLink)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#upLink = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#downLink = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaT53AudioControlInfoRecord);
            binder::impl_deserialize_for_parcelable!(r#CdmaT53AudioControlInfoRecord);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaT53AudioControlInfoRecord {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.CdmaT53AudioControlInfoRecord" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaT53AudioControlInfoRecord as _7_android_8_hardware_5_radio_5_voice_29_CdmaT53AudioControlInfoRecord;
            }
          }
                    pub mod CdmaT53ClirInfoRecord {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaT53ClirInfoRecord {
              pub r#cause: i8,
            }
            impl Default for r#CdmaT53ClirInfoRecord {
              fn default() -> Self {
                Self {
                  r#cause: 0,
                }
              }
            }
            impl binder::Parcelable for r#CdmaT53ClirInfoRecord {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cause)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cause = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaT53ClirInfoRecord);
            binder::impl_deserialize_for_parcelable!(r#CdmaT53ClirInfoRecord);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaT53ClirInfoRecord {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.CdmaT53ClirInfoRecord" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaT53ClirInfoRecord as _7_android_8_hardware_5_radio_5_voice_21_CdmaT53ClirInfoRecord;
            }
          }
                    pub mod CfData {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CfData {
              pub r#cfInfo: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo>,
            }
            pub const r#NUM_SERVICE_CLASSES: i32 = 7;
            impl Default for r#CfData {
              fn default() -> Self {
                Self {
                  r#cfInfo: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CfData {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cfInfo)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cfInfo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CfData);
            binder::impl_deserialize_for_parcelable!(r#CfData);
            impl binder::binder_impl::ParcelableMetadata for r#CfData {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.CfData" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CfData as _7_android_8_hardware_5_radio_5_voice_6_CfData;
            }
          }
                    pub mod ClipStatus {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#ClipStatus : [i32; 3] {
                r#CLIP_PROVISIONED = 0,
                r#CLIP_UNPROVISIONED = 1,
                r#UNKNOWN = 2,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#ClipStatus as _7_android_8_hardware_5_radio_5_voice_10_ClipStatus;
            }
          }
                    pub mod Dial {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#Dial {
              pub r#address: String,
              pub r#clir: i32,
              pub r#uusInfo: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_UusInfo>,
            }
            pub const r#CLIR_DEFAULT: i32 = 0;
            pub const r#CLIR_INVOCATION: i32 = 1;
            pub const r#CLIR_SUPPRESSION: i32 = 2;
            impl Default for r#Dial {
              fn default() -> Self {
                Self {
                  r#address: Default::default(),
                  r#clir: 0,
                  r#uusInfo: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#Dial {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#address)?;
                  subparcel.write(&self.r#clir)?;
                  subparcel.write(&self.r#uusInfo)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#address = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#clir = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#uusInfo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Dial);
            binder::impl_deserialize_for_parcelable!(r#Dial);
            impl binder::binder_impl::ParcelableMetadata for r#Dial {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.Dial" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Dial as _7_android_8_hardware_5_radio_5_voice_4_Dial;
            }
          }
                    pub mod EmergencyCallRouting {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#EmergencyCallRouting : [i32; 3] {
                r#UNKNOWN = 0,
                r#EMERGENCY = 1,
                r#NORMAL = 2,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#EmergencyCallRouting as _7_android_8_hardware_5_radio_5_voice_20_EmergencyCallRouting;
            }
          }
                    pub mod EmergencyNumber {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#EmergencyNumber {
              pub r#number: String,
              pub r#mcc: String,
              pub r#mnc: String,
              pub r#categories: i32,
              pub r#urns: Vec<String>,
              pub r#sources: i32,
            }
            pub const r#SOURCE_NETWORK_SIGNALING: i32 = 1;
            pub const r#SOURCE_SIM: i32 = 2;
            pub const r#SOURCE_MODEM_CONFIG: i32 = 4;
            pub const r#SOURCE_DEFAULT: i32 = 8;
            impl Default for r#EmergencyNumber {
              fn default() -> Self {
                Self {
                  r#number: Default::default(),
                  r#mcc: Default::default(),
                  r#mnc: Default::default(),
                  r#categories: 0,
                  r#urns: Default::default(),
                  r#sources: 0,
                }
              }
            }
            impl binder::Parcelable for r#EmergencyNumber {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#number)?;
                  subparcel.write(&self.r#mcc)?;
                  subparcel.write(&self.r#mnc)?;
                  subparcel.write(&self.r#categories)?;
                  subparcel.write(&self.r#urns)?;
                  subparcel.write(&self.r#sources)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#number = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mcc = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mnc = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#categories = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#urns = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sources = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#EmergencyNumber);
            binder::impl_deserialize_for_parcelable!(r#EmergencyNumber);
            impl binder::binder_impl::ParcelableMetadata for r#EmergencyNumber {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.EmergencyNumber" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#EmergencyNumber as _7_android_8_hardware_5_radio_5_voice_15_EmergencyNumber;
            }
          }
                    pub mod EmergencyServiceCategory {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#EmergencyServiceCategory : [i32; 8] {
                r#UNSPECIFIED = 0,
                r#POLICE = 1,
                r#AMBULANCE = 2,
                r#FIRE_BRIGADE = 4,
                r#MARINE_GUARD = 8,
                r#MOUNTAIN_RESCUE = 16,
                r#MIEC = 32,
                r#AIEC = 64,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#EmergencyServiceCategory as _7_android_8_hardware_5_radio_5_voice_24_EmergencyServiceCategory;
            }
          }
                    pub mod IRadioVoice {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioVoice["android.hardware.radio.voice.IRadioVoice"] {
                native: BnRadioVoice(on_transact),
                proxy: BpRadioVoice {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioVoiceAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioVoice: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.voice.IRadioVoice" }
              fn r#acceptCall(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#cancelPendingUssd(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#conference(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#dial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial) -> binder::Result<()>;
              fn r#emergencyDial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial, _arg_categories: i32, _arg_urns: &[String], _arg_routing: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_EmergencyCallRouting, _arg_hasKnownUserIntentEmergency: bool, _arg_isTesting: bool) -> binder::Result<()>;
              fn r#exitEmergencyCallbackMode(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#explicitCallTransfer(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getCallForwardStatus(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<()>;
              fn r#getCallWaiting(&self, _arg_serial: i32, _arg_serviceClass: i32) -> binder::Result<()>;
              fn r#getClip(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getClir(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getCurrentCalls(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getLastCallFailCause(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getMute(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getPreferredVoicePrivacy(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getTtyMode(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#handleStkCallSetupRequestFromSim(&self, _arg_serial: i32, _arg_accept: bool) -> binder::Result<()>;
              fn r#hangup(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<()>;
              fn r#hangupForegroundResumeBackground(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#hangupWaitingOrBackground(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#isVoNrEnabled(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#rejectCall(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#responseAcknowledgement(&self) -> binder::Result<()>;
              fn r#sendBurstDtmf(&self, _arg_serial: i32, _arg_dtmf: &str, _arg_on: i32, _arg_off: i32) -> binder::Result<()>;
              fn r#sendCdmaFeatureCode(&self, _arg_serial: i32, _arg_featureCode: &str) -> binder::Result<()>;
              fn r#sendDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<()>;
              fn r#sendUssd(&self, _arg_serial: i32, _arg_ussd: &str) -> binder::Result<()>;
              fn r#separateConnection(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<()>;
              fn r#setCallForward(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<()>;
              fn r#setCallWaiting(&self, _arg_serial: i32, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<()>;
              fn r#setClir(&self, _arg_serial: i32, _arg_status: i32) -> binder::Result<()>;
              fn r#setMute(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()>;
              fn r#setPreferredVoicePrivacy(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()>;
              fn r#setResponseFunctions(&self, _arg_radioVoiceResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_19_IRadioVoiceResponse>, _arg_radioVoiceIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_IRadioVoiceIndication>) -> binder::Result<()>;
              fn r#setTtyMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<()>;
              fn r#setVoNrEnabled(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()>;
              fn r#startDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<()>;
              fn r#stopDtmf(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#switchWaitingOrHoldingAndActive(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioVoiceDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioVoiceDefaultRef) -> IRadioVoiceDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioVoiceAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.voice.IRadioVoice" }
              fn r#acceptCall(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#cancelPendingUssd(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#conference(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#dial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial) -> std::future::Ready<binder::Result<()>>;
              fn r#emergencyDial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial, _arg_categories: i32, _arg_urns: &[String], _arg_routing: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_EmergencyCallRouting, _arg_hasKnownUserIntentEmergency: bool, _arg_isTesting: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#exitEmergencyCallbackMode(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#explicitCallTransfer(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getCallForwardStatus(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getCallWaiting(&self, _arg_serial: i32, _arg_serviceClass: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getClip(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getClir(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getCurrentCalls(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getLastCallFailCause(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getMute(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getPreferredVoicePrivacy(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getTtyMode(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#handleStkCallSetupRequestFromSim(&self, _arg_serial: i32, _arg_accept: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#hangup(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#hangupForegroundResumeBackground(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#hangupWaitingOrBackground(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#isVoNrEnabled(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#rejectCall(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#responseAcknowledgement(&self) -> std::future::Ready<binder::Result<()>>;
              fn r#sendBurstDtmf(&self, _arg_serial: i32, _arg_dtmf: &str, _arg_on: i32, _arg_off: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#sendCdmaFeatureCode(&self, _arg_serial: i32, _arg_featureCode: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#sendDtmf(&self, _arg_serial: i32, _arg_s: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#sendUssd(&self, _arg_serial: i32, _arg_ussd: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#separateConnection(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#setCallForward(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setCallWaiting(&self, _arg_serial: i32, _arg_enable: bool, _arg_serviceClass: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#setClir(&self, _arg_serial: i32, _arg_status: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#setMute(&self, _arg_serial: i32, _arg_enable: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#setPreferredVoicePrivacy(&self, _arg_serial: i32, _arg_enable: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#setResponseFunctions(&self, _arg_radioVoiceResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_19_IRadioVoiceResponse>, _arg_radioVoiceIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_IRadioVoiceIndication>) -> std::future::Ready<binder::Result<()>>;
              fn r#setTtyMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> std::future::Ready<binder::Result<()>>;
              fn r#setVoNrEnabled(&self, _arg_serial: i32, _arg_enable: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#startDtmf(&self, _arg_serial: i32, _arg_s: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#stopDtmf(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#switchWaitingOrHoldingAndActive(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioVoiceAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.voice.IRadioVoice" }
              async fn r#acceptCall(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#cancelPendingUssd(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#conference(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#dial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial) -> binder::Result<()>;
              async fn r#emergencyDial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial, _arg_categories: i32, _arg_urns: &[String], _arg_routing: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_EmergencyCallRouting, _arg_hasKnownUserIntentEmergency: bool, _arg_isTesting: bool) -> binder::Result<()>;
              async fn r#exitEmergencyCallbackMode(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#explicitCallTransfer(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getCallForwardStatus(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<()>;
              async fn r#getCallWaiting(&self, _arg_serial: i32, _arg_serviceClass: i32) -> binder::Result<()>;
              async fn r#getClip(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getClir(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getCurrentCalls(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getLastCallFailCause(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getMute(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getPreferredVoicePrivacy(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getTtyMode(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#handleStkCallSetupRequestFromSim(&self, _arg_serial: i32, _arg_accept: bool) -> binder::Result<()>;
              async fn r#hangup(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<()>;
              async fn r#hangupForegroundResumeBackground(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#hangupWaitingOrBackground(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#isVoNrEnabled(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#rejectCall(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#responseAcknowledgement(&self) -> binder::Result<()>;
              async fn r#sendBurstDtmf(&self, _arg_serial: i32, _arg_dtmf: &str, _arg_on: i32, _arg_off: i32) -> binder::Result<()>;
              async fn r#sendCdmaFeatureCode(&self, _arg_serial: i32, _arg_featureCode: &str) -> binder::Result<()>;
              async fn r#sendDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<()>;
              async fn r#sendUssd(&self, _arg_serial: i32, _arg_ussd: &str) -> binder::Result<()>;
              async fn r#separateConnection(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<()>;
              async fn r#setCallForward(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<()>;
              async fn r#setCallWaiting(&self, _arg_serial: i32, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<()>;
              async fn r#setClir(&self, _arg_serial: i32, _arg_status: i32) -> binder::Result<()>;
              async fn r#setMute(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()>;
              async fn r#setPreferredVoicePrivacy(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()>;
              async fn r#setResponseFunctions(&self, _arg_radioVoiceResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_19_IRadioVoiceResponse>, _arg_radioVoiceIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_IRadioVoiceIndication>) -> binder::Result<()>;
              async fn r#setTtyMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<()>;
              async fn r#setVoNrEnabled(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()>;
              async fn r#startDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<()>;
              async fn r#stopDtmf(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#switchWaitingOrHoldingAndActive(&self, _arg_serial: i32) -> binder::Result<()>;
            }
            impl BnRadioVoice {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioVoice>
              where
                T: IRadioVoiceAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioVoice for Wrapper<T, R>
                where
                  T: IRadioVoiceAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#acceptCall(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acceptCall(_arg_serial))
                  }
                  fn r#cancelPendingUssd(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#cancelPendingUssd(_arg_serial))
                  }
                  fn r#conference(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#conference(_arg_serial))
                  }
                  fn r#dial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#dial(_arg_serial, _arg_dialInfo))
                  }
                  fn r#emergencyDial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial, _arg_categories: i32, _arg_urns: &[String], _arg_routing: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_EmergencyCallRouting, _arg_hasKnownUserIntentEmergency: bool, _arg_isTesting: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#emergencyDial(_arg_serial, _arg_dialInfo, _arg_categories, _arg_urns, _arg_routing, _arg_hasKnownUserIntentEmergency, _arg_isTesting))
                  }
                  fn r#exitEmergencyCallbackMode(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#exitEmergencyCallbackMode(_arg_serial))
                  }
                  fn r#explicitCallTransfer(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#explicitCallTransfer(_arg_serial))
                  }
                  fn r#getCallForwardStatus(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCallForwardStatus(_arg_serial, _arg_callInfo))
                  }
                  fn r#getCallWaiting(&self, _arg_serial: i32, _arg_serviceClass: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCallWaiting(_arg_serial, _arg_serviceClass))
                  }
                  fn r#getClip(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getClip(_arg_serial))
                  }
                  fn r#getClir(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getClir(_arg_serial))
                  }
                  fn r#getCurrentCalls(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCurrentCalls(_arg_serial))
                  }
                  fn r#getLastCallFailCause(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getLastCallFailCause(_arg_serial))
                  }
                  fn r#getMute(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getMute(_arg_serial))
                  }
                  fn r#getPreferredVoicePrivacy(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getPreferredVoicePrivacy(_arg_serial))
                  }
                  fn r#getTtyMode(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getTtyMode(_arg_serial))
                  }
                  fn r#handleStkCallSetupRequestFromSim(&self, _arg_serial: i32, _arg_accept: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#handleStkCallSetupRequestFromSim(_arg_serial, _arg_accept))
                  }
                  fn r#hangup(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#hangup(_arg_serial, _arg_gsmIndex))
                  }
                  fn r#hangupForegroundResumeBackground(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#hangupForegroundResumeBackground(_arg_serial))
                  }
                  fn r#hangupWaitingOrBackground(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#hangupWaitingOrBackground(_arg_serial))
                  }
                  fn r#isVoNrEnabled(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#isVoNrEnabled(_arg_serial))
                  }
                  fn r#rejectCall(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#rejectCall(_arg_serial))
                  }
                  fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#responseAcknowledgement())
                  }
                  fn r#sendBurstDtmf(&self, _arg_serial: i32, _arg_dtmf: &str, _arg_on: i32, _arg_off: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendBurstDtmf(_arg_serial, _arg_dtmf, _arg_on, _arg_off))
                  }
                  fn r#sendCdmaFeatureCode(&self, _arg_serial: i32, _arg_featureCode: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendCdmaFeatureCode(_arg_serial, _arg_featureCode))
                  }
                  fn r#sendDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendDtmf(_arg_serial, _arg_s))
                  }
                  fn r#sendUssd(&self, _arg_serial: i32, _arg_ussd: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendUssd(_arg_serial, _arg_ussd))
                  }
                  fn r#separateConnection(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#separateConnection(_arg_serial, _arg_gsmIndex))
                  }
                  fn r#setCallForward(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCallForward(_arg_serial, _arg_callInfo))
                  }
                  fn r#setCallWaiting(&self, _arg_serial: i32, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCallWaiting(_arg_serial, _arg_enable, _arg_serviceClass))
                  }
                  fn r#setClir(&self, _arg_serial: i32, _arg_status: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setClir(_arg_serial, _arg_status))
                  }
                  fn r#setMute(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setMute(_arg_serial, _arg_enable))
                  }
                  fn r#setPreferredVoicePrivacy(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setPreferredVoicePrivacy(_arg_serial, _arg_enable))
                  }
                  fn r#setResponseFunctions(&self, _arg_radioVoiceResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_19_IRadioVoiceResponse>, _arg_radioVoiceIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_IRadioVoiceIndication>) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setResponseFunctions(_arg_radioVoiceResponse, _arg_radioVoiceIndication))
                  }
                  fn r#setTtyMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setTtyMode(_arg_serial, _arg_mode))
                  }
                  fn r#setVoNrEnabled(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setVoNrEnabled(_arg_serial, _arg_enable))
                  }
                  fn r#startDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#startDtmf(_arg_serial, _arg_s))
                  }
                  fn r#stopDtmf(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#stopDtmf(_arg_serial))
                  }
                  fn r#switchWaitingOrHoldingAndActive(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#switchWaitingOrHoldingAndActive(_arg_serial))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioVoiceDefault: Send + Sync {
              fn r#acceptCall(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#cancelPendingUssd(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#conference(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#dial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#emergencyDial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial, _arg_categories: i32, _arg_urns: &[String], _arg_routing: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_EmergencyCallRouting, _arg_hasKnownUserIntentEmergency: bool, _arg_isTesting: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#exitEmergencyCallbackMode(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#explicitCallTransfer(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCallForwardStatus(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCallWaiting(&self, _arg_serial: i32, _arg_serviceClass: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getClip(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getClir(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCurrentCalls(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getLastCallFailCause(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getMute(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getPreferredVoicePrivacy(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getTtyMode(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#handleStkCallSetupRequestFromSim(&self, _arg_serial: i32, _arg_accept: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#hangup(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#hangupForegroundResumeBackground(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#hangupWaitingOrBackground(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#isVoNrEnabled(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#rejectCall(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendBurstDtmf(&self, _arg_serial: i32, _arg_dtmf: &str, _arg_on: i32, _arg_off: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendCdmaFeatureCode(&self, _arg_serial: i32, _arg_featureCode: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendUssd(&self, _arg_serial: i32, _arg_ussd: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#separateConnection(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCallForward(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCallWaiting(&self, _arg_serial: i32, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setClir(&self, _arg_serial: i32, _arg_status: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setMute(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setPreferredVoicePrivacy(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setResponseFunctions(&self, _arg_radioVoiceResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_19_IRadioVoiceResponse>, _arg_radioVoiceIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_IRadioVoiceIndication>) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setTtyMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setVoNrEnabled(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#startDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#stopDtmf(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#switchWaitingOrHoldingAndActive(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#acceptCall: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#cancelPendingUssd: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#conference: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#dial: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#emergencyDial: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#exitEmergencyCallbackMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#explicitCallTransfer: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#getCallForwardStatus: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#getCallWaiting: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#getClip: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#getClir: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#getCurrentCalls: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#getLastCallFailCause: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#getMute: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#getPreferredVoicePrivacy: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#getTtyMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#handleStkCallSetupRequestFromSim: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
              pub const r#hangup: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
              pub const r#hangupForegroundResumeBackground: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
              pub const r#hangupWaitingOrBackground: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
              pub const r#isVoNrEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 20;
              pub const r#rejectCall: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 21;
              pub const r#responseAcknowledgement: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 22;
              pub const r#sendBurstDtmf: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 23;
              pub const r#sendCdmaFeatureCode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 24;
              pub const r#sendDtmf: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 25;
              pub const r#sendUssd: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 26;
              pub const r#separateConnection: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 27;
              pub const r#setCallForward: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 28;
              pub const r#setCallWaiting: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 29;
              pub const r#setClir: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 30;
              pub const r#setMute: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 31;
              pub const r#setPreferredVoicePrivacy: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 32;
              pub const r#setResponseFunctions: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 33;
              pub const r#setTtyMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 34;
              pub const r#setVoNrEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 35;
              pub const r#startDtmf: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 36;
              pub const r#stopDtmf: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 37;
              pub const r#switchWaitingOrHoldingAndActive: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 38;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioVoiceDefaultRef = Option<std::sync::Arc<dyn IRadioVoiceDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioVoiceDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "e9ffc70247a89e6c1e526c6334c37da46f33ebea";
            impl BpRadioVoice {
              fn build_parcel_acceptCall(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_acceptCall(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#acceptCall(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_cancelPendingUssd(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_cancelPendingUssd(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#cancelPendingUssd(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_conference(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_conference(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#conference(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_dial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_dialInfo)?;
                Ok(aidl_data)
              }
              fn read_response_dial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#dial(_arg_serial, _arg_dialInfo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_emergencyDial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial, _arg_categories: i32, _arg_urns: &[String], _arg_routing: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_EmergencyCallRouting, _arg_hasKnownUserIntentEmergency: bool, _arg_isTesting: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_dialInfo)?;
                aidl_data.write(&_arg_categories)?;
                aidl_data.write(_arg_urns)?;
                aidl_data.write(&_arg_routing)?;
                aidl_data.write(&_arg_hasKnownUserIntentEmergency)?;
                aidl_data.write(&_arg_isTesting)?;
                Ok(aidl_data)
              }
              fn read_response_emergencyDial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial, _arg_categories: i32, _arg_urns: &[String], _arg_routing: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_EmergencyCallRouting, _arg_hasKnownUserIntentEmergency: bool, _arg_isTesting: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#emergencyDial(_arg_serial, _arg_dialInfo, _arg_categories, _arg_urns, _arg_routing, _arg_hasKnownUserIntentEmergency, _arg_isTesting);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_exitEmergencyCallbackMode(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_exitEmergencyCallbackMode(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#exitEmergencyCallbackMode(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_explicitCallTransfer(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_explicitCallTransfer(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#explicitCallTransfer(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCallForwardStatus(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_callInfo)?;
                Ok(aidl_data)
              }
              fn read_response_getCallForwardStatus(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCallForwardStatus(_arg_serial, _arg_callInfo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCallWaiting(&self, _arg_serial: i32, _arg_serviceClass: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_serviceClass)?;
                Ok(aidl_data)
              }
              fn read_response_getCallWaiting(&self, _arg_serial: i32, _arg_serviceClass: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCallWaiting(_arg_serial, _arg_serviceClass);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getClip(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getClip(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getClip(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getClir(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getClir(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getClir(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCurrentCalls(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getCurrentCalls(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCurrentCalls(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getLastCallFailCause(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getLastCallFailCause(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getLastCallFailCause(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getMute(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getMute(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getMute(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getPreferredVoicePrivacy(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getPreferredVoicePrivacy(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getPreferredVoicePrivacy(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getTtyMode(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getTtyMode(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getTtyMode(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_handleStkCallSetupRequestFromSim(&self, _arg_serial: i32, _arg_accept: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_accept)?;
                Ok(aidl_data)
              }
              fn read_response_handleStkCallSetupRequestFromSim(&self, _arg_serial: i32, _arg_accept: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#handleStkCallSetupRequestFromSim(_arg_serial, _arg_accept);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_hangup(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_gsmIndex)?;
                Ok(aidl_data)
              }
              fn read_response_hangup(&self, _arg_serial: i32, _arg_gsmIndex: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#hangup(_arg_serial, _arg_gsmIndex);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_hangupForegroundResumeBackground(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_hangupForegroundResumeBackground(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#hangupForegroundResumeBackground(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_hangupWaitingOrBackground(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_hangupWaitingOrBackground(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#hangupWaitingOrBackground(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_isVoNrEnabled(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_isVoNrEnabled(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#isVoNrEnabled(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_rejectCall(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_rejectCall(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#rejectCall(_arg_serial);
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
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#responseAcknowledgement();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendBurstDtmf(&self, _arg_serial: i32, _arg_dtmf: &str, _arg_on: i32, _arg_off: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_dtmf)?;
                aidl_data.write(&_arg_on)?;
                aidl_data.write(&_arg_off)?;
                Ok(aidl_data)
              }
              fn read_response_sendBurstDtmf(&self, _arg_serial: i32, _arg_dtmf: &str, _arg_on: i32, _arg_off: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendBurstDtmf(_arg_serial, _arg_dtmf, _arg_on, _arg_off);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendCdmaFeatureCode(&self, _arg_serial: i32, _arg_featureCode: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_featureCode)?;
                Ok(aidl_data)
              }
              fn read_response_sendCdmaFeatureCode(&self, _arg_serial: i32, _arg_featureCode: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendCdmaFeatureCode(_arg_serial, _arg_featureCode);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_s)?;
                Ok(aidl_data)
              }
              fn read_response_sendDtmf(&self, _arg_serial: i32, _arg_s: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendDtmf(_arg_serial, _arg_s);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendUssd(&self, _arg_serial: i32, _arg_ussd: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_ussd)?;
                Ok(aidl_data)
              }
              fn read_response_sendUssd(&self, _arg_serial: i32, _arg_ussd: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendUssd(_arg_serial, _arg_ussd);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_separateConnection(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_gsmIndex)?;
                Ok(aidl_data)
              }
              fn read_response_separateConnection(&self, _arg_serial: i32, _arg_gsmIndex: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#separateConnection(_arg_serial, _arg_gsmIndex);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCallForward(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_callInfo)?;
                Ok(aidl_data)
              }
              fn read_response_setCallForward(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCallForward(_arg_serial, _arg_callInfo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCallWaiting(&self, _arg_serial: i32, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_enable)?;
                aidl_data.write(&_arg_serviceClass)?;
                Ok(aidl_data)
              }
              fn read_response_setCallWaiting(&self, _arg_serial: i32, _arg_enable: bool, _arg_serviceClass: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCallWaiting(_arg_serial, _arg_enable, _arg_serviceClass);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setClir(&self, _arg_serial: i32, _arg_status: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_status)?;
                Ok(aidl_data)
              }
              fn read_response_setClir(&self, _arg_serial: i32, _arg_status: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#setClir(_arg_serial, _arg_status);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setMute(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_enable)?;
                Ok(aidl_data)
              }
              fn read_response_setMute(&self, _arg_serial: i32, _arg_enable: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#setMute(_arg_serial, _arg_enable);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setPreferredVoicePrivacy(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_enable)?;
                Ok(aidl_data)
              }
              fn read_response_setPreferredVoicePrivacy(&self, _arg_serial: i32, _arg_enable: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#setPreferredVoicePrivacy(_arg_serial, _arg_enable);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setResponseFunctions(&self, _arg_radioVoiceResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_19_IRadioVoiceResponse>, _arg_radioVoiceIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_IRadioVoiceIndication>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_radioVoiceResponse)?;
                aidl_data.write(_arg_radioVoiceIndication)?;
                Ok(aidl_data)
              }
              fn read_response_setResponseFunctions(&self, _arg_radioVoiceResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_19_IRadioVoiceResponse>, _arg_radioVoiceIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_IRadioVoiceIndication>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#setResponseFunctions(_arg_radioVoiceResponse, _arg_radioVoiceIndication);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setTtyMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_mode)?;
                Ok(aidl_data)
              }
              fn read_response_setTtyMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#setTtyMode(_arg_serial, _arg_mode);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setVoNrEnabled(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_enable)?;
                Ok(aidl_data)
              }
              fn read_response_setVoNrEnabled(&self, _arg_serial: i32, _arg_enable: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#setVoNrEnabled(_arg_serial, _arg_enable);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_startDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_s)?;
                Ok(aidl_data)
              }
              fn read_response_startDtmf(&self, _arg_serial: i32, _arg_s: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#startDtmf(_arg_serial, _arg_s);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_stopDtmf(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_stopDtmf(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#stopDtmf(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_switchWaitingOrHoldingAndActive(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_switchWaitingOrHoldingAndActive(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoice>::getDefaultImpl() {
                    return _aidl_default_impl.r#switchWaitingOrHoldingAndActive(_arg_serial);
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
            impl IRadioVoice for BpRadioVoice {
              fn r#acceptCall(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acceptCall(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acceptCall, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acceptCall(_arg_serial, _aidl_reply)
              }
              fn r#cancelPendingUssd(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_cancelPendingUssd(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#cancelPendingUssd, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_cancelPendingUssd(_arg_serial, _aidl_reply)
              }
              fn r#conference(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_conference(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#conference, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_conference(_arg_serial, _aidl_reply)
              }
              fn r#dial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_dial(_arg_serial, _arg_dialInfo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#dial, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_dial(_arg_serial, _arg_dialInfo, _aidl_reply)
              }
              fn r#emergencyDial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial, _arg_categories: i32, _arg_urns: &[String], _arg_routing: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_EmergencyCallRouting, _arg_hasKnownUserIntentEmergency: bool, _arg_isTesting: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_emergencyDial(_arg_serial, _arg_dialInfo, _arg_categories, _arg_urns, _arg_routing, _arg_hasKnownUserIntentEmergency, _arg_isTesting)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#emergencyDial, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_emergencyDial(_arg_serial, _arg_dialInfo, _arg_categories, _arg_urns, _arg_routing, _arg_hasKnownUserIntentEmergency, _arg_isTesting, _aidl_reply)
              }
              fn r#exitEmergencyCallbackMode(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_exitEmergencyCallbackMode(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#exitEmergencyCallbackMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_exitEmergencyCallbackMode(_arg_serial, _aidl_reply)
              }
              fn r#explicitCallTransfer(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_explicitCallTransfer(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#explicitCallTransfer, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_explicitCallTransfer(_arg_serial, _aidl_reply)
              }
              fn r#getCallForwardStatus(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCallForwardStatus(_arg_serial, _arg_callInfo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCallForwardStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCallForwardStatus(_arg_serial, _arg_callInfo, _aidl_reply)
              }
              fn r#getCallWaiting(&self, _arg_serial: i32, _arg_serviceClass: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCallWaiting(_arg_serial, _arg_serviceClass)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCallWaiting, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCallWaiting(_arg_serial, _arg_serviceClass, _aidl_reply)
              }
              fn r#getClip(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getClip(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getClip, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getClip(_arg_serial, _aidl_reply)
              }
              fn r#getClir(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getClir(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getClir, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getClir(_arg_serial, _aidl_reply)
              }
              fn r#getCurrentCalls(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCurrentCalls(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCurrentCalls, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCurrentCalls(_arg_serial, _aidl_reply)
              }
              fn r#getLastCallFailCause(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getLastCallFailCause(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getLastCallFailCause, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getLastCallFailCause(_arg_serial, _aidl_reply)
              }
              fn r#getMute(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getMute(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getMute, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getMute(_arg_serial, _aidl_reply)
              }
              fn r#getPreferredVoicePrivacy(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getPreferredVoicePrivacy(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getPreferredVoicePrivacy, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getPreferredVoicePrivacy(_arg_serial, _aidl_reply)
              }
              fn r#getTtyMode(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getTtyMode(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getTtyMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getTtyMode(_arg_serial, _aidl_reply)
              }
              fn r#handleStkCallSetupRequestFromSim(&self, _arg_serial: i32, _arg_accept: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_handleStkCallSetupRequestFromSim(_arg_serial, _arg_accept)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#handleStkCallSetupRequestFromSim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_handleStkCallSetupRequestFromSim(_arg_serial, _arg_accept, _aidl_reply)
              }
              fn r#hangup(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_hangup(_arg_serial, _arg_gsmIndex)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#hangup, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_hangup(_arg_serial, _arg_gsmIndex, _aidl_reply)
              }
              fn r#hangupForegroundResumeBackground(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_hangupForegroundResumeBackground(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#hangupForegroundResumeBackground, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_hangupForegroundResumeBackground(_arg_serial, _aidl_reply)
              }
              fn r#hangupWaitingOrBackground(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_hangupWaitingOrBackground(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#hangupWaitingOrBackground, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_hangupWaitingOrBackground(_arg_serial, _aidl_reply)
              }
              fn r#isVoNrEnabled(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_isVoNrEnabled(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#isVoNrEnabled, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_isVoNrEnabled(_arg_serial, _aidl_reply)
              }
              fn r#rejectCall(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_rejectCall(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#rejectCall, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_rejectCall(_arg_serial, _aidl_reply)
              }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_responseAcknowledgement()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#responseAcknowledgement, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_responseAcknowledgement(_aidl_reply)
              }
              fn r#sendBurstDtmf(&self, _arg_serial: i32, _arg_dtmf: &str, _arg_on: i32, _arg_off: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendBurstDtmf(_arg_serial, _arg_dtmf, _arg_on, _arg_off)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendBurstDtmf, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendBurstDtmf(_arg_serial, _arg_dtmf, _arg_on, _arg_off, _aidl_reply)
              }
              fn r#sendCdmaFeatureCode(&self, _arg_serial: i32, _arg_featureCode: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendCdmaFeatureCode(_arg_serial, _arg_featureCode)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendCdmaFeatureCode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendCdmaFeatureCode(_arg_serial, _arg_featureCode, _aidl_reply)
              }
              fn r#sendDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendDtmf(_arg_serial, _arg_s)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendDtmf, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendDtmf(_arg_serial, _arg_s, _aidl_reply)
              }
              fn r#sendUssd(&self, _arg_serial: i32, _arg_ussd: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendUssd(_arg_serial, _arg_ussd)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendUssd, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendUssd(_arg_serial, _arg_ussd, _aidl_reply)
              }
              fn r#separateConnection(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_separateConnection(_arg_serial, _arg_gsmIndex)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#separateConnection, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_separateConnection(_arg_serial, _arg_gsmIndex, _aidl_reply)
              }
              fn r#setCallForward(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCallForward(_arg_serial, _arg_callInfo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCallForward, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCallForward(_arg_serial, _arg_callInfo, _aidl_reply)
              }
              fn r#setCallWaiting(&self, _arg_serial: i32, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCallWaiting(_arg_serial, _arg_enable, _arg_serviceClass)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCallWaiting, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCallWaiting(_arg_serial, _arg_enable, _arg_serviceClass, _aidl_reply)
              }
              fn r#setClir(&self, _arg_serial: i32, _arg_status: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setClir(_arg_serial, _arg_status)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setClir, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setClir(_arg_serial, _arg_status, _aidl_reply)
              }
              fn r#setMute(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setMute(_arg_serial, _arg_enable)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setMute, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setMute(_arg_serial, _arg_enable, _aidl_reply)
              }
              fn r#setPreferredVoicePrivacy(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setPreferredVoicePrivacy(_arg_serial, _arg_enable)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setPreferredVoicePrivacy, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setPreferredVoicePrivacy(_arg_serial, _arg_enable, _aidl_reply)
              }
              fn r#setResponseFunctions(&self, _arg_radioVoiceResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_19_IRadioVoiceResponse>, _arg_radioVoiceIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_IRadioVoiceIndication>) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setResponseFunctions(_arg_radioVoiceResponse, _arg_radioVoiceIndication)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setResponseFunctions(_arg_radioVoiceResponse, _arg_radioVoiceIndication, _aidl_reply)
              }
              fn r#setTtyMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setTtyMode(_arg_serial, _arg_mode)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setTtyMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setTtyMode(_arg_serial, _arg_mode, _aidl_reply)
              }
              fn r#setVoNrEnabled(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setVoNrEnabled(_arg_serial, _arg_enable)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setVoNrEnabled, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setVoNrEnabled(_arg_serial, _arg_enable, _aidl_reply)
              }
              fn r#startDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_startDtmf(_arg_serial, _arg_s)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#startDtmf, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_startDtmf(_arg_serial, _arg_s, _aidl_reply)
              }
              fn r#stopDtmf(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_stopDtmf(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#stopDtmf, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_stopDtmf(_arg_serial, _aidl_reply)
              }
              fn r#switchWaitingOrHoldingAndActive(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_switchWaitingOrHoldingAndActive(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#switchWaitingOrHoldingAndActive, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_switchWaitingOrHoldingAndActive(_arg_serial, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioVoiceAsync<P> for BpRadioVoice {
              fn r#acceptCall(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acceptCall(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acceptCall, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acceptCall(_arg_serial, _aidl_reply))
              }
              fn r#cancelPendingUssd(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_cancelPendingUssd(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#cancelPendingUssd, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_cancelPendingUssd(_arg_serial, _aidl_reply))
              }
              fn r#conference(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_conference(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#conference, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_conference(_arg_serial, _aidl_reply))
              }
              fn r#dial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_dial(_arg_serial, _arg_dialInfo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#dial, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_dial(_arg_serial, _arg_dialInfo, _aidl_reply))
              }
              fn r#emergencyDial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial, _arg_categories: i32, _arg_urns: &[String], _arg_routing: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_EmergencyCallRouting, _arg_hasKnownUserIntentEmergency: bool, _arg_isTesting: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_emergencyDial(_arg_serial, _arg_dialInfo, _arg_categories, _arg_urns, _arg_routing, _arg_hasKnownUserIntentEmergency, _arg_isTesting) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#emergencyDial, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_emergencyDial(_arg_serial, _arg_dialInfo, _arg_categories, _arg_urns, _arg_routing, _arg_hasKnownUserIntentEmergency, _arg_isTesting, _aidl_reply))
              }
              fn r#exitEmergencyCallbackMode(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_exitEmergencyCallbackMode(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#exitEmergencyCallbackMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_exitEmergencyCallbackMode(_arg_serial, _aidl_reply))
              }
              fn r#explicitCallTransfer(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_explicitCallTransfer(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#explicitCallTransfer, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_explicitCallTransfer(_arg_serial, _aidl_reply))
              }
              fn r#getCallForwardStatus(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCallForwardStatus(_arg_serial, _arg_callInfo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCallForwardStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCallForwardStatus(_arg_serial, _arg_callInfo, _aidl_reply))
              }
              fn r#getCallWaiting(&self, _arg_serial: i32, _arg_serviceClass: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCallWaiting(_arg_serial, _arg_serviceClass) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCallWaiting, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCallWaiting(_arg_serial, _arg_serviceClass, _aidl_reply))
              }
              fn r#getClip(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getClip(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getClip, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getClip(_arg_serial, _aidl_reply))
              }
              fn r#getClir(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getClir(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getClir, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getClir(_arg_serial, _aidl_reply))
              }
              fn r#getCurrentCalls(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCurrentCalls(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCurrentCalls, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCurrentCalls(_arg_serial, _aidl_reply))
              }
              fn r#getLastCallFailCause(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getLastCallFailCause(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getLastCallFailCause, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getLastCallFailCause(_arg_serial, _aidl_reply))
              }
              fn r#getMute(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getMute(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getMute, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getMute(_arg_serial, _aidl_reply))
              }
              fn r#getPreferredVoicePrivacy(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getPreferredVoicePrivacy(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getPreferredVoicePrivacy, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getPreferredVoicePrivacy(_arg_serial, _aidl_reply))
              }
              fn r#getTtyMode(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getTtyMode(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getTtyMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getTtyMode(_arg_serial, _aidl_reply))
              }
              fn r#handleStkCallSetupRequestFromSim(&self, _arg_serial: i32, _arg_accept: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_handleStkCallSetupRequestFromSim(_arg_serial, _arg_accept) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#handleStkCallSetupRequestFromSim, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_handleStkCallSetupRequestFromSim(_arg_serial, _arg_accept, _aidl_reply))
              }
              fn r#hangup(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_hangup(_arg_serial, _arg_gsmIndex) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#hangup, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_hangup(_arg_serial, _arg_gsmIndex, _aidl_reply))
              }
              fn r#hangupForegroundResumeBackground(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_hangupForegroundResumeBackground(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#hangupForegroundResumeBackground, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_hangupForegroundResumeBackground(_arg_serial, _aidl_reply))
              }
              fn r#hangupWaitingOrBackground(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_hangupWaitingOrBackground(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#hangupWaitingOrBackground, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_hangupWaitingOrBackground(_arg_serial, _aidl_reply))
              }
              fn r#isVoNrEnabled(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_isVoNrEnabled(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#isVoNrEnabled, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_isVoNrEnabled(_arg_serial, _aidl_reply))
              }
              fn r#rejectCall(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_rejectCall(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#rejectCall, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_rejectCall(_arg_serial, _aidl_reply))
              }
              fn r#responseAcknowledgement(&self) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_responseAcknowledgement() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#responseAcknowledgement, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_responseAcknowledgement(_aidl_reply))
              }
              fn r#sendBurstDtmf(&self, _arg_serial: i32, _arg_dtmf: &str, _arg_on: i32, _arg_off: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendBurstDtmf(_arg_serial, _arg_dtmf, _arg_on, _arg_off) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendBurstDtmf, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendBurstDtmf(_arg_serial, _arg_dtmf, _arg_on, _arg_off, _aidl_reply))
              }
              fn r#sendCdmaFeatureCode(&self, _arg_serial: i32, _arg_featureCode: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendCdmaFeatureCode(_arg_serial, _arg_featureCode) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendCdmaFeatureCode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendCdmaFeatureCode(_arg_serial, _arg_featureCode, _aidl_reply))
              }
              fn r#sendDtmf(&self, _arg_serial: i32, _arg_s: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendDtmf(_arg_serial, _arg_s) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendDtmf, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendDtmf(_arg_serial, _arg_s, _aidl_reply))
              }
              fn r#sendUssd(&self, _arg_serial: i32, _arg_ussd: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendUssd(_arg_serial, _arg_ussd) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendUssd, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendUssd(_arg_serial, _arg_ussd, _aidl_reply))
              }
              fn r#separateConnection(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_separateConnection(_arg_serial, _arg_gsmIndex) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#separateConnection, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_separateConnection(_arg_serial, _arg_gsmIndex, _aidl_reply))
              }
              fn r#setCallForward(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCallForward(_arg_serial, _arg_callInfo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCallForward, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCallForward(_arg_serial, _arg_callInfo, _aidl_reply))
              }
              fn r#setCallWaiting(&self, _arg_serial: i32, _arg_enable: bool, _arg_serviceClass: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCallWaiting(_arg_serial, _arg_enable, _arg_serviceClass) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCallWaiting, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCallWaiting(_arg_serial, _arg_enable, _arg_serviceClass, _aidl_reply))
              }
              fn r#setClir(&self, _arg_serial: i32, _arg_status: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setClir(_arg_serial, _arg_status) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setClir, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setClir(_arg_serial, _arg_status, _aidl_reply))
              }
              fn r#setMute(&self, _arg_serial: i32, _arg_enable: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setMute(_arg_serial, _arg_enable) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setMute, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setMute(_arg_serial, _arg_enable, _aidl_reply))
              }
              fn r#setPreferredVoicePrivacy(&self, _arg_serial: i32, _arg_enable: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setPreferredVoicePrivacy(_arg_serial, _arg_enable) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setPreferredVoicePrivacy, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setPreferredVoicePrivacy(_arg_serial, _arg_enable, _aidl_reply))
              }
              fn r#setResponseFunctions(&self, _arg_radioVoiceResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_19_IRadioVoiceResponse>, _arg_radioVoiceIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_IRadioVoiceIndication>) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setResponseFunctions(_arg_radioVoiceResponse, _arg_radioVoiceIndication) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setResponseFunctions(_arg_radioVoiceResponse, _arg_radioVoiceIndication, _aidl_reply))
              }
              fn r#setTtyMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setTtyMode(_arg_serial, _arg_mode) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setTtyMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setTtyMode(_arg_serial, _arg_mode, _aidl_reply))
              }
              fn r#setVoNrEnabled(&self, _arg_serial: i32, _arg_enable: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setVoNrEnabled(_arg_serial, _arg_enable) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setVoNrEnabled, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setVoNrEnabled(_arg_serial, _arg_enable, _aidl_reply))
              }
              fn r#startDtmf(&self, _arg_serial: i32, _arg_s: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_startDtmf(_arg_serial, _arg_s) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#startDtmf, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_startDtmf(_arg_serial, _arg_s, _aidl_reply))
              }
              fn r#stopDtmf(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_stopDtmf(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#stopDtmf, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_stopDtmf(_arg_serial, _aidl_reply))
              }
              fn r#switchWaitingOrHoldingAndActive(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_switchWaitingOrHoldingAndActive(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#switchWaitingOrHoldingAndActive, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_switchWaitingOrHoldingAndActive(_arg_serial, _aidl_reply))
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
            impl IRadioVoice for binder::binder_impl::Binder<BnRadioVoice> {
              fn r#acceptCall(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#acceptCall(_arg_serial) }
              fn r#cancelPendingUssd(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#cancelPendingUssd(_arg_serial) }
              fn r#conference(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#conference(_arg_serial) }
              fn r#dial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial) -> binder::Result<()> { self.0.r#dial(_arg_serial, _arg_dialInfo) }
              fn r#emergencyDial(&self, _arg_serial: i32, _arg_dialInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial, _arg_categories: i32, _arg_urns: &[String], _arg_routing: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_EmergencyCallRouting, _arg_hasKnownUserIntentEmergency: bool, _arg_isTesting: bool) -> binder::Result<()> { self.0.r#emergencyDial(_arg_serial, _arg_dialInfo, _arg_categories, _arg_urns, _arg_routing, _arg_hasKnownUserIntentEmergency, _arg_isTesting) }
              fn r#exitEmergencyCallbackMode(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#exitEmergencyCallbackMode(_arg_serial) }
              fn r#explicitCallTransfer(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#explicitCallTransfer(_arg_serial) }
              fn r#getCallForwardStatus(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<()> { self.0.r#getCallForwardStatus(_arg_serial, _arg_callInfo) }
              fn r#getCallWaiting(&self, _arg_serial: i32, _arg_serviceClass: i32) -> binder::Result<()> { self.0.r#getCallWaiting(_arg_serial, _arg_serviceClass) }
              fn r#getClip(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getClip(_arg_serial) }
              fn r#getClir(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getClir(_arg_serial) }
              fn r#getCurrentCalls(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getCurrentCalls(_arg_serial) }
              fn r#getLastCallFailCause(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getLastCallFailCause(_arg_serial) }
              fn r#getMute(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getMute(_arg_serial) }
              fn r#getPreferredVoicePrivacy(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getPreferredVoicePrivacy(_arg_serial) }
              fn r#getTtyMode(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getTtyMode(_arg_serial) }
              fn r#handleStkCallSetupRequestFromSim(&self, _arg_serial: i32, _arg_accept: bool) -> binder::Result<()> { self.0.r#handleStkCallSetupRequestFromSim(_arg_serial, _arg_accept) }
              fn r#hangup(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<()> { self.0.r#hangup(_arg_serial, _arg_gsmIndex) }
              fn r#hangupForegroundResumeBackground(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#hangupForegroundResumeBackground(_arg_serial) }
              fn r#hangupWaitingOrBackground(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#hangupWaitingOrBackground(_arg_serial) }
              fn r#isVoNrEnabled(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#isVoNrEnabled(_arg_serial) }
              fn r#rejectCall(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#rejectCall(_arg_serial) }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> { self.0.r#responseAcknowledgement() }
              fn r#sendBurstDtmf(&self, _arg_serial: i32, _arg_dtmf: &str, _arg_on: i32, _arg_off: i32) -> binder::Result<()> { self.0.r#sendBurstDtmf(_arg_serial, _arg_dtmf, _arg_on, _arg_off) }
              fn r#sendCdmaFeatureCode(&self, _arg_serial: i32, _arg_featureCode: &str) -> binder::Result<()> { self.0.r#sendCdmaFeatureCode(_arg_serial, _arg_featureCode) }
              fn r#sendDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<()> { self.0.r#sendDtmf(_arg_serial, _arg_s) }
              fn r#sendUssd(&self, _arg_serial: i32, _arg_ussd: &str) -> binder::Result<()> { self.0.r#sendUssd(_arg_serial, _arg_ussd) }
              fn r#separateConnection(&self, _arg_serial: i32, _arg_gsmIndex: i32) -> binder::Result<()> { self.0.r#separateConnection(_arg_serial, _arg_gsmIndex) }
              fn r#setCallForward(&self, _arg_serial: i32, _arg_callInfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo) -> binder::Result<()> { self.0.r#setCallForward(_arg_serial, _arg_callInfo) }
              fn r#setCallWaiting(&self, _arg_serial: i32, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<()> { self.0.r#setCallWaiting(_arg_serial, _arg_enable, _arg_serviceClass) }
              fn r#setClir(&self, _arg_serial: i32, _arg_status: i32) -> binder::Result<()> { self.0.r#setClir(_arg_serial, _arg_status) }
              fn r#setMute(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> { self.0.r#setMute(_arg_serial, _arg_enable) }
              fn r#setPreferredVoicePrivacy(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> { self.0.r#setPreferredVoicePrivacy(_arg_serial, _arg_enable) }
              fn r#setResponseFunctions(&self, _arg_radioVoiceResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_19_IRadioVoiceResponse>, _arg_radioVoiceIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_IRadioVoiceIndication>) -> binder::Result<()> { self.0.r#setResponseFunctions(_arg_radioVoiceResponse, _arg_radioVoiceIndication) }
              fn r#setTtyMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<()> { self.0.r#setTtyMode(_arg_serial, _arg_mode) }
              fn r#setVoNrEnabled(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> { self.0.r#setVoNrEnabled(_arg_serial, _arg_enable) }
              fn r#startDtmf(&self, _arg_serial: i32, _arg_s: &str) -> binder::Result<()> { self.0.r#startDtmf(_arg_serial, _arg_s) }
              fn r#stopDtmf(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#stopDtmf(_arg_serial) }
              fn r#switchWaitingOrHoldingAndActive(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#switchWaitingOrHoldingAndActive(_arg_serial) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioVoice, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#acceptCall => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acceptCall(_arg_serial);
                  Ok(())
                }
                transactions::r#cancelPendingUssd => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#cancelPendingUssd(_arg_serial);
                  Ok(())
                }
                transactions::r#conference => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#conference(_arg_serial);
                  Ok(())
                }
                transactions::r#dial => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_dialInfo: crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#dial(_arg_serial, &_arg_dialInfo);
                  Ok(())
                }
                transactions::r#emergencyDial => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_dialInfo: crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Dial = _aidl_data.read()?;
                  let _arg_categories: i32 = _aidl_data.read()?;
                  let _arg_urns: Vec<String> = _aidl_data.read()?;
                  let _arg_routing: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_EmergencyCallRouting = _aidl_data.read()?;
                  let _arg_hasKnownUserIntentEmergency: bool = _aidl_data.read()?;
                  let _arg_isTesting: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#emergencyDial(_arg_serial, &_arg_dialInfo, _arg_categories, &_arg_urns, _arg_routing, _arg_hasKnownUserIntentEmergency, _arg_isTesting);
                  Ok(())
                }
                transactions::r#exitEmergencyCallbackMode => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#exitEmergencyCallbackMode(_arg_serial);
                  Ok(())
                }
                transactions::r#explicitCallTransfer => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#explicitCallTransfer(_arg_serial);
                  Ok(())
                }
                transactions::r#getCallForwardStatus => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_callInfo: crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCallForwardStatus(_arg_serial, &_arg_callInfo);
                  Ok(())
                }
                transactions::r#getCallWaiting => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_serviceClass: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCallWaiting(_arg_serial, _arg_serviceClass);
                  Ok(())
                }
                transactions::r#getClip => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getClip(_arg_serial);
                  Ok(())
                }
                transactions::r#getClir => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getClir(_arg_serial);
                  Ok(())
                }
                transactions::r#getCurrentCalls => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCurrentCalls(_arg_serial);
                  Ok(())
                }
                transactions::r#getLastCallFailCause => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getLastCallFailCause(_arg_serial);
                  Ok(())
                }
                transactions::r#getMute => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getMute(_arg_serial);
                  Ok(())
                }
                transactions::r#getPreferredVoicePrivacy => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getPreferredVoicePrivacy(_arg_serial);
                  Ok(())
                }
                transactions::r#getTtyMode => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getTtyMode(_arg_serial);
                  Ok(())
                }
                transactions::r#handleStkCallSetupRequestFromSim => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_accept: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#handleStkCallSetupRequestFromSim(_arg_serial, _arg_accept);
                  Ok(())
                }
                transactions::r#hangup => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_gsmIndex: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#hangup(_arg_serial, _arg_gsmIndex);
                  Ok(())
                }
                transactions::r#hangupForegroundResumeBackground => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#hangupForegroundResumeBackground(_arg_serial);
                  Ok(())
                }
                transactions::r#hangupWaitingOrBackground => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#hangupWaitingOrBackground(_arg_serial);
                  Ok(())
                }
                transactions::r#isVoNrEnabled => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#isVoNrEnabled(_arg_serial);
                  Ok(())
                }
                transactions::r#rejectCall => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#rejectCall(_arg_serial);
                  Ok(())
                }
                transactions::r#responseAcknowledgement => {
                  let _aidl_return = _aidl_service.r#responseAcknowledgement();
                  Ok(())
                }
                transactions::r#sendBurstDtmf => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_dtmf: String = _aidl_data.read()?;
                  let _arg_on: i32 = _aidl_data.read()?;
                  let _arg_off: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendBurstDtmf(_arg_serial, &_arg_dtmf, _arg_on, _arg_off);
                  Ok(())
                }
                transactions::r#sendCdmaFeatureCode => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_featureCode: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendCdmaFeatureCode(_arg_serial, &_arg_featureCode);
                  Ok(())
                }
                transactions::r#sendDtmf => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_s: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendDtmf(_arg_serial, &_arg_s);
                  Ok(())
                }
                transactions::r#sendUssd => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_ussd: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendUssd(_arg_serial, &_arg_ussd);
                  Ok(())
                }
                transactions::r#separateConnection => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_gsmIndex: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#separateConnection(_arg_serial, _arg_gsmIndex);
                  Ok(())
                }
                transactions::r#setCallForward => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_callInfo: crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCallForward(_arg_serial, &_arg_callInfo);
                  Ok(())
                }
                transactions::r#setCallWaiting => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_enable: bool = _aidl_data.read()?;
                  let _arg_serviceClass: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCallWaiting(_arg_serial, _arg_enable, _arg_serviceClass);
                  Ok(())
                }
                transactions::r#setClir => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_status: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setClir(_arg_serial, _arg_status);
                  Ok(())
                }
                transactions::r#setMute => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_enable: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setMute(_arg_serial, _arg_enable);
                  Ok(())
                }
                transactions::r#setPreferredVoicePrivacy => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_enable: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setPreferredVoicePrivacy(_arg_serial, _arg_enable);
                  Ok(())
                }
                transactions::r#setResponseFunctions => {
                  let _arg_radioVoiceResponse: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_19_IRadioVoiceResponse> = _aidl_data.read()?;
                  let _arg_radioVoiceIndication: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_IRadioVoiceIndication> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setResponseFunctions(&_arg_radioVoiceResponse, &_arg_radioVoiceIndication);
                  Ok(())
                }
                transactions::r#setTtyMode => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setTtyMode(_arg_serial, _arg_mode);
                  Ok(())
                }
                transactions::r#setVoNrEnabled => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_enable: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setVoNrEnabled(_arg_serial, _arg_enable);
                  Ok(())
                }
                transactions::r#startDtmf => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_s: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#startDtmf(_arg_serial, &_arg_s);
                  Ok(())
                }
                transactions::r#stopDtmf => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#stopDtmf(_arg_serial);
                  Ok(())
                }
                transactions::r#switchWaitingOrHoldingAndActive => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#switchWaitingOrHoldingAndActive(_arg_serial);
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
             pub use super::r#IRadioVoice as _7_android_8_hardware_5_radio_5_voice_11_IRadioVoice;
            }
          }
                    pub mod IRadioVoiceIndication {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioVoiceIndication["android.hardware.radio.voice.IRadioVoiceIndication"] {
                native: BnRadioVoiceIndication(on_transact),
                proxy: BpRadioVoiceIndication {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioVoiceIndicationAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioVoiceIndication: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.voice.IRadioVoiceIndication" }
              fn r#callRing(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_isGsm: bool, _arg_record: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord) -> binder::Result<()>;
              fn r#callStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#cdmaCallWaiting(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_callWaitingRecord: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CdmaCallWaiting) -> binder::Result<()>;
              fn r#cdmaInfoRec(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaInformationRecord]) -> binder::Result<()>;
              fn r#cdmaOtaProvisionStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_22_CdmaOtaProvisionStatus) -> binder::Result<()>;
              fn r#currentEmergencyNumberList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_emergencyNumberList: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_EmergencyNumber]) -> binder::Result<()>;
              fn r#enterEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#exitEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#indicateRingbackTone(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_start: bool) -> binder::Result<()>;
              fn r#onSupplementaryServiceIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_ss: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_18_StkCcUnsolSsResult) -> binder::Result<()>;
              fn r#onUssd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_modeType: crate::mangled::_7_android_8_hardware_5_radio_5_voice_12_UssdModeType, _arg_msg: &str) -> binder::Result<()>;
              fn r#resendIncallMute(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#srvccStateNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_SrvccState) -> binder::Result<()>;
              fn r#stkCallControlAlphaNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_alpha: &str) -> binder::Result<()>;
              fn r#stkCallSetup(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_timeout: i64) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioVoiceIndicationDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioVoiceIndicationDefaultRef) -> IRadioVoiceIndicationDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioVoiceIndicationAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.voice.IRadioVoiceIndication" }
              fn r#callRing(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_isGsm: bool, _arg_record: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord) -> std::future::Ready<binder::Result<()>>;
              fn r#callStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#cdmaCallWaiting(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_callWaitingRecord: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CdmaCallWaiting) -> std::future::Ready<binder::Result<()>>;
              fn r#cdmaInfoRec(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaInformationRecord]) -> std::future::Ready<binder::Result<()>>;
              fn r#cdmaOtaProvisionStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_22_CdmaOtaProvisionStatus) -> std::future::Ready<binder::Result<()>>;
              fn r#currentEmergencyNumberList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_emergencyNumberList: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_EmergencyNumber]) -> std::future::Ready<binder::Result<()>>;
              fn r#enterEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#exitEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#indicateRingbackTone(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_start: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#onSupplementaryServiceIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_ss: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_18_StkCcUnsolSsResult) -> std::future::Ready<binder::Result<()>>;
              fn r#onUssd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_modeType: crate::mangled::_7_android_8_hardware_5_radio_5_voice_12_UssdModeType, _arg_msg: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#resendIncallMute(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#srvccStateNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_SrvccState) -> std::future::Ready<binder::Result<()>>;
              fn r#stkCallControlAlphaNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_alpha: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#stkCallSetup(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_timeout: i64) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioVoiceIndicationAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.voice.IRadioVoiceIndication" }
              async fn r#callRing(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_isGsm: bool, _arg_record: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord) -> binder::Result<()>;
              async fn r#callStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              async fn r#cdmaCallWaiting(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_callWaitingRecord: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CdmaCallWaiting) -> binder::Result<()>;
              async fn r#cdmaInfoRec(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaInformationRecord]) -> binder::Result<()>;
              async fn r#cdmaOtaProvisionStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_22_CdmaOtaProvisionStatus) -> binder::Result<()>;
              async fn r#currentEmergencyNumberList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_emergencyNumberList: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_EmergencyNumber]) -> binder::Result<()>;
              async fn r#enterEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              async fn r#exitEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              async fn r#indicateRingbackTone(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_start: bool) -> binder::Result<()>;
              async fn r#onSupplementaryServiceIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_ss: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_18_StkCcUnsolSsResult) -> binder::Result<()>;
              async fn r#onUssd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_modeType: crate::mangled::_7_android_8_hardware_5_radio_5_voice_12_UssdModeType, _arg_msg: &str) -> binder::Result<()>;
              async fn r#resendIncallMute(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              async fn r#srvccStateNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_SrvccState) -> binder::Result<()>;
              async fn r#stkCallControlAlphaNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_alpha: &str) -> binder::Result<()>;
              async fn r#stkCallSetup(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_timeout: i64) -> binder::Result<()>;
            }
            impl BnRadioVoiceIndication {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioVoiceIndication>
              where
                T: IRadioVoiceIndicationAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioVoiceIndication for Wrapper<T, R>
                where
                  T: IRadioVoiceIndicationAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#callRing(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_isGsm: bool, _arg_record: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#callRing(_arg_type, _arg_isGsm, _arg_record))
                  }
                  fn r#callStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#callStateChanged(_arg_type))
                  }
                  fn r#cdmaCallWaiting(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_callWaitingRecord: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CdmaCallWaiting) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#cdmaCallWaiting(_arg_type, _arg_callWaitingRecord))
                  }
                  fn r#cdmaInfoRec(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaInformationRecord]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#cdmaInfoRec(_arg_type, _arg_records))
                  }
                  fn r#cdmaOtaProvisionStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_22_CdmaOtaProvisionStatus) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#cdmaOtaProvisionStatus(_arg_type, _arg_status))
                  }
                  fn r#currentEmergencyNumberList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_emergencyNumberList: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_EmergencyNumber]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#currentEmergencyNumberList(_arg_type, _arg_emergencyNumberList))
                  }
                  fn r#enterEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#enterEmergencyCallbackMode(_arg_type))
                  }
                  fn r#exitEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#exitEmergencyCallbackMode(_arg_type))
                  }
                  fn r#indicateRingbackTone(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_start: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#indicateRingbackTone(_arg_type, _arg_start))
                  }
                  fn r#onSupplementaryServiceIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_ss: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_18_StkCcUnsolSsResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#onSupplementaryServiceIndication(_arg_type, _arg_ss))
                  }
                  fn r#onUssd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_modeType: crate::mangled::_7_android_8_hardware_5_radio_5_voice_12_UssdModeType, _arg_msg: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#onUssd(_arg_type, _arg_modeType, _arg_msg))
                  }
                  fn r#resendIncallMute(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#resendIncallMute(_arg_type))
                  }
                  fn r#srvccStateNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_SrvccState) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#srvccStateNotify(_arg_type, _arg_state))
                  }
                  fn r#stkCallControlAlphaNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_alpha: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#stkCallControlAlphaNotify(_arg_type, _arg_alpha))
                  }
                  fn r#stkCallSetup(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_timeout: i64) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#stkCallSetup(_arg_type, _arg_timeout))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioVoiceIndicationDefault: Send + Sync {
              fn r#callRing(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_isGsm: bool, _arg_record: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#callStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#cdmaCallWaiting(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_callWaitingRecord: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CdmaCallWaiting) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#cdmaInfoRec(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaInformationRecord]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#cdmaOtaProvisionStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_22_CdmaOtaProvisionStatus) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#currentEmergencyNumberList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_emergencyNumberList: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_EmergencyNumber]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#enterEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#exitEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#indicateRingbackTone(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_start: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#onSupplementaryServiceIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_ss: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_18_StkCcUnsolSsResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#onUssd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_modeType: crate::mangled::_7_android_8_hardware_5_radio_5_voice_12_UssdModeType, _arg_msg: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#resendIncallMute(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#srvccStateNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_SrvccState) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#stkCallControlAlphaNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_alpha: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#stkCallSetup(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_timeout: i64) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#callRing: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#callStateChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#cdmaCallWaiting: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#cdmaInfoRec: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#cdmaOtaProvisionStatus: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#currentEmergencyNumberList: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#enterEmergencyCallbackMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#exitEmergencyCallbackMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#indicateRingbackTone: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#onSupplementaryServiceIndication: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#onUssd: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#resendIncallMute: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#srvccStateNotify: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#stkCallControlAlphaNotify: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#stkCallSetup: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioVoiceIndicationDefaultRef = Option<std::sync::Arc<dyn IRadioVoiceIndicationDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioVoiceIndicationDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "e9ffc70247a89e6c1e526c6334c37da46f33ebea";
            impl BpRadioVoiceIndication {
              fn build_parcel_callRing(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_isGsm: bool, _arg_record: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_isGsm)?;
                aidl_data.write(_arg_record)?;
                Ok(aidl_data)
              }
              fn read_response_callRing(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_isGsm: bool, _arg_record: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#callRing(_arg_type, _arg_isGsm, _arg_record);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_callStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_callStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#callStateChanged(_arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_cdmaCallWaiting(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_callWaitingRecord: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CdmaCallWaiting) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_callWaitingRecord)?;
                Ok(aidl_data)
              }
              fn read_response_cdmaCallWaiting(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_callWaitingRecord: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CdmaCallWaiting, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#cdmaCallWaiting(_arg_type, _arg_callWaitingRecord);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_cdmaInfoRec(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaInformationRecord]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_records)?;
                Ok(aidl_data)
              }
              fn read_response_cdmaInfoRec(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaInformationRecord], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#cdmaInfoRec(_arg_type, _arg_records);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_cdmaOtaProvisionStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_22_CdmaOtaProvisionStatus) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_status)?;
                Ok(aidl_data)
              }
              fn read_response_cdmaOtaProvisionStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_22_CdmaOtaProvisionStatus, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#cdmaOtaProvisionStatus(_arg_type, _arg_status);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_currentEmergencyNumberList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_emergencyNumberList: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_EmergencyNumber]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_emergencyNumberList)?;
                Ok(aidl_data)
              }
              fn read_response_currentEmergencyNumberList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_emergencyNumberList: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_EmergencyNumber], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#currentEmergencyNumberList(_arg_type, _arg_emergencyNumberList);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_enterEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_enterEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#enterEmergencyCallbackMode(_arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_exitEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_exitEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#exitEmergencyCallbackMode(_arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_indicateRingbackTone(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_start: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_start)?;
                Ok(aidl_data)
              }
              fn read_response_indicateRingbackTone(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_start: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#indicateRingbackTone(_arg_type, _arg_start);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_onSupplementaryServiceIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_ss: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_18_StkCcUnsolSsResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_ss)?;
                Ok(aidl_data)
              }
              fn read_response_onSupplementaryServiceIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_ss: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_18_StkCcUnsolSsResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#onSupplementaryServiceIndication(_arg_type, _arg_ss);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_onUssd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_modeType: crate::mangled::_7_android_8_hardware_5_radio_5_voice_12_UssdModeType, _arg_msg: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_modeType)?;
                aidl_data.write(_arg_msg)?;
                Ok(aidl_data)
              }
              fn read_response_onUssd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_modeType: crate::mangled::_7_android_8_hardware_5_radio_5_voice_12_UssdModeType, _arg_msg: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#onUssd(_arg_type, _arg_modeType, _arg_msg);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_resendIncallMute(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_resendIncallMute(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#resendIncallMute(_arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_srvccStateNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_SrvccState) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_state)?;
                Ok(aidl_data)
              }
              fn read_response_srvccStateNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_SrvccState, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#srvccStateNotify(_arg_type, _arg_state);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_stkCallControlAlphaNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_alpha: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_alpha)?;
                Ok(aidl_data)
              }
              fn read_response_stkCallControlAlphaNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_alpha: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#stkCallControlAlphaNotify(_arg_type, _arg_alpha);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_stkCallSetup(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_timeout: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_timeout)?;
                Ok(aidl_data)
              }
              fn read_response_stkCallSetup(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_timeout: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#stkCallSetup(_arg_type, _arg_timeout);
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
            impl IRadioVoiceIndication for BpRadioVoiceIndication {
              fn r#callRing(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_isGsm: bool, _arg_record: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_callRing(_arg_type, _arg_isGsm, _arg_record)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#callRing, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_callRing(_arg_type, _arg_isGsm, _arg_record, _aidl_reply)
              }
              fn r#callStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_callStateChanged(_arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#callStateChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_callStateChanged(_arg_type, _aidl_reply)
              }
              fn r#cdmaCallWaiting(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_callWaitingRecord: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CdmaCallWaiting) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_cdmaCallWaiting(_arg_type, _arg_callWaitingRecord)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaCallWaiting, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_cdmaCallWaiting(_arg_type, _arg_callWaitingRecord, _aidl_reply)
              }
              fn r#cdmaInfoRec(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaInformationRecord]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_cdmaInfoRec(_arg_type, _arg_records)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaInfoRec, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_cdmaInfoRec(_arg_type, _arg_records, _aidl_reply)
              }
              fn r#cdmaOtaProvisionStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_22_CdmaOtaProvisionStatus) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_cdmaOtaProvisionStatus(_arg_type, _arg_status)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaOtaProvisionStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_cdmaOtaProvisionStatus(_arg_type, _arg_status, _aidl_reply)
              }
              fn r#currentEmergencyNumberList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_emergencyNumberList: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_EmergencyNumber]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_currentEmergencyNumberList(_arg_type, _arg_emergencyNumberList)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#currentEmergencyNumberList, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_currentEmergencyNumberList(_arg_type, _arg_emergencyNumberList, _aidl_reply)
              }
              fn r#enterEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_enterEmergencyCallbackMode(_arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#enterEmergencyCallbackMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_enterEmergencyCallbackMode(_arg_type, _aidl_reply)
              }
              fn r#exitEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_exitEmergencyCallbackMode(_arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#exitEmergencyCallbackMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_exitEmergencyCallbackMode(_arg_type, _aidl_reply)
              }
              fn r#indicateRingbackTone(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_start: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_indicateRingbackTone(_arg_type, _arg_start)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#indicateRingbackTone, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_indicateRingbackTone(_arg_type, _arg_start, _aidl_reply)
              }
              fn r#onSupplementaryServiceIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_ss: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_18_StkCcUnsolSsResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_onSupplementaryServiceIndication(_arg_type, _arg_ss)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#onSupplementaryServiceIndication, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_onSupplementaryServiceIndication(_arg_type, _arg_ss, _aidl_reply)
              }
              fn r#onUssd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_modeType: crate::mangled::_7_android_8_hardware_5_radio_5_voice_12_UssdModeType, _arg_msg: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_onUssd(_arg_type, _arg_modeType, _arg_msg)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#onUssd, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_onUssd(_arg_type, _arg_modeType, _arg_msg, _aidl_reply)
              }
              fn r#resendIncallMute(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_resendIncallMute(_arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#resendIncallMute, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_resendIncallMute(_arg_type, _aidl_reply)
              }
              fn r#srvccStateNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_SrvccState) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_srvccStateNotify(_arg_type, _arg_state)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#srvccStateNotify, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_srvccStateNotify(_arg_type, _arg_state, _aidl_reply)
              }
              fn r#stkCallControlAlphaNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_alpha: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_stkCallControlAlphaNotify(_arg_type, _arg_alpha)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#stkCallControlAlphaNotify, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_stkCallControlAlphaNotify(_arg_type, _arg_alpha, _aidl_reply)
              }
              fn r#stkCallSetup(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_timeout: i64) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_stkCallSetup(_arg_type, _arg_timeout)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#stkCallSetup, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_stkCallSetup(_arg_type, _arg_timeout, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioVoiceIndicationAsync<P> for BpRadioVoiceIndication {
              fn r#callRing(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_isGsm: bool, _arg_record: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_callRing(_arg_type, _arg_isGsm, _arg_record) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#callRing, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_callRing(_arg_type, _arg_isGsm, _arg_record, _aidl_reply))
              }
              fn r#callStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_callStateChanged(_arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#callStateChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_callStateChanged(_arg_type, _aidl_reply))
              }
              fn r#cdmaCallWaiting(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_callWaitingRecord: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CdmaCallWaiting) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_cdmaCallWaiting(_arg_type, _arg_callWaitingRecord) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaCallWaiting, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_cdmaCallWaiting(_arg_type, _arg_callWaitingRecord, _aidl_reply))
              }
              fn r#cdmaInfoRec(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaInformationRecord]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_cdmaInfoRec(_arg_type, _arg_records) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaInfoRec, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_cdmaInfoRec(_arg_type, _arg_records, _aidl_reply))
              }
              fn r#cdmaOtaProvisionStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_22_CdmaOtaProvisionStatus) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_cdmaOtaProvisionStatus(_arg_type, _arg_status) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaOtaProvisionStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_cdmaOtaProvisionStatus(_arg_type, _arg_status, _aidl_reply))
              }
              fn r#currentEmergencyNumberList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_emergencyNumberList: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_EmergencyNumber]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_currentEmergencyNumberList(_arg_type, _arg_emergencyNumberList) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#currentEmergencyNumberList, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_currentEmergencyNumberList(_arg_type, _arg_emergencyNumberList, _aidl_reply))
              }
              fn r#enterEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_enterEmergencyCallbackMode(_arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#enterEmergencyCallbackMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_enterEmergencyCallbackMode(_arg_type, _aidl_reply))
              }
              fn r#exitEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_exitEmergencyCallbackMode(_arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#exitEmergencyCallbackMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_exitEmergencyCallbackMode(_arg_type, _aidl_reply))
              }
              fn r#indicateRingbackTone(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_start: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_indicateRingbackTone(_arg_type, _arg_start) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#indicateRingbackTone, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_indicateRingbackTone(_arg_type, _arg_start, _aidl_reply))
              }
              fn r#onSupplementaryServiceIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_ss: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_18_StkCcUnsolSsResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_onSupplementaryServiceIndication(_arg_type, _arg_ss) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#onSupplementaryServiceIndication, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_onSupplementaryServiceIndication(_arg_type, _arg_ss, _aidl_reply))
              }
              fn r#onUssd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_modeType: crate::mangled::_7_android_8_hardware_5_radio_5_voice_12_UssdModeType, _arg_msg: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_onUssd(_arg_type, _arg_modeType, _arg_msg) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#onUssd, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_onUssd(_arg_type, _arg_modeType, _arg_msg, _aidl_reply))
              }
              fn r#resendIncallMute(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_resendIncallMute(_arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#resendIncallMute, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_resendIncallMute(_arg_type, _aidl_reply))
              }
              fn r#srvccStateNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_SrvccState) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_srvccStateNotify(_arg_type, _arg_state) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#srvccStateNotify, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_srvccStateNotify(_arg_type, _arg_state, _aidl_reply))
              }
              fn r#stkCallControlAlphaNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_alpha: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_stkCallControlAlphaNotify(_arg_type, _arg_alpha) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#stkCallControlAlphaNotify, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_stkCallControlAlphaNotify(_arg_type, _arg_alpha, _aidl_reply))
              }
              fn r#stkCallSetup(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_timeout: i64) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_stkCallSetup(_arg_type, _arg_timeout) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#stkCallSetup, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_stkCallSetup(_arg_type, _arg_timeout, _aidl_reply))
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
            impl IRadioVoiceIndication for binder::binder_impl::Binder<BnRadioVoiceIndication> {
              fn r#callRing(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_isGsm: bool, _arg_record: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord) -> binder::Result<()> { self.0.r#callRing(_arg_type, _arg_isGsm, _arg_record) }
              fn r#callStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#callStateChanged(_arg_type) }
              fn r#cdmaCallWaiting(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_callWaitingRecord: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CdmaCallWaiting) -> binder::Result<()> { self.0.r#cdmaCallWaiting(_arg_type, _arg_callWaitingRecord) }
              fn r#cdmaInfoRec(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaInformationRecord]) -> binder::Result<()> { self.0.r#cdmaInfoRec(_arg_type, _arg_records) }
              fn r#cdmaOtaProvisionStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_22_CdmaOtaProvisionStatus) -> binder::Result<()> { self.0.r#cdmaOtaProvisionStatus(_arg_type, _arg_status) }
              fn r#currentEmergencyNumberList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_emergencyNumberList: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_EmergencyNumber]) -> binder::Result<()> { self.0.r#currentEmergencyNumberList(_arg_type, _arg_emergencyNumberList) }
              fn r#enterEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#enterEmergencyCallbackMode(_arg_type) }
              fn r#exitEmergencyCallbackMode(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#exitEmergencyCallbackMode(_arg_type) }
              fn r#indicateRingbackTone(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_start: bool) -> binder::Result<()> { self.0.r#indicateRingbackTone(_arg_type, _arg_start) }
              fn r#onSupplementaryServiceIndication(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_ss: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_18_StkCcUnsolSsResult) -> binder::Result<()> { self.0.r#onSupplementaryServiceIndication(_arg_type, _arg_ss) }
              fn r#onUssd(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_modeType: crate::mangled::_7_android_8_hardware_5_radio_5_voice_12_UssdModeType, _arg_msg: &str) -> binder::Result<()> { self.0.r#onUssd(_arg_type, _arg_modeType, _arg_msg) }
              fn r#resendIncallMute(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#resendIncallMute(_arg_type) }
              fn r#srvccStateNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_SrvccState) -> binder::Result<()> { self.0.r#srvccStateNotify(_arg_type, _arg_state) }
              fn r#stkCallControlAlphaNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_alpha: &str) -> binder::Result<()> { self.0.r#stkCallControlAlphaNotify(_arg_type, _arg_alpha) }
              fn r#stkCallSetup(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_timeout: i64) -> binder::Result<()> { self.0.r#stkCallSetup(_arg_type, _arg_timeout) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioVoiceIndication, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#callRing => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_isGsm: bool = _aidl_data.read()?;
                  let _arg_record: crate::mangled::_7_android_8_hardware_5_radio_5_voice_20_CdmaSignalInfoRecord = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#callRing(_arg_type, _arg_isGsm, &_arg_record);
                  Ok(())
                }
                transactions::r#callStateChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#callStateChanged(_arg_type);
                  Ok(())
                }
                transactions::r#cdmaCallWaiting => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_callWaitingRecord: crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CdmaCallWaiting = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#cdmaCallWaiting(_arg_type, &_arg_callWaitingRecord);
                  Ok(())
                }
                transactions::r#cdmaInfoRec => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_records: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_CdmaInformationRecord> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#cdmaInfoRec(_arg_type, &_arg_records);
                  Ok(())
                }
                transactions::r#cdmaOtaProvisionStatus => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_22_CdmaOtaProvisionStatus = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#cdmaOtaProvisionStatus(_arg_type, _arg_status);
                  Ok(())
                }
                transactions::r#currentEmergencyNumberList => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_emergencyNumberList: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_EmergencyNumber> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#currentEmergencyNumberList(_arg_type, &_arg_emergencyNumberList);
                  Ok(())
                }
                transactions::r#enterEmergencyCallbackMode => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#enterEmergencyCallbackMode(_arg_type);
                  Ok(())
                }
                transactions::r#exitEmergencyCallbackMode => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#exitEmergencyCallbackMode(_arg_type);
                  Ok(())
                }
                transactions::r#indicateRingbackTone => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_start: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#indicateRingbackTone(_arg_type, _arg_start);
                  Ok(())
                }
                transactions::r#onSupplementaryServiceIndication => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_ss: crate::mangled::_7_android_8_hardware_5_radio_5_voice_18_StkCcUnsolSsResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#onSupplementaryServiceIndication(_arg_type, &_arg_ss);
                  Ok(())
                }
                transactions::r#onUssd => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_modeType: crate::mangled::_7_android_8_hardware_5_radio_5_voice_12_UssdModeType = _aidl_data.read()?;
                  let _arg_msg: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#onUssd(_arg_type, _arg_modeType, &_arg_msg);
                  Ok(())
                }
                transactions::r#resendIncallMute => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#resendIncallMute(_arg_type);
                  Ok(())
                }
                transactions::r#srvccStateNotify => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_state: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_SrvccState = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#srvccStateNotify(_arg_type, _arg_state);
                  Ok(())
                }
                transactions::r#stkCallControlAlphaNotify => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_alpha: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#stkCallControlAlphaNotify(_arg_type, &_arg_alpha);
                  Ok(())
                }
                transactions::r#stkCallSetup => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_timeout: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#stkCallSetup(_arg_type, _arg_timeout);
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
             pub use super::r#IRadioVoiceIndication as _7_android_8_hardware_5_radio_5_voice_21_IRadioVoiceIndication;
            }
          }
                    pub mod IRadioVoiceResponse {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioVoiceResponse["android.hardware.radio.voice.IRadioVoiceResponse"] {
                native: BnRadioVoiceResponse(on_transact),
                proxy: BpRadioVoiceResponse {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioVoiceResponseAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioVoiceResponse: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.voice.IRadioVoiceResponse" }
              fn r#acceptCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#cancelPendingUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#conferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#dialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#emergencyDialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#exitEmergencyCallbackModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#explicitCallTransferResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#getCallForwardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_callForwardInfos: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo]) -> binder::Result<()>;
              fn r#getCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<()>;
              fn r#getClipResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_ClipStatus) -> binder::Result<()>;
              fn r#getClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_n: i32, _arg_m: i32) -> binder::Result<()>;
              fn r#getCurrentCallsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_calls: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Call]) -> binder::Result<()>;
              fn r#getLastCallFailCauseResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_failCauseinfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_LastCallFailCauseInfo) -> binder::Result<()>;
              fn r#getMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()>;
              fn r#getPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()>;
              fn r#getTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<()>;
              fn r#handleStkCallSetupRequestFromSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#hangupConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#hangupForegroundResumeBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#hangupWaitingOrBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#isVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()>;
              fn r#rejectCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#sendBurstDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#sendCdmaFeatureCodeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#sendDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#sendUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#separateConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setCallForwardResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#startDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#stopDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#switchWaitingOrHoldingAndActiveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioVoiceResponseDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioVoiceResponseDefaultRef) -> IRadioVoiceResponseDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioVoiceResponseAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.voice.IRadioVoiceResponse" }
              fn r#acceptCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#cancelPendingUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#conferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#dialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#emergencyDialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#exitEmergencyCallbackModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#explicitCallTransferResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getCallForwardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_callForwardInfos: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#getCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _arg_serviceClass: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getClipResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_ClipStatus) -> std::future::Ready<binder::Result<()>>;
              fn r#getClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_n: i32, _arg_m: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getCurrentCallsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_calls: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Call]) -> std::future::Ready<binder::Result<()>>;
              fn r#getLastCallFailCauseResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_failCauseinfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_LastCallFailCauseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#getPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#getTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> std::future::Ready<binder::Result<()>>;
              fn r#handleStkCallSetupRequestFromSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#hangupConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#hangupForegroundResumeBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#hangupWaitingOrBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#isVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#rejectCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#sendBurstDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#sendCdmaFeatureCodeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#sendDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#sendUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#separateConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setCallForwardResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#startDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#stopDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#switchWaitingOrHoldingAndActiveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioVoiceResponseAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.voice.IRadioVoiceResponse" }
              async fn r#acceptCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#cancelPendingUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#conferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#dialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#emergencyDialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#exitEmergencyCallbackModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#explicitCallTransferResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#getCallForwardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_callForwardInfos: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo]) -> binder::Result<()>;
              async fn r#getCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<()>;
              async fn r#getClipResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_ClipStatus) -> binder::Result<()>;
              async fn r#getClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_n: i32, _arg_m: i32) -> binder::Result<()>;
              async fn r#getCurrentCallsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_calls: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Call]) -> binder::Result<()>;
              async fn r#getLastCallFailCauseResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_failCauseinfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_LastCallFailCauseInfo) -> binder::Result<()>;
              async fn r#getMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()>;
              async fn r#getPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()>;
              async fn r#getTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<()>;
              async fn r#handleStkCallSetupRequestFromSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#hangupConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#hangupForegroundResumeBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#hangupWaitingOrBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#isVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()>;
              async fn r#rejectCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#sendBurstDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#sendCdmaFeatureCodeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#sendDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#sendUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#separateConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setCallForwardResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#startDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#stopDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#switchWaitingOrHoldingAndActiveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
            }
            impl BnRadioVoiceResponse {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioVoiceResponse>
              where
                T: IRadioVoiceResponseAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioVoiceResponse for Wrapper<T, R>
                where
                  T: IRadioVoiceResponseAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#acceptCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acceptCallResponse(_arg_info))
                  }
                  fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acknowledgeRequest(_arg_serial))
                  }
                  fn r#cancelPendingUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#cancelPendingUssdResponse(_arg_info))
                  }
                  fn r#conferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#conferenceResponse(_arg_info))
                  }
                  fn r#dialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#dialResponse(_arg_info))
                  }
                  fn r#emergencyDialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#emergencyDialResponse(_arg_info))
                  }
                  fn r#exitEmergencyCallbackModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#exitEmergencyCallbackModeResponse(_arg_info))
                  }
                  fn r#explicitCallTransferResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#explicitCallTransferResponse(_arg_info))
                  }
                  fn r#getCallForwardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_callForwardInfos: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCallForwardStatusResponse(_arg_info, _arg_callForwardInfos))
                  }
                  fn r#getCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCallWaitingResponse(_arg_info, _arg_enable, _arg_serviceClass))
                  }
                  fn r#getClipResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_ClipStatus) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getClipResponse(_arg_info, _arg_status))
                  }
                  fn r#getClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_n: i32, _arg_m: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getClirResponse(_arg_info, _arg_n, _arg_m))
                  }
                  fn r#getCurrentCallsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_calls: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Call]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCurrentCallsResponse(_arg_info, _arg_calls))
                  }
                  fn r#getLastCallFailCauseResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_failCauseinfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_LastCallFailCauseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getLastCallFailCauseResponse(_arg_info, _arg_failCauseinfo))
                  }
                  fn r#getMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getMuteResponse(_arg_info, _arg_enable))
                  }
                  fn r#getPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getPreferredVoicePrivacyResponse(_arg_info, _arg_enable))
                  }
                  fn r#getTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getTtyModeResponse(_arg_info, _arg_mode))
                  }
                  fn r#handleStkCallSetupRequestFromSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#handleStkCallSetupRequestFromSimResponse(_arg_info))
                  }
                  fn r#hangupConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#hangupConnectionResponse(_arg_info))
                  }
                  fn r#hangupForegroundResumeBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#hangupForegroundResumeBackgroundResponse(_arg_info))
                  }
                  fn r#hangupWaitingOrBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#hangupWaitingOrBackgroundResponse(_arg_info))
                  }
                  fn r#isVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#isVoNrEnabledResponse(_arg_info, _arg_enable))
                  }
                  fn r#rejectCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#rejectCallResponse(_arg_info))
                  }
                  fn r#sendBurstDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendBurstDtmfResponse(_arg_info))
                  }
                  fn r#sendCdmaFeatureCodeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendCdmaFeatureCodeResponse(_arg_info))
                  }
                  fn r#sendDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendDtmfResponse(_arg_info))
                  }
                  fn r#sendUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendUssdResponse(_arg_info))
                  }
                  fn r#separateConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#separateConnectionResponse(_arg_info))
                  }
                  fn r#setCallForwardResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCallForwardResponse(_arg_info))
                  }
                  fn r#setCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCallWaitingResponse(_arg_info))
                  }
                  fn r#setClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setClirResponse(_arg_info))
                  }
                  fn r#setMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setMuteResponse(_arg_info))
                  }
                  fn r#setPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setPreferredVoicePrivacyResponse(_arg_info))
                  }
                  fn r#setTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setTtyModeResponse(_arg_info))
                  }
                  fn r#setVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setVoNrEnabledResponse(_arg_info))
                  }
                  fn r#startDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#startDtmfResponse(_arg_info))
                  }
                  fn r#stopDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#stopDtmfResponse(_arg_info))
                  }
                  fn r#switchWaitingOrHoldingAndActiveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#switchWaitingOrHoldingAndActiveResponse(_arg_info))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioVoiceResponseDefault: Send + Sync {
              fn r#acceptCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#cancelPendingUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#conferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#dialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#emergencyDialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#exitEmergencyCallbackModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#explicitCallTransferResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCallForwardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_callForwardInfos: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getClipResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_ClipStatus) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_n: i32, _arg_m: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCurrentCallsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_calls: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Call]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getLastCallFailCauseResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_failCauseinfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_LastCallFailCauseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#handleStkCallSetupRequestFromSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#hangupConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#hangupForegroundResumeBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#hangupWaitingOrBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#isVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#rejectCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendBurstDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendCdmaFeatureCodeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#separateConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCallForwardResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#startDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#stopDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#switchWaitingOrHoldingAndActiveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#acceptCallResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#acknowledgeRequest: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#cancelPendingUssdResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#conferenceResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#dialResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#emergencyDialResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#exitEmergencyCallbackModeResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#explicitCallTransferResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#getCallForwardStatusResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#getCallWaitingResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#getClipResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#getClirResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#getCurrentCallsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#getLastCallFailCauseResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#getMuteResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#getPreferredVoicePrivacyResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#getTtyModeResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
              pub const r#handleStkCallSetupRequestFromSimResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
              pub const r#hangupConnectionResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
              pub const r#hangupForegroundResumeBackgroundResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
              pub const r#hangupWaitingOrBackgroundResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 20;
              pub const r#isVoNrEnabledResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 21;
              pub const r#rejectCallResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 22;
              pub const r#sendBurstDtmfResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 23;
              pub const r#sendCdmaFeatureCodeResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 24;
              pub const r#sendDtmfResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 25;
              pub const r#sendUssdResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 26;
              pub const r#separateConnectionResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 27;
              pub const r#setCallForwardResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 28;
              pub const r#setCallWaitingResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 29;
              pub const r#setClirResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 30;
              pub const r#setMuteResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 31;
              pub const r#setPreferredVoicePrivacyResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 32;
              pub const r#setTtyModeResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 33;
              pub const r#setVoNrEnabledResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 34;
              pub const r#startDtmfResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 35;
              pub const r#stopDtmfResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 36;
              pub const r#switchWaitingOrHoldingAndActiveResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 37;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioVoiceResponseDefaultRef = Option<std::sync::Arc<dyn IRadioVoiceResponseDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioVoiceResponseDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "e9ffc70247a89e6c1e526c6334c37da46f33ebea";
            impl BpRadioVoiceResponse {
              fn build_parcel_acceptCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_acceptCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#acceptCallResponse(_arg_info);
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
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#acknowledgeRequest(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_cancelPendingUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_cancelPendingUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#cancelPendingUssdResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_conferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_conferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#conferenceResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_dialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_dialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#dialResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_emergencyDialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_emergencyDialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#emergencyDialResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_exitEmergencyCallbackModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_exitEmergencyCallbackModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#exitEmergencyCallbackModeResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_explicitCallTransferResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_explicitCallTransferResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#explicitCallTransferResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCallForwardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_callForwardInfos: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_callForwardInfos)?;
                Ok(aidl_data)
              }
              fn read_response_getCallForwardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_callForwardInfos: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCallForwardStatusResponse(_arg_info, _arg_callForwardInfos);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_enable)?;
                aidl_data.write(&_arg_serviceClass)?;
                Ok(aidl_data)
              }
              fn read_response_getCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _arg_serviceClass: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCallWaitingResponse(_arg_info, _arg_enable, _arg_serviceClass);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getClipResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_ClipStatus) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_status)?;
                Ok(aidl_data)
              }
              fn read_response_getClipResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_ClipStatus, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getClipResponse(_arg_info, _arg_status);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_n: i32, _arg_m: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_n)?;
                aidl_data.write(&_arg_m)?;
                Ok(aidl_data)
              }
              fn read_response_getClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_n: i32, _arg_m: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getClirResponse(_arg_info, _arg_n, _arg_m);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCurrentCallsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_calls: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Call]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_calls)?;
                Ok(aidl_data)
              }
              fn read_response_getCurrentCallsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_calls: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Call], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCurrentCallsResponse(_arg_info, _arg_calls);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getLastCallFailCauseResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_failCauseinfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_LastCallFailCauseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_failCauseinfo)?;
                Ok(aidl_data)
              }
              fn read_response_getLastCallFailCauseResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_failCauseinfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_LastCallFailCauseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getLastCallFailCauseResponse(_arg_info, _arg_failCauseinfo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_enable)?;
                Ok(aidl_data)
              }
              fn read_response_getMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getMuteResponse(_arg_info, _arg_enable);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_enable)?;
                Ok(aidl_data)
              }
              fn read_response_getPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getPreferredVoicePrivacyResponse(_arg_info, _arg_enable);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_mode)?;
                Ok(aidl_data)
              }
              fn read_response_getTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getTtyModeResponse(_arg_info, _arg_mode);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_handleStkCallSetupRequestFromSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_handleStkCallSetupRequestFromSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#handleStkCallSetupRequestFromSimResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_hangupConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_hangupConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#hangupConnectionResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_hangupForegroundResumeBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_hangupForegroundResumeBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#hangupForegroundResumeBackgroundResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_hangupWaitingOrBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_hangupWaitingOrBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#hangupWaitingOrBackgroundResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_isVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_enable)?;
                Ok(aidl_data)
              }
              fn read_response_isVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#isVoNrEnabledResponse(_arg_info, _arg_enable);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_rejectCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_rejectCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#rejectCallResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendBurstDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_sendBurstDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendBurstDtmfResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendCdmaFeatureCodeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_sendCdmaFeatureCodeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendCdmaFeatureCodeResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_sendDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendDtmfResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_sendUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_sendUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendUssdResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_separateConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_separateConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#separateConnectionResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCallForwardResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setCallForwardResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCallForwardResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCallWaitingResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setClirResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setMuteResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setPreferredVoicePrivacyResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setTtyModeResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setVoNrEnabledResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_startDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_startDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#startDtmfResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_stopDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_stopDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#stopDtmfResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_switchWaitingOrHoldingAndActiveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_switchWaitingOrHoldingAndActiveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioVoiceResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#switchWaitingOrHoldingAndActiveResponse(_arg_info);
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
            impl IRadioVoiceResponse for BpRadioVoiceResponse {
              fn r#acceptCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acceptCallResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acceptCallResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acceptCallResponse(_arg_info, _aidl_reply)
              }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acknowledgeRequest(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeRequest, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acknowledgeRequest(_arg_serial, _aidl_reply)
              }
              fn r#cancelPendingUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_cancelPendingUssdResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#cancelPendingUssdResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_cancelPendingUssdResponse(_arg_info, _aidl_reply)
              }
              fn r#conferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_conferenceResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#conferenceResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_conferenceResponse(_arg_info, _aidl_reply)
              }
              fn r#dialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_dialResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#dialResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_dialResponse(_arg_info, _aidl_reply)
              }
              fn r#emergencyDialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_emergencyDialResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#emergencyDialResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_emergencyDialResponse(_arg_info, _aidl_reply)
              }
              fn r#exitEmergencyCallbackModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_exitEmergencyCallbackModeResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#exitEmergencyCallbackModeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_exitEmergencyCallbackModeResponse(_arg_info, _aidl_reply)
              }
              fn r#explicitCallTransferResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_explicitCallTransferResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#explicitCallTransferResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_explicitCallTransferResponse(_arg_info, _aidl_reply)
              }
              fn r#getCallForwardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_callForwardInfos: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCallForwardStatusResponse(_arg_info, _arg_callForwardInfos)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCallForwardStatusResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCallForwardStatusResponse(_arg_info, _arg_callForwardInfos, _aidl_reply)
              }
              fn r#getCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCallWaitingResponse(_arg_info, _arg_enable, _arg_serviceClass)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCallWaitingResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCallWaitingResponse(_arg_info, _arg_enable, _arg_serviceClass, _aidl_reply)
              }
              fn r#getClipResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_ClipStatus) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getClipResponse(_arg_info, _arg_status)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getClipResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getClipResponse(_arg_info, _arg_status, _aidl_reply)
              }
              fn r#getClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_n: i32, _arg_m: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getClirResponse(_arg_info, _arg_n, _arg_m)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getClirResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getClirResponse(_arg_info, _arg_n, _arg_m, _aidl_reply)
              }
              fn r#getCurrentCallsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_calls: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Call]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCurrentCallsResponse(_arg_info, _arg_calls)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCurrentCallsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCurrentCallsResponse(_arg_info, _arg_calls, _aidl_reply)
              }
              fn r#getLastCallFailCauseResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_failCauseinfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_LastCallFailCauseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getLastCallFailCauseResponse(_arg_info, _arg_failCauseinfo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getLastCallFailCauseResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getLastCallFailCauseResponse(_arg_info, _arg_failCauseinfo, _aidl_reply)
              }
              fn r#getMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getMuteResponse(_arg_info, _arg_enable)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getMuteResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getMuteResponse(_arg_info, _arg_enable, _aidl_reply)
              }
              fn r#getPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getPreferredVoicePrivacyResponse(_arg_info, _arg_enable)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getPreferredVoicePrivacyResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getPreferredVoicePrivacyResponse(_arg_info, _arg_enable, _aidl_reply)
              }
              fn r#getTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getTtyModeResponse(_arg_info, _arg_mode)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getTtyModeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getTtyModeResponse(_arg_info, _arg_mode, _aidl_reply)
              }
              fn r#handleStkCallSetupRequestFromSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_handleStkCallSetupRequestFromSimResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#handleStkCallSetupRequestFromSimResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_handleStkCallSetupRequestFromSimResponse(_arg_info, _aidl_reply)
              }
              fn r#hangupConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_hangupConnectionResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#hangupConnectionResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_hangupConnectionResponse(_arg_info, _aidl_reply)
              }
              fn r#hangupForegroundResumeBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_hangupForegroundResumeBackgroundResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#hangupForegroundResumeBackgroundResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_hangupForegroundResumeBackgroundResponse(_arg_info, _aidl_reply)
              }
              fn r#hangupWaitingOrBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_hangupWaitingOrBackgroundResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#hangupWaitingOrBackgroundResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_hangupWaitingOrBackgroundResponse(_arg_info, _aidl_reply)
              }
              fn r#isVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_isVoNrEnabledResponse(_arg_info, _arg_enable)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#isVoNrEnabledResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_isVoNrEnabledResponse(_arg_info, _arg_enable, _aidl_reply)
              }
              fn r#rejectCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_rejectCallResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#rejectCallResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_rejectCallResponse(_arg_info, _aidl_reply)
              }
              fn r#sendBurstDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendBurstDtmfResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendBurstDtmfResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendBurstDtmfResponse(_arg_info, _aidl_reply)
              }
              fn r#sendCdmaFeatureCodeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendCdmaFeatureCodeResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendCdmaFeatureCodeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendCdmaFeatureCodeResponse(_arg_info, _aidl_reply)
              }
              fn r#sendDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendDtmfResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendDtmfResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendDtmfResponse(_arg_info, _aidl_reply)
              }
              fn r#sendUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendUssdResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendUssdResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_sendUssdResponse(_arg_info, _aidl_reply)
              }
              fn r#separateConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_separateConnectionResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#separateConnectionResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_separateConnectionResponse(_arg_info, _aidl_reply)
              }
              fn r#setCallForwardResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCallForwardResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCallForwardResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCallForwardResponse(_arg_info, _aidl_reply)
              }
              fn r#setCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCallWaitingResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCallWaitingResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCallWaitingResponse(_arg_info, _aidl_reply)
              }
              fn r#setClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setClirResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setClirResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setClirResponse(_arg_info, _aidl_reply)
              }
              fn r#setMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setMuteResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setMuteResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setMuteResponse(_arg_info, _aidl_reply)
              }
              fn r#setPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setPreferredVoicePrivacyResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setPreferredVoicePrivacyResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setPreferredVoicePrivacyResponse(_arg_info, _aidl_reply)
              }
              fn r#setTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setTtyModeResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setTtyModeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setTtyModeResponse(_arg_info, _aidl_reply)
              }
              fn r#setVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setVoNrEnabledResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setVoNrEnabledResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setVoNrEnabledResponse(_arg_info, _aidl_reply)
              }
              fn r#startDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_startDtmfResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#startDtmfResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_startDtmfResponse(_arg_info, _aidl_reply)
              }
              fn r#stopDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_stopDtmfResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#stopDtmfResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_stopDtmfResponse(_arg_info, _aidl_reply)
              }
              fn r#switchWaitingOrHoldingAndActiveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_switchWaitingOrHoldingAndActiveResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#switchWaitingOrHoldingAndActiveResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_switchWaitingOrHoldingAndActiveResponse(_arg_info, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioVoiceResponseAsync<P> for BpRadioVoiceResponse {
              fn r#acceptCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acceptCallResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acceptCallResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acceptCallResponse(_arg_info, _aidl_reply))
              }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acknowledgeRequest(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeRequest, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acknowledgeRequest(_arg_serial, _aidl_reply))
              }
              fn r#cancelPendingUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_cancelPendingUssdResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#cancelPendingUssdResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_cancelPendingUssdResponse(_arg_info, _aidl_reply))
              }
              fn r#conferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_conferenceResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#conferenceResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_conferenceResponse(_arg_info, _aidl_reply))
              }
              fn r#dialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_dialResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#dialResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_dialResponse(_arg_info, _aidl_reply))
              }
              fn r#emergencyDialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_emergencyDialResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#emergencyDialResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_emergencyDialResponse(_arg_info, _aidl_reply))
              }
              fn r#exitEmergencyCallbackModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_exitEmergencyCallbackModeResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#exitEmergencyCallbackModeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_exitEmergencyCallbackModeResponse(_arg_info, _aidl_reply))
              }
              fn r#explicitCallTransferResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_explicitCallTransferResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#explicitCallTransferResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_explicitCallTransferResponse(_arg_info, _aidl_reply))
              }
              fn r#getCallForwardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_callForwardInfos: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCallForwardStatusResponse(_arg_info, _arg_callForwardInfos) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCallForwardStatusResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCallForwardStatusResponse(_arg_info, _arg_callForwardInfos, _aidl_reply))
              }
              fn r#getCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _arg_serviceClass: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCallWaitingResponse(_arg_info, _arg_enable, _arg_serviceClass) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCallWaitingResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCallWaitingResponse(_arg_info, _arg_enable, _arg_serviceClass, _aidl_reply))
              }
              fn r#getClipResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_ClipStatus) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getClipResponse(_arg_info, _arg_status) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getClipResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getClipResponse(_arg_info, _arg_status, _aidl_reply))
              }
              fn r#getClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_n: i32, _arg_m: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getClirResponse(_arg_info, _arg_n, _arg_m) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getClirResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getClirResponse(_arg_info, _arg_n, _arg_m, _aidl_reply))
              }
              fn r#getCurrentCallsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_calls: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Call]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCurrentCallsResponse(_arg_info, _arg_calls) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCurrentCallsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCurrentCallsResponse(_arg_info, _arg_calls, _aidl_reply))
              }
              fn r#getLastCallFailCauseResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_failCauseinfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_LastCallFailCauseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getLastCallFailCauseResponse(_arg_info, _arg_failCauseinfo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getLastCallFailCauseResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getLastCallFailCauseResponse(_arg_info, _arg_failCauseinfo, _aidl_reply))
              }
              fn r#getMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getMuteResponse(_arg_info, _arg_enable) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getMuteResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getMuteResponse(_arg_info, _arg_enable, _aidl_reply))
              }
              fn r#getPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getPreferredVoicePrivacyResponse(_arg_info, _arg_enable) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getPreferredVoicePrivacyResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getPreferredVoicePrivacyResponse(_arg_info, _arg_enable, _aidl_reply))
              }
              fn r#getTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getTtyModeResponse(_arg_info, _arg_mode) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getTtyModeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getTtyModeResponse(_arg_info, _arg_mode, _aidl_reply))
              }
              fn r#handleStkCallSetupRequestFromSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_handleStkCallSetupRequestFromSimResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#handleStkCallSetupRequestFromSimResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_handleStkCallSetupRequestFromSimResponse(_arg_info, _aidl_reply))
              }
              fn r#hangupConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_hangupConnectionResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#hangupConnectionResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_hangupConnectionResponse(_arg_info, _aidl_reply))
              }
              fn r#hangupForegroundResumeBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_hangupForegroundResumeBackgroundResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#hangupForegroundResumeBackgroundResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_hangupForegroundResumeBackgroundResponse(_arg_info, _aidl_reply))
              }
              fn r#hangupWaitingOrBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_hangupWaitingOrBackgroundResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#hangupWaitingOrBackgroundResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_hangupWaitingOrBackgroundResponse(_arg_info, _aidl_reply))
              }
              fn r#isVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_isVoNrEnabledResponse(_arg_info, _arg_enable) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#isVoNrEnabledResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_isVoNrEnabledResponse(_arg_info, _arg_enable, _aidl_reply))
              }
              fn r#rejectCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_rejectCallResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#rejectCallResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_rejectCallResponse(_arg_info, _aidl_reply))
              }
              fn r#sendBurstDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendBurstDtmfResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendBurstDtmfResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendBurstDtmfResponse(_arg_info, _aidl_reply))
              }
              fn r#sendCdmaFeatureCodeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendCdmaFeatureCodeResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendCdmaFeatureCodeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendCdmaFeatureCodeResponse(_arg_info, _aidl_reply))
              }
              fn r#sendDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendDtmfResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendDtmfResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendDtmfResponse(_arg_info, _aidl_reply))
              }
              fn r#sendUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendUssdResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendUssdResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_sendUssdResponse(_arg_info, _aidl_reply))
              }
              fn r#separateConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_separateConnectionResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#separateConnectionResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_separateConnectionResponse(_arg_info, _aidl_reply))
              }
              fn r#setCallForwardResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCallForwardResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCallForwardResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCallForwardResponse(_arg_info, _aidl_reply))
              }
              fn r#setCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCallWaitingResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCallWaitingResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCallWaitingResponse(_arg_info, _aidl_reply))
              }
              fn r#setClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setClirResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setClirResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setClirResponse(_arg_info, _aidl_reply))
              }
              fn r#setMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setMuteResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setMuteResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setMuteResponse(_arg_info, _aidl_reply))
              }
              fn r#setPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setPreferredVoicePrivacyResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setPreferredVoicePrivacyResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setPreferredVoicePrivacyResponse(_arg_info, _aidl_reply))
              }
              fn r#setTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setTtyModeResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setTtyModeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setTtyModeResponse(_arg_info, _aidl_reply))
              }
              fn r#setVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setVoNrEnabledResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setVoNrEnabledResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setVoNrEnabledResponse(_arg_info, _aidl_reply))
              }
              fn r#startDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_startDtmfResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#startDtmfResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_startDtmfResponse(_arg_info, _aidl_reply))
              }
              fn r#stopDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_stopDtmfResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#stopDtmfResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_stopDtmfResponse(_arg_info, _aidl_reply))
              }
              fn r#switchWaitingOrHoldingAndActiveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_switchWaitingOrHoldingAndActiveResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#switchWaitingOrHoldingAndActiveResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_switchWaitingOrHoldingAndActiveResponse(_arg_info, _aidl_reply))
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
            impl IRadioVoiceResponse for binder::binder_impl::Binder<BnRadioVoiceResponse> {
              fn r#acceptCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#acceptCallResponse(_arg_info) }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#acknowledgeRequest(_arg_serial) }
              fn r#cancelPendingUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#cancelPendingUssdResponse(_arg_info) }
              fn r#conferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#conferenceResponse(_arg_info) }
              fn r#dialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#dialResponse(_arg_info) }
              fn r#emergencyDialResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#emergencyDialResponse(_arg_info) }
              fn r#exitEmergencyCallbackModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#exitEmergencyCallbackModeResponse(_arg_info) }
              fn r#explicitCallTransferResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#explicitCallTransferResponse(_arg_info) }
              fn r#getCallForwardStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_callForwardInfos: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo]) -> binder::Result<()> { self.0.r#getCallForwardStatusResponse(_arg_info, _arg_callForwardInfos) }
              fn r#getCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool, _arg_serviceClass: i32) -> binder::Result<()> { self.0.r#getCallWaitingResponse(_arg_info, _arg_enable, _arg_serviceClass) }
              fn r#getClipResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_ClipStatus) -> binder::Result<()> { self.0.r#getClipResponse(_arg_info, _arg_status) }
              fn r#getClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_n: i32, _arg_m: i32) -> binder::Result<()> { self.0.r#getClirResponse(_arg_info, _arg_n, _arg_m) }
              fn r#getCurrentCallsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_calls: &[crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Call]) -> binder::Result<()> { self.0.r#getCurrentCallsResponse(_arg_info, _arg_calls) }
              fn r#getLastCallFailCauseResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_failCauseinfo: &crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_LastCallFailCauseInfo) -> binder::Result<()> { self.0.r#getLastCallFailCauseResponse(_arg_info, _arg_failCauseinfo) }
              fn r#getMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()> { self.0.r#getMuteResponse(_arg_info, _arg_enable) }
              fn r#getPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()> { self.0.r#getPreferredVoicePrivacyResponse(_arg_info, _arg_enable) }
              fn r#getTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode) -> binder::Result<()> { self.0.r#getTtyModeResponse(_arg_info, _arg_mode) }
              fn r#handleStkCallSetupRequestFromSimResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#handleStkCallSetupRequestFromSimResponse(_arg_info) }
              fn r#hangupConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#hangupConnectionResponse(_arg_info) }
              fn r#hangupForegroundResumeBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#hangupForegroundResumeBackgroundResponse(_arg_info) }
              fn r#hangupWaitingOrBackgroundResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#hangupWaitingOrBackgroundResponse(_arg_info) }
              fn r#isVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_enable: bool) -> binder::Result<()> { self.0.r#isVoNrEnabledResponse(_arg_info, _arg_enable) }
              fn r#rejectCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#rejectCallResponse(_arg_info) }
              fn r#sendBurstDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#sendBurstDtmfResponse(_arg_info) }
              fn r#sendCdmaFeatureCodeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#sendCdmaFeatureCodeResponse(_arg_info) }
              fn r#sendDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#sendDtmfResponse(_arg_info) }
              fn r#sendUssdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#sendUssdResponse(_arg_info) }
              fn r#separateConnectionResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#separateConnectionResponse(_arg_info) }
              fn r#setCallForwardResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setCallForwardResponse(_arg_info) }
              fn r#setCallWaitingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setCallWaitingResponse(_arg_info) }
              fn r#setClirResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setClirResponse(_arg_info) }
              fn r#setMuteResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setMuteResponse(_arg_info) }
              fn r#setPreferredVoicePrivacyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setPreferredVoicePrivacyResponse(_arg_info) }
              fn r#setTtyModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setTtyModeResponse(_arg_info) }
              fn r#setVoNrEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setVoNrEnabledResponse(_arg_info) }
              fn r#startDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#startDtmfResponse(_arg_info) }
              fn r#stopDtmfResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#stopDtmfResponse(_arg_info) }
              fn r#switchWaitingOrHoldingAndActiveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#switchWaitingOrHoldingAndActiveResponse(_arg_info) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioVoiceResponse, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#acceptCallResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acceptCallResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#acknowledgeRequest => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acknowledgeRequest(_arg_serial);
                  Ok(())
                }
                transactions::r#cancelPendingUssdResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#cancelPendingUssdResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#conferenceResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#conferenceResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#dialResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#dialResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#emergencyDialResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#emergencyDialResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#exitEmergencyCallbackModeResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#exitEmergencyCallbackModeResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#explicitCallTransferResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#explicitCallTransferResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#getCallForwardStatusResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_callForwardInfos: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_15_CallForwardInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCallForwardStatusResponse(&_arg_info, &_arg_callForwardInfos);
                  Ok(())
                }
                transactions::r#getCallWaitingResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_enable: bool = _aidl_data.read()?;
                  let _arg_serviceClass: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCallWaitingResponse(&_arg_info, _arg_enable, _arg_serviceClass);
                  Ok(())
                }
                transactions::r#getClipResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_status: crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_ClipStatus = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getClipResponse(&_arg_info, _arg_status);
                  Ok(())
                }
                transactions::r#getClirResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_n: i32 = _aidl_data.read()?;
                  let _arg_m: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getClirResponse(&_arg_info, _arg_n, _arg_m);
                  Ok(())
                }
                transactions::r#getCurrentCallsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_calls: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_4_Call> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCurrentCallsResponse(&_arg_info, &_arg_calls);
                  Ok(())
                }
                transactions::r#getLastCallFailCauseResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_failCauseinfo: crate::mangled::_7_android_8_hardware_5_radio_5_voice_21_LastCallFailCauseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getLastCallFailCauseResponse(&_arg_info, &_arg_failCauseinfo);
                  Ok(())
                }
                transactions::r#getMuteResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_enable: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getMuteResponse(&_arg_info, _arg_enable);
                  Ok(())
                }
                transactions::r#getPreferredVoicePrivacyResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_enable: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getPreferredVoicePrivacyResponse(&_arg_info, _arg_enable);
                  Ok(())
                }
                transactions::r#getTtyModeResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_7_TtyMode = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getTtyModeResponse(&_arg_info, _arg_mode);
                  Ok(())
                }
                transactions::r#handleStkCallSetupRequestFromSimResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#handleStkCallSetupRequestFromSimResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#hangupConnectionResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#hangupConnectionResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#hangupForegroundResumeBackgroundResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#hangupForegroundResumeBackgroundResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#hangupWaitingOrBackgroundResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#hangupWaitingOrBackgroundResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#isVoNrEnabledResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_enable: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#isVoNrEnabledResponse(&_arg_info, _arg_enable);
                  Ok(())
                }
                transactions::r#rejectCallResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#rejectCallResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#sendBurstDtmfResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendBurstDtmfResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#sendCdmaFeatureCodeResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendCdmaFeatureCodeResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#sendDtmfResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendDtmfResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#sendUssdResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendUssdResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#separateConnectionResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#separateConnectionResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setCallForwardResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCallForwardResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setCallWaitingResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCallWaitingResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setClirResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setClirResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setMuteResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setMuteResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setPreferredVoicePrivacyResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setPreferredVoicePrivacyResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setTtyModeResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setTtyModeResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setVoNrEnabledResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setVoNrEnabledResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#startDtmfResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#startDtmfResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#stopDtmfResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#stopDtmfResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#switchWaitingOrHoldingAndActiveResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#switchWaitingOrHoldingAndActiveResponse(&_arg_info);
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
             pub use super::r#IRadioVoiceResponse as _7_android_8_hardware_5_radio_5_voice_19_IRadioVoiceResponse;
            }
          }
                    pub mod LastCallFailCause {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#LastCallFailCause : [i32; 96] {
                r#UNOBTAINABLE_NUMBER = 1,
                r#NO_ROUTE_TO_DESTINATION = 3,
                r#CHANNEL_UNACCEPTABLE = 6,
                r#OPERATOR_DETERMINED_BARRING = 8,
                r#NORMAL = 16,
                r#BUSY = 17,
                r#NO_USER_RESPONDING = 18,
                r#NO_ANSWER_FROM_USER = 19,
                r#CALL_REJECTED = 21,
                r#NUMBER_CHANGED = 22,
                r#PREEMPTION = 25,
                r#DESTINATION_OUT_OF_ORDER = 27,
                r#INVALID_NUMBER_FORMAT = 28,
                r#FACILITY_REJECTED = 29,
                r#RESP_TO_STATUS_ENQUIRY = 30,
                r#NORMAL_UNSPECIFIED = 31,
                r#CONGESTION = 34,
                r#NETWORK_OUT_OF_ORDER = 38,
                r#TEMPORARY_FAILURE = 41,
                r#SWITCHING_EQUIPMENT_CONGESTION = 42,
                r#ACCESS_INFORMATION_DISCARDED = 43,
                r#REQUESTED_CIRCUIT_OR_CHANNEL_NOT_AVAILABLE = 44,
                r#RESOURCES_UNAVAILABLE_OR_UNSPECIFIED = 47,
                r#QOS_UNAVAILABLE = 49,
                r#REQUESTED_FACILITY_NOT_SUBSCRIBED = 50,
                r#INCOMING_CALLS_BARRED_WITHIN_CUG = 55,
                r#BEARER_CAPABILITY_NOT_AUTHORIZED = 57,
                r#BEARER_CAPABILITY_UNAVAILABLE = 58,
                r#SERVICE_OPTION_NOT_AVAILABLE = 63,
                r#BEARER_SERVICE_NOT_IMPLEMENTED = 65,
                r#ACM_LIMIT_EXCEEDED = 68,
                r#REQUESTED_FACILITY_NOT_IMPLEMENTED = 69,
                r#ONLY_DIGITAL_INFORMATION_BEARER_AVAILABLE = 70,
                r#SERVICE_OR_OPTION_NOT_IMPLEMENTED = 79,
                r#INVALID_TRANSACTION_IDENTIFIER = 81,
                r#USER_NOT_MEMBER_OF_CUG = 87,
                r#INCOMPATIBLE_DESTINATION = 88,
                r#INVALID_TRANSIT_NW_SELECTION = 91,
                r#SEMANTICALLY_INCORRECT_MESSAGE = 95,
                r#INVALID_MANDATORY_INFORMATION = 96,
                r#MESSAGE_TYPE_NON_IMPLEMENTED = 97,
                r#MESSAGE_TYPE_NOT_COMPATIBLE_WITH_PROTOCOL_STATE = 98,
                r#INFORMATION_ELEMENT_NON_EXISTENT = 99,
                r#CONDITIONAL_IE_ERROR = 100,
                r#MESSAGE_NOT_COMPATIBLE_WITH_PROTOCOL_STATE = 101,
                r#RECOVERY_ON_TIMER_EXPIRED = 102,
                r#PROTOCOL_ERROR_UNSPECIFIED = 111,
                r#INTERWORKING_UNSPECIFIED = 127,
                r#CALL_BARRED = 240,
                r#FDN_BLOCKED = 241,
                r#IMSI_UNKNOWN_IN_VLR = 242,
                r#IMEI_NOT_ACCEPTED = 243,
                r#DIAL_MODIFIED_TO_USSD = 244,
                r#DIAL_MODIFIED_TO_SS = 245,
                r#DIAL_MODIFIED_TO_DIAL = 246,
                r#RADIO_OFF = 247,
                r#OUT_OF_SERVICE = 248,
                r#NO_VALID_SIM = 249,
                r#RADIO_INTERNAL_ERROR = 250,
                r#NETWORK_RESP_TIMEOUT = 251,
                r#NETWORK_REJECT = 252,
                r#RADIO_ACCESS_FAILURE = 253,
                r#RADIO_LINK_FAILURE = 254,
                r#RADIO_LINK_LOST = 255,
                r#RADIO_UPLINK_FAILURE = 256,
                r#RADIO_SETUP_FAILURE = 257,
                r#RADIO_RELEASE_NORMAL = 258,
                r#RADIO_RELEASE_ABNORMAL = 259,
                r#ACCESS_CLASS_BLOCKED = 260,
                r#NETWORK_DETACH = 261,
                r#CDMA_LOCKED_UNTIL_POWER_CYCLE = 1000,
                r#CDMA_DROP = 1001,
                r#CDMA_INTERCEPT = 1002,
                r#CDMA_REORDER = 1003,
                r#CDMA_SO_REJECT = 1004,
                r#CDMA_RETRY_ORDER = 1005,
                r#CDMA_ACCESS_FAILURE = 1006,
                r#CDMA_PREEMPTED = 1007,
                r#CDMA_NOT_EMERGENCY = 1008,
                r#CDMA_ACCESS_BLOCKED = 1009,
                r#OEM_CAUSE_1 = 61441,
                r#OEM_CAUSE_2 = 61442,
                r#OEM_CAUSE_3 = 61443,
                r#OEM_CAUSE_4 = 61444,
                r#OEM_CAUSE_5 = 61445,
                r#OEM_CAUSE_6 = 61446,
                r#OEM_CAUSE_7 = 61447,
                r#OEM_CAUSE_8 = 61448,
                r#OEM_CAUSE_9 = 61449,
                r#OEM_CAUSE_10 = 61450,
                r#OEM_CAUSE_11 = 61451,
                r#OEM_CAUSE_12 = 61452,
                r#OEM_CAUSE_13 = 61453,
                r#OEM_CAUSE_14 = 61454,
                r#OEM_CAUSE_15 = 61455,
                r#ERROR_UNSPECIFIED = 65535,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#LastCallFailCause as _7_android_8_hardware_5_radio_5_voice_17_LastCallFailCause;
            }
          }
                    pub mod LastCallFailCauseInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#LastCallFailCauseInfo {
              pub r#causeCode: crate::mangled::_7_android_8_hardware_5_radio_5_voice_17_LastCallFailCause,
              pub r#vendorCause: String,
            }
            impl Default for r#LastCallFailCauseInfo {
              fn default() -> Self {
                Self {
                  r#causeCode: Default::default(),
                  r#vendorCause: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#LastCallFailCauseInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#causeCode)?;
                  subparcel.write(&self.r#vendorCause)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#causeCode = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#vendorCause = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#LastCallFailCauseInfo);
            binder::impl_deserialize_for_parcelable!(r#LastCallFailCauseInfo);
            impl binder::binder_impl::ParcelableMetadata for r#LastCallFailCauseInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.LastCallFailCauseInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#LastCallFailCauseInfo as _7_android_8_hardware_5_radio_5_voice_21_LastCallFailCauseInfo;
            }
          }
                    pub mod SrvccState {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#SrvccState : [i32; 4] {
                r#HANDOVER_STARTED = 0,
                r#HANDOVER_COMPLETED = 1,
                r#HANDOVER_FAILED = 2,
                r#HANDOVER_CANCELED = 3,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#SrvccState as _7_android_8_hardware_5_radio_5_voice_10_SrvccState;
            }
          }
                    pub mod SsInfoData {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#SsInfoData {
              pub r#ssInfo: Vec<i32>,
            }
            pub const r#SS_INFO_MAX: i32 = 4;
            impl Default for r#SsInfoData {
              fn default() -> Self {
                Self {
                  r#ssInfo: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#SsInfoData {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#ssInfo)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#ssInfo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SsInfoData);
            binder::impl_deserialize_for_parcelable!(r#SsInfoData);
            impl binder::binder_impl::ParcelableMetadata for r#SsInfoData {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.SsInfoData" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SsInfoData as _7_android_8_hardware_5_radio_5_voice_10_SsInfoData;
            }
          }
                    pub mod StkCcUnsolSsResult {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#StkCcUnsolSsResult {
              pub r#serviceType: i32,
              pub r#requestType: i32,
              pub r#teleserviceType: i32,
              pub r#serviceClass: i32,
              pub r#result: crate::mangled::_7_android_8_hardware_5_radio_10_RadioError,
              pub r#ssInfo: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_10_SsInfoData>,
              pub r#cfData: Vec<crate::mangled::_7_android_8_hardware_5_radio_5_voice_6_CfData>,
            }
            pub const r#REQUEST_TYPE_ACTIVATION: i32 = 0;
            pub const r#REQUEST_TYPE_DEACTIVATION: i32 = 1;
            pub const r#REQUEST_TYPE_INTERROGATION: i32 = 2;
            pub const r#REQUEST_TYPE_REGISTRATION: i32 = 3;
            pub const r#REQUEST_TYPE_ERASURE: i32 = 4;
            pub const r#SERVICE_TYPE_CFU: i32 = 0;
            pub const r#SERVICE_TYPE_CF_BUSY: i32 = 1;
            pub const r#SERVICE_TYPE_CF_NO_REPLY: i32 = 2;
            pub const r#SERVICE_TYPE_CF_NOT_REACHABLE: i32 = 3;
            pub const r#SERVICE_TYPE_CF_ALL: i32 = 4;
            pub const r#SERVICE_TYPE_CF_ALL_CONDITIONAL: i32 = 5;
            pub const r#SERVICE_TYPE_CLIP: i32 = 6;
            pub const r#SERVICE_TYPE_CLIR: i32 = 7;
            pub const r#SERVICE_TYPE_COLP: i32 = 8;
            pub const r#SERVICE_TYPE_COLR: i32 = 9;
            pub const r#SERVICE_TYPE_WAIT: i32 = 10;
            pub const r#SERVICE_TYPE_BAOC: i32 = 11;
            pub const r#SERVICE_TYPE_BAOIC: i32 = 12;
            pub const r#SERVICE_TYPE_BAOIC_EXC_HOME: i32 = 13;
            pub const r#SERVICE_TYPE_BAIC: i32 = 14;
            pub const r#SERVICE_TYPE_BAIC_ROAMING: i32 = 15;
            pub const r#SERVICE_TYPE_ALL_BARRING: i32 = 16;
            pub const r#SERVICE_TYPE_OUTGOING_BARRING: i32 = 17;
            pub const r#SERVICE_TYPE_INCOMING_BARRING: i32 = 18;
            pub const r#TELESERVICE_TYPE_ALL_TELE_AND_BEARER_SERVICES: i32 = 0;
            pub const r#TELESERVICE_TYPE_ALL_TELESEVICES: i32 = 1;
            pub const r#TELESERVICE_TYPE_TELEPHONY: i32 = 2;
            pub const r#TELESERVICE_TYPE_ALL_DATA_TELESERVICES: i32 = 3;
            pub const r#TELESERVICE_TYPE_SMS_SERVICES: i32 = 4;
            pub const r#TELESERVICE_TYPE_ALL_TELESERVICES_EXCEPT_SMS: i32 = 5;
            pub const r#SUPP_SERVICE_CLASS_NONE: i32 = 0;
            pub const r#SUPP_SERVICE_CLASS_VOICE: i32 = 1;
            pub const r#SUPP_SERVICE_CLASS_DATA: i32 = 2;
            pub const r#SUPP_SERVICE_CLASS_FAX: i32 = 4;
            pub const r#SUPP_SERVICE_CLASS_SMS: i32 = 8;
            pub const r#SUPP_SERVICE_CLASS_DATA_SYNC: i32 = 16;
            pub const r#SUPP_SERVICE_CLASS_DATA_ASYNC: i32 = 32;
            pub const r#SUPP_SERVICE_CLASS_PACKET: i32 = 64;
            pub const r#SUPP_SERVICE_CLASS_PAD: i32 = 128;
            pub const r#SUPP_SERVICE_CLASS_MAX: i32 = 128;
            impl Default for r#StkCcUnsolSsResult {
              fn default() -> Self {
                Self {
                  r#serviceType: 0,
                  r#requestType: 0,
                  r#teleserviceType: 0,
                  r#serviceClass: 0,
                  r#result: Default::default(),
                  r#ssInfo: Default::default(),
                  r#cfData: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#StkCcUnsolSsResult {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#serviceType)?;
                  subparcel.write(&self.r#requestType)?;
                  subparcel.write(&self.r#teleserviceType)?;
                  subparcel.write(&self.r#serviceClass)?;
                  subparcel.write(&self.r#result)?;
                  subparcel.write(&self.r#ssInfo)?;
                  subparcel.write(&self.r#cfData)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#serviceType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#requestType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#teleserviceType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#serviceClass = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#result = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#ssInfo = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cfData = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#StkCcUnsolSsResult);
            binder::impl_deserialize_for_parcelable!(r#StkCcUnsolSsResult);
            impl binder::binder_impl::ParcelableMetadata for r#StkCcUnsolSsResult {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.StkCcUnsolSsResult" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#StkCcUnsolSsResult as _7_android_8_hardware_5_radio_5_voice_18_StkCcUnsolSsResult;
            }
          }
                    pub mod TtyMode {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#TtyMode : [i32; 4] {
                r#OFF = 0,
                r#FULL = 1,
                r#HCO = 2,
                r#VCO = 3,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#TtyMode as _7_android_8_hardware_5_radio_5_voice_7_TtyMode;
            }
          }
                    pub mod UssdModeType {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#UssdModeType : [i32; 6] {
                r#NOTIFY = 0,
                r#REQUEST = 1,
                r#NW_RELEASE = 2,
                r#LOCAL_CLIENT = 3,
                r#NOT_SUPPORTED = 4,
                r#NW_TIMEOUT = 5,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#UssdModeType as _7_android_8_hardware_5_radio_5_voice_12_UssdModeType;
            }
          }
                    pub mod UusInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#UusInfo {
              pub r#uusType: i32,
              pub r#uusDcs: i32,
              pub r#uusData: String,
            }
            pub const r#UUS_DCS_USP: i32 = 0;
            pub const r#UUS_DCS_OSIHLP: i32 = 1;
            pub const r#UUS_DCS_X244: i32 = 2;
            pub const r#UUS_DCS_RMCF: i32 = 3;
            pub const r#UUS_DCS_IA5C: i32 = 4;
            pub const r#UUS_TYPE_TYPE1_IMPLICIT: i32 = 0;
            pub const r#UUS_TYPE_TYPE1_REQUIRED: i32 = 1;
            pub const r#UUS_TYPE_TYPE1_NOT_REQUIRED: i32 = 2;
            pub const r#UUS_TYPE_TYPE2_REQUIRED: i32 = 3;
            pub const r#UUS_TYPE_TYPE2_NOT_REQUIRED: i32 = 4;
            pub const r#UUS_TYPE_TYPE3_REQUIRED: i32 = 5;
            pub const r#UUS_TYPE_TYPE3_NOT_REQUIRED: i32 = 6;
            impl Default for r#UusInfo {
              fn default() -> Self {
                Self {
                  r#uusType: 0,
                  r#uusDcs: 0,
                  r#uusData: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#UusInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#uusType)?;
                  subparcel.write(&self.r#uusDcs)?;
                  subparcel.write(&self.r#uusData)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#uusType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#uusDcs = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#uusData = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#UusInfo);
            binder::impl_deserialize_for_parcelable!(r#UusInfo);
            impl binder::binder_impl::ParcelableMetadata for r#UusInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.voice.UusInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#UusInfo as _7_android_8_hardware_5_radio_5_voice_7_UusInfo;
            }
          }
                }
            }
        }
    }
}
pub mod mangled {
    pub use super::aidl::android::hardware::radio::voice::AudioQuality::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::Call::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::CallForwardInfo::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::CdmaCallWaiting::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::CdmaDisplayInfoRecord::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::CdmaInformationRecord::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::CdmaLineControlInfoRecord::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::CdmaNumberInfoRecord::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::CdmaOtaProvisionStatus::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::CdmaRedirectingNumberInfoRecord::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::CdmaSignalInfoRecord::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::CdmaT53AudioControlInfoRecord::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::CdmaT53ClirInfoRecord::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::CfData::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::ClipStatus::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::Dial::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::EmergencyCallRouting::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::EmergencyNumber::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::EmergencyServiceCategory::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::IRadioVoice::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::IRadioVoiceIndication::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::IRadioVoiceResponse::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::LastCallFailCause::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::LastCallFailCauseInfo::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::SrvccState::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::SsInfoData::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::StkCcUnsolSsResult::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::TtyMode::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::UssdModeType::mangled::*;
  pub use super::aidl::android::hardware::radio::voice::UusInfo::mangled::*;
  pub(crate) use android_hardware_radio::mangled::*;
}
