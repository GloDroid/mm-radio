#![feature(custom_inner_attributes)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
pub use binder;
pub mod aidl {
    pub mod android {
        pub mod hardware {
            pub mod radio {
                pub mod network {
                    pub mod AccessTechnologySpecificInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub enum r#AccessTechnologySpecificInfo {
              Noinit(bool),
              CdmaInfo(crate::mangled::_7_android_8_hardware_5_radio_7_network_24_Cdma2000RegistrationInfo),
              EutranInfo(crate::mangled::_7_android_8_hardware_5_radio_7_network_22_EutranRegistrationInfo),
              NgranNrVopsInfo(crate::mangled::_7_android_8_hardware_5_radio_7_network_10_NrVopsInfo),
              GeranDtmSupported(bool),
            }
            impl Default for r#AccessTechnologySpecificInfo {
              fn default() -> Self {
                Self::Noinit(false)
              }
            }
            impl binder::Parcelable for r#AccessTechnologySpecificInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::Noinit(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::CdmaInfo(v) => {
                    parcel.write(&1i32)?;
                    parcel.write(v)
                  }
                  Self::EutranInfo(v) => {
                    parcel.write(&2i32)?;
                    parcel.write(v)
                  }
                  Self::NgranNrVopsInfo(v) => {
                    parcel.write(&3i32)?;
                    parcel.write(v)
                  }
                  Self::GeranDtmSupported(v) => {
                    parcel.write(&4i32)?;
                    parcel.write(v)
                  }
                }
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                let tag: i32 = parcel.read()?;
                match tag {
                  0 => {
                    let value: bool = parcel.read()?;
                    *self = Self::Noinit(value);
                    Ok(())
                  }
                  1 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_24_Cdma2000RegistrationInfo = parcel.read()?;
                    *self = Self::CdmaInfo(value);
                    Ok(())
                  }
                  2 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_22_EutranRegistrationInfo = parcel.read()?;
                    *self = Self::EutranInfo(value);
                    Ok(())
                  }
                  3 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_10_NrVopsInfo = parcel.read()?;
                    *self = Self::NgranNrVopsInfo(value);
                    Ok(())
                  }
                  4 => {
                    let value: bool = parcel.read()?;
                    *self = Self::GeranDtmSupported(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#AccessTechnologySpecificInfo);
            binder::impl_deserialize_for_parcelable!(r#AccessTechnologySpecificInfo);
            impl binder::binder_impl::ParcelableMetadata for r#AccessTechnologySpecificInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.AccessTechnologySpecificInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                r#Tag : [i32; 5] {
                  r#noinit = 0,
                  r#cdmaInfo = 1,
                  r#eutranInfo = 2,
                  r#ngranNrVopsInfo = 3,
                  r#geranDtmSupported = 4,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#AccessTechnologySpecificInfo as _7_android_8_hardware_5_radio_7_network_28_AccessTechnologySpecificInfo;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_5_radio_7_network_28_AccessTechnologySpecificInfo_3_Tag;
            }
          }
                    pub mod BarringInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#BarringInfo {
              pub r#serviceType: i32,
              pub r#barringType: i32,
              pub r#barringTypeSpecificInfo: Option<crate::mangled::_7_android_8_hardware_5_radio_7_network_23_BarringTypeSpecificInfo>,
            }
            pub const r#BARRING_TYPE_NONE: i32 = 0;
            pub const r#BARRING_TYPE_CONDITIONAL: i32 = 1;
            pub const r#BARRING_TYPE_UNCONDITIONAL: i32 = 2;
            pub const r#SERVICE_TYPE_CS_SERVICE: i32 = 0;
            pub const r#SERVICE_TYPE_PS_SERVICE: i32 = 1;
            pub const r#SERVICE_TYPE_CS_VOICE: i32 = 2;
            pub const r#SERVICE_TYPE_MO_SIGNALLING: i32 = 3;
            pub const r#SERVICE_TYPE_MO_DATA: i32 = 4;
            pub const r#SERVICE_TYPE_CS_FALLBACK: i32 = 5;
            pub const r#SERVICE_TYPE_MMTEL_VOICE: i32 = 6;
            pub const r#SERVICE_TYPE_MMTEL_VIDEO: i32 = 7;
            pub const r#SERVICE_TYPE_EMERGENCY: i32 = 8;
            pub const r#SERVICE_TYPE_SMS: i32 = 9;
            pub const r#SERVICE_TYPE_OPERATOR_1: i32 = 1001;
            pub const r#SERVICE_TYPE_OPERATOR_2: i32 = 1002;
            pub const r#SERVICE_TYPE_OPERATOR_3: i32 = 1003;
            pub const r#SERVICE_TYPE_OPERATOR_4: i32 = 1004;
            pub const r#SERVICE_TYPE_OPERATOR_5: i32 = 1005;
            pub const r#SERVICE_TYPE_OPERATOR_6: i32 = 1006;
            pub const r#SERVICE_TYPE_OPERATOR_7: i32 = 1007;
            pub const r#SERVICE_TYPE_OPERATOR_8: i32 = 1008;
            pub const r#SERVICE_TYPE_OPERATOR_9: i32 = 1009;
            pub const r#SERVICE_TYPE_OPERATOR_10: i32 = 1010;
            pub const r#SERVICE_TYPE_OPERATOR_11: i32 = 1011;
            pub const r#SERVICE_TYPE_OPERATOR_12: i32 = 1012;
            pub const r#SERVICE_TYPE_OPERATOR_13: i32 = 1013;
            pub const r#SERVICE_TYPE_OPERATOR_14: i32 = 1014;
            pub const r#SERVICE_TYPE_OPERATOR_15: i32 = 1015;
            pub const r#SERVICE_TYPE_OPERATOR_16: i32 = 1016;
            pub const r#SERVICE_TYPE_OPERATOR_17: i32 = 1017;
            pub const r#SERVICE_TYPE_OPERATOR_18: i32 = 1018;
            pub const r#SERVICE_TYPE_OPERATOR_19: i32 = 1019;
            pub const r#SERVICE_TYPE_OPERATOR_20: i32 = 1020;
            pub const r#SERVICE_TYPE_OPERATOR_21: i32 = 1021;
            pub const r#SERVICE_TYPE_OPERATOR_22: i32 = 1022;
            pub const r#SERVICE_TYPE_OPERATOR_23: i32 = 1023;
            pub const r#SERVICE_TYPE_OPERATOR_24: i32 = 1024;
            pub const r#SERVICE_TYPE_OPERATOR_25: i32 = 1025;
            pub const r#SERVICE_TYPE_OPERATOR_26: i32 = 1026;
            pub const r#SERVICE_TYPE_OPERATOR_27: i32 = 1027;
            pub const r#SERVICE_TYPE_OPERATOR_28: i32 = 1028;
            pub const r#SERVICE_TYPE_OPERATOR_29: i32 = 1029;
            pub const r#SERVICE_TYPE_OPERATOR_30: i32 = 1030;
            pub const r#SERVICE_TYPE_OPERATOR_31: i32 = 1031;
            pub const r#SERVICE_TYPE_OPERATOR_32: i32 = 1032;
            impl Default for r#BarringInfo {
              fn default() -> Self {
                Self {
                  r#serviceType: 0,
                  r#barringType: 0,
                  r#barringTypeSpecificInfo: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#BarringInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#serviceType)?;
                  subparcel.write(&self.r#barringType)?;
                  subparcel.write(&self.r#barringTypeSpecificInfo)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#serviceType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#barringType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#barringTypeSpecificInfo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#BarringInfo);
            binder::impl_deserialize_for_parcelable!(r#BarringInfo);
            impl binder::binder_impl::ParcelableMetadata for r#BarringInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.BarringInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#BarringInfo as _7_android_8_hardware_5_radio_7_network_11_BarringInfo;
            }
          }
                    pub mod BarringTypeSpecificInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#BarringTypeSpecificInfo {
              pub r#factor: i32,
              pub r#timeSeconds: i32,
              pub r#isBarred: bool,
            }
            impl Default for r#BarringTypeSpecificInfo {
              fn default() -> Self {
                Self {
                  r#factor: 0,
                  r#timeSeconds: 0,
                  r#isBarred: false,
                }
              }
            }
            impl binder::Parcelable for r#BarringTypeSpecificInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#factor)?;
                  subparcel.write(&self.r#timeSeconds)?;
                  subparcel.write(&self.r#isBarred)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#factor = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#timeSeconds = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#isBarred = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#BarringTypeSpecificInfo);
            binder::impl_deserialize_for_parcelable!(r#BarringTypeSpecificInfo);
            impl binder::binder_impl::ParcelableMetadata for r#BarringTypeSpecificInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.BarringTypeSpecificInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#BarringTypeSpecificInfo as _7_android_8_hardware_5_radio_7_network_23_BarringTypeSpecificInfo;
            }
          }
                    pub mod Cdma2000RegistrationInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#Cdma2000RegistrationInfo {
              pub r#cssSupported: bool,
              pub r#roamingIndicator: i32,
              pub r#systemIsInPrl: i32,
              pub r#defaultRoamingIndicator: i32,
            }
            pub const r#PRL_INDICATOR_NOT_REGISTERED: i32 = -1;
            pub const r#PRL_INDICATOR_NOT_IN_PRL: i32 = 0;
            pub const r#PRL_INDICATOR_IN_PRL: i32 = 1;
            impl Default for r#Cdma2000RegistrationInfo {
              fn default() -> Self {
                Self {
                  r#cssSupported: false,
                  r#roamingIndicator: 0,
                  r#systemIsInPrl: 0,
                  r#defaultRoamingIndicator: 0,
                }
              }
            }
            impl binder::Parcelable for r#Cdma2000RegistrationInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cssSupported)?;
                  subparcel.write(&self.r#roamingIndicator)?;
                  subparcel.write(&self.r#systemIsInPrl)?;
                  subparcel.write(&self.r#defaultRoamingIndicator)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cssSupported = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#roamingIndicator = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#systemIsInPrl = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#defaultRoamingIndicator = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Cdma2000RegistrationInfo);
            binder::impl_deserialize_for_parcelable!(r#Cdma2000RegistrationInfo);
            impl binder::binder_impl::ParcelableMetadata for r#Cdma2000RegistrationInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.Cdma2000RegistrationInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Cdma2000RegistrationInfo as _7_android_8_hardware_5_radio_7_network_24_Cdma2000RegistrationInfo;
            }
          }
                    pub mod CdmaRoamingType {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#CdmaRoamingType : [i32; 3] {
                r#HOME_NETWORK = 0,
                r#AFFILIATED_ROAM = 1,
                r#ANY_ROAM = 2,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaRoamingType as _7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType;
            }
          }
                    pub mod CdmaSignalStrength {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CdmaSignalStrength {
              pub r#dbm: i32,
              pub r#ecio: i32,
            }
            impl Default for r#CdmaSignalStrength {
              fn default() -> Self {
                Self {
                  r#dbm: 0,
                  r#ecio: 0,
                }
              }
            }
            impl binder::Parcelable for r#CdmaSignalStrength {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#dbm)?;
                  subparcel.write(&self.r#ecio)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#dbm = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#ecio = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CdmaSignalStrength);
            binder::impl_deserialize_for_parcelable!(r#CdmaSignalStrength);
            impl binder::binder_impl::ParcelableMetadata for r#CdmaSignalStrength {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CdmaSignalStrength" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CdmaSignalStrength as _7_android_8_hardware_5_radio_7_network_18_CdmaSignalStrength;
            }
          }
                    pub mod CellConnectionStatus {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#CellConnectionStatus : [i32; 3] {
                r#NONE = 0,
                r#PRIMARY_SERVING = 1,
                r#SECONDARY_SERVING = 2,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#CellConnectionStatus as _7_android_8_hardware_5_radio_7_network_20_CellConnectionStatus;
            }
          }
                    pub mod CellIdentity {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub enum r#CellIdentity {
              Noinit(bool),
              Gsm(crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CellIdentityGsm),
              Wcdma(crate::mangled::_7_android_8_hardware_5_radio_7_network_17_CellIdentityWcdma),
              Tdscdma(crate::mangled::_7_android_8_hardware_5_radio_7_network_19_CellIdentityTdscdma),
              Cdma(crate::mangled::_7_android_8_hardware_5_radio_7_network_16_CellIdentityCdma),
              Lte(crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CellIdentityLte),
              Nr(crate::mangled::_7_android_8_hardware_5_radio_7_network_14_CellIdentityNr),
            }
            impl Default for r#CellIdentity {
              fn default() -> Self {
                Self::Noinit(false)
              }
            }
            impl binder::Parcelable for r#CellIdentity {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::Noinit(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::Gsm(v) => {
                    parcel.write(&1i32)?;
                    parcel.write(v)
                  }
                  Self::Wcdma(v) => {
                    parcel.write(&2i32)?;
                    parcel.write(v)
                  }
                  Self::Tdscdma(v) => {
                    parcel.write(&3i32)?;
                    parcel.write(v)
                  }
                  Self::Cdma(v) => {
                    parcel.write(&4i32)?;
                    parcel.write(v)
                  }
                  Self::Lte(v) => {
                    parcel.write(&5i32)?;
                    parcel.write(v)
                  }
                  Self::Nr(v) => {
                    parcel.write(&6i32)?;
                    parcel.write(v)
                  }
                }
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                let tag: i32 = parcel.read()?;
                match tag {
                  0 => {
                    let value: bool = parcel.read()?;
                    *self = Self::Noinit(value);
                    Ok(())
                  }
                  1 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CellIdentityGsm = parcel.read()?;
                    *self = Self::Gsm(value);
                    Ok(())
                  }
                  2 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_17_CellIdentityWcdma = parcel.read()?;
                    *self = Self::Wcdma(value);
                    Ok(())
                  }
                  3 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_19_CellIdentityTdscdma = parcel.read()?;
                    *self = Self::Tdscdma(value);
                    Ok(())
                  }
                  4 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_16_CellIdentityCdma = parcel.read()?;
                    *self = Self::Cdma(value);
                    Ok(())
                  }
                  5 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CellIdentityLte = parcel.read()?;
                    *self = Self::Lte(value);
                    Ok(())
                  }
                  6 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_14_CellIdentityNr = parcel.read()?;
                    *self = Self::Nr(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellIdentity);
            binder::impl_deserialize_for_parcelable!(r#CellIdentity);
            impl binder::binder_impl::ParcelableMetadata for r#CellIdentity {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellIdentity" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                r#Tag : [i32; 7] {
                  r#noinit = 0,
                  r#gsm = 1,
                  r#wcdma = 2,
                  r#tdscdma = 3,
                  r#cdma = 4,
                  r#lte = 5,
                  r#nr = 6,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#CellIdentity as _7_android_8_hardware_5_radio_7_network_12_CellIdentity;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_5_radio_7_network_12_CellIdentity_3_Tag;
            }
          }
                    pub mod CellIdentityCdma {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellIdentityCdma {
              pub r#networkId: i32,
              pub r#systemId: i32,
              pub r#baseStationId: i32,
              pub r#longitude: i32,
              pub r#latitude: i32,
              pub r#operatorNames: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo,
            }
            impl Default for r#CellIdentityCdma {
              fn default() -> Self {
                Self {
                  r#networkId: 0,
                  r#systemId: 0,
                  r#baseStationId: 0,
                  r#longitude: 0,
                  r#latitude: 0,
                  r#operatorNames: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellIdentityCdma {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#networkId)?;
                  subparcel.write(&self.r#systemId)?;
                  subparcel.write(&self.r#baseStationId)?;
                  subparcel.write(&self.r#longitude)?;
                  subparcel.write(&self.r#latitude)?;
                  subparcel.write(&self.r#operatorNames)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#networkId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#systemId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#baseStationId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#longitude = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#latitude = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#operatorNames = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellIdentityCdma);
            binder::impl_deserialize_for_parcelable!(r#CellIdentityCdma);
            impl binder::binder_impl::ParcelableMetadata for r#CellIdentityCdma {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellIdentityCdma" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellIdentityCdma as _7_android_8_hardware_5_radio_7_network_16_CellIdentityCdma;
            }
          }
                    pub mod CellIdentityGsm {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellIdentityGsm {
              pub r#mcc: String,
              pub r#mnc: String,
              pub r#lac: i32,
              pub r#cid: i32,
              pub r#arfcn: i32,
              pub r#bsic: i8,
              pub r#operatorNames: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo,
              pub r#additionalPlmns: Vec<String>,
            }
            impl Default for r#CellIdentityGsm {
              fn default() -> Self {
                Self {
                  r#mcc: Default::default(),
                  r#mnc: Default::default(),
                  r#lac: 0,
                  r#cid: 0,
                  r#arfcn: 0,
                  r#bsic: 0,
                  r#operatorNames: Default::default(),
                  r#additionalPlmns: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellIdentityGsm {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#mcc)?;
                  subparcel.write(&self.r#mnc)?;
                  subparcel.write(&self.r#lac)?;
                  subparcel.write(&self.r#cid)?;
                  subparcel.write(&self.r#arfcn)?;
                  subparcel.write(&self.r#bsic)?;
                  subparcel.write(&self.r#operatorNames)?;
                  subparcel.write(&self.r#additionalPlmns)?;
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
                    self.r#lac = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cid = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#arfcn = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bsic = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#operatorNames = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#additionalPlmns = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellIdentityGsm);
            binder::impl_deserialize_for_parcelable!(r#CellIdentityGsm);
            impl binder::binder_impl::ParcelableMetadata for r#CellIdentityGsm {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellIdentityGsm" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellIdentityGsm as _7_android_8_hardware_5_radio_7_network_15_CellIdentityGsm;
            }
          }
                    pub mod CellIdentityLte {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellIdentityLte {
              pub r#mcc: String,
              pub r#mnc: String,
              pub r#ci: i32,
              pub r#pci: i32,
              pub r#tac: i32,
              pub r#earfcn: i32,
              pub r#operatorNames: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo,
              pub r#bandwidth: i32,
              pub r#additionalPlmns: Vec<String>,
              pub r#csgInfo: Option<crate::mangled::_7_android_8_hardware_5_radio_7_network_25_ClosedSubscriberGroupInfo>,
              pub r#bands: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_11_EutranBands>,
            }
            impl Default for r#CellIdentityLte {
              fn default() -> Self {
                Self {
                  r#mcc: Default::default(),
                  r#mnc: Default::default(),
                  r#ci: 0,
                  r#pci: 0,
                  r#tac: 0,
                  r#earfcn: 0,
                  r#operatorNames: Default::default(),
                  r#bandwidth: 0,
                  r#additionalPlmns: Default::default(),
                  r#csgInfo: Default::default(),
                  r#bands: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellIdentityLte {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#mcc)?;
                  subparcel.write(&self.r#mnc)?;
                  subparcel.write(&self.r#ci)?;
                  subparcel.write(&self.r#pci)?;
                  subparcel.write(&self.r#tac)?;
                  subparcel.write(&self.r#earfcn)?;
                  subparcel.write(&self.r#operatorNames)?;
                  subparcel.write(&self.r#bandwidth)?;
                  subparcel.write(&self.r#additionalPlmns)?;
                  subparcel.write(&self.r#csgInfo)?;
                  subparcel.write(&self.r#bands)?;
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
                    self.r#ci = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#pci = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#tac = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#earfcn = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#operatorNames = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bandwidth = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#additionalPlmns = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#csgInfo = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bands = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellIdentityLte);
            binder::impl_deserialize_for_parcelable!(r#CellIdentityLte);
            impl binder::binder_impl::ParcelableMetadata for r#CellIdentityLte {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellIdentityLte" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellIdentityLte as _7_android_8_hardware_5_radio_7_network_15_CellIdentityLte;
            }
          }
                    pub mod CellIdentityNr {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellIdentityNr {
              pub r#mcc: String,
              pub r#mnc: String,
              pub r#nci: i64,
              pub r#pci: i32,
              pub r#tac: i32,
              pub r#nrarfcn: i32,
              pub r#operatorNames: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo,
              pub r#additionalPlmns: Vec<String>,
              pub r#bands: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_10_NgranBands>,
            }
            impl Default for r#CellIdentityNr {
              fn default() -> Self {
                Self {
                  r#mcc: Default::default(),
                  r#mnc: Default::default(),
                  r#nci: 0,
                  r#pci: 0,
                  r#tac: 0,
                  r#nrarfcn: 0,
                  r#operatorNames: Default::default(),
                  r#additionalPlmns: Default::default(),
                  r#bands: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellIdentityNr {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#mcc)?;
                  subparcel.write(&self.r#mnc)?;
                  subparcel.write(&self.r#nci)?;
                  subparcel.write(&self.r#pci)?;
                  subparcel.write(&self.r#tac)?;
                  subparcel.write(&self.r#nrarfcn)?;
                  subparcel.write(&self.r#operatorNames)?;
                  subparcel.write(&self.r#additionalPlmns)?;
                  subparcel.write(&self.r#bands)?;
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
                    self.r#nci = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#pci = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#tac = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#nrarfcn = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#operatorNames = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#additionalPlmns = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bands = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellIdentityNr);
            binder::impl_deserialize_for_parcelable!(r#CellIdentityNr);
            impl binder::binder_impl::ParcelableMetadata for r#CellIdentityNr {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellIdentityNr" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellIdentityNr as _7_android_8_hardware_5_radio_7_network_14_CellIdentityNr;
            }
          }
                    pub mod CellIdentityTdscdma {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellIdentityTdscdma {
              pub r#mcc: String,
              pub r#mnc: String,
              pub r#lac: i32,
              pub r#cid: i32,
              pub r#cpid: i32,
              pub r#uarfcn: i32,
              pub r#operatorNames: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo,
              pub r#additionalPlmns: Vec<String>,
              pub r#csgInfo: Option<crate::mangled::_7_android_8_hardware_5_radio_7_network_25_ClosedSubscriberGroupInfo>,
            }
            impl Default for r#CellIdentityTdscdma {
              fn default() -> Self {
                Self {
                  r#mcc: Default::default(),
                  r#mnc: Default::default(),
                  r#lac: 0,
                  r#cid: 0,
                  r#cpid: 0,
                  r#uarfcn: 0,
                  r#operatorNames: Default::default(),
                  r#additionalPlmns: Default::default(),
                  r#csgInfo: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellIdentityTdscdma {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#mcc)?;
                  subparcel.write(&self.r#mnc)?;
                  subparcel.write(&self.r#lac)?;
                  subparcel.write(&self.r#cid)?;
                  subparcel.write(&self.r#cpid)?;
                  subparcel.write(&self.r#uarfcn)?;
                  subparcel.write(&self.r#operatorNames)?;
                  subparcel.write(&self.r#additionalPlmns)?;
                  subparcel.write(&self.r#csgInfo)?;
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
                    self.r#lac = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cid = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cpid = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#uarfcn = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#operatorNames = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#additionalPlmns = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#csgInfo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellIdentityTdscdma);
            binder::impl_deserialize_for_parcelable!(r#CellIdentityTdscdma);
            impl binder::binder_impl::ParcelableMetadata for r#CellIdentityTdscdma {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellIdentityTdscdma" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellIdentityTdscdma as _7_android_8_hardware_5_radio_7_network_19_CellIdentityTdscdma;
            }
          }
                    pub mod CellIdentityWcdma {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellIdentityWcdma {
              pub r#mcc: String,
              pub r#mnc: String,
              pub r#lac: i32,
              pub r#cid: i32,
              pub r#psc: i32,
              pub r#uarfcn: i32,
              pub r#operatorNames: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo,
              pub r#additionalPlmns: Vec<String>,
              pub r#csgInfo: Option<crate::mangled::_7_android_8_hardware_5_radio_7_network_25_ClosedSubscriberGroupInfo>,
            }
            impl Default for r#CellIdentityWcdma {
              fn default() -> Self {
                Self {
                  r#mcc: Default::default(),
                  r#mnc: Default::default(),
                  r#lac: 0,
                  r#cid: 0,
                  r#psc: 0,
                  r#uarfcn: 0,
                  r#operatorNames: Default::default(),
                  r#additionalPlmns: Default::default(),
                  r#csgInfo: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellIdentityWcdma {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#mcc)?;
                  subparcel.write(&self.r#mnc)?;
                  subparcel.write(&self.r#lac)?;
                  subparcel.write(&self.r#cid)?;
                  subparcel.write(&self.r#psc)?;
                  subparcel.write(&self.r#uarfcn)?;
                  subparcel.write(&self.r#operatorNames)?;
                  subparcel.write(&self.r#additionalPlmns)?;
                  subparcel.write(&self.r#csgInfo)?;
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
                    self.r#lac = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cid = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#psc = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#uarfcn = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#operatorNames = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#additionalPlmns = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#csgInfo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellIdentityWcdma);
            binder::impl_deserialize_for_parcelable!(r#CellIdentityWcdma);
            impl binder::binder_impl::ParcelableMetadata for r#CellIdentityWcdma {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellIdentityWcdma" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellIdentityWcdma as _7_android_8_hardware_5_radio_7_network_17_CellIdentityWcdma;
            }
          }
                    pub mod CellInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellInfo {
              pub r#registered: bool,
              pub r#connectionStatus: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_CellConnectionStatus,
              pub r#ratSpecificInfo: crate::mangled::_7_android_8_hardware_5_radio_7_network_23_CellInfoRatSpecificInfo,
            }
            impl Default for r#CellInfo {
              fn default() -> Self {
                Self {
                  r#registered: false,
                  r#connectionStatus: Default::default(),
                  r#ratSpecificInfo: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#registered)?;
                  subparcel.write(&self.r#connectionStatus)?;
                  subparcel.write(&self.r#ratSpecificInfo)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#registered = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#connectionStatus = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#ratSpecificInfo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellInfo);
            binder::impl_deserialize_for_parcelable!(r#CellInfo);
            impl binder::binder_impl::ParcelableMetadata for r#CellInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellInfo as _7_android_8_hardware_5_radio_7_network_8_CellInfo;
            }
          }
                    pub mod CellInfoCdma {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellInfoCdma {
              pub r#cellIdentityCdma: crate::mangled::_7_android_8_hardware_5_radio_7_network_16_CellIdentityCdma,
              pub r#signalStrengthCdma: crate::mangled::_7_android_8_hardware_5_radio_7_network_18_CdmaSignalStrength,
              pub r#signalStrengthEvdo: crate::mangled::_7_android_8_hardware_5_radio_7_network_18_EvdoSignalStrength,
            }
            impl Default for r#CellInfoCdma {
              fn default() -> Self {
                Self {
                  r#cellIdentityCdma: Default::default(),
                  r#signalStrengthCdma: Default::default(),
                  r#signalStrengthEvdo: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellInfoCdma {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cellIdentityCdma)?;
                  subparcel.write(&self.r#signalStrengthCdma)?;
                  subparcel.write(&self.r#signalStrengthEvdo)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cellIdentityCdma = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signalStrengthCdma = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signalStrengthEvdo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellInfoCdma);
            binder::impl_deserialize_for_parcelable!(r#CellInfoCdma);
            impl binder::binder_impl::ParcelableMetadata for r#CellInfoCdma {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellInfoCdma" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellInfoCdma as _7_android_8_hardware_5_radio_7_network_12_CellInfoCdma;
            }
          }
                    pub mod CellInfoGsm {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellInfoGsm {
              pub r#cellIdentityGsm: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CellIdentityGsm,
              pub r#signalStrengthGsm: crate::mangled::_7_android_8_hardware_5_radio_7_network_17_GsmSignalStrength,
            }
            impl Default for r#CellInfoGsm {
              fn default() -> Self {
                Self {
                  r#cellIdentityGsm: Default::default(),
                  r#signalStrengthGsm: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellInfoGsm {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cellIdentityGsm)?;
                  subparcel.write(&self.r#signalStrengthGsm)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cellIdentityGsm = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signalStrengthGsm = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellInfoGsm);
            binder::impl_deserialize_for_parcelable!(r#CellInfoGsm);
            impl binder::binder_impl::ParcelableMetadata for r#CellInfoGsm {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellInfoGsm" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellInfoGsm as _7_android_8_hardware_5_radio_7_network_11_CellInfoGsm;
            }
          }
                    pub mod CellInfoLte {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellInfoLte {
              pub r#cellIdentityLte: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CellIdentityLte,
              pub r#signalStrengthLte: crate::mangled::_7_android_8_hardware_5_radio_7_network_17_LteSignalStrength,
            }
            impl Default for r#CellInfoLte {
              fn default() -> Self {
                Self {
                  r#cellIdentityLte: Default::default(),
                  r#signalStrengthLte: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellInfoLte {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cellIdentityLte)?;
                  subparcel.write(&self.r#signalStrengthLte)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cellIdentityLte = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signalStrengthLte = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellInfoLte);
            binder::impl_deserialize_for_parcelable!(r#CellInfoLte);
            impl binder::binder_impl::ParcelableMetadata for r#CellInfoLte {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellInfoLte" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellInfoLte as _7_android_8_hardware_5_radio_7_network_11_CellInfoLte;
            }
          }
                    pub mod CellInfoNr {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellInfoNr {
              pub r#cellIdentityNr: crate::mangled::_7_android_8_hardware_5_radio_7_network_14_CellIdentityNr,
              pub r#signalStrengthNr: crate::mangled::_7_android_8_hardware_5_radio_7_network_16_NrSignalStrength,
            }
            impl Default for r#CellInfoNr {
              fn default() -> Self {
                Self {
                  r#cellIdentityNr: Default::default(),
                  r#signalStrengthNr: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellInfoNr {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cellIdentityNr)?;
                  subparcel.write(&self.r#signalStrengthNr)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cellIdentityNr = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signalStrengthNr = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellInfoNr);
            binder::impl_deserialize_for_parcelable!(r#CellInfoNr);
            impl binder::binder_impl::ParcelableMetadata for r#CellInfoNr {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellInfoNr" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellInfoNr as _7_android_8_hardware_5_radio_7_network_10_CellInfoNr;
            }
          }
                    pub mod CellInfoRatSpecificInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub enum r#CellInfoRatSpecificInfo {
              Gsm(crate::mangled::_7_android_8_hardware_5_radio_7_network_11_CellInfoGsm),
              Wcdma(crate::mangled::_7_android_8_hardware_5_radio_7_network_13_CellInfoWcdma),
              Tdscdma(crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CellInfoTdscdma),
              Lte(crate::mangled::_7_android_8_hardware_5_radio_7_network_11_CellInfoLte),
              Nr(crate::mangled::_7_android_8_hardware_5_radio_7_network_10_CellInfoNr),
              Cdma(crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellInfoCdma),
            }
            impl Default for r#CellInfoRatSpecificInfo {
              fn default() -> Self {
                Self::Gsm(Default::default())
              }
            }
            impl binder::Parcelable for r#CellInfoRatSpecificInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::Gsm(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::Wcdma(v) => {
                    parcel.write(&1i32)?;
                    parcel.write(v)
                  }
                  Self::Tdscdma(v) => {
                    parcel.write(&2i32)?;
                    parcel.write(v)
                  }
                  Self::Lte(v) => {
                    parcel.write(&3i32)?;
                    parcel.write(v)
                  }
                  Self::Nr(v) => {
                    parcel.write(&4i32)?;
                    parcel.write(v)
                  }
                  Self::Cdma(v) => {
                    parcel.write(&5i32)?;
                    parcel.write(v)
                  }
                }
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                let tag: i32 = parcel.read()?;
                match tag {
                  0 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_11_CellInfoGsm = parcel.read()?;
                    *self = Self::Gsm(value);
                    Ok(())
                  }
                  1 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_13_CellInfoWcdma = parcel.read()?;
                    *self = Self::Wcdma(value);
                    Ok(())
                  }
                  2 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CellInfoTdscdma = parcel.read()?;
                    *self = Self::Tdscdma(value);
                    Ok(())
                  }
                  3 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_11_CellInfoLte = parcel.read()?;
                    *self = Self::Lte(value);
                    Ok(())
                  }
                  4 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_10_CellInfoNr = parcel.read()?;
                    *self = Self::Nr(value);
                    Ok(())
                  }
                  5 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellInfoCdma = parcel.read()?;
                    *self = Self::Cdma(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellInfoRatSpecificInfo);
            binder::impl_deserialize_for_parcelable!(r#CellInfoRatSpecificInfo);
            impl binder::binder_impl::ParcelableMetadata for r#CellInfoRatSpecificInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellInfoRatSpecificInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                r#Tag : [i32; 6] {
                  r#gsm = 0,
                  r#wcdma = 1,
                  r#tdscdma = 2,
                  r#lte = 3,
                  r#nr = 4,
                  r#cdma = 5,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#CellInfoRatSpecificInfo as _7_android_8_hardware_5_radio_7_network_23_CellInfoRatSpecificInfo;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_5_radio_7_network_23_CellInfoRatSpecificInfo_3_Tag;
            }
          }
                    pub mod CellInfoTdscdma {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellInfoTdscdma {
              pub r#cellIdentityTdscdma: crate::mangled::_7_android_8_hardware_5_radio_7_network_19_CellIdentityTdscdma,
              pub r#signalStrengthTdscdma: crate::mangled::_7_android_8_hardware_5_radio_7_network_21_TdscdmaSignalStrength,
            }
            impl Default for r#CellInfoTdscdma {
              fn default() -> Self {
                Self {
                  r#cellIdentityTdscdma: Default::default(),
                  r#signalStrengthTdscdma: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellInfoTdscdma {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cellIdentityTdscdma)?;
                  subparcel.write(&self.r#signalStrengthTdscdma)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cellIdentityTdscdma = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signalStrengthTdscdma = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellInfoTdscdma);
            binder::impl_deserialize_for_parcelable!(r#CellInfoTdscdma);
            impl binder::binder_impl::ParcelableMetadata for r#CellInfoTdscdma {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellInfoTdscdma" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellInfoTdscdma as _7_android_8_hardware_5_radio_7_network_15_CellInfoTdscdma;
            }
          }
                    pub mod CellInfoWcdma {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#CellInfoWcdma {
              pub r#cellIdentityWcdma: crate::mangled::_7_android_8_hardware_5_radio_7_network_17_CellIdentityWcdma,
              pub r#signalStrengthWcdma: crate::mangled::_7_android_8_hardware_5_radio_7_network_19_WcdmaSignalStrength,
            }
            impl Default for r#CellInfoWcdma {
              fn default() -> Self {
                Self {
                  r#cellIdentityWcdma: Default::default(),
                  r#signalStrengthWcdma: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CellInfoWcdma {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cellIdentityWcdma)?;
                  subparcel.write(&self.r#signalStrengthWcdma)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cellIdentityWcdma = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signalStrengthWcdma = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CellInfoWcdma);
            binder::impl_deserialize_for_parcelable!(r#CellInfoWcdma);
            impl binder::binder_impl::ParcelableMetadata for r#CellInfoWcdma {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.CellInfoWcdma" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CellInfoWcdma as _7_android_8_hardware_5_radio_7_network_13_CellInfoWcdma;
            }
          }
                    pub mod ClosedSubscriberGroupInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#ClosedSubscriberGroupInfo {
              pub r#csgIndication: bool,
              pub r#homeNodebName: String,
              pub r#csgIdentity: i32,
            }
            impl Default for r#ClosedSubscriberGroupInfo {
              fn default() -> Self {
                Self {
                  r#csgIndication: false,
                  r#homeNodebName: Default::default(),
                  r#csgIdentity: 0,
                }
              }
            }
            impl binder::Parcelable for r#ClosedSubscriberGroupInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#csgIndication)?;
                  subparcel.write(&self.r#homeNodebName)?;
                  subparcel.write(&self.r#csgIdentity)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#csgIndication = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#homeNodebName = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#csgIdentity = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ClosedSubscriberGroupInfo);
            binder::impl_deserialize_for_parcelable!(r#ClosedSubscriberGroupInfo);
            impl binder::binder_impl::ParcelableMetadata for r#ClosedSubscriberGroupInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.ClosedSubscriberGroupInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ClosedSubscriberGroupInfo as _7_android_8_hardware_5_radio_7_network_25_ClosedSubscriberGroupInfo;
            }
          }
                    pub mod Domain {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#Domain : [i32; 2] {
                r#CS = 1,
                r#PS = 2,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Domain as _7_android_8_hardware_5_radio_7_network_6_Domain;
            }
          }
                    pub mod EutranBands {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#EutranBands : [i32; 60] {
                r#BAND_1 = 1,
                r#BAND_2 = 2,
                r#BAND_3 = 3,
                r#BAND_4 = 4,
                r#BAND_5 = 5,
                r#BAND_6 = 6,
                r#BAND_7 = 7,
                r#BAND_8 = 8,
                r#BAND_9 = 9,
                r#BAND_10 = 10,
                r#BAND_11 = 11,
                r#BAND_12 = 12,
                r#BAND_13 = 13,
                r#BAND_14 = 14,
                r#BAND_17 = 17,
                r#BAND_18 = 18,
                r#BAND_19 = 19,
                r#BAND_20 = 20,
                r#BAND_21 = 21,
                r#BAND_22 = 22,
                r#BAND_23 = 23,
                r#BAND_24 = 24,
                r#BAND_25 = 25,
                r#BAND_26 = 26,
                r#BAND_27 = 27,
                r#BAND_28 = 28,
                r#BAND_30 = 30,
                r#BAND_31 = 31,
                r#BAND_33 = 33,
                r#BAND_34 = 34,
                r#BAND_35 = 35,
                r#BAND_36 = 36,
                r#BAND_37 = 37,
                r#BAND_38 = 38,
                r#BAND_39 = 39,
                r#BAND_40 = 40,
                r#BAND_41 = 41,
                r#BAND_42 = 42,
                r#BAND_43 = 43,
                r#BAND_44 = 44,
                r#BAND_45 = 45,
                r#BAND_46 = 46,
                r#BAND_47 = 47,
                r#BAND_48 = 48,
                r#BAND_65 = 65,
                r#BAND_66 = 66,
                r#BAND_68 = 68,
                r#BAND_70 = 70,
                r#BAND_49 = 49,
                r#BAND_50 = 50,
                r#BAND_51 = 51,
                r#BAND_52 = 52,
                r#BAND_53 = 53,
                r#BAND_71 = 71,
                r#BAND_72 = 72,
                r#BAND_73 = 73,
                r#BAND_74 = 74,
                r#BAND_85 = 85,
                r#BAND_87 = 87,
                r#BAND_88 = 88,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#EutranBands as _7_android_8_hardware_5_radio_7_network_11_EutranBands;
            }
          }
                    pub mod EutranRegistrationInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#EutranRegistrationInfo {
              pub r#lteVopsInfo: crate::mangled::_7_android_8_hardware_5_radio_7_network_11_LteVopsInfo,
              pub r#nrIndicators: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_NrIndicators,
            }
            impl Default for r#EutranRegistrationInfo {
              fn default() -> Self {
                Self {
                  r#lteVopsInfo: Default::default(),
                  r#nrIndicators: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#EutranRegistrationInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#lteVopsInfo)?;
                  subparcel.write(&self.r#nrIndicators)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#lteVopsInfo = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#nrIndicators = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#EutranRegistrationInfo);
            binder::impl_deserialize_for_parcelable!(r#EutranRegistrationInfo);
            impl binder::binder_impl::ParcelableMetadata for r#EutranRegistrationInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.EutranRegistrationInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#EutranRegistrationInfo as _7_android_8_hardware_5_radio_7_network_22_EutranRegistrationInfo;
            }
          }
                    pub mod EvdoSignalStrength {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#EvdoSignalStrength {
              pub r#dbm: i32,
              pub r#ecio: i32,
              pub r#signalNoiseRatio: i32,
            }
            impl Default for r#EvdoSignalStrength {
              fn default() -> Self {
                Self {
                  r#dbm: 0,
                  r#ecio: 0,
                  r#signalNoiseRatio: 0,
                }
              }
            }
            impl binder::Parcelable for r#EvdoSignalStrength {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#dbm)?;
                  subparcel.write(&self.r#ecio)?;
                  subparcel.write(&self.r#signalNoiseRatio)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#dbm = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#ecio = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signalNoiseRatio = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#EvdoSignalStrength);
            binder::impl_deserialize_for_parcelable!(r#EvdoSignalStrength);
            impl binder::binder_impl::ParcelableMetadata for r#EvdoSignalStrength {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.EvdoSignalStrength" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#EvdoSignalStrength as _7_android_8_hardware_5_radio_7_network_18_EvdoSignalStrength;
            }
          }
                    pub mod GeranBands {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#GeranBands : [i32; 14] {
                r#BAND_T380 = 1,
                r#BAND_T410 = 2,
                r#BAND_450 = 3,
                r#BAND_480 = 4,
                r#BAND_710 = 5,
                r#BAND_750 = 6,
                r#BAND_T810 = 7,
                r#BAND_850 = 8,
                r#BAND_P900 = 9,
                r#BAND_E900 = 10,
                r#BAND_R900 = 11,
                r#BAND_DCS1800 = 12,
                r#BAND_PCS1900 = 13,
                r#BAND_ER900 = 14,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#GeranBands as _7_android_8_hardware_5_radio_7_network_10_GeranBands;
            }
          }
                    pub mod GsmSignalStrength {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#GsmSignalStrength {
              pub r#signalStrength: i32,
              pub r#bitErrorRate: i32,
              pub r#timingAdvance: i32,
            }
            impl Default for r#GsmSignalStrength {
              fn default() -> Self {
                Self {
                  r#signalStrength: 0,
                  r#bitErrorRate: 0,
                  r#timingAdvance: 0,
                }
              }
            }
            impl binder::Parcelable for r#GsmSignalStrength {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#signalStrength)?;
                  subparcel.write(&self.r#bitErrorRate)?;
                  subparcel.write(&self.r#timingAdvance)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#signalStrength = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bitErrorRate = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#timingAdvance = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#GsmSignalStrength);
            binder::impl_deserialize_for_parcelable!(r#GsmSignalStrength);
            impl binder::binder_impl::ParcelableMetadata for r#GsmSignalStrength {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.GsmSignalStrength" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#GsmSignalStrength as _7_android_8_hardware_5_radio_7_network_17_GsmSignalStrength;
            }
          }
                    pub mod IRadioNetwork {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioNetwork["android.hardware.radio.network.IRadioNetwork"] {
                native: BnRadioNetwork(on_transact),
                proxy: BpRadioNetwork {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioNetworkAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioNetwork: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.network.IRadioNetwork" }
              fn r#getAllowedNetworkTypesBitmap(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getAvailableBandModes(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getAvailableNetworks(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getBarringInfo(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getCdmaRoamingPreference(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getCellInfoList(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getDataRegistrationState(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getImsRegistrationState(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getNetworkSelectionMode(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getOperator(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getSignalStrength(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getSystemSelectionChannels(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getVoiceRadioTechnology(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getVoiceRegistrationState(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#isNrDualConnectivityEnabled(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#responseAcknowledgement(&self) -> binder::Result<()>;
              fn r#setAllowedNetworkTypesBitmap(&self, _arg_serial: i32, _arg_networkTypeBitmap: i32) -> binder::Result<()>;
              fn r#setBandMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode) -> binder::Result<()>;
              fn r#setBarringPassword(&self, _arg_serial: i32, _arg_facility: &str, _arg_oldPassword: &str, _arg_newPassword: &str) -> binder::Result<()>;
              fn r#setCdmaRoamingPreference(&self, _arg_serial: i32, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<()>;
              fn r#setCellInfoListRate(&self, _arg_serial: i32, _arg_rate: i32) -> binder::Result<()>;
              fn r#setIndicationFilter(&self, _arg_serial: i32, _arg_indicationFilter: i32) -> binder::Result<()>;
              fn r#setLinkCapacityReportingCriteria(&self, _arg_serial: i32, _arg_hysteresisMs: i32, _arg_hysteresisDlKbps: i32, _arg_hysteresisUlKbps: i32, _arg_thresholdsDownlinkKbps: &[i32], _arg_thresholdsUplinkKbps: &[i32], _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<()>;
              fn r#setLocationUpdates(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()>;
              fn r#setNetworkSelectionModeAutomatic(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#setNetworkSelectionModeManual(&self, _arg_serial: i32, _arg_operatorNumeric: &str, _arg_ran: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<()>;
              fn r#setNrDualConnectivityState(&self, _arg_serial: i32, _arg_nrDualConnectivityState: crate::mangled::_7_android_8_hardware_5_radio_7_network_23_NrDualConnectivityState) -> binder::Result<()>;
              fn r#setResponseFunctions(&self, _arg_radioNetworkResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_21_IRadioNetworkResponse>, _arg_radioNetworkIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_23_IRadioNetworkIndication>) -> binder::Result<()>;
              fn r#setSignalStrengthReportingCriteria(&self, _arg_serial: i32, _arg_signalThresholdInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SignalThresholdInfo]) -> binder::Result<()>;
              fn r#setSuppServiceNotifications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()>;
              fn r#setSystemSelectionChannels(&self, _arg_serial: i32, _arg_specifyChannels: bool, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<()>;
              fn r#startNetworkScan(&self, _arg_serial: i32, _arg_request: &crate::mangled::_7_android_8_hardware_5_radio_7_network_18_NetworkScanRequest) -> binder::Result<()>;
              fn r#stopNetworkScan(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#supplyNetworkDepersonalization(&self, _arg_serial: i32, _arg_netPin: &str) -> binder::Result<()>;
              fn r#setUsageSetting(&self, _arg_serial: i32, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<()>;
              fn r#getUsageSetting(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioNetworkDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioNetworkDefaultRef) -> IRadioNetworkDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioNetworkAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.network.IRadioNetwork" }
              fn r#getAllowedNetworkTypesBitmap(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getAvailableBandModes(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getAvailableNetworks(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getBarringInfo(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getCdmaRoamingPreference(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getCellInfoList(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getDataRegistrationState(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getImsRegistrationState(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getNetworkSelectionMode(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getOperator(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getSignalStrength(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getSystemSelectionChannels(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getVoiceRadioTechnology(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getVoiceRegistrationState(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#isNrDualConnectivityEnabled(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#responseAcknowledgement(&self) -> std::future::Ready<binder::Result<()>>;
              fn r#setAllowedNetworkTypesBitmap(&self, _arg_serial: i32, _arg_networkTypeBitmap: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#setBandMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode) -> std::future::Ready<binder::Result<()>>;
              fn r#setBarringPassword(&self, _arg_serial: i32, _arg_facility: &str, _arg_oldPassword: &str, _arg_newPassword: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#setCdmaRoamingPreference(&self, _arg_serial: i32, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> std::future::Ready<binder::Result<()>>;
              fn r#setCellInfoListRate(&self, _arg_serial: i32, _arg_rate: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#setIndicationFilter(&self, _arg_serial: i32, _arg_indicationFilter: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#setLinkCapacityReportingCriteria(&self, _arg_serial: i32, _arg_hysteresisMs: i32, _arg_hysteresisDlKbps: i32, _arg_hysteresisUlKbps: i32, _arg_thresholdsDownlinkKbps: &[i32], _arg_thresholdsUplinkKbps: &[i32], _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> std::future::Ready<binder::Result<()>>;
              fn r#setLocationUpdates(&self, _arg_serial: i32, _arg_enable: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#setNetworkSelectionModeAutomatic(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#setNetworkSelectionModeManual(&self, _arg_serial: i32, _arg_operatorNumeric: &str, _arg_ran: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> std::future::Ready<binder::Result<()>>;
              fn r#setNrDualConnectivityState(&self, _arg_serial: i32, _arg_nrDualConnectivityState: crate::mangled::_7_android_8_hardware_5_radio_7_network_23_NrDualConnectivityState) -> std::future::Ready<binder::Result<()>>;
              fn r#setResponseFunctions(&self, _arg_radioNetworkResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_21_IRadioNetworkResponse>, _arg_radioNetworkIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_23_IRadioNetworkIndication>) -> std::future::Ready<binder::Result<()>>;
              fn r#setSignalStrengthReportingCriteria(&self, _arg_serial: i32, _arg_signalThresholdInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SignalThresholdInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#setSuppServiceNotifications(&self, _arg_serial: i32, _arg_enable: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#setSystemSelectionChannels(&self, _arg_serial: i32, _arg_specifyChannels: bool, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> std::future::Ready<binder::Result<()>>;
              fn r#startNetworkScan(&self, _arg_serial: i32, _arg_request: &crate::mangled::_7_android_8_hardware_5_radio_7_network_18_NetworkScanRequest) -> std::future::Ready<binder::Result<()>>;
              fn r#stopNetworkScan(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#supplyNetworkDepersonalization(&self, _arg_serial: i32, _arg_netPin: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#setUsageSetting(&self, _arg_serial: i32, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> std::future::Ready<binder::Result<()>>;
              fn r#getUsageSetting(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioNetworkAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.network.IRadioNetwork" }
              async fn r#getAllowedNetworkTypesBitmap(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getAvailableBandModes(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getAvailableNetworks(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getBarringInfo(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getCdmaRoamingPreference(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getCellInfoList(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getDataRegistrationState(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getImsRegistrationState(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getNetworkSelectionMode(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getOperator(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getSignalStrength(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getSystemSelectionChannels(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getVoiceRadioTechnology(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getVoiceRegistrationState(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#isNrDualConnectivityEnabled(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#responseAcknowledgement(&self) -> binder::Result<()>;
              async fn r#setAllowedNetworkTypesBitmap(&self, _arg_serial: i32, _arg_networkTypeBitmap: i32) -> binder::Result<()>;
              async fn r#setBandMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode) -> binder::Result<()>;
              async fn r#setBarringPassword(&self, _arg_serial: i32, _arg_facility: &str, _arg_oldPassword: &str, _arg_newPassword: &str) -> binder::Result<()>;
              async fn r#setCdmaRoamingPreference(&self, _arg_serial: i32, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<()>;
              async fn r#setCellInfoListRate(&self, _arg_serial: i32, _arg_rate: i32) -> binder::Result<()>;
              async fn r#setIndicationFilter(&self, _arg_serial: i32, _arg_indicationFilter: i32) -> binder::Result<()>;
              async fn r#setLinkCapacityReportingCriteria(&self, _arg_serial: i32, _arg_hysteresisMs: i32, _arg_hysteresisDlKbps: i32, _arg_hysteresisUlKbps: i32, _arg_thresholdsDownlinkKbps: &[i32], _arg_thresholdsUplinkKbps: &[i32], _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<()>;
              async fn r#setLocationUpdates(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()>;
              async fn r#setNetworkSelectionModeAutomatic(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#setNetworkSelectionModeManual(&self, _arg_serial: i32, _arg_operatorNumeric: &str, _arg_ran: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<()>;
              async fn r#setNrDualConnectivityState(&self, _arg_serial: i32, _arg_nrDualConnectivityState: crate::mangled::_7_android_8_hardware_5_radio_7_network_23_NrDualConnectivityState) -> binder::Result<()>;
              async fn r#setResponseFunctions(&self, _arg_radioNetworkResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_21_IRadioNetworkResponse>, _arg_radioNetworkIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_23_IRadioNetworkIndication>) -> binder::Result<()>;
              async fn r#setSignalStrengthReportingCriteria(&self, _arg_serial: i32, _arg_signalThresholdInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SignalThresholdInfo]) -> binder::Result<()>;
              async fn r#setSuppServiceNotifications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()>;
              async fn r#setSystemSelectionChannels(&self, _arg_serial: i32, _arg_specifyChannels: bool, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<()>;
              async fn r#startNetworkScan(&self, _arg_serial: i32, _arg_request: &crate::mangled::_7_android_8_hardware_5_radio_7_network_18_NetworkScanRequest) -> binder::Result<()>;
              async fn r#stopNetworkScan(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#supplyNetworkDepersonalization(&self, _arg_serial: i32, _arg_netPin: &str) -> binder::Result<()>;
              async fn r#setUsageSetting(&self, _arg_serial: i32, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<()>;
              async fn r#getUsageSetting(&self, _arg_serial: i32) -> binder::Result<()>;
            }
            impl BnRadioNetwork {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioNetwork>
              where
                T: IRadioNetworkAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioNetwork for Wrapper<T, R>
                where
                  T: IRadioNetworkAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#getAllowedNetworkTypesBitmap(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getAllowedNetworkTypesBitmap(_arg_serial))
                  }
                  fn r#getAvailableBandModes(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getAvailableBandModes(_arg_serial))
                  }
                  fn r#getAvailableNetworks(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getAvailableNetworks(_arg_serial))
                  }
                  fn r#getBarringInfo(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getBarringInfo(_arg_serial))
                  }
                  fn r#getCdmaRoamingPreference(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCdmaRoamingPreference(_arg_serial))
                  }
                  fn r#getCellInfoList(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCellInfoList(_arg_serial))
                  }
                  fn r#getDataRegistrationState(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getDataRegistrationState(_arg_serial))
                  }
                  fn r#getImsRegistrationState(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getImsRegistrationState(_arg_serial))
                  }
                  fn r#getNetworkSelectionMode(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getNetworkSelectionMode(_arg_serial))
                  }
                  fn r#getOperator(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getOperator(_arg_serial))
                  }
                  fn r#getSignalStrength(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSignalStrength(_arg_serial))
                  }
                  fn r#getSystemSelectionChannels(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSystemSelectionChannels(_arg_serial))
                  }
                  fn r#getVoiceRadioTechnology(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getVoiceRadioTechnology(_arg_serial))
                  }
                  fn r#getVoiceRegistrationState(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getVoiceRegistrationState(_arg_serial))
                  }
                  fn r#isNrDualConnectivityEnabled(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#isNrDualConnectivityEnabled(_arg_serial))
                  }
                  fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#responseAcknowledgement())
                  }
                  fn r#setAllowedNetworkTypesBitmap(&self, _arg_serial: i32, _arg_networkTypeBitmap: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setAllowedNetworkTypesBitmap(_arg_serial, _arg_networkTypeBitmap))
                  }
                  fn r#setBandMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setBandMode(_arg_serial, _arg_mode))
                  }
                  fn r#setBarringPassword(&self, _arg_serial: i32, _arg_facility: &str, _arg_oldPassword: &str, _arg_newPassword: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setBarringPassword(_arg_serial, _arg_facility, _arg_oldPassword, _arg_newPassword))
                  }
                  fn r#setCdmaRoamingPreference(&self, _arg_serial: i32, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCdmaRoamingPreference(_arg_serial, _arg_type))
                  }
                  fn r#setCellInfoListRate(&self, _arg_serial: i32, _arg_rate: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCellInfoListRate(_arg_serial, _arg_rate))
                  }
                  fn r#setIndicationFilter(&self, _arg_serial: i32, _arg_indicationFilter: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setIndicationFilter(_arg_serial, _arg_indicationFilter))
                  }
                  fn r#setLinkCapacityReportingCriteria(&self, _arg_serial: i32, _arg_hysteresisMs: i32, _arg_hysteresisDlKbps: i32, _arg_hysteresisUlKbps: i32, _arg_thresholdsDownlinkKbps: &[i32], _arg_thresholdsUplinkKbps: &[i32], _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setLinkCapacityReportingCriteria(_arg_serial, _arg_hysteresisMs, _arg_hysteresisDlKbps, _arg_hysteresisUlKbps, _arg_thresholdsDownlinkKbps, _arg_thresholdsUplinkKbps, _arg_accessNetwork))
                  }
                  fn r#setLocationUpdates(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setLocationUpdates(_arg_serial, _arg_enable))
                  }
                  fn r#setNetworkSelectionModeAutomatic(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setNetworkSelectionModeAutomatic(_arg_serial))
                  }
                  fn r#setNetworkSelectionModeManual(&self, _arg_serial: i32, _arg_operatorNumeric: &str, _arg_ran: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setNetworkSelectionModeManual(_arg_serial, _arg_operatorNumeric, _arg_ran))
                  }
                  fn r#setNrDualConnectivityState(&self, _arg_serial: i32, _arg_nrDualConnectivityState: crate::mangled::_7_android_8_hardware_5_radio_7_network_23_NrDualConnectivityState) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setNrDualConnectivityState(_arg_serial, _arg_nrDualConnectivityState))
                  }
                  fn r#setResponseFunctions(&self, _arg_radioNetworkResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_21_IRadioNetworkResponse>, _arg_radioNetworkIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_23_IRadioNetworkIndication>) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setResponseFunctions(_arg_radioNetworkResponse, _arg_radioNetworkIndication))
                  }
                  fn r#setSignalStrengthReportingCriteria(&self, _arg_serial: i32, _arg_signalThresholdInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SignalThresholdInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setSignalStrengthReportingCriteria(_arg_serial, _arg_signalThresholdInfos))
                  }
                  fn r#setSuppServiceNotifications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setSuppServiceNotifications(_arg_serial, _arg_enable))
                  }
                  fn r#setSystemSelectionChannels(&self, _arg_serial: i32, _arg_specifyChannels: bool, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setSystemSelectionChannels(_arg_serial, _arg_specifyChannels, _arg_specifiers))
                  }
                  fn r#startNetworkScan(&self, _arg_serial: i32, _arg_request: &crate::mangled::_7_android_8_hardware_5_radio_7_network_18_NetworkScanRequest) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#startNetworkScan(_arg_serial, _arg_request))
                  }
                  fn r#stopNetworkScan(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#stopNetworkScan(_arg_serial))
                  }
                  fn r#supplyNetworkDepersonalization(&self, _arg_serial: i32, _arg_netPin: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#supplyNetworkDepersonalization(_arg_serial, _arg_netPin))
                  }
                  fn r#setUsageSetting(&self, _arg_serial: i32, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setUsageSetting(_arg_serial, _arg_usageSetting))
                  }
                  fn r#getUsageSetting(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getUsageSetting(_arg_serial))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioNetworkDefault: Send + Sync {
              fn r#getAllowedNetworkTypesBitmap(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getAvailableBandModes(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getAvailableNetworks(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getBarringInfo(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCdmaRoamingPreference(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCellInfoList(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDataRegistrationState(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getImsRegistrationState(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getNetworkSelectionMode(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getOperator(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSignalStrength(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSystemSelectionChannels(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getVoiceRadioTechnology(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getVoiceRegistrationState(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#isNrDualConnectivityEnabled(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setAllowedNetworkTypesBitmap(&self, _arg_serial: i32, _arg_networkTypeBitmap: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setBandMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setBarringPassword(&self, _arg_serial: i32, _arg_facility: &str, _arg_oldPassword: &str, _arg_newPassword: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCdmaRoamingPreference(&self, _arg_serial: i32, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCellInfoListRate(&self, _arg_serial: i32, _arg_rate: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setIndicationFilter(&self, _arg_serial: i32, _arg_indicationFilter: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setLinkCapacityReportingCriteria(&self, _arg_serial: i32, _arg_hysteresisMs: i32, _arg_hysteresisDlKbps: i32, _arg_hysteresisUlKbps: i32, _arg_thresholdsDownlinkKbps: &[i32], _arg_thresholdsUplinkKbps: &[i32], _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setLocationUpdates(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setNetworkSelectionModeAutomatic(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setNetworkSelectionModeManual(&self, _arg_serial: i32, _arg_operatorNumeric: &str, _arg_ran: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setNrDualConnectivityState(&self, _arg_serial: i32, _arg_nrDualConnectivityState: crate::mangled::_7_android_8_hardware_5_radio_7_network_23_NrDualConnectivityState) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setResponseFunctions(&self, _arg_radioNetworkResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_21_IRadioNetworkResponse>, _arg_radioNetworkIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_23_IRadioNetworkIndication>) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setSignalStrengthReportingCriteria(&self, _arg_serial: i32, _arg_signalThresholdInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SignalThresholdInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setSuppServiceNotifications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setSystemSelectionChannels(&self, _arg_serial: i32, _arg_specifyChannels: bool, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#startNetworkScan(&self, _arg_serial: i32, _arg_request: &crate::mangled::_7_android_8_hardware_5_radio_7_network_18_NetworkScanRequest) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#stopNetworkScan(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#supplyNetworkDepersonalization(&self, _arg_serial: i32, _arg_netPin: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setUsageSetting(&self, _arg_serial: i32, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getUsageSetting(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#getAllowedNetworkTypesBitmap: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#getAvailableBandModes: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#getAvailableNetworks: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#getBarringInfo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#getCdmaRoamingPreference: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#getCellInfoList: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#getDataRegistrationState: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#getImsRegistrationState: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#getNetworkSelectionMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#getOperator: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#getSignalStrength: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#getSystemSelectionChannels: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#getVoiceRadioTechnology: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#getVoiceRegistrationState: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#isNrDualConnectivityEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#responseAcknowledgement: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#setAllowedNetworkTypesBitmap: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
              pub const r#setBandMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
              pub const r#setBarringPassword: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
              pub const r#setCdmaRoamingPreference: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
              pub const r#setCellInfoListRate: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 20;
              pub const r#setIndicationFilter: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 21;
              pub const r#setLinkCapacityReportingCriteria: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 22;
              pub const r#setLocationUpdates: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 23;
              pub const r#setNetworkSelectionModeAutomatic: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 24;
              pub const r#setNetworkSelectionModeManual: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 25;
              pub const r#setNrDualConnectivityState: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 26;
              pub const r#setResponseFunctions: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 27;
              pub const r#setSignalStrengthReportingCriteria: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 28;
              pub const r#setSuppServiceNotifications: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 29;
              pub const r#setSystemSelectionChannels: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 30;
              pub const r#startNetworkScan: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 31;
              pub const r#stopNetworkScan: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 32;
              pub const r#supplyNetworkDepersonalization: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 33;
              pub const r#setUsageSetting: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 34;
              pub const r#getUsageSetting: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 35;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioNetworkDefaultRef = Option<std::sync::Arc<dyn IRadioNetworkDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioNetworkDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "57e8e923513d80a26102e450d335e89b4346be66";
            impl BpRadioNetwork {
              fn build_parcel_getAllowedNetworkTypesBitmap(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getAllowedNetworkTypesBitmap(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getAllowedNetworkTypesBitmap(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getAvailableBandModes(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getAvailableBandModes(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getAvailableBandModes(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getAvailableNetworks(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getAvailableNetworks(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getAvailableNetworks(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getBarringInfo(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getBarringInfo(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getBarringInfo(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCdmaRoamingPreference(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getCdmaRoamingPreference(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCdmaRoamingPreference(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCellInfoList(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getCellInfoList(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCellInfoList(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getDataRegistrationState(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getDataRegistrationState(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDataRegistrationState(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getImsRegistrationState(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getImsRegistrationState(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getImsRegistrationState(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getNetworkSelectionMode(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getNetworkSelectionMode(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getNetworkSelectionMode(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getOperator(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getOperator(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getOperator(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSignalStrength(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getSignalStrength(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSignalStrength(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSystemSelectionChannels(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getSystemSelectionChannels(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSystemSelectionChannels(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getVoiceRadioTechnology(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getVoiceRadioTechnology(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getVoiceRadioTechnology(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getVoiceRegistrationState(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getVoiceRegistrationState(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getVoiceRegistrationState(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_isNrDualConnectivityEnabled(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_isNrDualConnectivityEnabled(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#isNrDualConnectivityEnabled(_arg_serial);
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
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#responseAcknowledgement();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setAllowedNetworkTypesBitmap(&self, _arg_serial: i32, _arg_networkTypeBitmap: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_networkTypeBitmap)?;
                Ok(aidl_data)
              }
              fn read_response_setAllowedNetworkTypesBitmap(&self, _arg_serial: i32, _arg_networkTypeBitmap: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setAllowedNetworkTypesBitmap(_arg_serial, _arg_networkTypeBitmap);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setBandMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_mode)?;
                Ok(aidl_data)
              }
              fn read_response_setBandMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setBandMode(_arg_serial, _arg_mode);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setBarringPassword(&self, _arg_serial: i32, _arg_facility: &str, _arg_oldPassword: &str, _arg_newPassword: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_facility)?;
                aidl_data.write(_arg_oldPassword)?;
                aidl_data.write(_arg_newPassword)?;
                Ok(aidl_data)
              }
              fn read_response_setBarringPassword(&self, _arg_serial: i32, _arg_facility: &str, _arg_oldPassword: &str, _arg_newPassword: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setBarringPassword(_arg_serial, _arg_facility, _arg_oldPassword, _arg_newPassword);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCdmaRoamingPreference(&self, _arg_serial: i32, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_setCdmaRoamingPreference(&self, _arg_serial: i32, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCdmaRoamingPreference(_arg_serial, _arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCellInfoListRate(&self, _arg_serial: i32, _arg_rate: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_rate)?;
                Ok(aidl_data)
              }
              fn read_response_setCellInfoListRate(&self, _arg_serial: i32, _arg_rate: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCellInfoListRate(_arg_serial, _arg_rate);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setIndicationFilter(&self, _arg_serial: i32, _arg_indicationFilter: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_indicationFilter)?;
                Ok(aidl_data)
              }
              fn read_response_setIndicationFilter(&self, _arg_serial: i32, _arg_indicationFilter: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setIndicationFilter(_arg_serial, _arg_indicationFilter);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setLinkCapacityReportingCriteria(&self, _arg_serial: i32, _arg_hysteresisMs: i32, _arg_hysteresisDlKbps: i32, _arg_hysteresisUlKbps: i32, _arg_thresholdsDownlinkKbps: &[i32], _arg_thresholdsUplinkKbps: &[i32], _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_hysteresisMs)?;
                aidl_data.write(&_arg_hysteresisDlKbps)?;
                aidl_data.write(&_arg_hysteresisUlKbps)?;
                aidl_data.write(_arg_thresholdsDownlinkKbps)?;
                aidl_data.write(_arg_thresholdsUplinkKbps)?;
                aidl_data.write(&_arg_accessNetwork)?;
                Ok(aidl_data)
              }
              fn read_response_setLinkCapacityReportingCriteria(&self, _arg_serial: i32, _arg_hysteresisMs: i32, _arg_hysteresisDlKbps: i32, _arg_hysteresisUlKbps: i32, _arg_thresholdsDownlinkKbps: &[i32], _arg_thresholdsUplinkKbps: &[i32], _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setLinkCapacityReportingCriteria(_arg_serial, _arg_hysteresisMs, _arg_hysteresisDlKbps, _arg_hysteresisUlKbps, _arg_thresholdsDownlinkKbps, _arg_thresholdsUplinkKbps, _arg_accessNetwork);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setLocationUpdates(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_enable)?;
                Ok(aidl_data)
              }
              fn read_response_setLocationUpdates(&self, _arg_serial: i32, _arg_enable: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setLocationUpdates(_arg_serial, _arg_enable);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setNetworkSelectionModeAutomatic(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_setNetworkSelectionModeAutomatic(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setNetworkSelectionModeAutomatic(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setNetworkSelectionModeManual(&self, _arg_serial: i32, _arg_operatorNumeric: &str, _arg_ran: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_operatorNumeric)?;
                aidl_data.write(&_arg_ran)?;
                Ok(aidl_data)
              }
              fn read_response_setNetworkSelectionModeManual(&self, _arg_serial: i32, _arg_operatorNumeric: &str, _arg_ran: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setNetworkSelectionModeManual(_arg_serial, _arg_operatorNumeric, _arg_ran);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setNrDualConnectivityState(&self, _arg_serial: i32, _arg_nrDualConnectivityState: crate::mangled::_7_android_8_hardware_5_radio_7_network_23_NrDualConnectivityState) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_nrDualConnectivityState)?;
                Ok(aidl_data)
              }
              fn read_response_setNrDualConnectivityState(&self, _arg_serial: i32, _arg_nrDualConnectivityState: crate::mangled::_7_android_8_hardware_5_radio_7_network_23_NrDualConnectivityState, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setNrDualConnectivityState(_arg_serial, _arg_nrDualConnectivityState);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setResponseFunctions(&self, _arg_radioNetworkResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_21_IRadioNetworkResponse>, _arg_radioNetworkIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_23_IRadioNetworkIndication>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_radioNetworkResponse)?;
                aidl_data.write(_arg_radioNetworkIndication)?;
                Ok(aidl_data)
              }
              fn read_response_setResponseFunctions(&self, _arg_radioNetworkResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_21_IRadioNetworkResponse>, _arg_radioNetworkIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_23_IRadioNetworkIndication>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setResponseFunctions(_arg_radioNetworkResponse, _arg_radioNetworkIndication);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setSignalStrengthReportingCriteria(&self, _arg_serial: i32, _arg_signalThresholdInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SignalThresholdInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_signalThresholdInfos)?;
                Ok(aidl_data)
              }
              fn read_response_setSignalStrengthReportingCriteria(&self, _arg_serial: i32, _arg_signalThresholdInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SignalThresholdInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setSignalStrengthReportingCriteria(_arg_serial, _arg_signalThresholdInfos);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setSuppServiceNotifications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_enable)?;
                Ok(aidl_data)
              }
              fn read_response_setSuppServiceNotifications(&self, _arg_serial: i32, _arg_enable: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setSuppServiceNotifications(_arg_serial, _arg_enable);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setSystemSelectionChannels(&self, _arg_serial: i32, _arg_specifyChannels: bool, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_specifyChannels)?;
                aidl_data.write(_arg_specifiers)?;
                Ok(aidl_data)
              }
              fn read_response_setSystemSelectionChannels(&self, _arg_serial: i32, _arg_specifyChannels: bool, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setSystemSelectionChannels(_arg_serial, _arg_specifyChannels, _arg_specifiers);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_startNetworkScan(&self, _arg_serial: i32, _arg_request: &crate::mangled::_7_android_8_hardware_5_radio_7_network_18_NetworkScanRequest) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_request)?;
                Ok(aidl_data)
              }
              fn read_response_startNetworkScan(&self, _arg_serial: i32, _arg_request: &crate::mangled::_7_android_8_hardware_5_radio_7_network_18_NetworkScanRequest, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#startNetworkScan(_arg_serial, _arg_request);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_stopNetworkScan(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_stopNetworkScan(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#stopNetworkScan(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_supplyNetworkDepersonalization(&self, _arg_serial: i32, _arg_netPin: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_netPin)?;
                Ok(aidl_data)
              }
              fn read_response_supplyNetworkDepersonalization(&self, _arg_serial: i32, _arg_netPin: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#supplyNetworkDepersonalization(_arg_serial, _arg_netPin);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setUsageSetting(&self, _arg_serial: i32, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_usageSetting)?;
                Ok(aidl_data)
              }
              fn read_response_setUsageSetting(&self, _arg_serial: i32, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#setUsageSetting(_arg_serial, _arg_usageSetting);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getUsageSetting(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getUsageSetting(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetwork>::getDefaultImpl() {
                    return _aidl_default_impl.r#getUsageSetting(_arg_serial);
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
            impl IRadioNetwork for BpRadioNetwork {
              fn r#getAllowedNetworkTypesBitmap(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getAllowedNetworkTypesBitmap(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAllowedNetworkTypesBitmap, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getAllowedNetworkTypesBitmap(_arg_serial, _aidl_reply)
              }
              fn r#getAvailableBandModes(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getAvailableBandModes(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAvailableBandModes, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getAvailableBandModes(_arg_serial, _aidl_reply)
              }
              fn r#getAvailableNetworks(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getAvailableNetworks(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAvailableNetworks, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getAvailableNetworks(_arg_serial, _aidl_reply)
              }
              fn r#getBarringInfo(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getBarringInfo(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getBarringInfo, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getBarringInfo(_arg_serial, _aidl_reply)
              }
              fn r#getCdmaRoamingPreference(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCdmaRoamingPreference(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaRoamingPreference, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCdmaRoamingPreference(_arg_serial, _aidl_reply)
              }
              fn r#getCellInfoList(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCellInfoList(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCellInfoList, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCellInfoList(_arg_serial, _aidl_reply)
              }
              fn r#getDataRegistrationState(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getDataRegistrationState(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDataRegistrationState, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDataRegistrationState(_arg_serial, _aidl_reply)
              }
              fn r#getImsRegistrationState(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getImsRegistrationState(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getImsRegistrationState, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getImsRegistrationState(_arg_serial, _aidl_reply)
              }
              fn r#getNetworkSelectionMode(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getNetworkSelectionMode(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getNetworkSelectionMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getNetworkSelectionMode(_arg_serial, _aidl_reply)
              }
              fn r#getOperator(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getOperator(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getOperator, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getOperator(_arg_serial, _aidl_reply)
              }
              fn r#getSignalStrength(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSignalStrength(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSignalStrength, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSignalStrength(_arg_serial, _aidl_reply)
              }
              fn r#getSystemSelectionChannels(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSystemSelectionChannels(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSystemSelectionChannels, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSystemSelectionChannels(_arg_serial, _aidl_reply)
              }
              fn r#getVoiceRadioTechnology(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getVoiceRadioTechnology(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getVoiceRadioTechnology, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getVoiceRadioTechnology(_arg_serial, _aidl_reply)
              }
              fn r#getVoiceRegistrationState(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getVoiceRegistrationState(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getVoiceRegistrationState, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getVoiceRegistrationState(_arg_serial, _aidl_reply)
              }
              fn r#isNrDualConnectivityEnabled(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_isNrDualConnectivityEnabled(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#isNrDualConnectivityEnabled, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_isNrDualConnectivityEnabled(_arg_serial, _aidl_reply)
              }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_responseAcknowledgement()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#responseAcknowledgement, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_responseAcknowledgement(_aidl_reply)
              }
              fn r#setAllowedNetworkTypesBitmap(&self, _arg_serial: i32, _arg_networkTypeBitmap: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setAllowedNetworkTypesBitmap(_arg_serial, _arg_networkTypeBitmap)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setAllowedNetworkTypesBitmap, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setAllowedNetworkTypesBitmap(_arg_serial, _arg_networkTypeBitmap, _aidl_reply)
              }
              fn r#setBandMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setBandMode(_arg_serial, _arg_mode)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setBandMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setBandMode(_arg_serial, _arg_mode, _aidl_reply)
              }
              fn r#setBarringPassword(&self, _arg_serial: i32, _arg_facility: &str, _arg_oldPassword: &str, _arg_newPassword: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setBarringPassword(_arg_serial, _arg_facility, _arg_oldPassword, _arg_newPassword)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setBarringPassword, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setBarringPassword(_arg_serial, _arg_facility, _arg_oldPassword, _arg_newPassword, _aidl_reply)
              }
              fn r#setCdmaRoamingPreference(&self, _arg_serial: i32, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCdmaRoamingPreference(_arg_serial, _arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaRoamingPreference, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCdmaRoamingPreference(_arg_serial, _arg_type, _aidl_reply)
              }
              fn r#setCellInfoListRate(&self, _arg_serial: i32, _arg_rate: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCellInfoListRate(_arg_serial, _arg_rate)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCellInfoListRate, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCellInfoListRate(_arg_serial, _arg_rate, _aidl_reply)
              }
              fn r#setIndicationFilter(&self, _arg_serial: i32, _arg_indicationFilter: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setIndicationFilter(_arg_serial, _arg_indicationFilter)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setIndicationFilter, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setIndicationFilter(_arg_serial, _arg_indicationFilter, _aidl_reply)
              }
              fn r#setLinkCapacityReportingCriteria(&self, _arg_serial: i32, _arg_hysteresisMs: i32, _arg_hysteresisDlKbps: i32, _arg_hysteresisUlKbps: i32, _arg_thresholdsDownlinkKbps: &[i32], _arg_thresholdsUplinkKbps: &[i32], _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setLinkCapacityReportingCriteria(_arg_serial, _arg_hysteresisMs, _arg_hysteresisDlKbps, _arg_hysteresisUlKbps, _arg_thresholdsDownlinkKbps, _arg_thresholdsUplinkKbps, _arg_accessNetwork)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setLinkCapacityReportingCriteria, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setLinkCapacityReportingCriteria(_arg_serial, _arg_hysteresisMs, _arg_hysteresisDlKbps, _arg_hysteresisUlKbps, _arg_thresholdsDownlinkKbps, _arg_thresholdsUplinkKbps, _arg_accessNetwork, _aidl_reply)
              }
              fn r#setLocationUpdates(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setLocationUpdates(_arg_serial, _arg_enable)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setLocationUpdates, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setLocationUpdates(_arg_serial, _arg_enable, _aidl_reply)
              }
              fn r#setNetworkSelectionModeAutomatic(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setNetworkSelectionModeAutomatic(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNetworkSelectionModeAutomatic, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setNetworkSelectionModeAutomatic(_arg_serial, _aidl_reply)
              }
              fn r#setNetworkSelectionModeManual(&self, _arg_serial: i32, _arg_operatorNumeric: &str, _arg_ran: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setNetworkSelectionModeManual(_arg_serial, _arg_operatorNumeric, _arg_ran)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNetworkSelectionModeManual, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setNetworkSelectionModeManual(_arg_serial, _arg_operatorNumeric, _arg_ran, _aidl_reply)
              }
              fn r#setNrDualConnectivityState(&self, _arg_serial: i32, _arg_nrDualConnectivityState: crate::mangled::_7_android_8_hardware_5_radio_7_network_23_NrDualConnectivityState) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setNrDualConnectivityState(_arg_serial, _arg_nrDualConnectivityState)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNrDualConnectivityState, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setNrDualConnectivityState(_arg_serial, _arg_nrDualConnectivityState, _aidl_reply)
              }
              fn r#setResponseFunctions(&self, _arg_radioNetworkResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_21_IRadioNetworkResponse>, _arg_radioNetworkIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_23_IRadioNetworkIndication>) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setResponseFunctions(_arg_radioNetworkResponse, _arg_radioNetworkIndication)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setResponseFunctions(_arg_radioNetworkResponse, _arg_radioNetworkIndication, _aidl_reply)
              }
              fn r#setSignalStrengthReportingCriteria(&self, _arg_serial: i32, _arg_signalThresholdInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SignalThresholdInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setSignalStrengthReportingCriteria(_arg_serial, _arg_signalThresholdInfos)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSignalStrengthReportingCriteria, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setSignalStrengthReportingCriteria(_arg_serial, _arg_signalThresholdInfos, _aidl_reply)
              }
              fn r#setSuppServiceNotifications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setSuppServiceNotifications(_arg_serial, _arg_enable)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSuppServiceNotifications, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setSuppServiceNotifications(_arg_serial, _arg_enable, _aidl_reply)
              }
              fn r#setSystemSelectionChannels(&self, _arg_serial: i32, _arg_specifyChannels: bool, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setSystemSelectionChannels(_arg_serial, _arg_specifyChannels, _arg_specifiers)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSystemSelectionChannels, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setSystemSelectionChannels(_arg_serial, _arg_specifyChannels, _arg_specifiers, _aidl_reply)
              }
              fn r#startNetworkScan(&self, _arg_serial: i32, _arg_request: &crate::mangled::_7_android_8_hardware_5_radio_7_network_18_NetworkScanRequest) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_startNetworkScan(_arg_serial, _arg_request)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#startNetworkScan, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_startNetworkScan(_arg_serial, _arg_request, _aidl_reply)
              }
              fn r#stopNetworkScan(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_stopNetworkScan(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#stopNetworkScan, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_stopNetworkScan(_arg_serial, _aidl_reply)
              }
              fn r#supplyNetworkDepersonalization(&self, _arg_serial: i32, _arg_netPin: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_supplyNetworkDepersonalization(_arg_serial, _arg_netPin)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyNetworkDepersonalization, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_supplyNetworkDepersonalization(_arg_serial, _arg_netPin, _aidl_reply)
              }
              fn r#setUsageSetting(&self, _arg_serial: i32, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setUsageSetting(_arg_serial, _arg_usageSetting)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setUsageSetting, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setUsageSetting(_arg_serial, _arg_usageSetting, _aidl_reply)
              }
              fn r#getUsageSetting(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getUsageSetting(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getUsageSetting, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getUsageSetting(_arg_serial, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioNetworkAsync<P> for BpRadioNetwork {
              fn r#getAllowedNetworkTypesBitmap(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getAllowedNetworkTypesBitmap(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAllowedNetworkTypesBitmap, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getAllowedNetworkTypesBitmap(_arg_serial, _aidl_reply))
              }
              fn r#getAvailableBandModes(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getAvailableBandModes(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAvailableBandModes, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getAvailableBandModes(_arg_serial, _aidl_reply))
              }
              fn r#getAvailableNetworks(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getAvailableNetworks(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAvailableNetworks, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getAvailableNetworks(_arg_serial, _aidl_reply))
              }
              fn r#getBarringInfo(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getBarringInfo(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getBarringInfo, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getBarringInfo(_arg_serial, _aidl_reply))
              }
              fn r#getCdmaRoamingPreference(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCdmaRoamingPreference(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaRoamingPreference, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCdmaRoamingPreference(_arg_serial, _aidl_reply))
              }
              fn r#getCellInfoList(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCellInfoList(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCellInfoList, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCellInfoList(_arg_serial, _aidl_reply))
              }
              fn r#getDataRegistrationState(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getDataRegistrationState(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDataRegistrationState, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getDataRegistrationState(_arg_serial, _aidl_reply))
              }
              fn r#getImsRegistrationState(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getImsRegistrationState(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getImsRegistrationState, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getImsRegistrationState(_arg_serial, _aidl_reply))
              }
              fn r#getNetworkSelectionMode(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getNetworkSelectionMode(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getNetworkSelectionMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getNetworkSelectionMode(_arg_serial, _aidl_reply))
              }
              fn r#getOperator(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getOperator(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getOperator, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getOperator(_arg_serial, _aidl_reply))
              }
              fn r#getSignalStrength(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSignalStrength(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSignalStrength, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSignalStrength(_arg_serial, _aidl_reply))
              }
              fn r#getSystemSelectionChannels(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSystemSelectionChannels(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSystemSelectionChannels, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSystemSelectionChannels(_arg_serial, _aidl_reply))
              }
              fn r#getVoiceRadioTechnology(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getVoiceRadioTechnology(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getVoiceRadioTechnology, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getVoiceRadioTechnology(_arg_serial, _aidl_reply))
              }
              fn r#getVoiceRegistrationState(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getVoiceRegistrationState(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getVoiceRegistrationState, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getVoiceRegistrationState(_arg_serial, _aidl_reply))
              }
              fn r#isNrDualConnectivityEnabled(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_isNrDualConnectivityEnabled(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#isNrDualConnectivityEnabled, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_isNrDualConnectivityEnabled(_arg_serial, _aidl_reply))
              }
              fn r#responseAcknowledgement(&self) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_responseAcknowledgement() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#responseAcknowledgement, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_responseAcknowledgement(_aidl_reply))
              }
              fn r#setAllowedNetworkTypesBitmap(&self, _arg_serial: i32, _arg_networkTypeBitmap: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setAllowedNetworkTypesBitmap(_arg_serial, _arg_networkTypeBitmap) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setAllowedNetworkTypesBitmap, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setAllowedNetworkTypesBitmap(_arg_serial, _arg_networkTypeBitmap, _aidl_reply))
              }
              fn r#setBandMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setBandMode(_arg_serial, _arg_mode) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setBandMode, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setBandMode(_arg_serial, _arg_mode, _aidl_reply))
              }
              fn r#setBarringPassword(&self, _arg_serial: i32, _arg_facility: &str, _arg_oldPassword: &str, _arg_newPassword: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setBarringPassword(_arg_serial, _arg_facility, _arg_oldPassword, _arg_newPassword) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setBarringPassword, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setBarringPassword(_arg_serial, _arg_facility, _arg_oldPassword, _arg_newPassword, _aidl_reply))
              }
              fn r#setCdmaRoamingPreference(&self, _arg_serial: i32, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCdmaRoamingPreference(_arg_serial, _arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaRoamingPreference, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCdmaRoamingPreference(_arg_serial, _arg_type, _aidl_reply))
              }
              fn r#setCellInfoListRate(&self, _arg_serial: i32, _arg_rate: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCellInfoListRate(_arg_serial, _arg_rate) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCellInfoListRate, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCellInfoListRate(_arg_serial, _arg_rate, _aidl_reply))
              }
              fn r#setIndicationFilter(&self, _arg_serial: i32, _arg_indicationFilter: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setIndicationFilter(_arg_serial, _arg_indicationFilter) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setIndicationFilter, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setIndicationFilter(_arg_serial, _arg_indicationFilter, _aidl_reply))
              }
              fn r#setLinkCapacityReportingCriteria(&self, _arg_serial: i32, _arg_hysteresisMs: i32, _arg_hysteresisDlKbps: i32, _arg_hysteresisUlKbps: i32, _arg_thresholdsDownlinkKbps: &[i32], _arg_thresholdsUplinkKbps: &[i32], _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setLinkCapacityReportingCriteria(_arg_serial, _arg_hysteresisMs, _arg_hysteresisDlKbps, _arg_hysteresisUlKbps, _arg_thresholdsDownlinkKbps, _arg_thresholdsUplinkKbps, _arg_accessNetwork) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setLinkCapacityReportingCriteria, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setLinkCapacityReportingCriteria(_arg_serial, _arg_hysteresisMs, _arg_hysteresisDlKbps, _arg_hysteresisUlKbps, _arg_thresholdsDownlinkKbps, _arg_thresholdsUplinkKbps, _arg_accessNetwork, _aidl_reply))
              }
              fn r#setLocationUpdates(&self, _arg_serial: i32, _arg_enable: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setLocationUpdates(_arg_serial, _arg_enable) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setLocationUpdates, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setLocationUpdates(_arg_serial, _arg_enable, _aidl_reply))
              }
              fn r#setNetworkSelectionModeAutomatic(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setNetworkSelectionModeAutomatic(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNetworkSelectionModeAutomatic, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setNetworkSelectionModeAutomatic(_arg_serial, _aidl_reply))
              }
              fn r#setNetworkSelectionModeManual(&self, _arg_serial: i32, _arg_operatorNumeric: &str, _arg_ran: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setNetworkSelectionModeManual(_arg_serial, _arg_operatorNumeric, _arg_ran) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNetworkSelectionModeManual, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setNetworkSelectionModeManual(_arg_serial, _arg_operatorNumeric, _arg_ran, _aidl_reply))
              }
              fn r#setNrDualConnectivityState(&self, _arg_serial: i32, _arg_nrDualConnectivityState: crate::mangled::_7_android_8_hardware_5_radio_7_network_23_NrDualConnectivityState) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setNrDualConnectivityState(_arg_serial, _arg_nrDualConnectivityState) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNrDualConnectivityState, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setNrDualConnectivityState(_arg_serial, _arg_nrDualConnectivityState, _aidl_reply))
              }
              fn r#setResponseFunctions(&self, _arg_radioNetworkResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_21_IRadioNetworkResponse>, _arg_radioNetworkIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_23_IRadioNetworkIndication>) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setResponseFunctions(_arg_radioNetworkResponse, _arg_radioNetworkIndication) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setResponseFunctions(_arg_radioNetworkResponse, _arg_radioNetworkIndication, _aidl_reply))
              }
              fn r#setSignalStrengthReportingCriteria(&self, _arg_serial: i32, _arg_signalThresholdInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SignalThresholdInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setSignalStrengthReportingCriteria(_arg_serial, _arg_signalThresholdInfos) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSignalStrengthReportingCriteria, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setSignalStrengthReportingCriteria(_arg_serial, _arg_signalThresholdInfos, _aidl_reply))
              }
              fn r#setSuppServiceNotifications(&self, _arg_serial: i32, _arg_enable: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setSuppServiceNotifications(_arg_serial, _arg_enable) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSuppServiceNotifications, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setSuppServiceNotifications(_arg_serial, _arg_enable, _aidl_reply))
              }
              fn r#setSystemSelectionChannels(&self, _arg_serial: i32, _arg_specifyChannels: bool, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setSystemSelectionChannels(_arg_serial, _arg_specifyChannels, _arg_specifiers) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSystemSelectionChannels, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setSystemSelectionChannels(_arg_serial, _arg_specifyChannels, _arg_specifiers, _aidl_reply))
              }
              fn r#startNetworkScan(&self, _arg_serial: i32, _arg_request: &crate::mangled::_7_android_8_hardware_5_radio_7_network_18_NetworkScanRequest) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_startNetworkScan(_arg_serial, _arg_request) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#startNetworkScan, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_startNetworkScan(_arg_serial, _arg_request, _aidl_reply))
              }
              fn r#stopNetworkScan(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_stopNetworkScan(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#stopNetworkScan, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_stopNetworkScan(_arg_serial, _aidl_reply))
              }
              fn r#supplyNetworkDepersonalization(&self, _arg_serial: i32, _arg_netPin: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_supplyNetworkDepersonalization(_arg_serial, _arg_netPin) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyNetworkDepersonalization, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_supplyNetworkDepersonalization(_arg_serial, _arg_netPin, _aidl_reply))
              }
              fn r#setUsageSetting(&self, _arg_serial: i32, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setUsageSetting(_arg_serial, _arg_usageSetting) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setUsageSetting, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setUsageSetting(_arg_serial, _arg_usageSetting, _aidl_reply))
              }
              fn r#getUsageSetting(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getUsageSetting(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getUsageSetting, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getUsageSetting(_arg_serial, _aidl_reply))
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
            impl IRadioNetwork for binder::binder_impl::Binder<BnRadioNetwork> {
              fn r#getAllowedNetworkTypesBitmap(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getAllowedNetworkTypesBitmap(_arg_serial) }
              fn r#getAvailableBandModes(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getAvailableBandModes(_arg_serial) }
              fn r#getAvailableNetworks(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getAvailableNetworks(_arg_serial) }
              fn r#getBarringInfo(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getBarringInfo(_arg_serial) }
              fn r#getCdmaRoamingPreference(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getCdmaRoamingPreference(_arg_serial) }
              fn r#getCellInfoList(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getCellInfoList(_arg_serial) }
              fn r#getDataRegistrationState(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getDataRegistrationState(_arg_serial) }
              fn r#getImsRegistrationState(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getImsRegistrationState(_arg_serial) }
              fn r#getNetworkSelectionMode(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getNetworkSelectionMode(_arg_serial) }
              fn r#getOperator(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getOperator(_arg_serial) }
              fn r#getSignalStrength(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getSignalStrength(_arg_serial) }
              fn r#getSystemSelectionChannels(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getSystemSelectionChannels(_arg_serial) }
              fn r#getVoiceRadioTechnology(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getVoiceRadioTechnology(_arg_serial) }
              fn r#getVoiceRegistrationState(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getVoiceRegistrationState(_arg_serial) }
              fn r#isNrDualConnectivityEnabled(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#isNrDualConnectivityEnabled(_arg_serial) }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> { self.0.r#responseAcknowledgement() }
              fn r#setAllowedNetworkTypesBitmap(&self, _arg_serial: i32, _arg_networkTypeBitmap: i32) -> binder::Result<()> { self.0.r#setAllowedNetworkTypesBitmap(_arg_serial, _arg_networkTypeBitmap) }
              fn r#setBandMode(&self, _arg_serial: i32, _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode) -> binder::Result<()> { self.0.r#setBandMode(_arg_serial, _arg_mode) }
              fn r#setBarringPassword(&self, _arg_serial: i32, _arg_facility: &str, _arg_oldPassword: &str, _arg_newPassword: &str) -> binder::Result<()> { self.0.r#setBarringPassword(_arg_serial, _arg_facility, _arg_oldPassword, _arg_newPassword) }
              fn r#setCdmaRoamingPreference(&self, _arg_serial: i32, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<()> { self.0.r#setCdmaRoamingPreference(_arg_serial, _arg_type) }
              fn r#setCellInfoListRate(&self, _arg_serial: i32, _arg_rate: i32) -> binder::Result<()> { self.0.r#setCellInfoListRate(_arg_serial, _arg_rate) }
              fn r#setIndicationFilter(&self, _arg_serial: i32, _arg_indicationFilter: i32) -> binder::Result<()> { self.0.r#setIndicationFilter(_arg_serial, _arg_indicationFilter) }
              fn r#setLinkCapacityReportingCriteria(&self, _arg_serial: i32, _arg_hysteresisMs: i32, _arg_hysteresisDlKbps: i32, _arg_hysteresisUlKbps: i32, _arg_thresholdsDownlinkKbps: &[i32], _arg_thresholdsUplinkKbps: &[i32], _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<()> { self.0.r#setLinkCapacityReportingCriteria(_arg_serial, _arg_hysteresisMs, _arg_hysteresisDlKbps, _arg_hysteresisUlKbps, _arg_thresholdsDownlinkKbps, _arg_thresholdsUplinkKbps, _arg_accessNetwork) }
              fn r#setLocationUpdates(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> { self.0.r#setLocationUpdates(_arg_serial, _arg_enable) }
              fn r#setNetworkSelectionModeAutomatic(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#setNetworkSelectionModeAutomatic(_arg_serial) }
              fn r#setNetworkSelectionModeManual(&self, _arg_serial: i32, _arg_operatorNumeric: &str, _arg_ran: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork) -> binder::Result<()> { self.0.r#setNetworkSelectionModeManual(_arg_serial, _arg_operatorNumeric, _arg_ran) }
              fn r#setNrDualConnectivityState(&self, _arg_serial: i32, _arg_nrDualConnectivityState: crate::mangled::_7_android_8_hardware_5_radio_7_network_23_NrDualConnectivityState) -> binder::Result<()> { self.0.r#setNrDualConnectivityState(_arg_serial, _arg_nrDualConnectivityState) }
              fn r#setResponseFunctions(&self, _arg_radioNetworkResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_21_IRadioNetworkResponse>, _arg_radioNetworkIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_23_IRadioNetworkIndication>) -> binder::Result<()> { self.0.r#setResponseFunctions(_arg_radioNetworkResponse, _arg_radioNetworkIndication) }
              fn r#setSignalStrengthReportingCriteria(&self, _arg_serial: i32, _arg_signalThresholdInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SignalThresholdInfo]) -> binder::Result<()> { self.0.r#setSignalStrengthReportingCriteria(_arg_serial, _arg_signalThresholdInfos) }
              fn r#setSuppServiceNotifications(&self, _arg_serial: i32, _arg_enable: bool) -> binder::Result<()> { self.0.r#setSuppServiceNotifications(_arg_serial, _arg_enable) }
              fn r#setSystemSelectionChannels(&self, _arg_serial: i32, _arg_specifyChannels: bool, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<()> { self.0.r#setSystemSelectionChannels(_arg_serial, _arg_specifyChannels, _arg_specifiers) }
              fn r#startNetworkScan(&self, _arg_serial: i32, _arg_request: &crate::mangled::_7_android_8_hardware_5_radio_7_network_18_NetworkScanRequest) -> binder::Result<()> { self.0.r#startNetworkScan(_arg_serial, _arg_request) }
              fn r#stopNetworkScan(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#stopNetworkScan(_arg_serial) }
              fn r#supplyNetworkDepersonalization(&self, _arg_serial: i32, _arg_netPin: &str) -> binder::Result<()> { self.0.r#supplyNetworkDepersonalization(_arg_serial, _arg_netPin) }
              fn r#setUsageSetting(&self, _arg_serial: i32, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<()> { self.0.r#setUsageSetting(_arg_serial, _arg_usageSetting) }
              fn r#getUsageSetting(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getUsageSetting(_arg_serial) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioNetwork, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#getAllowedNetworkTypesBitmap => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getAllowedNetworkTypesBitmap(_arg_serial);
                  Ok(())
                }
                transactions::r#getAvailableBandModes => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getAvailableBandModes(_arg_serial);
                  Ok(())
                }
                transactions::r#getAvailableNetworks => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getAvailableNetworks(_arg_serial);
                  Ok(())
                }
                transactions::r#getBarringInfo => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getBarringInfo(_arg_serial);
                  Ok(())
                }
                transactions::r#getCdmaRoamingPreference => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCdmaRoamingPreference(_arg_serial);
                  Ok(())
                }
                transactions::r#getCellInfoList => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCellInfoList(_arg_serial);
                  Ok(())
                }
                transactions::r#getDataRegistrationState => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDataRegistrationState(_arg_serial);
                  Ok(())
                }
                transactions::r#getImsRegistrationState => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getImsRegistrationState(_arg_serial);
                  Ok(())
                }
                transactions::r#getNetworkSelectionMode => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getNetworkSelectionMode(_arg_serial);
                  Ok(())
                }
                transactions::r#getOperator => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getOperator(_arg_serial);
                  Ok(())
                }
                transactions::r#getSignalStrength => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSignalStrength(_arg_serial);
                  Ok(())
                }
                transactions::r#getSystemSelectionChannels => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSystemSelectionChannels(_arg_serial);
                  Ok(())
                }
                transactions::r#getVoiceRadioTechnology => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getVoiceRadioTechnology(_arg_serial);
                  Ok(())
                }
                transactions::r#getVoiceRegistrationState => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getVoiceRegistrationState(_arg_serial);
                  Ok(())
                }
                transactions::r#isNrDualConnectivityEnabled => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#isNrDualConnectivityEnabled(_arg_serial);
                  Ok(())
                }
                transactions::r#responseAcknowledgement => {
                  let _aidl_return = _aidl_service.r#responseAcknowledgement();
                  Ok(())
                }
                transactions::r#setAllowedNetworkTypesBitmap => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_networkTypeBitmap: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setAllowedNetworkTypesBitmap(_arg_serial, _arg_networkTypeBitmap);
                  Ok(())
                }
                transactions::r#setBandMode => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_mode: crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setBandMode(_arg_serial, _arg_mode);
                  Ok(())
                }
                transactions::r#setBarringPassword => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_facility: String = _aidl_data.read()?;
                  let _arg_oldPassword: String = _aidl_data.read()?;
                  let _arg_newPassword: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setBarringPassword(_arg_serial, &_arg_facility, &_arg_oldPassword, &_arg_newPassword);
                  Ok(())
                }
                transactions::r#setCdmaRoamingPreference => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCdmaRoamingPreference(_arg_serial, _arg_type);
                  Ok(())
                }
                transactions::r#setCellInfoListRate => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_rate: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCellInfoListRate(_arg_serial, _arg_rate);
                  Ok(())
                }
                transactions::r#setIndicationFilter => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_indicationFilter: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setIndicationFilter(_arg_serial, _arg_indicationFilter);
                  Ok(())
                }
                transactions::r#setLinkCapacityReportingCriteria => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_hysteresisMs: i32 = _aidl_data.read()?;
                  let _arg_hysteresisDlKbps: i32 = _aidl_data.read()?;
                  let _arg_hysteresisUlKbps: i32 = _aidl_data.read()?;
                  let _arg_thresholdsDownlinkKbps: Vec<i32> = _aidl_data.read()?;
                  let _arg_thresholdsUplinkKbps: Vec<i32> = _aidl_data.read()?;
                  let _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setLinkCapacityReportingCriteria(_arg_serial, _arg_hysteresisMs, _arg_hysteresisDlKbps, _arg_hysteresisUlKbps, &_arg_thresholdsDownlinkKbps, &_arg_thresholdsUplinkKbps, _arg_accessNetwork);
                  Ok(())
                }
                transactions::r#setLocationUpdates => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_enable: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setLocationUpdates(_arg_serial, _arg_enable);
                  Ok(())
                }
                transactions::r#setNetworkSelectionModeAutomatic => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setNetworkSelectionModeAutomatic(_arg_serial);
                  Ok(())
                }
                transactions::r#setNetworkSelectionModeManual => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_operatorNumeric: String = _aidl_data.read()?;
                  let _arg_ran: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setNetworkSelectionModeManual(_arg_serial, &_arg_operatorNumeric, _arg_ran);
                  Ok(())
                }
                transactions::r#setNrDualConnectivityState => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_nrDualConnectivityState: crate::mangled::_7_android_8_hardware_5_radio_7_network_23_NrDualConnectivityState = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setNrDualConnectivityState(_arg_serial, _arg_nrDualConnectivityState);
                  Ok(())
                }
                transactions::r#setResponseFunctions => {
                  let _arg_radioNetworkResponse: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_21_IRadioNetworkResponse> = _aidl_data.read()?;
                  let _arg_radioNetworkIndication: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_7_network_23_IRadioNetworkIndication> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setResponseFunctions(&_arg_radioNetworkResponse, &_arg_radioNetworkIndication);
                  Ok(())
                }
                transactions::r#setSignalStrengthReportingCriteria => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_signalThresholdInfos: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SignalThresholdInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setSignalStrengthReportingCriteria(_arg_serial, &_arg_signalThresholdInfos);
                  Ok(())
                }
                transactions::r#setSuppServiceNotifications => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_enable: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setSuppServiceNotifications(_arg_serial, _arg_enable);
                  Ok(())
                }
                transactions::r#setSystemSelectionChannels => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_specifyChannels: bool = _aidl_data.read()?;
                  let _arg_specifiers: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setSystemSelectionChannels(_arg_serial, _arg_specifyChannels, &_arg_specifiers);
                  Ok(())
                }
                transactions::r#startNetworkScan => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_request: crate::mangled::_7_android_8_hardware_5_radio_7_network_18_NetworkScanRequest = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#startNetworkScan(_arg_serial, &_arg_request);
                  Ok(())
                }
                transactions::r#stopNetworkScan => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#stopNetworkScan(_arg_serial);
                  Ok(())
                }
                transactions::r#supplyNetworkDepersonalization => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_netPin: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#supplyNetworkDepersonalization(_arg_serial, &_arg_netPin);
                  Ok(())
                }
                transactions::r#setUsageSetting => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setUsageSetting(_arg_serial, _arg_usageSetting);
                  Ok(())
                }
                transactions::r#getUsageSetting => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getUsageSetting(_arg_serial);
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
             pub use super::r#IRadioNetwork as _7_android_8_hardware_5_radio_7_network_13_IRadioNetwork;
            }
          }
                    pub mod IRadioNetworkIndication {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioNetworkIndication["android.hardware.radio.network.IRadioNetworkIndication"] {
                native: BnRadioNetworkIndication(on_transact),
                proxy: BpRadioNetworkIndication {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioNetworkIndicationAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioNetworkIndication: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.network.IRadioNetworkIndication" }
              fn r#barringInfoChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<()>;
              fn r#cdmaPrlChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_version: i32) -> binder::Result<()>;
              fn r#cellInfoList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<()>;
              fn r#currentLinkCapacityEstimate(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_lce: &crate::mangled::_7_android_8_hardware_5_radio_7_network_20_LinkCapacityEstimate) -> binder::Result<()>;
              fn r#currentPhysicalChannelConfigs(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_21_PhysicalChannelConfig]) -> binder::Result<()>;
              fn r#currentSignalStrength(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<()>;
              fn r#imsNetworkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#networkScanResult(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_7_network_17_NetworkScanResult) -> binder::Result<()>;
              fn r#networkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              fn r#nitzTimeReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_nitzTime: &str, _arg_receivedTimeMs: i64, _arg_ageMs: i64) -> binder::Result<()>;
              fn r#registrationFailed(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_chosenPlmn: &str, _arg_domain: i32, _arg_causeCode: i32, _arg_additionalCauseCode: i32) -> binder::Result<()>;
              fn r#restrictedStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_PhoneRestrictedState) -> binder::Result<()>;
              fn r#suppSvcNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_suppSvc: &crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SuppSvcNotification) -> binder::Result<()>;
              fn r#voiceRadioTechChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioNetworkIndicationDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioNetworkIndicationDefaultRef) -> IRadioNetworkIndicationDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioNetworkIndicationAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.network.IRadioNetworkIndication" }
              fn r#barringInfoChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#cdmaPrlChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_version: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#cellInfoList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#currentLinkCapacityEstimate(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_lce: &crate::mangled::_7_android_8_hardware_5_radio_7_network_20_LinkCapacityEstimate) -> std::future::Ready<binder::Result<()>>;
              fn r#currentPhysicalChannelConfigs(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_21_PhysicalChannelConfig]) -> std::future::Ready<binder::Result<()>>;
              fn r#currentSignalStrength(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> std::future::Ready<binder::Result<()>>;
              fn r#imsNetworkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#networkScanResult(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_7_network_17_NetworkScanResult) -> std::future::Ready<binder::Result<()>>;
              fn r#networkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>>;
              fn r#nitzTimeReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_nitzTime: &str, _arg_receivedTimeMs: i64, _arg_ageMs: i64) -> std::future::Ready<binder::Result<()>>;
              fn r#registrationFailed(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_chosenPlmn: &str, _arg_domain: i32, _arg_causeCode: i32, _arg_additionalCauseCode: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#restrictedStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_PhoneRestrictedState) -> std::future::Ready<binder::Result<()>>;
              fn r#suppSvcNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_suppSvc: &crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SuppSvcNotification) -> std::future::Ready<binder::Result<()>>;
              fn r#voiceRadioTechChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioNetworkIndicationAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.network.IRadioNetworkIndication" }
              async fn r#barringInfoChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<()>;
              async fn r#cdmaPrlChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_version: i32) -> binder::Result<()>;
              async fn r#cellInfoList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<()>;
              async fn r#currentLinkCapacityEstimate(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_lce: &crate::mangled::_7_android_8_hardware_5_radio_7_network_20_LinkCapacityEstimate) -> binder::Result<()>;
              async fn r#currentPhysicalChannelConfigs(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_21_PhysicalChannelConfig]) -> binder::Result<()>;
              async fn r#currentSignalStrength(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<()>;
              async fn r#imsNetworkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              async fn r#networkScanResult(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_7_network_17_NetworkScanResult) -> binder::Result<()>;
              async fn r#networkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()>;
              async fn r#nitzTimeReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_nitzTime: &str, _arg_receivedTimeMs: i64, _arg_ageMs: i64) -> binder::Result<()>;
              async fn r#registrationFailed(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_chosenPlmn: &str, _arg_domain: i32, _arg_causeCode: i32, _arg_additionalCauseCode: i32) -> binder::Result<()>;
              async fn r#restrictedStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_PhoneRestrictedState) -> binder::Result<()>;
              async fn r#suppSvcNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_suppSvc: &crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SuppSvcNotification) -> binder::Result<()>;
              async fn r#voiceRadioTechChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<()>;
            }
            impl BnRadioNetworkIndication {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioNetworkIndication>
              where
                T: IRadioNetworkIndicationAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioNetworkIndication for Wrapper<T, R>
                where
                  T: IRadioNetworkIndicationAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#barringInfoChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#barringInfoChanged(_arg_type, _arg_cellIdentity, _arg_barringInfos))
                  }
                  fn r#cdmaPrlChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_version: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#cdmaPrlChanged(_arg_type, _arg_version))
                  }
                  fn r#cellInfoList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#cellInfoList(_arg_type, _arg_records))
                  }
                  fn r#currentLinkCapacityEstimate(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_lce: &crate::mangled::_7_android_8_hardware_5_radio_7_network_20_LinkCapacityEstimate) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#currentLinkCapacityEstimate(_arg_type, _arg_lce))
                  }
                  fn r#currentPhysicalChannelConfigs(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_21_PhysicalChannelConfig]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#currentPhysicalChannelConfigs(_arg_type, _arg_configs))
                  }
                  fn r#currentSignalStrength(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#currentSignalStrength(_arg_type, _arg_signalStrength))
                  }
                  fn r#imsNetworkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#imsNetworkStateChanged(_arg_type))
                  }
                  fn r#networkScanResult(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_7_network_17_NetworkScanResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#networkScanResult(_arg_type, _arg_result))
                  }
                  fn r#networkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#networkStateChanged(_arg_type))
                  }
                  fn r#nitzTimeReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_nitzTime: &str, _arg_receivedTimeMs: i64, _arg_ageMs: i64) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#nitzTimeReceived(_arg_type, _arg_nitzTime, _arg_receivedTimeMs, _arg_ageMs))
                  }
                  fn r#registrationFailed(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_chosenPlmn: &str, _arg_domain: i32, _arg_causeCode: i32, _arg_additionalCauseCode: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#registrationFailed(_arg_type, _arg_cellIdentity, _arg_chosenPlmn, _arg_domain, _arg_causeCode, _arg_additionalCauseCode))
                  }
                  fn r#restrictedStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_PhoneRestrictedState) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#restrictedStateChanged(_arg_type, _arg_state))
                  }
                  fn r#suppSvcNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_suppSvc: &crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SuppSvcNotification) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#suppSvcNotify(_arg_type, _arg_suppSvc))
                  }
                  fn r#voiceRadioTechChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#voiceRadioTechChanged(_arg_type, _arg_rat))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioNetworkIndicationDefault: Send + Sync {
              fn r#barringInfoChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#cdmaPrlChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_version: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#cellInfoList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#currentLinkCapacityEstimate(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_lce: &crate::mangled::_7_android_8_hardware_5_radio_7_network_20_LinkCapacityEstimate) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#currentPhysicalChannelConfigs(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_21_PhysicalChannelConfig]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#currentSignalStrength(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#imsNetworkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#networkScanResult(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_7_network_17_NetworkScanResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#networkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#nitzTimeReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_nitzTime: &str, _arg_receivedTimeMs: i64, _arg_ageMs: i64) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#registrationFailed(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_chosenPlmn: &str, _arg_domain: i32, _arg_causeCode: i32, _arg_additionalCauseCode: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#restrictedStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_PhoneRestrictedState) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#suppSvcNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_suppSvc: &crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SuppSvcNotification) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#voiceRadioTechChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#barringInfoChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#cdmaPrlChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#cellInfoList: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#currentLinkCapacityEstimate: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#currentPhysicalChannelConfigs: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#currentSignalStrength: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#imsNetworkStateChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#networkScanResult: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#networkStateChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#nitzTimeReceived: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#registrationFailed: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#restrictedStateChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#suppSvcNotify: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#voiceRadioTechChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioNetworkIndicationDefaultRef = Option<std::sync::Arc<dyn IRadioNetworkIndicationDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioNetworkIndicationDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "57e8e923513d80a26102e450d335e89b4346be66";
            impl BpRadioNetworkIndication {
              fn build_parcel_barringInfoChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_cellIdentity)?;
                aidl_data.write(_arg_barringInfos)?;
                Ok(aidl_data)
              }
              fn read_response_barringInfoChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#barringInfoChanged(_arg_type, _arg_cellIdentity, _arg_barringInfos);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_cdmaPrlChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_version: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_version)?;
                Ok(aidl_data)
              }
              fn read_response_cdmaPrlChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_version: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#cdmaPrlChanged(_arg_type, _arg_version);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_cellInfoList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_records)?;
                Ok(aidl_data)
              }
              fn read_response_cellInfoList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#cellInfoList(_arg_type, _arg_records);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_currentLinkCapacityEstimate(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_lce: &crate::mangled::_7_android_8_hardware_5_radio_7_network_20_LinkCapacityEstimate) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_lce)?;
                Ok(aidl_data)
              }
              fn read_response_currentLinkCapacityEstimate(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_lce: &crate::mangled::_7_android_8_hardware_5_radio_7_network_20_LinkCapacityEstimate, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#currentLinkCapacityEstimate(_arg_type, _arg_lce);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_currentPhysicalChannelConfigs(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_21_PhysicalChannelConfig]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_configs)?;
                Ok(aidl_data)
              }
              fn read_response_currentPhysicalChannelConfigs(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_21_PhysicalChannelConfig], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#currentPhysicalChannelConfigs(_arg_type, _arg_configs);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_currentSignalStrength(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_signalStrength)?;
                Ok(aidl_data)
              }
              fn read_response_currentSignalStrength(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#currentSignalStrength(_arg_type, _arg_signalStrength);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_imsNetworkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_imsNetworkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#imsNetworkStateChanged(_arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_networkScanResult(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_7_network_17_NetworkScanResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_result)?;
                Ok(aidl_data)
              }
              fn read_response_networkScanResult(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_7_network_17_NetworkScanResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#networkScanResult(_arg_type, _arg_result);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_networkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_networkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#networkStateChanged(_arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_nitzTimeReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_nitzTime: &str, _arg_receivedTimeMs: i64, _arg_ageMs: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_nitzTime)?;
                aidl_data.write(&_arg_receivedTimeMs)?;
                aidl_data.write(&_arg_ageMs)?;
                Ok(aidl_data)
              }
              fn read_response_nitzTimeReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_nitzTime: &str, _arg_receivedTimeMs: i64, _arg_ageMs: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#nitzTimeReceived(_arg_type, _arg_nitzTime, _arg_receivedTimeMs, _arg_ageMs);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_registrationFailed(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_chosenPlmn: &str, _arg_domain: i32, _arg_causeCode: i32, _arg_additionalCauseCode: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_cellIdentity)?;
                aidl_data.write(_arg_chosenPlmn)?;
                aidl_data.write(&_arg_domain)?;
                aidl_data.write(&_arg_causeCode)?;
                aidl_data.write(&_arg_additionalCauseCode)?;
                Ok(aidl_data)
              }
              fn read_response_registrationFailed(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_chosenPlmn: &str, _arg_domain: i32, _arg_causeCode: i32, _arg_additionalCauseCode: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#registrationFailed(_arg_type, _arg_cellIdentity, _arg_chosenPlmn, _arg_domain, _arg_causeCode, _arg_additionalCauseCode);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_restrictedStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_PhoneRestrictedState) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_state)?;
                Ok(aidl_data)
              }
              fn read_response_restrictedStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_PhoneRestrictedState, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#restrictedStateChanged(_arg_type, _arg_state);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_suppSvcNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_suppSvc: &crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SuppSvcNotification) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_suppSvc)?;
                Ok(aidl_data)
              }
              fn read_response_suppSvcNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_suppSvc: &crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SuppSvcNotification, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#suppSvcNotify(_arg_type, _arg_suppSvc);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_voiceRadioTechChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(&_arg_rat)?;
                Ok(aidl_data)
              }
              fn read_response_voiceRadioTechChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#voiceRadioTechChanged(_arg_type, _arg_rat);
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
            impl IRadioNetworkIndication for BpRadioNetworkIndication {
              fn r#barringInfoChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_barringInfoChanged(_arg_type, _arg_cellIdentity, _arg_barringInfos)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#barringInfoChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_barringInfoChanged(_arg_type, _arg_cellIdentity, _arg_barringInfos, _aidl_reply)
              }
              fn r#cdmaPrlChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_version: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_cdmaPrlChanged(_arg_type, _arg_version)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaPrlChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_cdmaPrlChanged(_arg_type, _arg_version, _aidl_reply)
              }
              fn r#cellInfoList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_cellInfoList(_arg_type, _arg_records)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#cellInfoList, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_cellInfoList(_arg_type, _arg_records, _aidl_reply)
              }
              fn r#currentLinkCapacityEstimate(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_lce: &crate::mangled::_7_android_8_hardware_5_radio_7_network_20_LinkCapacityEstimate) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_currentLinkCapacityEstimate(_arg_type, _arg_lce)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#currentLinkCapacityEstimate, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_currentLinkCapacityEstimate(_arg_type, _arg_lce, _aidl_reply)
              }
              fn r#currentPhysicalChannelConfigs(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_21_PhysicalChannelConfig]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_currentPhysicalChannelConfigs(_arg_type, _arg_configs)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#currentPhysicalChannelConfigs, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_currentPhysicalChannelConfigs(_arg_type, _arg_configs, _aidl_reply)
              }
              fn r#currentSignalStrength(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_currentSignalStrength(_arg_type, _arg_signalStrength)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#currentSignalStrength, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_currentSignalStrength(_arg_type, _arg_signalStrength, _aidl_reply)
              }
              fn r#imsNetworkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_imsNetworkStateChanged(_arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#imsNetworkStateChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_imsNetworkStateChanged(_arg_type, _aidl_reply)
              }
              fn r#networkScanResult(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_7_network_17_NetworkScanResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_networkScanResult(_arg_type, _arg_result)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#networkScanResult, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_networkScanResult(_arg_type, _arg_result, _aidl_reply)
              }
              fn r#networkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_networkStateChanged(_arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#networkStateChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_networkStateChanged(_arg_type, _aidl_reply)
              }
              fn r#nitzTimeReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_nitzTime: &str, _arg_receivedTimeMs: i64, _arg_ageMs: i64) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_nitzTimeReceived(_arg_type, _arg_nitzTime, _arg_receivedTimeMs, _arg_ageMs)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#nitzTimeReceived, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_nitzTimeReceived(_arg_type, _arg_nitzTime, _arg_receivedTimeMs, _arg_ageMs, _aidl_reply)
              }
              fn r#registrationFailed(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_chosenPlmn: &str, _arg_domain: i32, _arg_causeCode: i32, _arg_additionalCauseCode: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_registrationFailed(_arg_type, _arg_cellIdentity, _arg_chosenPlmn, _arg_domain, _arg_causeCode, _arg_additionalCauseCode)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#registrationFailed, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_registrationFailed(_arg_type, _arg_cellIdentity, _arg_chosenPlmn, _arg_domain, _arg_causeCode, _arg_additionalCauseCode, _aidl_reply)
              }
              fn r#restrictedStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_PhoneRestrictedState) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_restrictedStateChanged(_arg_type, _arg_state)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#restrictedStateChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_restrictedStateChanged(_arg_type, _arg_state, _aidl_reply)
              }
              fn r#suppSvcNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_suppSvc: &crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SuppSvcNotification) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_suppSvcNotify(_arg_type, _arg_suppSvc)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#suppSvcNotify, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_suppSvcNotify(_arg_type, _arg_suppSvc, _aidl_reply)
              }
              fn r#voiceRadioTechChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_voiceRadioTechChanged(_arg_type, _arg_rat)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#voiceRadioTechChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_voiceRadioTechChanged(_arg_type, _arg_rat, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioNetworkIndicationAsync<P> for BpRadioNetworkIndication {
              fn r#barringInfoChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_barringInfoChanged(_arg_type, _arg_cellIdentity, _arg_barringInfos) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#barringInfoChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_barringInfoChanged(_arg_type, _arg_cellIdentity, _arg_barringInfos, _aidl_reply))
              }
              fn r#cdmaPrlChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_version: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_cdmaPrlChanged(_arg_type, _arg_version) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#cdmaPrlChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_cdmaPrlChanged(_arg_type, _arg_version, _aidl_reply))
              }
              fn r#cellInfoList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_cellInfoList(_arg_type, _arg_records) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#cellInfoList, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_cellInfoList(_arg_type, _arg_records, _aidl_reply))
              }
              fn r#currentLinkCapacityEstimate(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_lce: &crate::mangled::_7_android_8_hardware_5_radio_7_network_20_LinkCapacityEstimate) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_currentLinkCapacityEstimate(_arg_type, _arg_lce) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#currentLinkCapacityEstimate, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_currentLinkCapacityEstimate(_arg_type, _arg_lce, _aidl_reply))
              }
              fn r#currentPhysicalChannelConfigs(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_21_PhysicalChannelConfig]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_currentPhysicalChannelConfigs(_arg_type, _arg_configs) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#currentPhysicalChannelConfigs, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_currentPhysicalChannelConfigs(_arg_type, _arg_configs, _aidl_reply))
              }
              fn r#currentSignalStrength(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_currentSignalStrength(_arg_type, _arg_signalStrength) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#currentSignalStrength, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_currentSignalStrength(_arg_type, _arg_signalStrength, _aidl_reply))
              }
              fn r#imsNetworkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_imsNetworkStateChanged(_arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#imsNetworkStateChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_imsNetworkStateChanged(_arg_type, _aidl_reply))
              }
              fn r#networkScanResult(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_7_network_17_NetworkScanResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_networkScanResult(_arg_type, _arg_result) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#networkScanResult, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_networkScanResult(_arg_type, _arg_result, _aidl_reply))
              }
              fn r#networkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_networkStateChanged(_arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#networkStateChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_networkStateChanged(_arg_type, _aidl_reply))
              }
              fn r#nitzTimeReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_nitzTime: &str, _arg_receivedTimeMs: i64, _arg_ageMs: i64) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_nitzTimeReceived(_arg_type, _arg_nitzTime, _arg_receivedTimeMs, _arg_ageMs) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#nitzTimeReceived, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_nitzTimeReceived(_arg_type, _arg_nitzTime, _arg_receivedTimeMs, _arg_ageMs, _aidl_reply))
              }
              fn r#registrationFailed(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_chosenPlmn: &str, _arg_domain: i32, _arg_causeCode: i32, _arg_additionalCauseCode: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_registrationFailed(_arg_type, _arg_cellIdentity, _arg_chosenPlmn, _arg_domain, _arg_causeCode, _arg_additionalCauseCode) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#registrationFailed, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_registrationFailed(_arg_type, _arg_cellIdentity, _arg_chosenPlmn, _arg_domain, _arg_causeCode, _arg_additionalCauseCode, _aidl_reply))
              }
              fn r#restrictedStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_PhoneRestrictedState) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_restrictedStateChanged(_arg_type, _arg_state) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#restrictedStateChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_restrictedStateChanged(_arg_type, _arg_state, _aidl_reply))
              }
              fn r#suppSvcNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_suppSvc: &crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SuppSvcNotification) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_suppSvcNotify(_arg_type, _arg_suppSvc) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#suppSvcNotify, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_suppSvcNotify(_arg_type, _arg_suppSvc, _aidl_reply))
              }
              fn r#voiceRadioTechChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_voiceRadioTechChanged(_arg_type, _arg_rat) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#voiceRadioTechChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_voiceRadioTechChanged(_arg_type, _arg_rat, _aidl_reply))
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
            impl IRadioNetworkIndication for binder::binder_impl::Binder<BnRadioNetworkIndication> {
              fn r#barringInfoChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<()> { self.0.r#barringInfoChanged(_arg_type, _arg_cellIdentity, _arg_barringInfos) }
              fn r#cdmaPrlChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_version: i32) -> binder::Result<()> { self.0.r#cdmaPrlChanged(_arg_type, _arg_version) }
              fn r#cellInfoList(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_records: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<()> { self.0.r#cellInfoList(_arg_type, _arg_records) }
              fn r#currentLinkCapacityEstimate(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_lce: &crate::mangled::_7_android_8_hardware_5_radio_7_network_20_LinkCapacityEstimate) -> binder::Result<()> { self.0.r#currentLinkCapacityEstimate(_arg_type, _arg_lce) }
              fn r#currentPhysicalChannelConfigs(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_configs: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_21_PhysicalChannelConfig]) -> binder::Result<()> { self.0.r#currentPhysicalChannelConfigs(_arg_type, _arg_configs) }
              fn r#currentSignalStrength(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<()> { self.0.r#currentSignalStrength(_arg_type, _arg_signalStrength) }
              fn r#imsNetworkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#imsNetworkStateChanged(_arg_type) }
              fn r#networkScanResult(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_result: &crate::mangled::_7_android_8_hardware_5_radio_7_network_17_NetworkScanResult) -> binder::Result<()> { self.0.r#networkScanResult(_arg_type, _arg_result) }
              fn r#networkStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType) -> binder::Result<()> { self.0.r#networkStateChanged(_arg_type) }
              fn r#nitzTimeReceived(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_nitzTime: &str, _arg_receivedTimeMs: i64, _arg_ageMs: i64) -> binder::Result<()> { self.0.r#nitzTimeReceived(_arg_type, _arg_nitzTime, _arg_receivedTimeMs, _arg_ageMs) }
              fn r#registrationFailed(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_chosenPlmn: &str, _arg_domain: i32, _arg_causeCode: i32, _arg_additionalCauseCode: i32) -> binder::Result<()> { self.0.r#registrationFailed(_arg_type, _arg_cellIdentity, _arg_chosenPlmn, _arg_domain, _arg_causeCode, _arg_additionalCauseCode) }
              fn r#restrictedStateChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_state: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_PhoneRestrictedState) -> binder::Result<()> { self.0.r#restrictedStateChanged(_arg_type, _arg_state) }
              fn r#suppSvcNotify(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_suppSvc: &crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SuppSvcNotification) -> binder::Result<()> { self.0.r#suppSvcNotify(_arg_type, _arg_suppSvc) }
              fn r#voiceRadioTechChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<()> { self.0.r#voiceRadioTechChanged(_arg_type, _arg_rat) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioNetworkIndication, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#barringInfoChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_cellIdentity: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity = _aidl_data.read()?;
                  let _arg_barringInfos: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#barringInfoChanged(_arg_type, &_arg_cellIdentity, &_arg_barringInfos);
                  Ok(())
                }
                transactions::r#cdmaPrlChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_version: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#cdmaPrlChanged(_arg_type, _arg_version);
                  Ok(())
                }
                transactions::r#cellInfoList => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_records: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#cellInfoList(_arg_type, &_arg_records);
                  Ok(())
                }
                transactions::r#currentLinkCapacityEstimate => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_lce: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_LinkCapacityEstimate = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#currentLinkCapacityEstimate(_arg_type, &_arg_lce);
                  Ok(())
                }
                transactions::r#currentPhysicalChannelConfigs => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_configs: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_21_PhysicalChannelConfig> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#currentPhysicalChannelConfigs(_arg_type, &_arg_configs);
                  Ok(())
                }
                transactions::r#currentSignalStrength => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_signalStrength: crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#currentSignalStrength(_arg_type, &_arg_signalStrength);
                  Ok(())
                }
                transactions::r#imsNetworkStateChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#imsNetworkStateChanged(_arg_type);
                  Ok(())
                }
                transactions::r#networkScanResult => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_result: crate::mangled::_7_android_8_hardware_5_radio_7_network_17_NetworkScanResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#networkScanResult(_arg_type, &_arg_result);
                  Ok(())
                }
                transactions::r#networkStateChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#networkStateChanged(_arg_type);
                  Ok(())
                }
                transactions::r#nitzTimeReceived => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_nitzTime: String = _aidl_data.read()?;
                  let _arg_receivedTimeMs: i64 = _aidl_data.read()?;
                  let _arg_ageMs: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#nitzTimeReceived(_arg_type, &_arg_nitzTime, _arg_receivedTimeMs, _arg_ageMs);
                  Ok(())
                }
                transactions::r#registrationFailed => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_cellIdentity: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity = _aidl_data.read()?;
                  let _arg_chosenPlmn: String = _aidl_data.read()?;
                  let _arg_domain: i32 = _aidl_data.read()?;
                  let _arg_causeCode: i32 = _aidl_data.read()?;
                  let _arg_additionalCauseCode: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#registrationFailed(_arg_type, &_arg_cellIdentity, &_arg_chosenPlmn, _arg_domain, _arg_causeCode, _arg_additionalCauseCode);
                  Ok(())
                }
                transactions::r#restrictedStateChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_state: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_PhoneRestrictedState = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#restrictedStateChanged(_arg_type, _arg_state);
                  Ok(())
                }
                transactions::r#suppSvcNotify => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_suppSvc: crate::mangled::_7_android_8_hardware_5_radio_7_network_19_SuppSvcNotification = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#suppSvcNotify(_arg_type, &_arg_suppSvc);
                  Ok(())
                }
                transactions::r#voiceRadioTechChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#voiceRadioTechChanged(_arg_type, _arg_rat);
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
             pub use super::r#IRadioNetworkIndication as _7_android_8_hardware_5_radio_7_network_23_IRadioNetworkIndication;
            }
          }
                    pub mod IRadioNetworkResponse {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioNetworkResponse["android.hardware.radio.network.IRadioNetworkResponse"] {
                native: BnRadioNetworkResponse(on_transact),
                proxy: BpRadioNetworkResponse {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioNetworkResponseAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioNetworkResponse: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.network.IRadioNetworkResponse" }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkTypeBitmap: i32) -> binder::Result<()>;
              fn r#getAvailableBandModesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_bandModes: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode]) -> binder::Result<()>;
              fn r#getAvailableNetworksResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo]) -> binder::Result<()>;
              fn r#getBarringInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<()>;
              fn r#getCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<()>;
              fn r#getCellInfoListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellInfo: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<()>;
              fn r#getDataRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dataRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<()>;
              fn r#getImsRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isRegistered: bool, _arg_ratFamily: crate::mangled::_7_android_8_hardware_5_radio_21_RadioTechnologyFamily) -> binder::Result<()>;
              fn r#getNetworkSelectionModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_manual: bool) -> binder::Result<()>;
              fn r#getOperatorResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_longName: &str, _arg_shortName: &str, _arg_numeric: &str) -> binder::Result<()>;
              fn r#getSignalStrengthResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<()>;
              fn r#getSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<()>;
              fn r#getVoiceRadioTechnologyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<()>;
              fn r#getVoiceRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_voiceRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<()>;
              fn r#isNrDualConnectivityEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<()>;
              fn r#setAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setBandModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setBarringPasswordResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setCellInfoListRateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setIndicationFilterResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setLinkCapacityReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setLocationUpdatesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setNetworkSelectionModeAutomaticResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setNetworkSelectionModeManualResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setNrDualConnectivityStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setSignalStrengthReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setSuppServiceNotificationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#startNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#stopNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#supplyNetworkDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              fn r#setUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#getUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioNetworkResponseDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioNetworkResponseDefaultRef) -> IRadioNetworkResponseDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioNetworkResponseAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.network.IRadioNetworkResponse" }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkTypeBitmap: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getAvailableBandModesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_bandModes: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode]) -> std::future::Ready<binder::Result<()>>;
              fn r#getAvailableNetworksResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#getBarringInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#getCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> std::future::Ready<binder::Result<()>>;
              fn r#getCellInfoListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellInfo: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#getDataRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dataRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> std::future::Ready<binder::Result<()>>;
              fn r#getImsRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isRegistered: bool, _arg_ratFamily: crate::mangled::_7_android_8_hardware_5_radio_21_RadioTechnologyFamily) -> std::future::Ready<binder::Result<()>>;
              fn r#getNetworkSelectionModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_manual: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#getOperatorResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_longName: &str, _arg_shortName: &str, _arg_numeric: &str) -> std::future::Ready<binder::Result<()>>;
              fn r#getSignalStrengthResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> std::future::Ready<binder::Result<()>>;
              fn r#getSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> std::future::Ready<binder::Result<()>>;
              fn r#getVoiceRadioTechnologyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> std::future::Ready<binder::Result<()>>;
              fn r#getVoiceRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_voiceRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> std::future::Ready<binder::Result<()>>;
              fn r#isNrDualConnectivityEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#setAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setBandModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setBarringPasswordResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setCellInfoListRateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setIndicationFilterResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setLinkCapacityReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setLocationUpdatesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setNetworkSelectionModeAutomaticResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setNetworkSelectionModeManualResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setNrDualConnectivityStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setSignalStrengthReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setSuppServiceNotificationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#startNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#stopNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#supplyNetworkDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#setUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioNetworkResponseAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.network.IRadioNetworkResponse" }
              async fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkTypeBitmap: i32) -> binder::Result<()>;
              async fn r#getAvailableBandModesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_bandModes: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode]) -> binder::Result<()>;
              async fn r#getAvailableNetworksResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo]) -> binder::Result<()>;
              async fn r#getBarringInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<()>;
              async fn r#getCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<()>;
              async fn r#getCellInfoListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellInfo: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<()>;
              async fn r#getDataRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dataRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<()>;
              async fn r#getImsRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isRegistered: bool, _arg_ratFamily: crate::mangled::_7_android_8_hardware_5_radio_21_RadioTechnologyFamily) -> binder::Result<()>;
              async fn r#getNetworkSelectionModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_manual: bool) -> binder::Result<()>;
              async fn r#getOperatorResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_longName: &str, _arg_shortName: &str, _arg_numeric: &str) -> binder::Result<()>;
              async fn r#getSignalStrengthResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<()>;
              async fn r#getSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<()>;
              async fn r#getVoiceRadioTechnologyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<()>;
              async fn r#getVoiceRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_voiceRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<()>;
              async fn r#isNrDualConnectivityEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<()>;
              async fn r#setAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setBandModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setBarringPasswordResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setCellInfoListRateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setIndicationFilterResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setLinkCapacityReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setLocationUpdatesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setNetworkSelectionModeAutomaticResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setNetworkSelectionModeManualResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setNrDualConnectivityStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setSignalStrengthReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setSuppServiceNotificationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#startNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#stopNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#supplyNetworkDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()>;
              async fn r#setUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#getUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<()>;
            }
            impl BnRadioNetworkResponse {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioNetworkResponse>
              where
                T: IRadioNetworkResponseAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioNetworkResponse for Wrapper<T, R>
                where
                  T: IRadioNetworkResponseAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acknowledgeRequest(_arg_serial))
                  }
                  fn r#getAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkTypeBitmap: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getAllowedNetworkTypesBitmapResponse(_arg_info, _arg_networkTypeBitmap))
                  }
                  fn r#getAvailableBandModesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_bandModes: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getAvailableBandModesResponse(_arg_info, _arg_bandModes))
                  }
                  fn r#getAvailableNetworksResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getAvailableNetworksResponse(_arg_info, _arg_networkInfos))
                  }
                  fn r#getBarringInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getBarringInfoResponse(_arg_info, _arg_cellIdentity, _arg_barringInfos))
                  }
                  fn r#getCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCdmaRoamingPreferenceResponse(_arg_info, _arg_type))
                  }
                  fn r#getCellInfoListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellInfo: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getCellInfoListResponse(_arg_info, _arg_cellInfo))
                  }
                  fn r#getDataRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dataRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getDataRegistrationStateResponse(_arg_info, _arg_dataRegResponse))
                  }
                  fn r#getImsRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isRegistered: bool, _arg_ratFamily: crate::mangled::_7_android_8_hardware_5_radio_21_RadioTechnologyFamily) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getImsRegistrationStateResponse(_arg_info, _arg_isRegistered, _arg_ratFamily))
                  }
                  fn r#getNetworkSelectionModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_manual: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getNetworkSelectionModeResponse(_arg_info, _arg_manual))
                  }
                  fn r#getOperatorResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_longName: &str, _arg_shortName: &str, _arg_numeric: &str) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getOperatorResponse(_arg_info, _arg_longName, _arg_shortName, _arg_numeric))
                  }
                  fn r#getSignalStrengthResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSignalStrengthResponse(_arg_info, _arg_signalStrength))
                  }
                  fn r#getSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSystemSelectionChannelsResponse(_arg_info, _arg_specifiers))
                  }
                  fn r#getVoiceRadioTechnologyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getVoiceRadioTechnologyResponse(_arg_info, _arg_rat))
                  }
                  fn r#getVoiceRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_voiceRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getVoiceRegistrationStateResponse(_arg_info, _arg_voiceRegResponse))
                  }
                  fn r#isNrDualConnectivityEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#isNrDualConnectivityEnabledResponse(_arg_info, _arg_isEnabled))
                  }
                  fn r#setAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setAllowedNetworkTypesBitmapResponse(_arg_info))
                  }
                  fn r#setBandModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setBandModeResponse(_arg_info))
                  }
                  fn r#setBarringPasswordResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setBarringPasswordResponse(_arg_info))
                  }
                  fn r#setCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCdmaRoamingPreferenceResponse(_arg_info))
                  }
                  fn r#setCellInfoListRateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setCellInfoListRateResponse(_arg_info))
                  }
                  fn r#setIndicationFilterResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setIndicationFilterResponse(_arg_info))
                  }
                  fn r#setLinkCapacityReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setLinkCapacityReportingCriteriaResponse(_arg_info))
                  }
                  fn r#setLocationUpdatesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setLocationUpdatesResponse(_arg_info))
                  }
                  fn r#setNetworkSelectionModeAutomaticResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setNetworkSelectionModeAutomaticResponse(_arg_info))
                  }
                  fn r#setNetworkSelectionModeManualResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setNetworkSelectionModeManualResponse(_arg_info))
                  }
                  fn r#setNrDualConnectivityStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setNrDualConnectivityStateResponse(_arg_info))
                  }
                  fn r#setSignalStrengthReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setSignalStrengthReportingCriteriaResponse(_arg_info))
                  }
                  fn r#setSuppServiceNotificationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setSuppServiceNotificationsResponse(_arg_info))
                  }
                  fn r#setSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setSystemSelectionChannelsResponse(_arg_info))
                  }
                  fn r#startNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#startNetworkScanResponse(_arg_info))
                  }
                  fn r#stopNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#stopNetworkScanResponse(_arg_info))
                  }
                  fn r#supplyNetworkDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#supplyNetworkDepersonalizationResponse(_arg_info, _arg_remainingRetries))
                  }
                  fn r#setUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setUsageSettingResponse(_arg_info))
                  }
                  fn r#getUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getUsageSettingResponse(_arg_info, _arg_usageSetting))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioNetworkResponseDefault: Send + Sync {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkTypeBitmap: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getAvailableBandModesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_bandModes: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getAvailableNetworksResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getBarringInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCellInfoListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellInfo: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDataRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dataRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getImsRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isRegistered: bool, _arg_ratFamily: crate::mangled::_7_android_8_hardware_5_radio_21_RadioTechnologyFamily) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getNetworkSelectionModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_manual: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getOperatorResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_longName: &str, _arg_shortName: &str, _arg_numeric: &str) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSignalStrengthResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getVoiceRadioTechnologyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getVoiceRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_voiceRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#isNrDualConnectivityEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setBandModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setBarringPasswordResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setCellInfoListRateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setIndicationFilterResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setLinkCapacityReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setLocationUpdatesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setNetworkSelectionModeAutomaticResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setNetworkSelectionModeManualResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setNrDualConnectivityStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setSignalStrengthReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setSuppServiceNotificationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#startNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#stopNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#supplyNetworkDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#acknowledgeRequest: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#getAllowedNetworkTypesBitmapResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#getAvailableBandModesResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#getAvailableNetworksResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#getBarringInfoResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#getCdmaRoamingPreferenceResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#getCellInfoListResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#getDataRegistrationStateResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#getImsRegistrationStateResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#getNetworkSelectionModeResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#getOperatorResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#getSignalStrengthResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#getSystemSelectionChannelsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#getVoiceRadioTechnologyResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#getVoiceRegistrationStateResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#isNrDualConnectivityEnabledResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#setAllowedNetworkTypesBitmapResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
              pub const r#setBandModeResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
              pub const r#setBarringPasswordResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
              pub const r#setCdmaRoamingPreferenceResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
              pub const r#setCellInfoListRateResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 20;
              pub const r#setIndicationFilterResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 21;
              pub const r#setLinkCapacityReportingCriteriaResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 22;
              pub const r#setLocationUpdatesResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 23;
              pub const r#setNetworkSelectionModeAutomaticResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 24;
              pub const r#setNetworkSelectionModeManualResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 25;
              pub const r#setNrDualConnectivityStateResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 26;
              pub const r#setSignalStrengthReportingCriteriaResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 27;
              pub const r#setSuppServiceNotificationsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 28;
              pub const r#setSystemSelectionChannelsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 29;
              pub const r#startNetworkScanResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 30;
              pub const r#stopNetworkScanResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 31;
              pub const r#supplyNetworkDepersonalizationResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 32;
              pub const r#setUsageSettingResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 33;
              pub const r#getUsageSettingResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 34;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioNetworkResponseDefaultRef = Option<std::sync::Arc<dyn IRadioNetworkResponseDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioNetworkResponseDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "57e8e923513d80a26102e450d335e89b4346be66";
            impl BpRadioNetworkResponse {
              fn build_parcel_acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_acknowledgeRequest(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#acknowledgeRequest(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkTypeBitmap: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_networkTypeBitmap)?;
                Ok(aidl_data)
              }
              fn read_response_getAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkTypeBitmap: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getAllowedNetworkTypesBitmapResponse(_arg_info, _arg_networkTypeBitmap);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getAvailableBandModesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_bandModes: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_bandModes)?;
                Ok(aidl_data)
              }
              fn read_response_getAvailableBandModesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_bandModes: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getAvailableBandModesResponse(_arg_info, _arg_bandModes);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getAvailableNetworksResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_networkInfos)?;
                Ok(aidl_data)
              }
              fn read_response_getAvailableNetworksResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getAvailableNetworksResponse(_arg_info, _arg_networkInfos);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getBarringInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_cellIdentity)?;
                aidl_data.write(_arg_barringInfos)?;
                Ok(aidl_data)
              }
              fn read_response_getBarringInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getBarringInfoResponse(_arg_info, _arg_cellIdentity, _arg_barringInfos);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_getCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCdmaRoamingPreferenceResponse(_arg_info, _arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getCellInfoListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellInfo: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_cellInfo)?;
                Ok(aidl_data)
              }
              fn read_response_getCellInfoListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellInfo: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCellInfoListResponse(_arg_info, _arg_cellInfo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getDataRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dataRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_dataRegResponse)?;
                Ok(aidl_data)
              }
              fn read_response_getDataRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dataRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDataRegistrationStateResponse(_arg_info, _arg_dataRegResponse);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getImsRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isRegistered: bool, _arg_ratFamily: crate::mangled::_7_android_8_hardware_5_radio_21_RadioTechnologyFamily) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_isRegistered)?;
                aidl_data.write(&_arg_ratFamily)?;
                Ok(aidl_data)
              }
              fn read_response_getImsRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isRegistered: bool, _arg_ratFamily: crate::mangled::_7_android_8_hardware_5_radio_21_RadioTechnologyFamily, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getImsRegistrationStateResponse(_arg_info, _arg_isRegistered, _arg_ratFamily);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getNetworkSelectionModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_manual: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_manual)?;
                Ok(aidl_data)
              }
              fn read_response_getNetworkSelectionModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_manual: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getNetworkSelectionModeResponse(_arg_info, _arg_manual);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getOperatorResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_longName: &str, _arg_shortName: &str, _arg_numeric: &str) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_longName)?;
                aidl_data.write(_arg_shortName)?;
                aidl_data.write(_arg_numeric)?;
                Ok(aidl_data)
              }
              fn read_response_getOperatorResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_longName: &str, _arg_shortName: &str, _arg_numeric: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getOperatorResponse(_arg_info, _arg_longName, _arg_shortName, _arg_numeric);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSignalStrengthResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_signalStrength)?;
                Ok(aidl_data)
              }
              fn read_response_getSignalStrengthResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSignalStrengthResponse(_arg_info, _arg_signalStrength);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_specifiers)?;
                Ok(aidl_data)
              }
              fn read_response_getSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSystemSelectionChannelsResponse(_arg_info, _arg_specifiers);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getVoiceRadioTechnologyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_rat)?;
                Ok(aidl_data)
              }
              fn read_response_getVoiceRadioTechnologyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getVoiceRadioTechnologyResponse(_arg_info, _arg_rat);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getVoiceRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_voiceRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_voiceRegResponse)?;
                Ok(aidl_data)
              }
              fn read_response_getVoiceRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_voiceRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getVoiceRegistrationStateResponse(_arg_info, _arg_voiceRegResponse);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_isNrDualConnectivityEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_isEnabled)?;
                Ok(aidl_data)
              }
              fn read_response_isNrDualConnectivityEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#isNrDualConnectivityEnabledResponse(_arg_info, _arg_isEnabled);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setAllowedNetworkTypesBitmapResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setBandModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setBandModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setBandModeResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setBarringPasswordResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setBarringPasswordResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setBarringPasswordResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCdmaRoamingPreferenceResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setCellInfoListRateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setCellInfoListRateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setCellInfoListRateResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setIndicationFilterResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setIndicationFilterResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setIndicationFilterResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setLinkCapacityReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setLinkCapacityReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setLinkCapacityReportingCriteriaResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setLocationUpdatesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setLocationUpdatesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setLocationUpdatesResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setNetworkSelectionModeAutomaticResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setNetworkSelectionModeAutomaticResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setNetworkSelectionModeAutomaticResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setNetworkSelectionModeManualResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setNetworkSelectionModeManualResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setNetworkSelectionModeManualResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setNrDualConnectivityStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setNrDualConnectivityStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setNrDualConnectivityStateResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setSignalStrengthReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setSignalStrengthReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setSignalStrengthReportingCriteriaResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setSuppServiceNotificationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setSuppServiceNotificationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setSuppServiceNotificationsResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setSystemSelectionChannelsResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_startNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_startNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#startNetworkScanResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_stopNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_stopNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#stopNetworkScanResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_supplyNetworkDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_remainingRetries)?;
                Ok(aidl_data)
              }
              fn read_response_supplyNetworkDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#supplyNetworkDepersonalizationResponse(_arg_info, _arg_remainingRetries);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setUsageSettingResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_usageSetting)?;
                Ok(aidl_data)
              }
              fn read_response_getUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioNetworkResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getUsageSettingResponse(_arg_info, _arg_usageSetting);
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
            impl IRadioNetworkResponse for BpRadioNetworkResponse {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acknowledgeRequest(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeRequest, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acknowledgeRequest(_arg_serial, _aidl_reply)
              }
              fn r#getAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkTypeBitmap: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getAllowedNetworkTypesBitmapResponse(_arg_info, _arg_networkTypeBitmap)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAllowedNetworkTypesBitmapResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getAllowedNetworkTypesBitmapResponse(_arg_info, _arg_networkTypeBitmap, _aidl_reply)
              }
              fn r#getAvailableBandModesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_bandModes: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getAvailableBandModesResponse(_arg_info, _arg_bandModes)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAvailableBandModesResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getAvailableBandModesResponse(_arg_info, _arg_bandModes, _aidl_reply)
              }
              fn r#getAvailableNetworksResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getAvailableNetworksResponse(_arg_info, _arg_networkInfos)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAvailableNetworksResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getAvailableNetworksResponse(_arg_info, _arg_networkInfos, _aidl_reply)
              }
              fn r#getBarringInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getBarringInfoResponse(_arg_info, _arg_cellIdentity, _arg_barringInfos)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getBarringInfoResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getBarringInfoResponse(_arg_info, _arg_cellIdentity, _arg_barringInfos, _aidl_reply)
              }
              fn r#getCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCdmaRoamingPreferenceResponse(_arg_info, _arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaRoamingPreferenceResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCdmaRoamingPreferenceResponse(_arg_info, _arg_type, _aidl_reply)
              }
              fn r#getCellInfoListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellInfo: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getCellInfoListResponse(_arg_info, _arg_cellInfo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCellInfoListResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCellInfoListResponse(_arg_info, _arg_cellInfo, _aidl_reply)
              }
              fn r#getDataRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dataRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getDataRegistrationStateResponse(_arg_info, _arg_dataRegResponse)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDataRegistrationStateResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDataRegistrationStateResponse(_arg_info, _arg_dataRegResponse, _aidl_reply)
              }
              fn r#getImsRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isRegistered: bool, _arg_ratFamily: crate::mangled::_7_android_8_hardware_5_radio_21_RadioTechnologyFamily) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getImsRegistrationStateResponse(_arg_info, _arg_isRegistered, _arg_ratFamily)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getImsRegistrationStateResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getImsRegistrationStateResponse(_arg_info, _arg_isRegistered, _arg_ratFamily, _aidl_reply)
              }
              fn r#getNetworkSelectionModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_manual: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getNetworkSelectionModeResponse(_arg_info, _arg_manual)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getNetworkSelectionModeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getNetworkSelectionModeResponse(_arg_info, _arg_manual, _aidl_reply)
              }
              fn r#getOperatorResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_longName: &str, _arg_shortName: &str, _arg_numeric: &str) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getOperatorResponse(_arg_info, _arg_longName, _arg_shortName, _arg_numeric)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getOperatorResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getOperatorResponse(_arg_info, _arg_longName, _arg_shortName, _arg_numeric, _aidl_reply)
              }
              fn r#getSignalStrengthResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSignalStrengthResponse(_arg_info, _arg_signalStrength)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSignalStrengthResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSignalStrengthResponse(_arg_info, _arg_signalStrength, _aidl_reply)
              }
              fn r#getSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSystemSelectionChannelsResponse(_arg_info, _arg_specifiers)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSystemSelectionChannelsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSystemSelectionChannelsResponse(_arg_info, _arg_specifiers, _aidl_reply)
              }
              fn r#getVoiceRadioTechnologyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getVoiceRadioTechnologyResponse(_arg_info, _arg_rat)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getVoiceRadioTechnologyResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getVoiceRadioTechnologyResponse(_arg_info, _arg_rat, _aidl_reply)
              }
              fn r#getVoiceRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_voiceRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getVoiceRegistrationStateResponse(_arg_info, _arg_voiceRegResponse)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getVoiceRegistrationStateResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getVoiceRegistrationStateResponse(_arg_info, _arg_voiceRegResponse, _aidl_reply)
              }
              fn r#isNrDualConnectivityEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_isNrDualConnectivityEnabledResponse(_arg_info, _arg_isEnabled)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#isNrDualConnectivityEnabledResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_isNrDualConnectivityEnabledResponse(_arg_info, _arg_isEnabled, _aidl_reply)
              }
              fn r#setAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setAllowedNetworkTypesBitmapResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setAllowedNetworkTypesBitmapResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setAllowedNetworkTypesBitmapResponse(_arg_info, _aidl_reply)
              }
              fn r#setBandModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setBandModeResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setBandModeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setBandModeResponse(_arg_info, _aidl_reply)
              }
              fn r#setBarringPasswordResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setBarringPasswordResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setBarringPasswordResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setBarringPasswordResponse(_arg_info, _aidl_reply)
              }
              fn r#setCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCdmaRoamingPreferenceResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaRoamingPreferenceResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCdmaRoamingPreferenceResponse(_arg_info, _aidl_reply)
              }
              fn r#setCellInfoListRateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setCellInfoListRateResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCellInfoListRateResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setCellInfoListRateResponse(_arg_info, _aidl_reply)
              }
              fn r#setIndicationFilterResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setIndicationFilterResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setIndicationFilterResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setIndicationFilterResponse(_arg_info, _aidl_reply)
              }
              fn r#setLinkCapacityReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setLinkCapacityReportingCriteriaResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setLinkCapacityReportingCriteriaResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setLinkCapacityReportingCriteriaResponse(_arg_info, _aidl_reply)
              }
              fn r#setLocationUpdatesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setLocationUpdatesResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setLocationUpdatesResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setLocationUpdatesResponse(_arg_info, _aidl_reply)
              }
              fn r#setNetworkSelectionModeAutomaticResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setNetworkSelectionModeAutomaticResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNetworkSelectionModeAutomaticResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setNetworkSelectionModeAutomaticResponse(_arg_info, _aidl_reply)
              }
              fn r#setNetworkSelectionModeManualResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setNetworkSelectionModeManualResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNetworkSelectionModeManualResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setNetworkSelectionModeManualResponse(_arg_info, _aidl_reply)
              }
              fn r#setNrDualConnectivityStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setNrDualConnectivityStateResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNrDualConnectivityStateResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setNrDualConnectivityStateResponse(_arg_info, _aidl_reply)
              }
              fn r#setSignalStrengthReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setSignalStrengthReportingCriteriaResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSignalStrengthReportingCriteriaResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setSignalStrengthReportingCriteriaResponse(_arg_info, _aidl_reply)
              }
              fn r#setSuppServiceNotificationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setSuppServiceNotificationsResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSuppServiceNotificationsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setSuppServiceNotificationsResponse(_arg_info, _aidl_reply)
              }
              fn r#setSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setSystemSelectionChannelsResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSystemSelectionChannelsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setSystemSelectionChannelsResponse(_arg_info, _aidl_reply)
              }
              fn r#startNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_startNetworkScanResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#startNetworkScanResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_startNetworkScanResponse(_arg_info, _aidl_reply)
              }
              fn r#stopNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_stopNetworkScanResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#stopNetworkScanResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_stopNetworkScanResponse(_arg_info, _aidl_reply)
              }
              fn r#supplyNetworkDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_supplyNetworkDepersonalizationResponse(_arg_info, _arg_remainingRetries)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyNetworkDepersonalizationResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_supplyNetworkDepersonalizationResponse(_arg_info, _arg_remainingRetries, _aidl_reply)
              }
              fn r#setUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setUsageSettingResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setUsageSettingResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setUsageSettingResponse(_arg_info, _aidl_reply)
              }
              fn r#getUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getUsageSettingResponse(_arg_info, _arg_usageSetting)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getUsageSettingResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getUsageSettingResponse(_arg_info, _arg_usageSetting, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioNetworkResponseAsync<P> for BpRadioNetworkResponse {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acknowledgeRequest(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeRequest, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acknowledgeRequest(_arg_serial, _aidl_reply))
              }
              fn r#getAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkTypeBitmap: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getAllowedNetworkTypesBitmapResponse(_arg_info, _arg_networkTypeBitmap) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAllowedNetworkTypesBitmapResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getAllowedNetworkTypesBitmapResponse(_arg_info, _arg_networkTypeBitmap, _aidl_reply))
              }
              fn r#getAvailableBandModesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_bandModes: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getAvailableBandModesResponse(_arg_info, _arg_bandModes) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAvailableBandModesResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getAvailableBandModesResponse(_arg_info, _arg_bandModes, _aidl_reply))
              }
              fn r#getAvailableNetworksResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getAvailableNetworksResponse(_arg_info, _arg_networkInfos) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAvailableNetworksResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getAvailableNetworksResponse(_arg_info, _arg_networkInfos, _aidl_reply))
              }
              fn r#getBarringInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getBarringInfoResponse(_arg_info, _arg_cellIdentity, _arg_barringInfos) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getBarringInfoResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getBarringInfoResponse(_arg_info, _arg_cellIdentity, _arg_barringInfos, _aidl_reply))
              }
              fn r#getCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCdmaRoamingPreferenceResponse(_arg_info, _arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCdmaRoamingPreferenceResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCdmaRoamingPreferenceResponse(_arg_info, _arg_type, _aidl_reply))
              }
              fn r#getCellInfoListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellInfo: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getCellInfoListResponse(_arg_info, _arg_cellInfo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCellInfoListResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getCellInfoListResponse(_arg_info, _arg_cellInfo, _aidl_reply))
              }
              fn r#getDataRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dataRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getDataRegistrationStateResponse(_arg_info, _arg_dataRegResponse) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDataRegistrationStateResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getDataRegistrationStateResponse(_arg_info, _arg_dataRegResponse, _aidl_reply))
              }
              fn r#getImsRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isRegistered: bool, _arg_ratFamily: crate::mangled::_7_android_8_hardware_5_radio_21_RadioTechnologyFamily) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getImsRegistrationStateResponse(_arg_info, _arg_isRegistered, _arg_ratFamily) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getImsRegistrationStateResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getImsRegistrationStateResponse(_arg_info, _arg_isRegistered, _arg_ratFamily, _aidl_reply))
              }
              fn r#getNetworkSelectionModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_manual: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getNetworkSelectionModeResponse(_arg_info, _arg_manual) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getNetworkSelectionModeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getNetworkSelectionModeResponse(_arg_info, _arg_manual, _aidl_reply))
              }
              fn r#getOperatorResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_longName: &str, _arg_shortName: &str, _arg_numeric: &str) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getOperatorResponse(_arg_info, _arg_longName, _arg_shortName, _arg_numeric) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getOperatorResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getOperatorResponse(_arg_info, _arg_longName, _arg_shortName, _arg_numeric, _aidl_reply))
              }
              fn r#getSignalStrengthResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSignalStrengthResponse(_arg_info, _arg_signalStrength) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSignalStrengthResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSignalStrengthResponse(_arg_info, _arg_signalStrength, _aidl_reply))
              }
              fn r#getSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSystemSelectionChannelsResponse(_arg_info, _arg_specifiers) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSystemSelectionChannelsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSystemSelectionChannelsResponse(_arg_info, _arg_specifiers, _aidl_reply))
              }
              fn r#getVoiceRadioTechnologyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getVoiceRadioTechnologyResponse(_arg_info, _arg_rat) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getVoiceRadioTechnologyResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getVoiceRadioTechnologyResponse(_arg_info, _arg_rat, _aidl_reply))
              }
              fn r#getVoiceRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_voiceRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getVoiceRegistrationStateResponse(_arg_info, _arg_voiceRegResponse) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getVoiceRegistrationStateResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getVoiceRegistrationStateResponse(_arg_info, _arg_voiceRegResponse, _aidl_reply))
              }
              fn r#isNrDualConnectivityEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_isNrDualConnectivityEnabledResponse(_arg_info, _arg_isEnabled) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#isNrDualConnectivityEnabledResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_isNrDualConnectivityEnabledResponse(_arg_info, _arg_isEnabled, _aidl_reply))
              }
              fn r#setAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setAllowedNetworkTypesBitmapResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setAllowedNetworkTypesBitmapResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setAllowedNetworkTypesBitmapResponse(_arg_info, _aidl_reply))
              }
              fn r#setBandModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setBandModeResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setBandModeResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setBandModeResponse(_arg_info, _aidl_reply))
              }
              fn r#setBarringPasswordResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setBarringPasswordResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setBarringPasswordResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setBarringPasswordResponse(_arg_info, _aidl_reply))
              }
              fn r#setCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCdmaRoamingPreferenceResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCdmaRoamingPreferenceResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCdmaRoamingPreferenceResponse(_arg_info, _aidl_reply))
              }
              fn r#setCellInfoListRateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setCellInfoListRateResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setCellInfoListRateResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setCellInfoListRateResponse(_arg_info, _aidl_reply))
              }
              fn r#setIndicationFilterResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setIndicationFilterResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setIndicationFilterResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setIndicationFilterResponse(_arg_info, _aidl_reply))
              }
              fn r#setLinkCapacityReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setLinkCapacityReportingCriteriaResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setLinkCapacityReportingCriteriaResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setLinkCapacityReportingCriteriaResponse(_arg_info, _aidl_reply))
              }
              fn r#setLocationUpdatesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setLocationUpdatesResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setLocationUpdatesResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setLocationUpdatesResponse(_arg_info, _aidl_reply))
              }
              fn r#setNetworkSelectionModeAutomaticResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setNetworkSelectionModeAutomaticResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNetworkSelectionModeAutomaticResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setNetworkSelectionModeAutomaticResponse(_arg_info, _aidl_reply))
              }
              fn r#setNetworkSelectionModeManualResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setNetworkSelectionModeManualResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNetworkSelectionModeManualResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setNetworkSelectionModeManualResponse(_arg_info, _aidl_reply))
              }
              fn r#setNrDualConnectivityStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setNrDualConnectivityStateResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNrDualConnectivityStateResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setNrDualConnectivityStateResponse(_arg_info, _aidl_reply))
              }
              fn r#setSignalStrengthReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setSignalStrengthReportingCriteriaResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSignalStrengthReportingCriteriaResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setSignalStrengthReportingCriteriaResponse(_arg_info, _aidl_reply))
              }
              fn r#setSuppServiceNotificationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setSuppServiceNotificationsResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSuppServiceNotificationsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setSuppServiceNotificationsResponse(_arg_info, _aidl_reply))
              }
              fn r#setSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setSystemSelectionChannelsResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSystemSelectionChannelsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setSystemSelectionChannelsResponse(_arg_info, _aidl_reply))
              }
              fn r#startNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_startNetworkScanResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#startNetworkScanResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_startNetworkScanResponse(_arg_info, _aidl_reply))
              }
              fn r#stopNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_stopNetworkScanResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#stopNetworkScanResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_stopNetworkScanResponse(_arg_info, _aidl_reply))
              }
              fn r#supplyNetworkDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_supplyNetworkDepersonalizationResponse(_arg_info, _arg_remainingRetries) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#supplyNetworkDepersonalizationResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_supplyNetworkDepersonalizationResponse(_arg_info, _arg_remainingRetries, _aidl_reply))
              }
              fn r#setUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setUsageSettingResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setUsageSettingResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setUsageSettingResponse(_arg_info, _aidl_reply))
              }
              fn r#getUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getUsageSettingResponse(_arg_info, _arg_usageSetting) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getUsageSettingResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getUsageSettingResponse(_arg_info, _arg_usageSetting, _aidl_reply))
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
            impl IRadioNetworkResponse for binder::binder_impl::Binder<BnRadioNetworkResponse> {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#acknowledgeRequest(_arg_serial) }
              fn r#getAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkTypeBitmap: i32) -> binder::Result<()> { self.0.r#getAllowedNetworkTypesBitmapResponse(_arg_info, _arg_networkTypeBitmap) }
              fn r#getAvailableBandModesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_bandModes: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode]) -> binder::Result<()> { self.0.r#getAvailableBandModesResponse(_arg_info, _arg_bandModes) }
              fn r#getAvailableNetworksResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_networkInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo]) -> binder::Result<()> { self.0.r#getAvailableNetworksResponse(_arg_info, _arg_networkInfos) }
              fn r#getBarringInfoResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellIdentity: &crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity, _arg_barringInfos: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo]) -> binder::Result<()> { self.0.r#getBarringInfoResponse(_arg_info, _arg_cellIdentity, _arg_barringInfos) }
              fn r#getCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType) -> binder::Result<()> { self.0.r#getCdmaRoamingPreferenceResponse(_arg_info, _arg_type) }
              fn r#getCellInfoListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_cellInfo: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo]) -> binder::Result<()> { self.0.r#getCellInfoListResponse(_arg_info, _arg_cellInfo) }
              fn r#getDataRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dataRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<()> { self.0.r#getDataRegistrationStateResponse(_arg_info, _arg_dataRegResponse) }
              fn r#getImsRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isRegistered: bool, _arg_ratFamily: crate::mangled::_7_android_8_hardware_5_radio_21_RadioTechnologyFamily) -> binder::Result<()> { self.0.r#getImsRegistrationStateResponse(_arg_info, _arg_isRegistered, _arg_ratFamily) }
              fn r#getNetworkSelectionModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_manual: bool) -> binder::Result<()> { self.0.r#getNetworkSelectionModeResponse(_arg_info, _arg_manual) }
              fn r#getOperatorResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_longName: &str, _arg_shortName: &str, _arg_numeric: &str) -> binder::Result<()> { self.0.r#getOperatorResponse(_arg_info, _arg_longName, _arg_shortName, _arg_numeric) }
              fn r#getSignalStrengthResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_signalStrength: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength) -> binder::Result<()> { self.0.r#getSignalStrengthResponse(_arg_info, _arg_signalStrength) }
              fn r#getSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_specifiers: &[crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier]) -> binder::Result<()> { self.0.r#getSystemSelectionChannelsResponse(_arg_info, _arg_specifiers) }
              fn r#getVoiceRadioTechnologyResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology) -> binder::Result<()> { self.0.r#getVoiceRadioTechnologyResponse(_arg_info, _arg_rat) }
              fn r#getVoiceRegistrationStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_voiceRegResponse: &crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult) -> binder::Result<()> { self.0.r#getVoiceRegistrationStateResponse(_arg_info, _arg_voiceRegResponse) }
              fn r#isNrDualConnectivityEnabledResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_isEnabled: bool) -> binder::Result<()> { self.0.r#isNrDualConnectivityEnabledResponse(_arg_info, _arg_isEnabled) }
              fn r#setAllowedNetworkTypesBitmapResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setAllowedNetworkTypesBitmapResponse(_arg_info) }
              fn r#setBandModeResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setBandModeResponse(_arg_info) }
              fn r#setBarringPasswordResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setBarringPasswordResponse(_arg_info) }
              fn r#setCdmaRoamingPreferenceResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setCdmaRoamingPreferenceResponse(_arg_info) }
              fn r#setCellInfoListRateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setCellInfoListRateResponse(_arg_info) }
              fn r#setIndicationFilterResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setIndicationFilterResponse(_arg_info) }
              fn r#setLinkCapacityReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setLinkCapacityReportingCriteriaResponse(_arg_info) }
              fn r#setLocationUpdatesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setLocationUpdatesResponse(_arg_info) }
              fn r#setNetworkSelectionModeAutomaticResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setNetworkSelectionModeAutomaticResponse(_arg_info) }
              fn r#setNetworkSelectionModeManualResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setNetworkSelectionModeManualResponse(_arg_info) }
              fn r#setNrDualConnectivityStateResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setNrDualConnectivityStateResponse(_arg_info) }
              fn r#setSignalStrengthReportingCriteriaResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setSignalStrengthReportingCriteriaResponse(_arg_info) }
              fn r#setSuppServiceNotificationsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setSuppServiceNotificationsResponse(_arg_info) }
              fn r#setSystemSelectionChannelsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setSystemSelectionChannelsResponse(_arg_info) }
              fn r#startNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#startNetworkScanResponse(_arg_info) }
              fn r#stopNetworkScanResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#stopNetworkScanResponse(_arg_info) }
              fn r#supplyNetworkDepersonalizationResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_remainingRetries: i32) -> binder::Result<()> { self.0.r#supplyNetworkDepersonalizationResponse(_arg_info, _arg_remainingRetries) }
              fn r#setUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setUsageSettingResponse(_arg_info) }
              fn r#getUsageSettingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting) -> binder::Result<()> { self.0.r#getUsageSettingResponse(_arg_info, _arg_usageSetting) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioNetworkResponse, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#acknowledgeRequest => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acknowledgeRequest(_arg_serial);
                  Ok(())
                }
                transactions::r#getAllowedNetworkTypesBitmapResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_networkTypeBitmap: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getAllowedNetworkTypesBitmapResponse(&_arg_info, _arg_networkTypeBitmap);
                  Ok(())
                }
                transactions::r#getAvailableBandModesResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_bandModes: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_13_RadioBandMode> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getAvailableBandModesResponse(&_arg_info, &_arg_bandModes);
                  Ok(())
                }
                transactions::r#getAvailableNetworksResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_networkInfos: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_12_OperatorInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getAvailableNetworksResponse(&_arg_info, &_arg_networkInfos);
                  Ok(())
                }
                transactions::r#getBarringInfoResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_cellIdentity: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity = _aidl_data.read()?;
                  let _arg_barringInfos: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_11_BarringInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getBarringInfoResponse(&_arg_info, &_arg_cellIdentity, &_arg_barringInfos);
                  Ok(())
                }
                transactions::r#getCdmaRoamingPreferenceResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_7_network_15_CdmaRoamingType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCdmaRoamingPreferenceResponse(&_arg_info, _arg_type);
                  Ok(())
                }
                transactions::r#getCellInfoListResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_cellInfo: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getCellInfoListResponse(&_arg_info, &_arg_cellInfo);
                  Ok(())
                }
                transactions::r#getDataRegistrationStateResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_dataRegResponse: crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDataRegistrationStateResponse(&_arg_info, &_arg_dataRegResponse);
                  Ok(())
                }
                transactions::r#getImsRegistrationStateResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_isRegistered: bool = _aidl_data.read()?;
                  let _arg_ratFamily: crate::mangled::_7_android_8_hardware_5_radio_21_RadioTechnologyFamily = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getImsRegistrationStateResponse(&_arg_info, _arg_isRegistered, _arg_ratFamily);
                  Ok(())
                }
                transactions::r#getNetworkSelectionModeResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_manual: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getNetworkSelectionModeResponse(&_arg_info, _arg_manual);
                  Ok(())
                }
                transactions::r#getOperatorResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_longName: String = _aidl_data.read()?;
                  let _arg_shortName: String = _aidl_data.read()?;
                  let _arg_numeric: String = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getOperatorResponse(&_arg_info, &_arg_longName, &_arg_shortName, &_arg_numeric);
                  Ok(())
                }
                transactions::r#getSignalStrengthResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_signalStrength: crate::mangled::_7_android_8_hardware_5_radio_7_network_14_SignalStrength = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSignalStrengthResponse(&_arg_info, &_arg_signalStrength);
                  Ok(())
                }
                transactions::r#getSystemSelectionChannelsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_specifiers: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSystemSelectionChannelsResponse(&_arg_info, &_arg_specifiers);
                  Ok(())
                }
                transactions::r#getVoiceRadioTechnologyResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getVoiceRadioTechnologyResponse(&_arg_info, _arg_rat);
                  Ok(())
                }
                transactions::r#getVoiceRegistrationStateResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_voiceRegResponse: crate::mangled::_7_android_8_hardware_5_radio_7_network_14_RegStateResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getVoiceRegistrationStateResponse(&_arg_info, &_arg_voiceRegResponse);
                  Ok(())
                }
                transactions::r#isNrDualConnectivityEnabledResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_isEnabled: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#isNrDualConnectivityEnabledResponse(&_arg_info, _arg_isEnabled);
                  Ok(())
                }
                transactions::r#setAllowedNetworkTypesBitmapResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setAllowedNetworkTypesBitmapResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setBandModeResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setBandModeResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setBarringPasswordResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setBarringPasswordResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setCdmaRoamingPreferenceResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCdmaRoamingPreferenceResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setCellInfoListRateResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setCellInfoListRateResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setIndicationFilterResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setIndicationFilterResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setLinkCapacityReportingCriteriaResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setLinkCapacityReportingCriteriaResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setLocationUpdatesResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setLocationUpdatesResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setNetworkSelectionModeAutomaticResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setNetworkSelectionModeAutomaticResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setNetworkSelectionModeManualResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setNetworkSelectionModeManualResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setNrDualConnectivityStateResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setNrDualConnectivityStateResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setSignalStrengthReportingCriteriaResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setSignalStrengthReportingCriteriaResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setSuppServiceNotificationsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setSuppServiceNotificationsResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setSystemSelectionChannelsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setSystemSelectionChannelsResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#startNetworkScanResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#startNetworkScanResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#stopNetworkScanResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#stopNetworkScanResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#supplyNetworkDepersonalizationResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_remainingRetries: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#supplyNetworkDepersonalizationResponse(&_arg_info, _arg_remainingRetries);
                  Ok(())
                }
                transactions::r#setUsageSettingResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setUsageSettingResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#getUsageSettingResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_usageSetting: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_UsageSetting = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getUsageSettingResponse(&_arg_info, _arg_usageSetting);
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
             pub use super::r#IRadioNetworkResponse as _7_android_8_hardware_5_radio_7_network_21_IRadioNetworkResponse;
            }
          }
                    pub mod IndicationFilter {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#IndicationFilter : [i32; 9] {
                r#NONE = 0,
                r#ALL = -1,
                r#SIGNAL_STRENGTH = 1,
                r#FULL_NETWORK_STATE = 2,
                r#DATA_CALL_DORMANCY_CHANGED = 4,
                r#LINK_CAPACITY_ESTIMATE = 8,
                r#PHYSICAL_CHANNEL_CONFIG = 16,
                r#REGISTRATION_FAILURE = 32,
                r#BARRING_INFO = 64,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#IndicationFilter as _7_android_8_hardware_5_radio_7_network_16_IndicationFilter;
            }
          }
                    pub mod LceDataInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#LceDataInfo {
              pub r#lastHopCapacityKbps: i32,
              pub r#confidenceLevel: i8,
              pub r#lceSuspended: bool,
            }
            impl Default for r#LceDataInfo {
              fn default() -> Self {
                Self {
                  r#lastHopCapacityKbps: 0,
                  r#confidenceLevel: 0,
                  r#lceSuspended: false,
                }
              }
            }
            impl binder::Parcelable for r#LceDataInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#lastHopCapacityKbps)?;
                  subparcel.write(&self.r#confidenceLevel)?;
                  subparcel.write(&self.r#lceSuspended)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#lastHopCapacityKbps = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#confidenceLevel = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#lceSuspended = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#LceDataInfo);
            binder::impl_deserialize_for_parcelable!(r#LceDataInfo);
            impl binder::binder_impl::ParcelableMetadata for r#LceDataInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.LceDataInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#LceDataInfo as _7_android_8_hardware_5_radio_7_network_11_LceDataInfo;
            }
          }
                    pub mod LinkCapacityEstimate {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#LinkCapacityEstimate {
              pub r#downlinkCapacityKbps: i32,
              pub r#uplinkCapacityKbps: i32,
              pub r#secondaryDownlinkCapacityKbps: i32,
              pub r#secondaryUplinkCapacityKbps: i32,
            }
            impl Default for r#LinkCapacityEstimate {
              fn default() -> Self {
                Self {
                  r#downlinkCapacityKbps: 0,
                  r#uplinkCapacityKbps: 0,
                  r#secondaryDownlinkCapacityKbps: 0,
                  r#secondaryUplinkCapacityKbps: 0,
                }
              }
            }
            impl binder::Parcelable for r#LinkCapacityEstimate {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#downlinkCapacityKbps)?;
                  subparcel.write(&self.r#uplinkCapacityKbps)?;
                  subparcel.write(&self.r#secondaryDownlinkCapacityKbps)?;
                  subparcel.write(&self.r#secondaryUplinkCapacityKbps)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#downlinkCapacityKbps = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#uplinkCapacityKbps = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#secondaryDownlinkCapacityKbps = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#secondaryUplinkCapacityKbps = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#LinkCapacityEstimate);
            binder::impl_deserialize_for_parcelable!(r#LinkCapacityEstimate);
            impl binder::binder_impl::ParcelableMetadata for r#LinkCapacityEstimate {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.LinkCapacityEstimate" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#LinkCapacityEstimate as _7_android_8_hardware_5_radio_7_network_20_LinkCapacityEstimate;
            }
          }
                    pub mod LteSignalStrength {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#LteSignalStrength {
              pub r#signalStrength: i32,
              pub r#rsrp: i32,
              pub r#rsrq: i32,
              pub r#rssnr: i32,
              pub r#cqi: i32,
              pub r#timingAdvance: i32,
              pub r#cqiTableIndex: i32,
            }
            impl Default for r#LteSignalStrength {
              fn default() -> Self {
                Self {
                  r#signalStrength: 0,
                  r#rsrp: 0,
                  r#rsrq: 0,
                  r#rssnr: 0,
                  r#cqi: 0,
                  r#timingAdvance: 0,
                  r#cqiTableIndex: 0,
                }
              }
            }
            impl binder::Parcelable for r#LteSignalStrength {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#signalStrength)?;
                  subparcel.write(&self.r#rsrp)?;
                  subparcel.write(&self.r#rsrq)?;
                  subparcel.write(&self.r#rssnr)?;
                  subparcel.write(&self.r#cqi)?;
                  subparcel.write(&self.r#timingAdvance)?;
                  subparcel.write(&self.r#cqiTableIndex)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#signalStrength = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#rsrp = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#rsrq = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#rssnr = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cqi = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#timingAdvance = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cqiTableIndex = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#LteSignalStrength);
            binder::impl_deserialize_for_parcelable!(r#LteSignalStrength);
            impl binder::binder_impl::ParcelableMetadata for r#LteSignalStrength {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.LteSignalStrength" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#LteSignalStrength as _7_android_8_hardware_5_radio_7_network_17_LteSignalStrength;
            }
          }
                    pub mod LteVopsInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#LteVopsInfo {
              pub r#isVopsSupported: bool,
              pub r#isEmcBearerSupported: bool,
            }
            impl Default for r#LteVopsInfo {
              fn default() -> Self {
                Self {
                  r#isVopsSupported: false,
                  r#isEmcBearerSupported: false,
                }
              }
            }
            impl binder::Parcelable for r#LteVopsInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#isVopsSupported)?;
                  subparcel.write(&self.r#isEmcBearerSupported)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#isVopsSupported = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#isEmcBearerSupported = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#LteVopsInfo);
            binder::impl_deserialize_for_parcelable!(r#LteVopsInfo);
            impl binder::binder_impl::ParcelableMetadata for r#LteVopsInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.LteVopsInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#LteVopsInfo as _7_android_8_hardware_5_radio_7_network_11_LteVopsInfo;
            }
          }
                    pub mod NetworkScanRequest {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#NetworkScanRequest {
              pub r#type: i32,
              pub r#interval: i32,
              pub r#specifiers: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier>,
              pub r#maxSearchTime: i32,
              pub r#incrementalResults: bool,
              pub r#incrementalResultsPeriodicity: i32,
              pub r#mccMncs: Vec<String>,
            }
            pub const r#RADIO_ACCESS_SPECIFIER_MAX_SIZE: i32 = 8;
            pub const r#INCREMENTAL_RESULTS_PREIODICITY_RANGE_MIN: i32 = 1;
            pub const r#INCREMENTAL_RESULTS_PREIODICITY_RANGE_MAX: i32 = 10;
            pub const r#MAX_SEARCH_TIME_RANGE_MIN: i32 = 60;
            pub const r#MAX_SEARCH_TIME_RANGE_MAX: i32 = 3600;
            pub const r#SCAN_INTERVAL_RANGE_MIN: i32 = 5;
            pub const r#SCAN_INTERVAL_RANGE_MAX: i32 = 300;
            pub const r#SCAN_TYPE_ONE_SHOT: i32 = 0;
            pub const r#SCAN_TYPE_PERIODIC: i32 = 1;
            impl Default for r#NetworkScanRequest {
              fn default() -> Self {
                Self {
                  r#type: 0,
                  r#interval: 0,
                  r#specifiers: Default::default(),
                  r#maxSearchTime: 0,
                  r#incrementalResults: false,
                  r#incrementalResultsPeriodicity: 0,
                  r#mccMncs: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#NetworkScanRequest {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#type)?;
                  subparcel.write(&self.r#interval)?;
                  subparcel.write(&self.r#specifiers)?;
                  subparcel.write(&self.r#maxSearchTime)?;
                  subparcel.write(&self.r#incrementalResults)?;
                  subparcel.write(&self.r#incrementalResultsPeriodicity)?;
                  subparcel.write(&self.r#mccMncs)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#type = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#interval = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#specifiers = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxSearchTime = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#incrementalResults = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#incrementalResultsPeriodicity = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mccMncs = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#NetworkScanRequest);
            binder::impl_deserialize_for_parcelable!(r#NetworkScanRequest);
            impl binder::binder_impl::ParcelableMetadata for r#NetworkScanRequest {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.NetworkScanRequest" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#NetworkScanRequest as _7_android_8_hardware_5_radio_7_network_18_NetworkScanRequest;
            }
          }
                    pub mod NetworkScanResult {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#NetworkScanResult {
              pub r#status: i32,
              pub r#error: crate::mangled::_7_android_8_hardware_5_radio_10_RadioError,
              pub r#networkInfos: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_8_CellInfo>,
            }
            pub const r#SCAN_STATUS_PARTIAL: i32 = 1;
            pub const r#SCAN_STATUS_COMPLETE: i32 = 2;
            impl Default for r#NetworkScanResult {
              fn default() -> Self {
                Self {
                  r#status: 0,
                  r#error: Default::default(),
                  r#networkInfos: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#NetworkScanResult {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#status)?;
                  subparcel.write(&self.r#error)?;
                  subparcel.write(&self.r#networkInfos)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#status = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#error = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#networkInfos = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#NetworkScanResult);
            binder::impl_deserialize_for_parcelable!(r#NetworkScanResult);
            impl binder::binder_impl::ParcelableMetadata for r#NetworkScanResult {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.NetworkScanResult" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#NetworkScanResult as _7_android_8_hardware_5_radio_7_network_17_NetworkScanResult;
            }
          }
                    pub mod NgranBands {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#NgranBands : [i32; 53] {
                r#BAND_1 = 1,
                r#BAND_2 = 2,
                r#BAND_3 = 3,
                r#BAND_5 = 5,
                r#BAND_7 = 7,
                r#BAND_8 = 8,
                r#BAND_12 = 12,
                r#BAND_14 = 14,
                r#BAND_18 = 18,
                r#BAND_20 = 20,
                r#BAND_25 = 25,
                r#BAND_26 = 26,
                r#BAND_28 = 28,
                r#BAND_29 = 29,
                r#BAND_30 = 30,
                r#BAND_34 = 34,
                r#BAND_38 = 38,
                r#BAND_39 = 39,
                r#BAND_40 = 40,
                r#BAND_41 = 41,
                r#BAND_46 = 46,
                r#BAND_48 = 48,
                r#BAND_50 = 50,
                r#BAND_51 = 51,
                r#BAND_53 = 53,
                r#BAND_65 = 65,
                r#BAND_66 = 66,
                r#BAND_70 = 70,
                r#BAND_71 = 71,
                r#BAND_74 = 74,
                r#BAND_75 = 75,
                r#BAND_76 = 76,
                r#BAND_77 = 77,
                r#BAND_78 = 78,
                r#BAND_79 = 79,
                r#BAND_80 = 80,
                r#BAND_81 = 81,
                r#BAND_82 = 82,
                r#BAND_83 = 83,
                r#BAND_84 = 84,
                r#BAND_86 = 86,
                r#BAND_89 = 89,
                r#BAND_90 = 90,
                r#BAND_91 = 91,
                r#BAND_92 = 92,
                r#BAND_93 = 93,
                r#BAND_94 = 94,
                r#BAND_95 = 95,
                r#BAND_96 = 96,
                r#BAND_257 = 257,
                r#BAND_258 = 258,
                r#BAND_260 = 260,
                r#BAND_261 = 261,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#NgranBands as _7_android_8_hardware_5_radio_7_network_10_NgranBands;
            }
          }
                    pub mod NrDualConnectivityState {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#NrDualConnectivityState : [i8; 3] {
                r#ENABLE = 1,
                r#DISABLE = 2,
                r#DISABLE_IMMEDIATE = 3,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#NrDualConnectivityState as _7_android_8_hardware_5_radio_7_network_23_NrDualConnectivityState;
            }
          }
                    pub mod NrIndicators {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#NrIndicators {
              pub r#isEndcAvailable: bool,
              pub r#isDcNrRestricted: bool,
              pub r#isNrAvailable: bool,
            }
            impl Default for r#NrIndicators {
              fn default() -> Self {
                Self {
                  r#isEndcAvailable: false,
                  r#isDcNrRestricted: false,
                  r#isNrAvailable: false,
                }
              }
            }
            impl binder::Parcelable for r#NrIndicators {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#isEndcAvailable)?;
                  subparcel.write(&self.r#isDcNrRestricted)?;
                  subparcel.write(&self.r#isNrAvailable)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#isEndcAvailable = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#isDcNrRestricted = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#isNrAvailable = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#NrIndicators);
            binder::impl_deserialize_for_parcelable!(r#NrIndicators);
            impl binder::binder_impl::ParcelableMetadata for r#NrIndicators {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.NrIndicators" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#NrIndicators as _7_android_8_hardware_5_radio_7_network_12_NrIndicators;
            }
          }
                    pub mod NrSignalStrength {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#NrSignalStrength {
              pub r#ssRsrp: i32,
              pub r#ssRsrq: i32,
              pub r#ssSinr: i32,
              pub r#csiRsrp: i32,
              pub r#csiRsrq: i32,
              pub r#csiSinr: i32,
              pub r#csiCqiTableIndex: i32,
              pub r#csiCqiReport: Vec<u8>,
            }
            impl Default for r#NrSignalStrength {
              fn default() -> Self {
                Self {
                  r#ssRsrp: 0,
                  r#ssRsrq: 0,
                  r#ssSinr: 0,
                  r#csiRsrp: 0,
                  r#csiRsrq: 0,
                  r#csiSinr: 0,
                  r#csiCqiTableIndex: 0,
                  r#csiCqiReport: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#NrSignalStrength {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#ssRsrp)?;
                  subparcel.write(&self.r#ssRsrq)?;
                  subparcel.write(&self.r#ssSinr)?;
                  subparcel.write(&self.r#csiRsrp)?;
                  subparcel.write(&self.r#csiRsrq)?;
                  subparcel.write(&self.r#csiSinr)?;
                  subparcel.write(&self.r#csiCqiTableIndex)?;
                  subparcel.write(&self.r#csiCqiReport)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#ssRsrp = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#ssRsrq = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#ssSinr = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#csiRsrp = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#csiRsrq = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#csiSinr = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#csiCqiTableIndex = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#csiCqiReport = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#NrSignalStrength);
            binder::impl_deserialize_for_parcelable!(r#NrSignalStrength);
            impl binder::binder_impl::ParcelableMetadata for r#NrSignalStrength {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.NrSignalStrength" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#NrSignalStrength as _7_android_8_hardware_5_radio_7_network_16_NrSignalStrength;
            }
          }
                    pub mod NrVopsInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#NrVopsInfo {
              pub r#vopsSupported: i8,
              pub r#emcSupported: i8,
              pub r#emfSupported: i8,
            }
            pub const r#EMC_INDICATOR_NOT_SUPPORTED: i8 = 0;
            pub const r#EMC_INDICATOR_NR_CONNECTED_TO_5GCN: i8 = 1;
            pub const r#EMC_INDICATOR_EUTRA_CONNECTED_TO_5GCN: i8 = 2;
            pub const r#EMC_INDICATOR_BOTH_NR_EUTRA_CONNECTED_TO_5GCN: i8 = 3;
            pub const r#EMF_INDICATOR_NOT_SUPPORTED: i8 = 0;
            pub const r#EMF_INDICATOR_NR_CONNECTED_TO_5GCN: i8 = 1;
            pub const r#EMF_INDICATOR_EUTRA_CONNECTED_TO_5GCN: i8 = 2;
            pub const r#EMF_INDICATOR_BOTH_NR_EUTRA_CONNECTED_TO_5GCN: i8 = 3;
            pub const r#VOPS_INDICATOR_VOPS_NOT_SUPPORTED: i8 = 0;
            pub const r#VOPS_INDICATOR_VOPS_OVER_3GPP: i8 = 1;
            pub const r#VOPS_INDICATOR_VOPS_OVER_NON_3GPP: i8 = 2;
            impl Default for r#NrVopsInfo {
              fn default() -> Self {
                Self {
                  r#vopsSupported: 0,
                  r#emcSupported: 0,
                  r#emfSupported: 0,
                }
              }
            }
            impl binder::Parcelable for r#NrVopsInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#vopsSupported)?;
                  subparcel.write(&self.r#emcSupported)?;
                  subparcel.write(&self.r#emfSupported)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#vopsSupported = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#emcSupported = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#emfSupported = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#NrVopsInfo);
            binder::impl_deserialize_for_parcelable!(r#NrVopsInfo);
            impl binder::binder_impl::ParcelableMetadata for r#NrVopsInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.NrVopsInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#NrVopsInfo as _7_android_8_hardware_5_radio_7_network_10_NrVopsInfo;
            }
          }
                    pub mod OperatorInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#OperatorInfo {
              pub r#alphaLong: String,
              pub r#alphaShort: String,
              pub r#operatorNumeric: String,
              pub r#status: i32,
            }
            pub const r#STATUS_UNKNOWN: i32 = 0;
            pub const r#STATUS_AVAILABLE: i32 = 1;
            pub const r#STATUS_CURRENT: i32 = 2;
            pub const r#STATUS_FORBIDDEN: i32 = 3;
            impl Default for r#OperatorInfo {
              fn default() -> Self {
                Self {
                  r#alphaLong: Default::default(),
                  r#alphaShort: Default::default(),
                  r#operatorNumeric: Default::default(),
                  r#status: 0,
                }
              }
            }
            impl binder::Parcelable for r#OperatorInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#alphaLong)?;
                  subparcel.write(&self.r#alphaShort)?;
                  subparcel.write(&self.r#operatorNumeric)?;
                  subparcel.write(&self.r#status)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#alphaLong = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#alphaShort = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#operatorNumeric = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#status = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#OperatorInfo);
            binder::impl_deserialize_for_parcelable!(r#OperatorInfo);
            impl binder::binder_impl::ParcelableMetadata for r#OperatorInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.OperatorInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#OperatorInfo as _7_android_8_hardware_5_radio_7_network_12_OperatorInfo;
            }
          }
                    pub mod PhoneRestrictedState {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#PhoneRestrictedState : [i32; 5] {
                r#NONE = 0,
                r#CS_EMERGENCY = 1,
                r#CS_NORMAL = 2,
                r#CS_ALL = 4,
                r#PS_ALL = 16,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PhoneRestrictedState as _7_android_8_hardware_5_radio_7_network_20_PhoneRestrictedState;
            }
          }
                    pub mod PhysicalChannelConfig {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#PhysicalChannelConfig {
              pub r#status: crate::mangled::_7_android_8_hardware_5_radio_7_network_20_CellConnectionStatus,
              pub r#rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology,
              pub r#downlinkChannelNumber: i32,
              pub r#uplinkChannelNumber: i32,
              pub r#cellBandwidthDownlinkKhz: i32,
              pub r#cellBandwidthUplinkKhz: i32,
              pub r#contextIds: Vec<i32>,
              pub r#physicalCellId: i32,
              pub r#band: crate::mangled::_7_android_8_hardware_5_radio_7_network_25_PhysicalChannelConfigBand,
            }
            impl Default for r#PhysicalChannelConfig {
              fn default() -> Self {
                Self {
                  r#status: Default::default(),
                  r#rat: Default::default(),
                  r#downlinkChannelNumber: 0,
                  r#uplinkChannelNumber: 0,
                  r#cellBandwidthDownlinkKhz: 0,
                  r#cellBandwidthUplinkKhz: 0,
                  r#contextIds: Default::default(),
                  r#physicalCellId: 0,
                  r#band: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#PhysicalChannelConfig {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#status)?;
                  subparcel.write(&self.r#rat)?;
                  subparcel.write(&self.r#downlinkChannelNumber)?;
                  subparcel.write(&self.r#uplinkChannelNumber)?;
                  subparcel.write(&self.r#cellBandwidthDownlinkKhz)?;
                  subparcel.write(&self.r#cellBandwidthUplinkKhz)?;
                  subparcel.write(&self.r#contextIds)?;
                  subparcel.write(&self.r#physicalCellId)?;
                  subparcel.write(&self.r#band)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#status = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#rat = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#downlinkChannelNumber = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#uplinkChannelNumber = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cellBandwidthDownlinkKhz = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cellBandwidthUplinkKhz = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#contextIds = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#physicalCellId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#band = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PhysicalChannelConfig);
            binder::impl_deserialize_for_parcelable!(r#PhysicalChannelConfig);
            impl binder::binder_impl::ParcelableMetadata for r#PhysicalChannelConfig {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.PhysicalChannelConfig" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PhysicalChannelConfig as _7_android_8_hardware_5_radio_7_network_21_PhysicalChannelConfig;
            }
          }
                    pub mod PhysicalChannelConfigBand {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub enum r#PhysicalChannelConfigBand {
              Noinit(bool),
              GeranBand(crate::mangled::_7_android_8_hardware_5_radio_7_network_10_GeranBands),
              UtranBand(crate::mangled::_7_android_8_hardware_5_radio_7_network_10_UtranBands),
              EutranBand(crate::mangled::_7_android_8_hardware_5_radio_7_network_11_EutranBands),
              NgranBand(crate::mangled::_7_android_8_hardware_5_radio_7_network_10_NgranBands),
            }
            impl Default for r#PhysicalChannelConfigBand {
              fn default() -> Self {
                Self::Noinit(false)
              }
            }
            impl binder::Parcelable for r#PhysicalChannelConfigBand {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::Noinit(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::GeranBand(v) => {
                    parcel.write(&1i32)?;
                    parcel.write(v)
                  }
                  Self::UtranBand(v) => {
                    parcel.write(&2i32)?;
                    parcel.write(v)
                  }
                  Self::EutranBand(v) => {
                    parcel.write(&3i32)?;
                    parcel.write(v)
                  }
                  Self::NgranBand(v) => {
                    parcel.write(&4i32)?;
                    parcel.write(v)
                  }
                }
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                let tag: i32 = parcel.read()?;
                match tag {
                  0 => {
                    let value: bool = parcel.read()?;
                    *self = Self::Noinit(value);
                    Ok(())
                  }
                  1 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_10_GeranBands = parcel.read()?;
                    *self = Self::GeranBand(value);
                    Ok(())
                  }
                  2 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_10_UtranBands = parcel.read()?;
                    *self = Self::UtranBand(value);
                    Ok(())
                  }
                  3 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_11_EutranBands = parcel.read()?;
                    *self = Self::EutranBand(value);
                    Ok(())
                  }
                  4 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_7_network_10_NgranBands = parcel.read()?;
                    *self = Self::NgranBand(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#PhysicalChannelConfigBand);
            binder::impl_deserialize_for_parcelable!(r#PhysicalChannelConfigBand);
            impl binder::binder_impl::ParcelableMetadata for r#PhysicalChannelConfigBand {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.PhysicalChannelConfigBand" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                r#Tag : [i32; 5] {
                  r#noinit = 0,
                  r#geranBand = 1,
                  r#utranBand = 2,
                  r#eutranBand = 3,
                  r#ngranBand = 4,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PhysicalChannelConfigBand as _7_android_8_hardware_5_radio_7_network_25_PhysicalChannelConfigBand;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_5_radio_7_network_25_PhysicalChannelConfigBand_3_Tag;
            }
          }
                    pub mod RadioAccessSpecifier {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#RadioAccessSpecifier {
              pub r#accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork,
              pub r#bands: crate::mangled::_7_android_8_hardware_5_radio_7_network_25_RadioAccessSpecifierBands,
              pub r#channels: Vec<i32>,
            }
            impl Default for r#RadioAccessSpecifier {
              fn default() -> Self {
                Self {
                  r#accessNetwork: Default::default(),
                  r#bands: Default::default(),
                  r#channels: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#RadioAccessSpecifier {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#accessNetwork)?;
                  subparcel.write(&self.r#bands)?;
                  subparcel.write(&self.r#channels)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#accessNetwork = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bands = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#channels = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#RadioAccessSpecifier);
            binder::impl_deserialize_for_parcelable!(r#RadioAccessSpecifier);
            impl binder::binder_impl::ParcelableMetadata for r#RadioAccessSpecifier {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.RadioAccessSpecifier" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#RadioAccessSpecifier as _7_android_8_hardware_5_radio_7_network_20_RadioAccessSpecifier;
            }
          }
                    pub mod RadioAccessSpecifierBands {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub enum r#RadioAccessSpecifierBands {
              Noinit(bool),
              GeranBands(Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_10_GeranBands>),
              UtranBands(Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_10_UtranBands>),
              EutranBands(Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_11_EutranBands>),
              NgranBands(Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_10_NgranBands>),
            }
            impl Default for r#RadioAccessSpecifierBands {
              fn default() -> Self {
                Self::Noinit(false)
              }
            }
            impl binder::Parcelable for r#RadioAccessSpecifierBands {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::Noinit(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::GeranBands(v) => {
                    parcel.write(&1i32)?;
                    parcel.write(v)
                  }
                  Self::UtranBands(v) => {
                    parcel.write(&2i32)?;
                    parcel.write(v)
                  }
                  Self::EutranBands(v) => {
                    parcel.write(&3i32)?;
                    parcel.write(v)
                  }
                  Self::NgranBands(v) => {
                    parcel.write(&4i32)?;
                    parcel.write(v)
                  }
                }
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                let tag: i32 = parcel.read()?;
                match tag {
                  0 => {
                    let value: bool = parcel.read()?;
                    *self = Self::Noinit(value);
                    Ok(())
                  }
                  1 => {
                    let value: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_10_GeranBands> = parcel.read()?;
                    *self = Self::GeranBands(value);
                    Ok(())
                  }
                  2 => {
                    let value: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_10_UtranBands> = parcel.read()?;
                    *self = Self::UtranBands(value);
                    Ok(())
                  }
                  3 => {
                    let value: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_11_EutranBands> = parcel.read()?;
                    *self = Self::EutranBands(value);
                    Ok(())
                  }
                  4 => {
                    let value: Vec<crate::mangled::_7_android_8_hardware_5_radio_7_network_10_NgranBands> = parcel.read()?;
                    *self = Self::NgranBands(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#RadioAccessSpecifierBands);
            binder::impl_deserialize_for_parcelable!(r#RadioAccessSpecifierBands);
            impl binder::binder_impl::ParcelableMetadata for r#RadioAccessSpecifierBands {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.RadioAccessSpecifierBands" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                r#Tag : [i32; 5] {
                  r#noinit = 0,
                  r#geranBands = 1,
                  r#utranBands = 2,
                  r#eutranBands = 3,
                  r#ngranBands = 4,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#RadioAccessSpecifierBands as _7_android_8_hardware_5_radio_7_network_25_RadioAccessSpecifierBands;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_5_radio_7_network_25_RadioAccessSpecifierBands_3_Tag;
            }
          }
                    pub mod RadioBandMode {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#RadioBandMode : [i32; 19] {
                r#BAND_MODE_UNSPECIFIED = 0,
                r#BAND_MODE_EURO = 1,
                r#BAND_MODE_USA = 2,
                r#BAND_MODE_JPN = 3,
                r#BAND_MODE_AUS = 4,
                r#BAND_MODE_AUS_2 = 5,
                r#BAND_MODE_CELL_800 = 6,
                r#BAND_MODE_PCS = 7,
                r#BAND_MODE_JTACS = 8,
                r#BAND_MODE_KOREA_PCS = 9,
                r#BAND_MODE_5_450M = 10,
                r#BAND_MODE_IMT2000 = 11,
                r#BAND_MODE_7_700M_2 = 12,
                r#BAND_MODE_8_1800M = 13,
                r#BAND_MODE_9_900M = 14,
                r#BAND_MODE_10_800M_2 = 15,
                r#BAND_MODE_EURO_PAMR_400M = 16,
                r#BAND_MODE_AWS = 17,
                r#BAND_MODE_USA_2500M = 18,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#RadioBandMode as _7_android_8_hardware_5_radio_7_network_13_RadioBandMode;
            }
          }
                    pub mod RegState {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#RegState : [i32; 10] {
                r#NOT_REG_MT_NOT_SEARCHING_OP = 0,
                r#REG_HOME = 1,
                r#NOT_REG_MT_SEARCHING_OP = 2,
                r#REG_DENIED = 3,
                r#UNKNOWN = 4,
                r#REG_ROAMING = 5,
                r#NOT_REG_MT_NOT_SEARCHING_OP_EM = 10,
                r#NOT_REG_MT_SEARCHING_OP_EM = 12,
                r#REG_DENIED_EM = 13,
                r#UNKNOWN_EM = 14,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#RegState as _7_android_8_hardware_5_radio_7_network_8_RegState;
            }
          }
                    pub mod RegStateResult {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#RegStateResult {
              pub r#regState: crate::mangled::_7_android_8_hardware_5_radio_7_network_8_RegState,
              pub r#rat: crate::mangled::_7_android_8_hardware_5_radio_15_RadioTechnology,
              pub r#reasonForDenial: crate::mangled::_7_android_8_hardware_5_radio_7_network_21_RegistrationFailCause,
              pub r#cellIdentity: crate::mangled::_7_android_8_hardware_5_radio_7_network_12_CellIdentity,
              pub r#registeredPlmn: String,
              pub r#accessTechnologySpecificInfo: crate::mangled::_7_android_8_hardware_5_radio_7_network_28_AccessTechnologySpecificInfo,
            }
            impl Default for r#RegStateResult {
              fn default() -> Self {
                Self {
                  r#regState: Default::default(),
                  r#rat: Default::default(),
                  r#reasonForDenial: Default::default(),
                  r#cellIdentity: Default::default(),
                  r#registeredPlmn: Default::default(),
                  r#accessTechnologySpecificInfo: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#RegStateResult {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#regState)?;
                  subparcel.write(&self.r#rat)?;
                  subparcel.write(&self.r#reasonForDenial)?;
                  subparcel.write(&self.r#cellIdentity)?;
                  subparcel.write(&self.r#registeredPlmn)?;
                  subparcel.write(&self.r#accessTechnologySpecificInfo)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#regState = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#rat = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#reasonForDenial = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cellIdentity = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#registeredPlmn = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#accessTechnologySpecificInfo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#RegStateResult);
            binder::impl_deserialize_for_parcelable!(r#RegStateResult);
            impl binder::binder_impl::ParcelableMetadata for r#RegStateResult {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.RegStateResult" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#RegStateResult as _7_android_8_hardware_5_radio_7_network_14_RegStateResult;
            }
          }
                    pub mod RegistrationFailCause {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#RegistrationFailCause : [i32; 52] {
                r#NONE = 0,
                r#IMSI_UNKNOWN_IN_HLR = 2,
                r#ILLEGAL_MS = 3,
                r#IMSI_UNKNOWN_IN_VLR = 4,
                r#IMEI_NOT_ACCEPTED = 5,
                r#ILLEGAL_ME = 6,
                r#GPRS_SERVICES_NOT_ALLOWED = 7,
                r#GPRS_AND_NON_GPRS_SERVICES_NOT_ALLOWED = 8,
                r#MS_IDENTITY_CANNOT_BE_DERIVED_BY_NETWORK = 9,
                r#IMPLICITLY_DETACHED = 10,
                r#PLMN_NOT_ALLOWED = 11,
                r#LOCATION_AREA_NOT_ALLOWED = 12,
                r#ROAMING_NOT_ALLOWED = 13,
                r#GPRS_SERVICES_NOT_ALLOWED_IN_PLMN = 14,
                r#NO_SUITABLE_CELLS = 15,
                r#MSC_TEMPORARILY_NOT_REACHABLE = 15,
                r#NETWORK_FAILURE = 17,
                r#MAC_FAILURE = 20,
                r#SYNC_FAILURE = 21,
                r#CONGESTION = 22,
                r#GSM_AUTHENTICATION_UNACCEPTABLE = 23,
                r#NOT_AUTHORIZED_FOR_THIS_CSG = 25,
                r#SMS_PROVIDED_BY_GPRS_IN_ROUTING_AREA = 26,
                r#SERVICE_OPTION_NOT_SUPPORTED = 32,
                r#SERVICE_OPTION_NOT_SUBSCRIBED = 33,
                r#SERVICE_OPTION_TEMPORARILY_OUT_OF_ORDER = 34,
                r#CALL_CANNOT_BE_IDENTIFIED = 38,
                r#NO_PDP_CONTEXT_ACTIVATED = 40,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_1 = 48,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_2 = 49,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_3 = 50,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_4 = 51,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_5 = 52,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_6 = 53,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_7 = 54,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_8 = 55,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_9 = 56,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_10 = 57,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_11 = 58,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_12 = 59,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_13 = 60,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_14 = 61,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_15 = 62,
                r#RETRY_UPON_ENTRY_INTO_NEW_CELL_16 = 63,
                r#SEMANTICALLY_INCORRECT_MESSAGE = 95,
                r#INVALID_MANDATORY_INFORMATION = 96,
                r#MESSAGE_TYPE_NON_EXISTENT_OR_NOT_IMPLEMENTED = 97,
                r#MESSAGE_TYPE_NOT_COMPATIBLE_WITH_PROTOCOL_STATE = 98,
                r#INFORMATION_ELEMENT_NON_EXISTENT_OR_NOT_IMPLEMENTED = 99,
                r#CONDITIONAL_IE_ERROR = 100,
                r#MESSAGE_NOT_COMPATIBLE_WITH_PROTOCOL_STATE = 101,
                r#PROTOCOL_ERROR_UNSPECIFIED = 111,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#RegistrationFailCause as _7_android_8_hardware_5_radio_7_network_21_RegistrationFailCause;
            }
          }
                    pub mod SignalStrength {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#SignalStrength {
              pub r#gsm: crate::mangled::_7_android_8_hardware_5_radio_7_network_17_GsmSignalStrength,
              pub r#cdma: crate::mangled::_7_android_8_hardware_5_radio_7_network_18_CdmaSignalStrength,
              pub r#evdo: crate::mangled::_7_android_8_hardware_5_radio_7_network_18_EvdoSignalStrength,
              pub r#lte: crate::mangled::_7_android_8_hardware_5_radio_7_network_17_LteSignalStrength,
              pub r#tdscdma: crate::mangled::_7_android_8_hardware_5_radio_7_network_21_TdscdmaSignalStrength,
              pub r#wcdma: crate::mangled::_7_android_8_hardware_5_radio_7_network_19_WcdmaSignalStrength,
              pub r#nr: crate::mangled::_7_android_8_hardware_5_radio_7_network_16_NrSignalStrength,
            }
            impl Default for r#SignalStrength {
              fn default() -> Self {
                Self {
                  r#gsm: Default::default(),
                  r#cdma: Default::default(),
                  r#evdo: Default::default(),
                  r#lte: Default::default(),
                  r#tdscdma: Default::default(),
                  r#wcdma: Default::default(),
                  r#nr: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#SignalStrength {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#gsm)?;
                  subparcel.write(&self.r#cdma)?;
                  subparcel.write(&self.r#evdo)?;
                  subparcel.write(&self.r#lte)?;
                  subparcel.write(&self.r#tdscdma)?;
                  subparcel.write(&self.r#wcdma)?;
                  subparcel.write(&self.r#nr)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#gsm = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cdma = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#evdo = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#lte = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#tdscdma = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#wcdma = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#nr = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SignalStrength);
            binder::impl_deserialize_for_parcelable!(r#SignalStrength);
            impl binder::binder_impl::ParcelableMetadata for r#SignalStrength {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.SignalStrength" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SignalStrength as _7_android_8_hardware_5_radio_7_network_14_SignalStrength;
            }
          }
                    pub mod SignalThresholdInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#SignalThresholdInfo {
              pub r#signalMeasurement: i32,
              pub r#hysteresisMs: i32,
              pub r#hysteresisDb: i32,
              pub r#thresholds: Vec<i32>,
              pub r#isEnabled: bool,
              pub r#ran: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork,
            }
            pub const r#SIGNAL_MEASUREMENT_TYPE_RSSI: i32 = 1;
            pub const r#SIGNAL_MEASUREMENT_TYPE_RSCP: i32 = 2;
            pub const r#SIGNAL_MEASUREMENT_TYPE_RSRP: i32 = 3;
            pub const r#SIGNAL_MEASUREMENT_TYPE_RSRQ: i32 = 4;
            pub const r#SIGNAL_MEASUREMENT_TYPE_RSSNR: i32 = 5;
            pub const r#SIGNAL_MEASUREMENT_TYPE_SSRSRP: i32 = 6;
            pub const r#SIGNAL_MEASUREMENT_TYPE_SSRSRQ: i32 = 7;
            pub const r#SIGNAL_MEASUREMENT_TYPE_SSSINR: i32 = 8;
            impl Default for r#SignalThresholdInfo {
              fn default() -> Self {
                Self {
                  r#signalMeasurement: 0,
                  r#hysteresisMs: 0,
                  r#hysteresisDb: 0,
                  r#thresholds: Default::default(),
                  r#isEnabled: false,
                  r#ran: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#SignalThresholdInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#signalMeasurement)?;
                  subparcel.write(&self.r#hysteresisMs)?;
                  subparcel.write(&self.r#hysteresisDb)?;
                  subparcel.write(&self.r#thresholds)?;
                  subparcel.write(&self.r#isEnabled)?;
                  subparcel.write(&self.r#ran)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#signalMeasurement = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#hysteresisMs = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#hysteresisDb = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#thresholds = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#isEnabled = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#ran = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SignalThresholdInfo);
            binder::impl_deserialize_for_parcelable!(r#SignalThresholdInfo);
            impl binder::binder_impl::ParcelableMetadata for r#SignalThresholdInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.SignalThresholdInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SignalThresholdInfo as _7_android_8_hardware_5_radio_7_network_19_SignalThresholdInfo;
            }
          }
                    pub mod SuppSvcNotification {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#SuppSvcNotification {
              pub r#isMT: bool,
              pub r#code: i32,
              pub r#index: i32,
              pub r#type: i32,
              pub r#number: String,
            }
            impl Default for r#SuppSvcNotification {
              fn default() -> Self {
                Self {
                  r#isMT: false,
                  r#code: 0,
                  r#index: 0,
                  r#type: 0,
                  r#number: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#SuppSvcNotification {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#isMT)?;
                  subparcel.write(&self.r#code)?;
                  subparcel.write(&self.r#index)?;
                  subparcel.write(&self.r#type)?;
                  subparcel.write(&self.r#number)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#isMT = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#code = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#index = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#type = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#number = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SuppSvcNotification);
            binder::impl_deserialize_for_parcelable!(r#SuppSvcNotification);
            impl binder::binder_impl::ParcelableMetadata for r#SuppSvcNotification {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.SuppSvcNotification" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SuppSvcNotification as _7_android_8_hardware_5_radio_7_network_19_SuppSvcNotification;
            }
          }
                    pub mod TdscdmaSignalStrength {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#TdscdmaSignalStrength {
              pub r#signalStrength: i32,
              pub r#bitErrorRate: i32,
              pub r#rscp: i32,
            }
            impl Default for r#TdscdmaSignalStrength {
              fn default() -> Self {
                Self {
                  r#signalStrength: 0,
                  r#bitErrorRate: 0,
                  r#rscp: 0,
                }
              }
            }
            impl binder::Parcelable for r#TdscdmaSignalStrength {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#signalStrength)?;
                  subparcel.write(&self.r#bitErrorRate)?;
                  subparcel.write(&self.r#rscp)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#signalStrength = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bitErrorRate = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#rscp = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#TdscdmaSignalStrength);
            binder::impl_deserialize_for_parcelable!(r#TdscdmaSignalStrength);
            impl binder::binder_impl::ParcelableMetadata for r#TdscdmaSignalStrength {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.TdscdmaSignalStrength" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#TdscdmaSignalStrength as _7_android_8_hardware_5_radio_7_network_21_TdscdmaSignalStrength;
            }
          }
                    pub mod UsageSetting {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#UsageSetting : [i32; 2] {
                r#VOICE_CENTRIC = 1,
                r#DATA_CENTRIC = 2,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#UsageSetting as _7_android_8_hardware_5_radio_7_network_12_UsageSetting;
            }
          }
                    pub mod UtranBands {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#UtranBands : [i32; 26] {
                r#BAND_1 = 1,
                r#BAND_2 = 2,
                r#BAND_3 = 3,
                r#BAND_4 = 4,
                r#BAND_5 = 5,
                r#BAND_6 = 6,
                r#BAND_7 = 7,
                r#BAND_8 = 8,
                r#BAND_9 = 9,
                r#BAND_10 = 10,
                r#BAND_11 = 11,
                r#BAND_12 = 12,
                r#BAND_13 = 13,
                r#BAND_14 = 14,
                r#BAND_19 = 19,
                r#BAND_20 = 20,
                r#BAND_21 = 21,
                r#BAND_22 = 22,
                r#BAND_25 = 25,
                r#BAND_26 = 26,
                r#BAND_A = 101,
                r#BAND_B = 102,
                r#BAND_C = 103,
                r#BAND_D = 104,
                r#BAND_E = 105,
                r#BAND_F = 106,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#UtranBands as _7_android_8_hardware_5_radio_7_network_10_UtranBands;
            }
          }
                    pub mod WcdmaSignalStrength {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#WcdmaSignalStrength {
              pub r#signalStrength: i32,
              pub r#bitErrorRate: i32,
              pub r#rscp: i32,
              pub r#ecno: i32,
            }
            impl Default for r#WcdmaSignalStrength {
              fn default() -> Self {
                Self {
                  r#signalStrength: 0,
                  r#bitErrorRate: 0,
                  r#rscp: 0,
                  r#ecno: 0,
                }
              }
            }
            impl binder::Parcelable for r#WcdmaSignalStrength {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#signalStrength)?;
                  subparcel.write(&self.r#bitErrorRate)?;
                  subparcel.write(&self.r#rscp)?;
                  subparcel.write(&self.r#ecno)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#signalStrength = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bitErrorRate = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#rscp = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#ecno = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#WcdmaSignalStrength);
            binder::impl_deserialize_for_parcelable!(r#WcdmaSignalStrength);
            impl binder::binder_impl::ParcelableMetadata for r#WcdmaSignalStrength {
              fn get_descriptor() -> &'static str { "android.hardware.radio.network.WcdmaSignalStrength" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#WcdmaSignalStrength as _7_android_8_hardware_5_radio_7_network_19_WcdmaSignalStrength;
            }
          }
                }
            }
        }
    }
}
pub mod mangled {
    pub use super::aidl::android::hardware::radio::network::AccessTechnologySpecificInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::BarringInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::BarringTypeSpecificInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::Cdma2000RegistrationInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CdmaRoamingType::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CdmaSignalStrength::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellConnectionStatus::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellIdentity::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellIdentityCdma::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellIdentityGsm::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellIdentityLte::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellIdentityNr::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellIdentityTdscdma::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellIdentityWcdma::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellInfoCdma::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellInfoGsm::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellInfoLte::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellInfoNr::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellInfoRatSpecificInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellInfoTdscdma::mangled::*;
    pub use super::aidl::android::hardware::radio::network::CellInfoWcdma::mangled::*;
    pub use super::aidl::android::hardware::radio::network::ClosedSubscriberGroupInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::Domain::mangled::*;
    pub use super::aidl::android::hardware::radio::network::EutranBands::mangled::*;
    pub use super::aidl::android::hardware::radio::network::EutranRegistrationInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::EvdoSignalStrength::mangled::*;
    pub use super::aidl::android::hardware::radio::network::GeranBands::mangled::*;
    pub use super::aidl::android::hardware::radio::network::GsmSignalStrength::mangled::*;
    pub use super::aidl::android::hardware::radio::network::IRadioNetwork::mangled::*;
    pub use super::aidl::android::hardware::radio::network::IRadioNetworkIndication::mangled::*;
    pub use super::aidl::android::hardware::radio::network::IRadioNetworkResponse::mangled::*;
    pub use super::aidl::android::hardware::radio::network::IndicationFilter::mangled::*;
    pub use super::aidl::android::hardware::radio::network::LceDataInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::LinkCapacityEstimate::mangled::*;
    pub use super::aidl::android::hardware::radio::network::LteSignalStrength::mangled::*;
    pub use super::aidl::android::hardware::radio::network::LteVopsInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::NetworkScanRequest::mangled::*;
    pub use super::aidl::android::hardware::radio::network::NetworkScanResult::mangled::*;
    pub use super::aidl::android::hardware::radio::network::NgranBands::mangled::*;
    pub use super::aidl::android::hardware::radio::network::NrDualConnectivityState::mangled::*;
    pub use super::aidl::android::hardware::radio::network::NrIndicators::mangled::*;
    pub use super::aidl::android::hardware::radio::network::NrSignalStrength::mangled::*;
    pub use super::aidl::android::hardware::radio::network::NrVopsInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::OperatorInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::PhoneRestrictedState::mangled::*;
    pub use super::aidl::android::hardware::radio::network::PhysicalChannelConfig::mangled::*;
    pub use super::aidl::android::hardware::radio::network::PhysicalChannelConfigBand::mangled::*;
    pub use super::aidl::android::hardware::radio::network::RadioAccessSpecifier::mangled::*;
    pub use super::aidl::android::hardware::radio::network::RadioAccessSpecifierBands::mangled::*;
    pub use super::aidl::android::hardware::radio::network::RadioBandMode::mangled::*;
    pub use super::aidl::android::hardware::radio::network::RegState::mangled::*;
    pub use super::aidl::android::hardware::radio::network::RegStateResult::mangled::*;
    pub use super::aidl::android::hardware::radio::network::RegistrationFailCause::mangled::*;
    pub use super::aidl::android::hardware::radio::network::SignalStrength::mangled::*;
    pub use super::aidl::android::hardware::radio::network::SignalThresholdInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::network::SuppSvcNotification::mangled::*;
    pub use super::aidl::android::hardware::radio::network::TdscdmaSignalStrength::mangled::*;
    pub use super::aidl::android::hardware::radio::network::UsageSetting::mangled::*;
    pub use super::aidl::android::hardware::radio::network::UtranBands::mangled::*;
    pub use super::aidl::android::hardware::radio::network::WcdmaSignalStrength::mangled::*;
    pub(crate) use android_hardware_radio::mangled::*;
}
