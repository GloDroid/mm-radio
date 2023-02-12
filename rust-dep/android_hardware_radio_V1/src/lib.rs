#![allow(non_snake_case)]
#![allow(missing_docs)]
pub use binder;
pub mod aidl {
    pub mod android {
        pub mod hardware {
            pub mod radio {
                pub mod AccessNetwork {
                    #![forbid(unsafe_code)]
                    #![allow(non_upper_case_globals)]
                    use binder::declare_binder_enum;
                    declare_binder_enum! {
                      r#AccessNetwork : [i32; 7] {
                        r#UNKNOWN = 0,
                        r#GERAN = 1,
                        r#UTRAN = 2,
                        r#EUTRAN = 3,
                        r#CDMA2000 = 4,
                        r#IWLAN = 5,
                        r#NGRAN = 6,
                      }
                    }
                    pub(crate) mod mangled {
                        pub use super::r#AccessNetwork as _7_android_8_hardware_5_radio_13_AccessNetwork;
                    }
                }
                pub mod RadioAccessFamily {
                    #![forbid(unsafe_code)]
                    #![allow(non_upper_case_globals)]
                    use binder::declare_binder_enum;
                    declare_binder_enum! {
                      r#RadioAccessFamily : [i32; 21] {
                        r#UNKNOWN = 1,
                        r#GPRS = 2,
                        r#EDGE = 4,
                        r#UMTS = 8,
                        r#IS95A = 16,
                        r#IS95B = 32,
                        r#ONE_X_RTT = 64,
                        r#EVDO_0 = 128,
                        r#EVDO_A = 256,
                        r#HSDPA = 512,
                        r#HSUPA = 1024,
                        r#HSPA = 2048,
                        r#EVDO_B = 4096,
                        r#EHRPD = 8192,
                        r#LTE = 16384,
                        r#HSPAP = 32768,
                        r#GSM = 65536,
                        r#TD_SCDMA = 131072,
                        r#IWLAN = 262144,
                        r#LTE_CA = 524288,
                        r#NR = 1048576,
                      }
                    }
                    pub(crate) mod mangled {
                        pub use super::r#RadioAccessFamily as _7_android_8_hardware_5_radio_17_RadioAccessFamily;
                    }
                }
                pub mod RadioConst {
                    #![forbid(unsafe_code)]

                    #[derive(Debug)]
                    pub struct r#RadioConst {}
                    pub const r#MAX_RILDS: i32 = 3;
                    pub const r#MAX_UUID_LENGTH: i32 = 64;
                    pub const r#CARD_MAX_APPS: i32 = 8;
                    pub const r#P2_CONSTANT_NO_P2: i32 = -1;
                    impl Default for r#RadioConst {
                        fn default() -> Self {
                            Self {}
                        }
                    }
                    impl binder::Parcelable for r#RadioConst {
                        fn write_to_parcel(
                            &self,
                            parcel: &mut binder::binder_impl::BorrowedParcel,
                        ) -> std::result::Result<(), binder::StatusCode> {
                            parcel.sized_write(|_subparcel| Ok(()))
                        }
                        fn read_from_parcel(
                            &mut self,
                            parcel: &binder::binder_impl::BorrowedParcel,
                        ) -> std::result::Result<(), binder::StatusCode> {
                            parcel.sized_read(|_subparcel| Ok(()))
                        }
                    }
                    binder::impl_serialize_for_parcelable!(r#RadioConst);
                    binder::impl_deserialize_for_parcelable!(r#RadioConst);
                    impl binder::binder_impl::ParcelableMetadata for r#RadioConst {
                        fn get_descriptor() -> &'static str {
                            "android.hardware.radio.RadioConst"
                        }
                        fn get_stability(&self) -> binder::binder_impl::Stability {
                            binder::binder_impl::Stability::Vintf
                        }
                    }
                    pub(crate) mod mangled {
                        pub use super::r#RadioConst as _7_android_8_hardware_5_radio_10_RadioConst;
                    }
                }
                pub mod RadioError {
                    #![forbid(unsafe_code)]
                    #![allow(non_upper_case_globals)]
                    use binder::declare_binder_enum;
                    declare_binder_enum! {
                      r#RadioError : [i32; 89] {
                        r#NONE = 0,
                        r#RADIO_NOT_AVAILABLE = 1,
                        r#GENERIC_FAILURE = 2,
                        r#PASSWORD_INCORRECT = 3,
                        r#SIM_PIN2 = 4,
                        r#SIM_PUK2 = 5,
                        r#REQUEST_NOT_SUPPORTED = 6,
                        r#CANCELLED = 7,
                        r#OP_NOT_ALLOWED_DURING_VOICE_CALL = 8,
                        r#OP_NOT_ALLOWED_BEFORE_REG_TO_NW = 9,
                        r#SMS_SEND_FAIL_RETRY = 10,
                        r#SIM_ABSENT = 11,
                        r#SUBSCRIPTION_NOT_AVAILABLE = 12,
                        r#MODE_NOT_SUPPORTED = 13,
                        r#FDN_CHECK_FAILURE = 14,
                        r#ILLEGAL_SIM_OR_ME = 15,
                        r#MISSING_RESOURCE = 16,
                        r#NO_SUCH_ELEMENT = 17,
                        r#DIAL_MODIFIED_TO_USSD = 18,
                        r#DIAL_MODIFIED_TO_SS = 19,
                        r#DIAL_MODIFIED_TO_DIAL = 20,
                        r#USSD_MODIFIED_TO_DIAL = 21,
                        r#USSD_MODIFIED_TO_SS = 22,
                        r#USSD_MODIFIED_TO_USSD = 23,
                        r#SS_MODIFIED_TO_DIAL = 24,
                        r#SS_MODIFIED_TO_USSD = 25,
                        r#SUBSCRIPTION_NOT_SUPPORTED = 26,
                        r#SS_MODIFIED_TO_SS = 27,
                        r#LCE_NOT_SUPPORTED = 36,
                        r#NO_MEMORY = 37,
                        r#INTERNAL_ERR = 38,
                        r#SYSTEM_ERR = 39,
                        r#MODEM_ERR = 40,
                        r#INVALID_STATE = 41,
                        r#NO_RESOURCES = 42,
                        r#SIM_ERR = 43,
                        r#INVALID_ARGUMENTS = 44,
                        r#INVALID_SIM_STATE = 45,
                        r#INVALID_MODEM_STATE = 46,
                        r#INVALID_CALL_ID = 47,
                        r#NO_SMS_TO_ACK = 48,
                        r#NETWORK_ERR = 49,
                        r#REQUEST_RATE_LIMITED = 50,
                        r#SIM_BUSY = 51,
                        r#SIM_FULL = 52,
                        r#NETWORK_REJECT = 53,
                        r#OPERATION_NOT_ALLOWED = 54,
                        r#EMPTY_RECORD = 55,
                        r#INVALID_SMS_FORMAT = 56,
                        r#ENCODING_ERR = 57,
                        r#INVALID_SMSC_ADDRESS = 58,
                        r#NO_SUCH_ENTRY = 59,
                        r#NETWORK_NOT_READY = 60,
                        r#NOT_PROVISIONED = 61,
                        r#NO_SUBSCRIPTION = 62,
                        r#NO_NETWORK_FOUND = 63,
                        r#DEVICE_IN_USE = 64,
                        r#ABORTED = 65,
                        r#INVALID_RESPONSE = 66,
                        r#OEM_ERROR_1 = 501,
                        r#OEM_ERROR_2 = 502,
                        r#OEM_ERROR_3 = 503,
                        r#OEM_ERROR_4 = 504,
                        r#OEM_ERROR_5 = 505,
                        r#OEM_ERROR_6 = 506,
                        r#OEM_ERROR_7 = 507,
                        r#OEM_ERROR_8 = 508,
                        r#OEM_ERROR_9 = 509,
                        r#OEM_ERROR_10 = 510,
                        r#OEM_ERROR_11 = 511,
                        r#OEM_ERROR_12 = 512,
                        r#OEM_ERROR_13 = 513,
                        r#OEM_ERROR_14 = 514,
                        r#OEM_ERROR_15 = 515,
                        r#OEM_ERROR_16 = 516,
                        r#OEM_ERROR_17 = 517,
                        r#OEM_ERROR_18 = 518,
                        r#OEM_ERROR_19 = 519,
                        r#OEM_ERROR_20 = 520,
                        r#OEM_ERROR_21 = 521,
                        r#OEM_ERROR_22 = 522,
                        r#OEM_ERROR_23 = 523,
                        r#OEM_ERROR_24 = 524,
                        r#OEM_ERROR_25 = 525,
                        r#SIMULTANEOUS_SMS_AND_CALL_NOT_ALLOWED = 67,
                        r#ACCESS_BARRED = 68,
                        r#BLOCKED_DUE_TO_CALL = 69,
                        r#RF_HARDWARE_ISSUE = 70,
                        r#NO_RF_CALIBRATION_INFO = 71,
                      }
                    }
                    pub(crate) mod mangled {
                        pub use super::r#RadioError as _7_android_8_hardware_5_radio_10_RadioError;
                    }
                }
                pub mod RadioIndicationType {
                    #![forbid(unsafe_code)]
                    #![allow(non_upper_case_globals)]
                    use binder::declare_binder_enum;
                    declare_binder_enum! {
                      r#RadioIndicationType : [i32; 2] {
                        r#UNSOLICITED = 0,
                        r#UNSOLICITED_ACK_EXP = 1,
                      }
                    }
                    pub(crate) mod mangled {
                        pub use super::r#RadioIndicationType as _7_android_8_hardware_5_radio_19_RadioIndicationType;
                    }
                }
                pub mod RadioResponseInfo {
                    #![forbid(unsafe_code)]

                    #[derive(Debug)]
                    pub struct r#RadioResponseInfo {
                        pub r#type:
                            crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseType,
                        pub r#serial: i32,
                        pub r#error: crate::mangled::_7_android_8_hardware_5_radio_10_RadioError,
                    }
                    impl Default for r#RadioResponseInfo {
                        fn default() -> Self {
                            Self {
                                r#type: Default::default(),
                                r#serial: 0,
                                r#error: Default::default(),
                            }
                        }
                    }
                    impl binder::Parcelable for r#RadioResponseInfo {
                        fn write_to_parcel(
                            &self,
                            parcel: &mut binder::binder_impl::BorrowedParcel,
                        ) -> std::result::Result<(), binder::StatusCode> {
                            parcel.sized_write(|subparcel| {
                                subparcel.write(&self.r#type)?;
                                subparcel.write(&self.r#serial)?;
                                subparcel.write(&self.r#error)?;
                                Ok(())
                            })
                        }
                        fn read_from_parcel(
                            &mut self,
                            parcel: &binder::binder_impl::BorrowedParcel,
                        ) -> std::result::Result<(), binder::StatusCode> {
                            parcel.sized_read(|subparcel| {
                                if subparcel.has_more_data() {
                                    self.r#type = subparcel.read()?;
                                }
                                if subparcel.has_more_data() {
                                    self.r#serial = subparcel.read()?;
                                }
                                if subparcel.has_more_data() {
                                    self.r#error = subparcel.read()?;
                                }
                                Ok(())
                            })
                        }
                    }
                    binder::impl_serialize_for_parcelable!(r#RadioResponseInfo);
                    binder::impl_deserialize_for_parcelable!(r#RadioResponseInfo);
                    impl binder::binder_impl::ParcelableMetadata for r#RadioResponseInfo {
                        fn get_descriptor() -> &'static str {
                            "android.hardware.radio.RadioResponseInfo"
                        }
                        fn get_stability(&self) -> binder::binder_impl::Stability {
                            binder::binder_impl::Stability::Vintf
                        }
                    }
                    pub(crate) mod mangled {
                        pub use super::r#RadioResponseInfo as _7_android_8_hardware_5_radio_17_RadioResponseInfo;
                    }
                }
                pub mod RadioResponseInfoModem {
                    #![forbid(unsafe_code)]

                    #[derive(Debug)]
                    pub struct r#RadioResponseInfoModem {
                        pub r#type:
                            crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseType,
                        pub r#serial: i32,
                        pub r#error: crate::mangled::_7_android_8_hardware_5_radio_10_RadioError,
                        pub r#isEnabled: bool,
                    }
                    impl Default for r#RadioResponseInfoModem {
                        fn default() -> Self {
                            Self {
                                r#type: Default::default(),
                                r#serial: 0,
                                r#error: Default::default(),
                                r#isEnabled: false,
                            }
                        }
                    }
                    impl binder::Parcelable for r#RadioResponseInfoModem {
                        fn write_to_parcel(
                            &self,
                            parcel: &mut binder::binder_impl::BorrowedParcel,
                        ) -> std::result::Result<(), binder::StatusCode> {
                            parcel.sized_write(|subparcel| {
                                subparcel.write(&self.r#type)?;
                                subparcel.write(&self.r#serial)?;
                                subparcel.write(&self.r#error)?;
                                subparcel.write(&self.r#isEnabled)?;
                                Ok(())
                            })
                        }
                        fn read_from_parcel(
                            &mut self,
                            parcel: &binder::binder_impl::BorrowedParcel,
                        ) -> std::result::Result<(), binder::StatusCode> {
                            parcel.sized_read(|subparcel| {
                                if subparcel.has_more_data() {
                                    self.r#type = subparcel.read()?;
                                }
                                if subparcel.has_more_data() {
                                    self.r#serial = subparcel.read()?;
                                }
                                if subparcel.has_more_data() {
                                    self.r#error = subparcel.read()?;
                                }
                                if subparcel.has_more_data() {
                                    self.r#isEnabled = subparcel.read()?;
                                }
                                Ok(())
                            })
                        }
                    }
                    binder::impl_serialize_for_parcelable!(r#RadioResponseInfoModem);
                    binder::impl_deserialize_for_parcelable!(r#RadioResponseInfoModem);
                    impl binder::binder_impl::ParcelableMetadata for r#RadioResponseInfoModem {
                        fn get_descriptor() -> &'static str {
                            "android.hardware.radio.RadioResponseInfoModem"
                        }
                        fn get_stability(&self) -> binder::binder_impl::Stability {
                            binder::binder_impl::Stability::Vintf
                        }
                    }
                    pub(crate) mod mangled {
                        pub use super::r#RadioResponseInfoModem as _7_android_8_hardware_5_radio_22_RadioResponseInfoModem;
                    }
                }
                pub mod RadioResponseType {
                    #![forbid(unsafe_code)]
                    #![allow(non_upper_case_globals)]
                    use binder::declare_binder_enum;
                    declare_binder_enum! {
                      r#RadioResponseType : [i32; 3] {
                        r#SOLICITED = 0,
                        r#SOLICITED_ACK = 1,
                        r#SOLICITED_ACK_EXP = 2,
                      }
                    }
                    pub(crate) mod mangled {
                        pub use super::r#RadioResponseType as _7_android_8_hardware_5_radio_17_RadioResponseType;
                    }
                }
                pub mod RadioTechnology {
                    #![forbid(unsafe_code)]
                    #![allow(non_upper_case_globals)]
                    use binder::declare_binder_enum;
                    declare_binder_enum! {
                      r#RadioTechnology : [i32; 21] {
                        r#UNKNOWN = 0,
                        r#GPRS = 1,
                        r#EDGE = 2,
                        r#UMTS = 3,
                        r#IS95A = 4,
                        r#IS95B = 5,
                        r#ONE_X_RTT = 6,
                        r#EVDO_0 = 7,
                        r#EVDO_A = 8,
                        r#HSDPA = 9,
                        r#HSUPA = 10,
                        r#HSPA = 11,
                        r#EVDO_B = 12,
                        r#EHRPD = 13,
                        r#LTE = 14,
                        r#HSPAP = 15,
                        r#GSM = 16,
                        r#TD_SCDMA = 17,
                        r#IWLAN = 18,
                        r#LTE_CA = 19,
                        r#NR = 20,
                      }
                    }
                    pub(crate) mod mangled {
                        pub use super::r#RadioTechnology as _7_android_8_hardware_5_radio_15_RadioTechnology;
                    }
                }
                pub mod RadioTechnologyFamily {
                    #![forbid(unsafe_code)]
                    #![allow(non_upper_case_globals)]
                    use binder::declare_binder_enum;
                    declare_binder_enum! {
                      r#RadioTechnologyFamily : [i32; 2] {
                        r#THREE_GPP = 0,
                        r#THREE_GPP2 = 1,
                      }
                    }
                    pub(crate) mod mangled {
                        pub use super::r#RadioTechnologyFamily as _7_android_8_hardware_5_radio_21_RadioTechnologyFamily;
                    }
                }
            }
        }
    }
}
pub mod mangled {
    pub use super::aidl::android::hardware::radio::AccessNetwork::mangled::*;
    pub use super::aidl::android::hardware::radio::RadioAccessFamily::mangled::*;
    pub use super::aidl::android::hardware::radio::RadioConst::mangled::*;
    pub use super::aidl::android::hardware::radio::RadioError::mangled::*;
    pub use super::aidl::android::hardware::radio::RadioIndicationType::mangled::*;
    pub use super::aidl::android::hardware::radio::RadioResponseInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::RadioResponseInfoModem::mangled::*;
    pub use super::aidl::android::hardware::radio::RadioResponseType::mangled::*;
    pub use super::aidl::android::hardware::radio::RadioTechnology::mangled::*;
    pub use super::aidl::android::hardware::radio::RadioTechnologyFamily::mangled::*;
}
