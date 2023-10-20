#![feature(custom_inner_attributes)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod hardware {
      pub mod radio {
        pub mod data {
          pub mod ApnAuthType {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#ApnAuthType : [i32; 4] {
                r#NO_PAP_NO_CHAP = 0,
                r#PAP_NO_CHAP = 1,
                r#NO_PAP_CHAP = 2,
                r#PAP_CHAP = 3,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#ApnAuthType as _7_android_8_hardware_5_radio_4_data_11_ApnAuthType;
            }
          }
          pub mod ApnTypes {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#ApnTypes : [i32; 16] {
                r#NONE = 0,
                r#DEFAULT = 1,
                r#MMS = 2,
                r#SUPL = 4,
                r#DUN = 8,
                r#HIPRI = 16,
                r#FOTA = 32,
                r#IMS = 64,
                r#CBS = 128,
                r#IA = 256,
                r#EMERGENCY = 512,
                r#MCX = 1024,
                r#XCAP = 2048,
                r#VSIM = 4096,
                r#BIP = 8192,
                r#ENTERPRISE = 16384,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#ApnTypes as _7_android_8_hardware_5_radio_4_data_8_ApnTypes;
            }
          }
          pub mod DataCallFailCause {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#DataCallFailCause : [i32; 340] {
                r#NONE = 0,
                r#OPERATOR_BARRED = 8,
                r#NAS_SIGNALLING = 14,
                r#INSUFFICIENT_RESOURCES = 26,
                r#MISSING_UNKNOWN_APN = 27,
                r#UNKNOWN_PDP_ADDRESS_TYPE = 28,
                r#USER_AUTHENTICATION = 29,
                r#ACTIVATION_REJECT_GGSN = 30,
                r#ACTIVATION_REJECT_UNSPECIFIED = 31,
                r#SERVICE_OPTION_NOT_SUPPORTED = 32,
                r#SERVICE_OPTION_NOT_SUBSCRIBED = 33,
                r#SERVICE_OPTION_OUT_OF_ORDER = 34,
                r#NSAPI_IN_USE = 35,
                r#REGULAR_DEACTIVATION = 36,
                r#QOS_NOT_ACCEPTED = 37,
                r#NETWORK_FAILURE = 38,
                r#UMTS_REACTIVATION_REQ = 39,
                r#FEATURE_NOT_SUPP = 40,
                r#TFT_SEMANTIC_ERROR = 41,
                r#TFT_SYTAX_ERROR = 42,
                r#UNKNOWN_PDP_CONTEXT = 43,
                r#FILTER_SEMANTIC_ERROR = 44,
                r#FILTER_SYTAX_ERROR = 45,
                r#PDP_WITHOUT_ACTIVE_TFT = 46,
                r#ONLY_IPV4_ALLOWED = 50,
                r#ONLY_IPV6_ALLOWED = 51,
                r#ONLY_SINGLE_BEARER_ALLOWED = 52,
                r#ESM_INFO_NOT_RECEIVED = 53,
                r#PDN_CONN_DOES_NOT_EXIST = 54,
                r#MULTI_CONN_TO_SAME_PDN_NOT_ALLOWED = 55,
                r#MAX_ACTIVE_PDP_CONTEXT_REACHED = 65,
                r#UNSUPPORTED_APN_IN_CURRENT_PLMN = 66,
                r#INVALID_TRANSACTION_ID = 81,
                r#MESSAGE_INCORRECT_SEMANTIC = 95,
                r#INVALID_MANDATORY_INFO = 96,
                r#MESSAGE_TYPE_UNSUPPORTED = 97,
                r#MSG_TYPE_NONCOMPATIBLE_STATE = 98,
                r#UNKNOWN_INFO_ELEMENT = 99,
                r#CONDITIONAL_IE_ERROR = 100,
                r#MSG_AND_PROTOCOL_STATE_UNCOMPATIBLE = 101,
                r#PROTOCOL_ERRORS = 111,
                r#APN_TYPE_CONFLICT = 112,
                r#INVALID_PCSCF_ADDR = 113,
                r#INTERNAL_CALL_PREEMPT_BY_HIGH_PRIO_APN = 114,
                r#EMM_ACCESS_BARRED = 115,
                r#EMERGENCY_IFACE_ONLY = 116,
                r#IFACE_MISMATCH = 117,
                r#COMPANION_IFACE_IN_USE = 118,
                r#IP_ADDRESS_MISMATCH = 119,
                r#IFACE_AND_POL_FAMILY_MISMATCH = 120,
                r#EMM_ACCESS_BARRED_INFINITE_RETRY = 121,
                r#AUTH_FAILURE_ON_EMERGENCY_CALL = 122,
                r#OEM_DCFAILCAUSE_1 = 4097,
                r#OEM_DCFAILCAUSE_2 = 4098,
                r#OEM_DCFAILCAUSE_3 = 4099,
                r#OEM_DCFAILCAUSE_4 = 4100,
                r#OEM_DCFAILCAUSE_5 = 4101,
                r#OEM_DCFAILCAUSE_6 = 4102,
                r#OEM_DCFAILCAUSE_7 = 4103,
                r#OEM_DCFAILCAUSE_8 = 4104,
                r#OEM_DCFAILCAUSE_9 = 4105,
                r#OEM_DCFAILCAUSE_10 = 4106,
                r#OEM_DCFAILCAUSE_11 = 4107,
                r#OEM_DCFAILCAUSE_12 = 4108,
                r#OEM_DCFAILCAUSE_13 = 4109,
                r#OEM_DCFAILCAUSE_14 = 4110,
                r#OEM_DCFAILCAUSE_15 = 4111,
                r#VOICE_REGISTRATION_FAIL = -1,
                r#DATA_REGISTRATION_FAIL = -2,
                r#SIGNAL_LOST = -3,
                r#PREF_RADIO_TECH_CHANGED = -4,
                r#RADIO_POWER_OFF = -5,
                r#TETHERED_CALL_ACTIVE = -6,
                r#ERROR_UNSPECIFIED = 65535,
                r#LLC_SNDCP = 25,
                r#ACTIVATION_REJECTED_BCM_VIOLATION = 48,
                r#COLLISION_WITH_NETWORK_INITIATED_REQUEST = 56,
                r#ONLY_IPV4V6_ALLOWED = 57,
                r#ONLY_NON_IP_ALLOWED = 58,
                r#UNSUPPORTED_QCI_VALUE = 59,
                r#BEARER_HANDLING_NOT_SUPPORTED = 60,
                r#INVALID_DNS_ADDR = 123,
                r#INVALID_PCSCF_OR_DNS_ADDRESS = 124,
                r#CALL_PREEMPT_BY_EMERGENCY_APN = 127,
                r#UE_INITIATED_DETACH_OR_DISCONNECT = 128,
                r#MIP_FA_REASON_UNSPECIFIED = 2000,
                r#MIP_FA_ADMIN_PROHIBITED = 2001,
                r#MIP_FA_INSUFFICIENT_RESOURCES = 2002,
                r#MIP_FA_MOBILE_NODE_AUTHENTICATION_FAILURE = 2003,
                r#MIP_FA_HOME_AGENT_AUTHENTICATION_FAILURE = 2004,
                r#MIP_FA_REQUESTED_LIFETIME_TOO_LONG = 2005,
                r#MIP_FA_MALFORMED_REQUEST = 2006,
                r#MIP_FA_MALFORMED_REPLY = 2007,
                r#MIP_FA_ENCAPSULATION_UNAVAILABLE = 2008,
                r#MIP_FA_VJ_HEADER_COMPRESSION_UNAVAILABLE = 2009,
                r#MIP_FA_REVERSE_TUNNEL_UNAVAILABLE = 2010,
                r#MIP_FA_REVERSE_TUNNEL_IS_MANDATORY = 2011,
                r#MIP_FA_DELIVERY_STYLE_NOT_SUPPORTED = 2012,
                r#MIP_FA_MISSING_NAI = 2013,
                r#MIP_FA_MISSING_HOME_AGENT = 2014,
                r#MIP_FA_MISSING_HOME_ADDRESS = 2015,
                r#MIP_FA_UNKNOWN_CHALLENGE = 2016,
                r#MIP_FA_MISSING_CHALLENGE = 2017,
                r#MIP_FA_STALE_CHALLENGE = 2018,
                r#MIP_HA_REASON_UNSPECIFIED = 2019,
                r#MIP_HA_ADMIN_PROHIBITED = 2020,
                r#MIP_HA_INSUFFICIENT_RESOURCES = 2021,
                r#MIP_HA_MOBILE_NODE_AUTHENTICATION_FAILURE = 2022,
                r#MIP_HA_FOREIGN_AGENT_AUTHENTICATION_FAILURE = 2023,
                r#MIP_HA_REGISTRATION_ID_MISMATCH = 2024,
                r#MIP_HA_MALFORMED_REQUEST = 2025,
                r#MIP_HA_UNKNOWN_HOME_AGENT_ADDRESS = 2026,
                r#MIP_HA_REVERSE_TUNNEL_UNAVAILABLE = 2027,
                r#MIP_HA_REVERSE_TUNNEL_IS_MANDATORY = 2028,
                r#MIP_HA_ENCAPSULATION_UNAVAILABLE = 2029,
                r#CLOSE_IN_PROGRESS = 2030,
                r#NETWORK_INITIATED_TERMINATION = 2031,
                r#MODEM_APP_PREEMPTED = 2032,
                r#PDN_IPV4_CALL_DISALLOWED = 2033,
                r#PDN_IPV4_CALL_THROTTLED = 2034,
                r#PDN_IPV6_CALL_DISALLOWED = 2035,
                r#PDN_IPV6_CALL_THROTTLED = 2036,
                r#MODEM_RESTART = 2037,
                r#PDP_PPP_NOT_SUPPORTED = 2038,
                r#UNPREFERRED_RAT = 2039,
                r#PHYSICAL_LINK_CLOSE_IN_PROGRESS = 2040,
                r#APN_PENDING_HANDOVER = 2041,
                r#PROFILE_BEARER_INCOMPATIBLE = 2042,
                r#SIM_CARD_CHANGED = 2043,
                r#LOW_POWER_MODE_OR_POWERING_DOWN = 2044,
                r#APN_DISABLED = 2045,
                r#MAX_PPP_INACTIVITY_TIMER_EXPIRED = 2046,
                r#IPV6_ADDRESS_TRANSFER_FAILED = 2047,
                r#TRAT_SWAP_FAILED = 2048,
                r#EHRPD_TO_HRPD_FALLBACK = 2049,
                r#MIP_CONFIG_FAILURE = 2050,
                r#PDN_INACTIVITY_TIMER_EXPIRED = 2051,
                r#MAX_IPV4_CONNECTIONS = 2052,
                r#MAX_IPV6_CONNECTIONS = 2053,
                r#APN_MISMATCH = 2054,
                r#IP_VERSION_MISMATCH = 2055,
                r#DUN_CALL_DISALLOWED = 2056,
                r#INTERNAL_EPC_NONEPC_TRANSITION = 2057,
                r#INTERFACE_IN_USE = 2058,
                r#APN_DISALLOWED_ON_ROAMING = 2059,
                r#APN_PARAMETERS_CHANGED = 2060,
                r#NULL_APN_DISALLOWED = 2061,
                r#THERMAL_MITIGATION = 2062,
                r#DATA_SETTINGS_DISABLED = 2063,
                r#DATA_ROAMING_SETTINGS_DISABLED = 2064,
                r#DDS_SWITCHED = 2065,
                r#FORBIDDEN_APN_NAME = 2066,
                r#DDS_SWITCH_IN_PROGRESS = 2067,
                r#CALL_DISALLOWED_IN_ROAMING = 2068,
                r#NON_IP_NOT_SUPPORTED = 2069,
                r#PDN_NON_IP_CALL_THROTTLED = 2070,
                r#PDN_NON_IP_CALL_DISALLOWED = 2071,
                r#CDMA_LOCK = 2072,
                r#CDMA_INTERCEPT = 2073,
                r#CDMA_REORDER = 2074,
                r#CDMA_RELEASE_DUE_TO_SO_REJECTION = 2075,
                r#CDMA_INCOMING_CALL = 2076,
                r#CDMA_ALERT_STOP = 2077,
                r#CHANNEL_ACQUISITION_FAILURE = 2078,
                r#MAX_ACCESS_PROBE = 2079,
                r#CONCURRENT_SERVICE_NOT_SUPPORTED_BY_BASE_STATION = 2080,
                r#NO_RESPONSE_FROM_BASE_STATION = 2081,
                r#REJECTED_BY_BASE_STATION = 2082,
                r#CONCURRENT_SERVICES_INCOMPATIBLE = 2083,
                r#NO_CDMA_SERVICE = 2084,
                r#RUIM_NOT_PRESENT = 2085,
                r#CDMA_RETRY_ORDER = 2086,
                r#ACCESS_BLOCK = 2087,
                r#ACCESS_BLOCK_ALL = 2088,
                r#IS707B_MAX_ACCESS_PROBES = 2089,
                r#THERMAL_EMERGENCY = 2090,
                r#CONCURRENT_SERVICES_NOT_ALLOWED = 2091,
                r#INCOMING_CALL_REJECTED = 2092,
                r#NO_SERVICE_ON_GATEWAY = 2093,
                r#NO_GPRS_CONTEXT = 2094,
                r#ILLEGAL_MS = 2095,
                r#ILLEGAL_ME = 2096,
                r#GPRS_SERVICES_AND_NON_GPRS_SERVICES_NOT_ALLOWED = 2097,
                r#GPRS_SERVICES_NOT_ALLOWED = 2098,
                r#MS_IDENTITY_CANNOT_BE_DERIVED_BY_THE_NETWORK = 2099,
                r#IMPLICITLY_DETACHED = 2100,
                r#PLMN_NOT_ALLOWED = 2101,
                r#LOCATION_AREA_NOT_ALLOWED = 2102,
                r#GPRS_SERVICES_NOT_ALLOWED_IN_THIS_PLMN = 2103,
                r#PDP_DUPLICATE = 2104,
                r#UE_RAT_CHANGE = 2105,
                r#CONGESTION = 2106,
                r#NO_PDP_CONTEXT_ACTIVATED = 2107,
                r#ACCESS_CLASS_DSAC_REJECTION = 2108,
                r#PDP_ACTIVATE_MAX_RETRY_FAILED = 2109,
                r#RADIO_ACCESS_BEARER_FAILURE = 2110,
                r#ESM_UNKNOWN_EPS_BEARER_CONTEXT = 2111,
                r#DRB_RELEASED_BY_RRC = 2112,
                r#CONNECTION_RELEASED = 2113,
                r#EMM_DETACHED = 2114,
                r#EMM_ATTACH_FAILED = 2115,
                r#EMM_ATTACH_STARTED = 2116,
                r#LTE_NAS_SERVICE_REQUEST_FAILED = 2117,
                r#DUPLICATE_BEARER_ID = 2118,
                r#ESM_COLLISION_SCENARIOS = 2119,
                r#ESM_BEARER_DEACTIVATED_TO_SYNC_WITH_NETWORK = 2120,
                r#ESM_NW_ACTIVATED_DED_BEARER_WITH_ID_OF_DEF_BEARER = 2121,
                r#ESM_BAD_OTA_MESSAGE = 2122,
                r#ESM_DOWNLOAD_SERVER_REJECTED_THE_CALL = 2123,
                r#ESM_CONTEXT_TRANSFERRED_DUE_TO_IRAT = 2124,
                r#DS_EXPLICIT_DEACTIVATION = 2125,
                r#ESM_LOCAL_CAUSE_NONE = 2126,
                r#LTE_THROTTLING_NOT_REQUIRED = 2127,
                r#ACCESS_CONTROL_LIST_CHECK_FAILURE = 2128,
                r#SERVICE_NOT_ALLOWED_ON_PLMN = 2129,
                r#EMM_T3417_EXPIRED = 2130,
                r#EMM_T3417_EXT_EXPIRED = 2131,
                r#RRC_UPLINK_DATA_TRANSMISSION_FAILURE = 2132,
                r#RRC_UPLINK_DELIVERY_FAILED_DUE_TO_HANDOVER = 2133,
                r#RRC_UPLINK_CONNECTION_RELEASE = 2134,
                r#RRC_UPLINK_RADIO_LINK_FAILURE = 2135,
                r#RRC_UPLINK_ERROR_REQUEST_FROM_NAS = 2136,
                r#RRC_CONNECTION_ACCESS_STRATUM_FAILURE = 2137,
                r#RRC_CONNECTION_ANOTHER_PROCEDURE_IN_PROGRESS = 2138,
                r#RRC_CONNECTION_ACCESS_BARRED = 2139,
                r#RRC_CONNECTION_CELL_RESELECTION = 2140,
                r#RRC_CONNECTION_CONFIG_FAILURE = 2141,
                r#RRC_CONNECTION_TIMER_EXPIRED = 2142,
                r#RRC_CONNECTION_LINK_FAILURE = 2143,
                r#RRC_CONNECTION_CELL_NOT_CAMPED = 2144,
                r#RRC_CONNECTION_SYSTEM_INTERVAL_FAILURE = 2145,
                r#RRC_CONNECTION_REJECT_BY_NETWORK = 2146,
                r#RRC_CONNECTION_NORMAL_RELEASE = 2147,
                r#RRC_CONNECTION_RADIO_LINK_FAILURE = 2148,
                r#RRC_CONNECTION_REESTABLISHMENT_FAILURE = 2149,
                r#RRC_CONNECTION_OUT_OF_SERVICE_DURING_CELL_REGISTER = 2150,
                r#RRC_CONNECTION_ABORT_REQUEST = 2151,
                r#RRC_CONNECTION_SYSTEM_INFORMATION_BLOCK_READ_ERROR = 2152,
                r#NETWORK_INITIATED_DETACH_WITH_AUTO_REATTACH = 2153,
                r#NETWORK_INITIATED_DETACH_NO_AUTO_REATTACH = 2154,
                r#ESM_PROCEDURE_TIME_OUT = 2155,
                r#INVALID_CONNECTION_ID = 2156,
                r#MAXIMIUM_NSAPIS_EXCEEDED = 2157,
                r#INVALID_PRIMARY_NSAPI = 2158,
                r#CANNOT_ENCODE_OTA_MESSAGE = 2159,
                r#RADIO_ACCESS_BEARER_SETUP_FAILURE = 2160,
                r#PDP_ESTABLISH_TIMEOUT_EXPIRED = 2161,
                r#PDP_MODIFY_TIMEOUT_EXPIRED = 2162,
                r#PDP_INACTIVE_TIMEOUT_EXPIRED = 2163,
                r#PDP_LOWERLAYER_ERROR = 2164,
                r#PDP_MODIFY_COLLISION = 2165,
                r#MAXINUM_SIZE_OF_L2_MESSAGE_EXCEEDED = 2166,
                r#NAS_REQUEST_REJECTED_BY_NETWORK = 2167,
                r#RRC_CONNECTION_INVALID_REQUEST = 2168,
                r#RRC_CONNECTION_TRACKING_AREA_ID_CHANGED = 2169,
                r#RRC_CONNECTION_RF_UNAVAILABLE = 2170,
                r#RRC_CONNECTION_ABORTED_DUE_TO_IRAT_CHANGE = 2171,
                r#RRC_CONNECTION_RELEASED_SECURITY_NOT_ACTIVE = 2172,
                r#RRC_CONNECTION_ABORTED_AFTER_HANDOVER = 2173,
                r#RRC_CONNECTION_ABORTED_AFTER_IRAT_CELL_CHANGE = 2174,
                r#RRC_CONNECTION_ABORTED_DURING_IRAT_CELL_CHANGE = 2175,
                r#IMSI_UNKNOWN_IN_HOME_SUBSCRIBER_SERVER = 2176,
                r#IMEI_NOT_ACCEPTED = 2177,
                r#EPS_SERVICES_AND_NON_EPS_SERVICES_NOT_ALLOWED = 2178,
                r#EPS_SERVICES_NOT_ALLOWED_IN_PLMN = 2179,
                r#MSC_TEMPORARILY_NOT_REACHABLE = 2180,
                r#CS_DOMAIN_NOT_AVAILABLE = 2181,
                r#ESM_FAILURE = 2182,
                r#MAC_FAILURE = 2183,
                r#SYNCHRONIZATION_FAILURE = 2184,
                r#UE_SECURITY_CAPABILITIES_MISMATCH = 2185,
                r#SECURITY_MODE_REJECTED = 2186,
                r#UNACCEPTABLE_NON_EPS_AUTHENTICATION = 2187,
                r#CS_FALLBACK_CALL_ESTABLISHMENT_NOT_ALLOWED = 2188,
                r#NO_EPS_BEARER_CONTEXT_ACTIVATED = 2189,
                r#INVALID_EMM_STATE = 2190,
                r#NAS_LAYER_FAILURE = 2191,
                r#MULTIPLE_PDP_CALL_NOT_ALLOWED = 2192,
                r#EMBMS_NOT_ENABLED = 2193,
                r#IRAT_HANDOVER_FAILED = 2194,
                r#EMBMS_REGULAR_DEACTIVATION = 2195,
                r#TEST_LOOPBACK_REGULAR_DEACTIVATION = 2196,
                r#LOWER_LAYER_REGISTRATION_FAILURE = 2197,
                r#DATA_PLAN_EXPIRED = 2198,
                r#UMTS_HANDOVER_TO_IWLAN = 2199,
                r#EVDO_CONNECTION_DENY_BY_GENERAL_OR_NETWORK_BUSY = 2200,
                r#EVDO_CONNECTION_DENY_BY_BILLING_OR_AUTHENTICATION_FAILURE = 2201,
                r#EVDO_HDR_CHANGED = 2202,
                r#EVDO_HDR_EXITED = 2203,
                r#EVDO_HDR_NO_SESSION = 2204,
                r#EVDO_USING_GPS_FIX_INSTEAD_OF_HDR_CALL = 2205,
                r#EVDO_HDR_CONNECTION_SETUP_TIMEOUT = 2206,
                r#FAILED_TO_ACQUIRE_COLOCATED_HDR = 2207,
                r#OTASP_COMMIT_IN_PROGRESS = 2208,
                r#NO_HYBRID_HDR_SERVICE = 2209,
                r#HDR_NO_LOCK_GRANTED = 2210,
                r#DBM_OR_SMS_IN_PROGRESS = 2211,
                r#HDR_FADE = 2212,
                r#HDR_ACCESS_FAILURE = 2213,
                r#UNSUPPORTED_1X_PREV = 2214,
                r#LOCAL_END = 2215,
                r#NO_SERVICE = 2216,
                r#FADE = 2217,
                r#NORMAL_RELEASE = 2218,
                r#ACCESS_ATTEMPT_ALREADY_IN_PROGRESS = 2219,
                r#REDIRECTION_OR_HANDOFF_IN_PROGRESS = 2220,
                r#EMERGENCY_MODE = 2221,
                r#PHONE_IN_USE = 2222,
                r#INVALID_MODE = 2223,
                r#INVALID_SIM_STATE = 2224,
                r#NO_COLLOCATED_HDR = 2225,
                r#UE_IS_ENTERING_POWERSAVE_MODE = 2226,
                r#DUAL_SWITCH = 2227,
                r#PPP_TIMEOUT = 2228,
                r#PPP_AUTH_FAILURE = 2229,
                r#PPP_OPTION_MISMATCH = 2230,
                r#PPP_PAP_FAILURE = 2231,
                r#PPP_CHAP_FAILURE = 2232,
                r#PPP_CLOSE_IN_PROGRESS = 2233,
                r#LIMITED_TO_IPV4 = 2234,
                r#LIMITED_TO_IPV6 = 2235,
                r#VSNCP_TIMEOUT = 2236,
                r#VSNCP_GEN_ERROR = 2237,
                r#VSNCP_APN_UNAUTHORIZED = 2238,
                r#VSNCP_PDN_LIMIT_EXCEEDED = 2239,
                r#VSNCP_NO_PDN_GATEWAY_ADDRESS = 2240,
                r#VSNCP_PDN_GATEWAY_UNREACHABLE = 2241,
                r#VSNCP_PDN_GATEWAY_REJECT = 2242,
                r#VSNCP_INSUFFICIENT_PARAMETERS = 2243,
                r#VSNCP_RESOURCE_UNAVAILABLE = 2244,
                r#VSNCP_ADMINISTRATIVELY_PROHIBITED = 2245,
                r#VSNCP_PDN_ID_IN_USE = 2246,
                r#VSNCP_SUBSCRIBER_LIMITATION = 2247,
                r#VSNCP_PDN_EXISTS_FOR_THIS_APN = 2248,
                r#VSNCP_RECONNECT_NOT_ALLOWED = 2249,
                r#IPV6_PREFIX_UNAVAILABLE = 2250,
                r#HANDOFF_PREFERENCE_CHANGED = 2251,
                r#SLICE_REJECTED = 2252,
                r#MATCH_ALL_RULE_NOT_ALLOWED = 2253,
                r#ALL_MATCHING_RULES_FAILED = 2254,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#DataCallFailCause as _7_android_8_hardware_5_radio_4_data_17_DataCallFailCause;
            }
          }
          pub mod DataProfileInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#DataProfileInfo {
              pub r#profileId: i32,
              pub r#apn: String,
              pub r#protocol: crate::mangled::_7_android_8_hardware_5_radio_4_data_15_PdpProtocolType,
              pub r#roamingProtocol: crate::mangled::_7_android_8_hardware_5_radio_4_data_15_PdpProtocolType,
              pub r#authType: crate::mangled::_7_android_8_hardware_5_radio_4_data_11_ApnAuthType,
              pub r#user: String,
              pub r#password: String,
              pub r#type: i32,
              pub r#maxConnsTime: i32,
              pub r#maxConns: i32,
              pub r#waitTime: i32,
              pub r#enabled: bool,
              pub r#supportedApnTypesBitmap: i32,
              pub r#bearerBitmap: i32,
              pub r#mtuV4: i32,
              pub r#mtuV6: i32,
              pub r#preferred: bool,
              pub r#persistent: bool,
              pub r#alwaysOn: bool,
              pub r#trafficDescriptor: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_TrafficDescriptor,
            }
            pub const r#ID_DEFAULT: i32 = 0;
            pub const r#ID_TETHERED: i32 = 1;
            pub const r#ID_IMS: i32 = 2;
            pub const r#ID_FOTA: i32 = 3;
            pub const r#ID_CBS: i32 = 4;
            pub const r#ID_OEM_BASE: i32 = 1000;
            pub const r#ID_INVALID: i32 = -1;
            pub const r#TYPE_COMMON: i32 = 0;
            pub const r#TYPE_3GPP: i32 = 1;
            pub const r#TYPE_3GPP2: i32 = 2;
            impl Default for r#DataProfileInfo {
              fn default() -> Self {
                Self {
                  r#profileId: 0,
                  r#apn: Default::default(),
                  r#protocol: Default::default(),
                  r#roamingProtocol: Default::default(),
                  r#authType: Default::default(),
                  r#user: Default::default(),
                  r#password: Default::default(),
                  r#type: 0,
                  r#maxConnsTime: 0,
                  r#maxConns: 0,
                  r#waitTime: 0,
                  r#enabled: false,
                  r#supportedApnTypesBitmap: 0,
                  r#bearerBitmap: 0,
                  r#mtuV4: 0,
                  r#mtuV6: 0,
                  r#preferred: false,
                  r#persistent: false,
                  r#alwaysOn: false,
                  r#trafficDescriptor: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#DataProfileInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#profileId)?;
                  subparcel.write(&self.r#apn)?;
                  subparcel.write(&self.r#protocol)?;
                  subparcel.write(&self.r#roamingProtocol)?;
                  subparcel.write(&self.r#authType)?;
                  subparcel.write(&self.r#user)?;
                  subparcel.write(&self.r#password)?;
                  subparcel.write(&self.r#type)?;
                  subparcel.write(&self.r#maxConnsTime)?;
                  subparcel.write(&self.r#maxConns)?;
                  subparcel.write(&self.r#waitTime)?;
                  subparcel.write(&self.r#enabled)?;
                  subparcel.write(&self.r#supportedApnTypesBitmap)?;
                  subparcel.write(&self.r#bearerBitmap)?;
                  subparcel.write(&self.r#mtuV4)?;
                  subparcel.write(&self.r#mtuV6)?;
                  subparcel.write(&self.r#preferred)?;
                  subparcel.write(&self.r#persistent)?;
                  subparcel.write(&self.r#alwaysOn)?;
                  subparcel.write(&self.r#trafficDescriptor)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#profileId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#apn = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#protocol = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#roamingProtocol = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#authType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#user = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#password = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#type = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxConnsTime = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxConns = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#waitTime = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#enabled = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#supportedApnTypesBitmap = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bearerBitmap = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mtuV4 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mtuV6 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#preferred = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#persistent = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#alwaysOn = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#trafficDescriptor = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#DataProfileInfo);
            binder::impl_deserialize_for_parcelable!(r#DataProfileInfo);
            impl binder::binder_impl::ParcelableMetadata for r#DataProfileInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.DataProfileInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#DataProfileInfo as _7_android_8_hardware_5_radio_4_data_15_DataProfileInfo;
            }
          }
          pub mod DataRequestReason {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#DataRequestReason : [i32; 3] {
                r#NORMAL = 1,
                r#SHUTDOWN = 2,
                r#HANDOVER = 3,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#DataRequestReason as _7_android_8_hardware_5_radio_4_data_17_DataRequestReason;
            }
          }
          pub mod DataThrottlingAction {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#DataThrottlingAction : [i8; 4] {
                r#NO_DATA_THROTTLING = 0,
                r#THROTTLE_SECONDARY_CARRIER = 1,
                r#THROTTLE_ANCHOR_CARRIER = 2,
                r#HOLD = 3,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#DataThrottlingAction as _7_android_8_hardware_5_radio_4_data_20_DataThrottlingAction;
            }
          }
          pub mod EpsQos {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#EpsQos {
              pub r#qci: i32,
              pub r#downlink: crate::mangled::_7_android_8_hardware_5_radio_4_data_12_QosBandwidth,
              pub r#uplink: crate::mangled::_7_android_8_hardware_5_radio_4_data_12_QosBandwidth,
            }
            impl Default for r#EpsQos {
              fn default() -> Self {
                Self {
                  r#qci: 0,
                  r#downlink: Default::default(),
                  r#uplink: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#EpsQos {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#qci)?;
                  subparcel.write(&self.r#downlink)?;
                  subparcel.write(&self.r#uplink)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#qci = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#downlink = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#uplink = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#EpsQos);
            binder::impl_deserialize_for_parcelable!(r#EpsQos);
            impl binder::binder_impl::ParcelableMetadata for r#EpsQos {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.EpsQos" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#EpsQos as _7_android_8_hardware_5_radio_4_data_6_EpsQos;
            }
          }
          pub mod IRadioData {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioData["android.hardware.radio.data.IRadioData"] {
                native: BnRadioData(on_transact),
                proxy: BpRadioData {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioDataAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioData: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.data.IRadioData" }
              fn r#allocatePduSessionId(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#cancelHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<()>;
              fn r#deactivateDataCall(&self, _arg_serial: i32, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason) -> binder::Result<()>;
              fn r#getDataCallList(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getSlicingConfig(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#releasePduSessionId(&self, _arg_serial: i32, _arg_id: i32) -> binder::Result<()>;
              fn r#responseAcknowledgement(&self) -> binder::Result<()>;
              fn r#setDataAllowed(&self, _arg_serial: i32, _arg_allow: bool) -> binder::Result<()>;
              fn r#setDataProfile(&self, _arg_serial: i32, _arg_profiles: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo]) -> binder::Result<()>;
              fn r#setDataThrottling(&self, _arg_serial: i32, _arg_dataThrottlingAction: crate::mangled::_7_android_8_hardware_5_radio_4_data_20_DataThrottlingAction, _arg_completionDurationMillis: i64) -> binder::Result<()>;
              fn r#setInitialAttachApn(&self, _arg_serial: i32, _arg_dataProfileInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo>) -> binder::Result<()>;
              fn r#setResponseFunctions(&self, _arg_radioDataResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_18_IRadioDataResponse>, _arg_radioDataIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_20_IRadioDataIndication>) -> binder::Result<()>;
              fn r#setupDataCall(&self, _arg_serial: i32, _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo, _arg_roamingAllowed: bool, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason, _arg_addresses: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_11_LinkAddress], _arg_dnses: &[String], _arg_pduSessionId: i32, _arg_sliceInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>, _arg_matchAllRuleAllowed: bool) -> binder::Result<()>;
              fn r#startHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<()>;
              fn r#startKeepalive(&self, _arg_serial: i32, _arg_keepalive: &crate::mangled::_7_android_8_hardware_5_radio_4_data_16_KeepaliveRequest) -> binder::Result<()>;
              fn r#stopKeepalive(&self, _arg_serial: i32, _arg_sessionHandle: i32) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioDataDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioDataDefaultRef) -> IRadioDataDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioDataAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.data.IRadioData" }
              fn r#allocatePduSessionId(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#cancelHandover(&self, _arg_serial: i32, _arg_callId: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#deactivateDataCall(&self, _arg_serial: i32, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason) -> std::future::Ready<binder::Result<()>>;
              fn r#getDataCallList(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getSlicingConfig(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#releasePduSessionId(&self, _arg_serial: i32, _arg_id: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#responseAcknowledgement(&self) -> std::future::Ready<binder::Result<()>>;
              fn r#setDataAllowed(&self, _arg_serial: i32, _arg_allow: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#setDataProfile(&self, _arg_serial: i32, _arg_profiles: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo]) -> std::future::Ready<binder::Result<()>>;
              fn r#setDataThrottling(&self, _arg_serial: i32, _arg_dataThrottlingAction: crate::mangled::_7_android_8_hardware_5_radio_4_data_20_DataThrottlingAction, _arg_completionDurationMillis: i64) -> std::future::Ready<binder::Result<()>>;
              fn r#setInitialAttachApn(&self, _arg_serial: i32, _arg_dataProfileInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo>) -> std::future::Ready<binder::Result<()>>;
              fn r#setResponseFunctions(&self, _arg_radioDataResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_18_IRadioDataResponse>, _arg_radioDataIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_20_IRadioDataIndication>) -> std::future::Ready<binder::Result<()>>;
              fn r#setupDataCall(&self, _arg_serial: i32, _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo, _arg_roamingAllowed: bool, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason, _arg_addresses: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_11_LinkAddress], _arg_dnses: &[String], _arg_pduSessionId: i32, _arg_sliceInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>, _arg_matchAllRuleAllowed: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#startHandover(&self, _arg_serial: i32, _arg_callId: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#startKeepalive(&self, _arg_serial: i32, _arg_keepalive: &crate::mangled::_7_android_8_hardware_5_radio_4_data_16_KeepaliveRequest) -> std::future::Ready<binder::Result<()>>;
              fn r#stopKeepalive(&self, _arg_serial: i32, _arg_sessionHandle: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioDataAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.data.IRadioData" }
              async fn r#allocatePduSessionId(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#cancelHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<()>;
              async fn r#deactivateDataCall(&self, _arg_serial: i32, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason) -> binder::Result<()>;
              async fn r#getDataCallList(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getSlicingConfig(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#releasePduSessionId(&self, _arg_serial: i32, _arg_id: i32) -> binder::Result<()>;
              async fn r#responseAcknowledgement(&self) -> binder::Result<()>;
              async fn r#setDataAllowed(&self, _arg_serial: i32, _arg_allow: bool) -> binder::Result<()>;
              async fn r#setDataProfile(&self, _arg_serial: i32, _arg_profiles: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo]) -> binder::Result<()>;
              async fn r#setDataThrottling(&self, _arg_serial: i32, _arg_dataThrottlingAction: crate::mangled::_7_android_8_hardware_5_radio_4_data_20_DataThrottlingAction, _arg_completionDurationMillis: i64) -> binder::Result<()>;
              async fn r#setInitialAttachApn(&self, _arg_serial: i32, _arg_dataProfileInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo>) -> binder::Result<()>;
              async fn r#setResponseFunctions(&self, _arg_radioDataResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_18_IRadioDataResponse>, _arg_radioDataIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_20_IRadioDataIndication>) -> binder::Result<()>;
              async fn r#setupDataCall(&self, _arg_serial: i32, _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo, _arg_roamingAllowed: bool, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason, _arg_addresses: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_11_LinkAddress], _arg_dnses: &[String], _arg_pduSessionId: i32, _arg_sliceInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>, _arg_matchAllRuleAllowed: bool) -> binder::Result<()>;
              async fn r#startHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<()>;
              async fn r#startKeepalive(&self, _arg_serial: i32, _arg_keepalive: &crate::mangled::_7_android_8_hardware_5_radio_4_data_16_KeepaliveRequest) -> binder::Result<()>;
              async fn r#stopKeepalive(&self, _arg_serial: i32, _arg_sessionHandle: i32) -> binder::Result<()>;
            }
            impl BnRadioData {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioData>
              where
                T: IRadioDataAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioData for Wrapper<T, R>
                where
                  T: IRadioDataAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#allocatePduSessionId(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#allocatePduSessionId(_arg_serial))
                  }
                  fn r#cancelHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#cancelHandover(_arg_serial, _arg_callId))
                  }
                  fn r#deactivateDataCall(&self, _arg_serial: i32, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#deactivateDataCall(_arg_serial, _arg_cid, _arg_reason))
                  }
                  fn r#getDataCallList(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getDataCallList(_arg_serial))
                  }
                  fn r#getSlicingConfig(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSlicingConfig(_arg_serial))
                  }
                  fn r#releasePduSessionId(&self, _arg_serial: i32, _arg_id: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#releasePduSessionId(_arg_serial, _arg_id))
                  }
                  fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#responseAcknowledgement())
                  }
                  fn r#setDataAllowed(&self, _arg_serial: i32, _arg_allow: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setDataAllowed(_arg_serial, _arg_allow))
                  }
                  fn r#setDataProfile(&self, _arg_serial: i32, _arg_profiles: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setDataProfile(_arg_serial, _arg_profiles))
                  }
                  fn r#setDataThrottling(&self, _arg_serial: i32, _arg_dataThrottlingAction: crate::mangled::_7_android_8_hardware_5_radio_4_data_20_DataThrottlingAction, _arg_completionDurationMillis: i64) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setDataThrottling(_arg_serial, _arg_dataThrottlingAction, _arg_completionDurationMillis))
                  }
                  fn r#setInitialAttachApn(&self, _arg_serial: i32, _arg_dataProfileInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo>) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setInitialAttachApn(_arg_serial, _arg_dataProfileInfo))
                  }
                  fn r#setResponseFunctions(&self, _arg_radioDataResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_18_IRadioDataResponse>, _arg_radioDataIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_20_IRadioDataIndication>) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setResponseFunctions(_arg_radioDataResponse, _arg_radioDataIndication))
                  }
                  fn r#setupDataCall(&self, _arg_serial: i32, _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo, _arg_roamingAllowed: bool, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason, _arg_addresses: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_11_LinkAddress], _arg_dnses: &[String], _arg_pduSessionId: i32, _arg_sliceInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>, _arg_matchAllRuleAllowed: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setupDataCall(_arg_serial, _arg_accessNetwork, _arg_dataProfileInfo, _arg_roamingAllowed, _arg_reason, _arg_addresses, _arg_dnses, _arg_pduSessionId, _arg_sliceInfo, _arg_matchAllRuleAllowed))
                  }
                  fn r#startHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#startHandover(_arg_serial, _arg_callId))
                  }
                  fn r#startKeepalive(&self, _arg_serial: i32, _arg_keepalive: &crate::mangled::_7_android_8_hardware_5_radio_4_data_16_KeepaliveRequest) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#startKeepalive(_arg_serial, _arg_keepalive))
                  }
                  fn r#stopKeepalive(&self, _arg_serial: i32, _arg_sessionHandle: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#stopKeepalive(_arg_serial, _arg_sessionHandle))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioDataDefault: Send + Sync {
              fn r#allocatePduSessionId(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#cancelHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#deactivateDataCall(&self, _arg_serial: i32, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDataCallList(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSlicingConfig(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#releasePduSessionId(&self, _arg_serial: i32, _arg_id: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setDataAllowed(&self, _arg_serial: i32, _arg_allow: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setDataProfile(&self, _arg_serial: i32, _arg_profiles: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setDataThrottling(&self, _arg_serial: i32, _arg_dataThrottlingAction: crate::mangled::_7_android_8_hardware_5_radio_4_data_20_DataThrottlingAction, _arg_completionDurationMillis: i64) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setInitialAttachApn(&self, _arg_serial: i32, _arg_dataProfileInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo>) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setResponseFunctions(&self, _arg_radioDataResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_18_IRadioDataResponse>, _arg_radioDataIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_20_IRadioDataIndication>) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setupDataCall(&self, _arg_serial: i32, _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo, _arg_roamingAllowed: bool, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason, _arg_addresses: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_11_LinkAddress], _arg_dnses: &[String], _arg_pduSessionId: i32, _arg_sliceInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>, _arg_matchAllRuleAllowed: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#startHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#startKeepalive(&self, _arg_serial: i32, _arg_keepalive: &crate::mangled::_7_android_8_hardware_5_radio_4_data_16_KeepaliveRequest) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#stopKeepalive(&self, _arg_serial: i32, _arg_sessionHandle: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#allocatePduSessionId: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#cancelHandover: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#deactivateDataCall: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#getDataCallList: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#getSlicingConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#releasePduSessionId: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#responseAcknowledgement: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#setDataAllowed: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#setDataProfile: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#setDataThrottling: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#setInitialAttachApn: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#setResponseFunctions: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#setupDataCall: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#startHandover: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#startKeepalive: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#stopKeepalive: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioDataDefaultRef = Option<std::sync::Arc<dyn IRadioDataDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioDataDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "6d7a86008ea4fe79ced2a86b526a92618eb4c84a";
            impl BpRadioData {
              fn build_parcel_allocatePduSessionId(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_allocatePduSessionId(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#allocatePduSessionId(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_cancelHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_callId)?;
                Ok(aidl_data)
              }
              fn read_response_cancelHandover(&self, _arg_serial: i32, _arg_callId: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#cancelHandover(_arg_serial, _arg_callId);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_deactivateDataCall(&self, _arg_serial: i32, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_cid)?;
                aidl_data.write(&_arg_reason)?;
                Ok(aidl_data)
              }
              fn read_response_deactivateDataCall(&self, _arg_serial: i32, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#deactivateDataCall(_arg_serial, _arg_cid, _arg_reason);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getDataCallList(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getDataCallList(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDataCallList(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSlicingConfig(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getSlicingConfig(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSlicingConfig(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_releasePduSessionId(&self, _arg_serial: i32, _arg_id: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_id)?;
                Ok(aidl_data)
              }
              fn read_response_releasePduSessionId(&self, _arg_serial: i32, _arg_id: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#releasePduSessionId(_arg_serial, _arg_id);
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
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#responseAcknowledgement();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setDataAllowed(&self, _arg_serial: i32, _arg_allow: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_allow)?;
                Ok(aidl_data)
              }
              fn read_response_setDataAllowed(&self, _arg_serial: i32, _arg_allow: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#setDataAllowed(_arg_serial, _arg_allow);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setDataProfile(&self, _arg_serial: i32, _arg_profiles: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_profiles)?;
                Ok(aidl_data)
              }
              fn read_response_setDataProfile(&self, _arg_serial: i32, _arg_profiles: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#setDataProfile(_arg_serial, _arg_profiles);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setDataThrottling(&self, _arg_serial: i32, _arg_dataThrottlingAction: crate::mangled::_7_android_8_hardware_5_radio_4_data_20_DataThrottlingAction, _arg_completionDurationMillis: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_dataThrottlingAction)?;
                aidl_data.write(&_arg_completionDurationMillis)?;
                Ok(aidl_data)
              }
              fn read_response_setDataThrottling(&self, _arg_serial: i32, _arg_dataThrottlingAction: crate::mangled::_7_android_8_hardware_5_radio_4_data_20_DataThrottlingAction, _arg_completionDurationMillis: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#setDataThrottling(_arg_serial, _arg_dataThrottlingAction, _arg_completionDurationMillis);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setInitialAttachApn(&self, _arg_serial: i32, _arg_dataProfileInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_dataProfileInfo)?;
                Ok(aidl_data)
              }
              fn read_response_setInitialAttachApn(&self, _arg_serial: i32, _arg_dataProfileInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#setInitialAttachApn(_arg_serial, _arg_dataProfileInfo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setResponseFunctions(&self, _arg_radioDataResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_18_IRadioDataResponse>, _arg_radioDataIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_20_IRadioDataIndication>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_radioDataResponse)?;
                aidl_data.write(_arg_radioDataIndication)?;
                Ok(aidl_data)
              }
              fn read_response_setResponseFunctions(&self, _arg_radioDataResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_18_IRadioDataResponse>, _arg_radioDataIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_20_IRadioDataIndication>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#setResponseFunctions(_arg_radioDataResponse, _arg_radioDataIndication);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setupDataCall(&self, _arg_serial: i32, _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo, _arg_roamingAllowed: bool, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason, _arg_addresses: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_11_LinkAddress], _arg_dnses: &[String], _arg_pduSessionId: i32, _arg_sliceInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>, _arg_matchAllRuleAllowed: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_accessNetwork)?;
                aidl_data.write(_arg_dataProfileInfo)?;
                aidl_data.write(&_arg_roamingAllowed)?;
                aidl_data.write(&_arg_reason)?;
                aidl_data.write(_arg_addresses)?;
                aidl_data.write(_arg_dnses)?;
                aidl_data.write(&_arg_pduSessionId)?;
                aidl_data.write(&_arg_sliceInfo)?;
                aidl_data.write(&_arg_matchAllRuleAllowed)?;
                Ok(aidl_data)
              }
              fn read_response_setupDataCall(&self, _arg_serial: i32, _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo, _arg_roamingAllowed: bool, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason, _arg_addresses: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_11_LinkAddress], _arg_dnses: &[String], _arg_pduSessionId: i32, _arg_sliceInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>, _arg_matchAllRuleAllowed: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#setupDataCall(_arg_serial, _arg_accessNetwork, _arg_dataProfileInfo, _arg_roamingAllowed, _arg_reason, _arg_addresses, _arg_dnses, _arg_pduSessionId, _arg_sliceInfo, _arg_matchAllRuleAllowed);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_startHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_callId)?;
                Ok(aidl_data)
              }
              fn read_response_startHandover(&self, _arg_serial: i32, _arg_callId: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#startHandover(_arg_serial, _arg_callId);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_startKeepalive(&self, _arg_serial: i32, _arg_keepalive: &crate::mangled::_7_android_8_hardware_5_radio_4_data_16_KeepaliveRequest) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_keepalive)?;
                Ok(aidl_data)
              }
              fn read_response_startKeepalive(&self, _arg_serial: i32, _arg_keepalive: &crate::mangled::_7_android_8_hardware_5_radio_4_data_16_KeepaliveRequest, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#startKeepalive(_arg_serial, _arg_keepalive);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_stopKeepalive(&self, _arg_serial: i32, _arg_sessionHandle: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_sessionHandle)?;
                Ok(aidl_data)
              }
              fn read_response_stopKeepalive(&self, _arg_serial: i32, _arg_sessionHandle: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioData>::getDefaultImpl() {
                    return _aidl_default_impl.r#stopKeepalive(_arg_serial, _arg_sessionHandle);
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
            impl IRadioData for BpRadioData {
              fn r#allocatePduSessionId(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_allocatePduSessionId(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#allocatePduSessionId, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_allocatePduSessionId(_arg_serial, _aidl_reply)
              }
              fn r#cancelHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_cancelHandover(_arg_serial, _arg_callId)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#cancelHandover, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_cancelHandover(_arg_serial, _arg_callId, _aidl_reply)
              }
              fn r#deactivateDataCall(&self, _arg_serial: i32, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_deactivateDataCall(_arg_serial, _arg_cid, _arg_reason)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#deactivateDataCall, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_deactivateDataCall(_arg_serial, _arg_cid, _arg_reason, _aidl_reply)
              }
              fn r#getDataCallList(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getDataCallList(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDataCallList, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDataCallList(_arg_serial, _aidl_reply)
              }
              fn r#getSlicingConfig(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSlicingConfig(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSlicingConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSlicingConfig(_arg_serial, _aidl_reply)
              }
              fn r#releasePduSessionId(&self, _arg_serial: i32, _arg_id: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_releasePduSessionId(_arg_serial, _arg_id)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#releasePduSessionId, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_releasePduSessionId(_arg_serial, _arg_id, _aidl_reply)
              }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_responseAcknowledgement()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#responseAcknowledgement, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_responseAcknowledgement(_aidl_reply)
              }
              fn r#setDataAllowed(&self, _arg_serial: i32, _arg_allow: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setDataAllowed(_arg_serial, _arg_allow)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDataAllowed, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setDataAllowed(_arg_serial, _arg_allow, _aidl_reply)
              }
              fn r#setDataProfile(&self, _arg_serial: i32, _arg_profiles: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setDataProfile(_arg_serial, _arg_profiles)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDataProfile, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setDataProfile(_arg_serial, _arg_profiles, _aidl_reply)
              }
              fn r#setDataThrottling(&self, _arg_serial: i32, _arg_dataThrottlingAction: crate::mangled::_7_android_8_hardware_5_radio_4_data_20_DataThrottlingAction, _arg_completionDurationMillis: i64) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setDataThrottling(_arg_serial, _arg_dataThrottlingAction, _arg_completionDurationMillis)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDataThrottling, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setDataThrottling(_arg_serial, _arg_dataThrottlingAction, _arg_completionDurationMillis, _aidl_reply)
              }
              fn r#setInitialAttachApn(&self, _arg_serial: i32, _arg_dataProfileInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo>) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setInitialAttachApn(_arg_serial, _arg_dataProfileInfo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setInitialAttachApn, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setInitialAttachApn(_arg_serial, _arg_dataProfileInfo, _aidl_reply)
              }
              fn r#setResponseFunctions(&self, _arg_radioDataResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_18_IRadioDataResponse>, _arg_radioDataIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_20_IRadioDataIndication>) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setResponseFunctions(_arg_radioDataResponse, _arg_radioDataIndication)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setResponseFunctions(_arg_radioDataResponse, _arg_radioDataIndication, _aidl_reply)
              }
              fn r#setupDataCall(&self, _arg_serial: i32, _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo, _arg_roamingAllowed: bool, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason, _arg_addresses: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_11_LinkAddress], _arg_dnses: &[String], _arg_pduSessionId: i32, _arg_sliceInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>, _arg_matchAllRuleAllowed: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setupDataCall(_arg_serial, _arg_accessNetwork, _arg_dataProfileInfo, _arg_roamingAllowed, _arg_reason, _arg_addresses, _arg_dnses, _arg_pduSessionId, _arg_sliceInfo, _arg_matchAllRuleAllowed)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setupDataCall, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setupDataCall(_arg_serial, _arg_accessNetwork, _arg_dataProfileInfo, _arg_roamingAllowed, _arg_reason, _arg_addresses, _arg_dnses, _arg_pduSessionId, _arg_sliceInfo, _arg_matchAllRuleAllowed, _aidl_reply)
              }
              fn r#startHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_startHandover(_arg_serial, _arg_callId)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#startHandover, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_startHandover(_arg_serial, _arg_callId, _aidl_reply)
              }
              fn r#startKeepalive(&self, _arg_serial: i32, _arg_keepalive: &crate::mangled::_7_android_8_hardware_5_radio_4_data_16_KeepaliveRequest) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_startKeepalive(_arg_serial, _arg_keepalive)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#startKeepalive, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_startKeepalive(_arg_serial, _arg_keepalive, _aidl_reply)
              }
              fn r#stopKeepalive(&self, _arg_serial: i32, _arg_sessionHandle: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_stopKeepalive(_arg_serial, _arg_sessionHandle)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#stopKeepalive, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_stopKeepalive(_arg_serial, _arg_sessionHandle, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioDataAsync<P> for BpRadioData {
              fn r#allocatePduSessionId(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_allocatePduSessionId(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#allocatePduSessionId, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_allocatePduSessionId(_arg_serial, _aidl_reply))
              }
              fn r#cancelHandover(&self, _arg_serial: i32, _arg_callId: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_cancelHandover(_arg_serial, _arg_callId) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#cancelHandover, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_cancelHandover(_arg_serial, _arg_callId, _aidl_reply))
              }
              fn r#deactivateDataCall(&self, _arg_serial: i32, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_deactivateDataCall(_arg_serial, _arg_cid, _arg_reason) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#deactivateDataCall, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_deactivateDataCall(_arg_serial, _arg_cid, _arg_reason, _aidl_reply))
              }
              fn r#getDataCallList(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getDataCallList(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDataCallList, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getDataCallList(_arg_serial, _aidl_reply))
              }
              fn r#getSlicingConfig(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSlicingConfig(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSlicingConfig, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSlicingConfig(_arg_serial, _aidl_reply))
              }
              fn r#releasePduSessionId(&self, _arg_serial: i32, _arg_id: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_releasePduSessionId(_arg_serial, _arg_id) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#releasePduSessionId, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_releasePduSessionId(_arg_serial, _arg_id, _aidl_reply))
              }
              fn r#responseAcknowledgement(&self) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_responseAcknowledgement() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#responseAcknowledgement, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_responseAcknowledgement(_aidl_reply))
              }
              fn r#setDataAllowed(&self, _arg_serial: i32, _arg_allow: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setDataAllowed(_arg_serial, _arg_allow) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDataAllowed, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setDataAllowed(_arg_serial, _arg_allow, _aidl_reply))
              }
              fn r#setDataProfile(&self, _arg_serial: i32, _arg_profiles: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setDataProfile(_arg_serial, _arg_profiles) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDataProfile, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setDataProfile(_arg_serial, _arg_profiles, _aidl_reply))
              }
              fn r#setDataThrottling(&self, _arg_serial: i32, _arg_dataThrottlingAction: crate::mangled::_7_android_8_hardware_5_radio_4_data_20_DataThrottlingAction, _arg_completionDurationMillis: i64) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setDataThrottling(_arg_serial, _arg_dataThrottlingAction, _arg_completionDurationMillis) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDataThrottling, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setDataThrottling(_arg_serial, _arg_dataThrottlingAction, _arg_completionDurationMillis, _aidl_reply))
              }
              fn r#setInitialAttachApn(&self, _arg_serial: i32, _arg_dataProfileInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo>) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setInitialAttachApn(_arg_serial, _arg_dataProfileInfo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setInitialAttachApn, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setInitialAttachApn(_arg_serial, _arg_dataProfileInfo, _aidl_reply))
              }
              fn r#setResponseFunctions(&self, _arg_radioDataResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_18_IRadioDataResponse>, _arg_radioDataIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_20_IRadioDataIndication>) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setResponseFunctions(_arg_radioDataResponse, _arg_radioDataIndication) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setResponseFunctions(_arg_radioDataResponse, _arg_radioDataIndication, _aidl_reply))
              }
              fn r#setupDataCall(&self, _arg_serial: i32, _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo, _arg_roamingAllowed: bool, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason, _arg_addresses: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_11_LinkAddress], _arg_dnses: &[String], _arg_pduSessionId: i32, _arg_sliceInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>, _arg_matchAllRuleAllowed: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setupDataCall(_arg_serial, _arg_accessNetwork, _arg_dataProfileInfo, _arg_roamingAllowed, _arg_reason, _arg_addresses, _arg_dnses, _arg_pduSessionId, _arg_sliceInfo, _arg_matchAllRuleAllowed) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setupDataCall, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setupDataCall(_arg_serial, _arg_accessNetwork, _arg_dataProfileInfo, _arg_roamingAllowed, _arg_reason, _arg_addresses, _arg_dnses, _arg_pduSessionId, _arg_sliceInfo, _arg_matchAllRuleAllowed, _aidl_reply))
              }
              fn r#startHandover(&self, _arg_serial: i32, _arg_callId: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_startHandover(_arg_serial, _arg_callId) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#startHandover, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_startHandover(_arg_serial, _arg_callId, _aidl_reply))
              }
              fn r#startKeepalive(&self, _arg_serial: i32, _arg_keepalive: &crate::mangled::_7_android_8_hardware_5_radio_4_data_16_KeepaliveRequest) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_startKeepalive(_arg_serial, _arg_keepalive) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#startKeepalive, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_startKeepalive(_arg_serial, _arg_keepalive, _aidl_reply))
              }
              fn r#stopKeepalive(&self, _arg_serial: i32, _arg_sessionHandle: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_stopKeepalive(_arg_serial, _arg_sessionHandle) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#stopKeepalive, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_stopKeepalive(_arg_serial, _arg_sessionHandle, _aidl_reply))
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
            impl IRadioData for binder::binder_impl::Binder<BnRadioData> {
              fn r#allocatePduSessionId(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#allocatePduSessionId(_arg_serial) }
              fn r#cancelHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<()> { self.0.r#cancelHandover(_arg_serial, _arg_callId) }
              fn r#deactivateDataCall(&self, _arg_serial: i32, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason) -> binder::Result<()> { self.0.r#deactivateDataCall(_arg_serial, _arg_cid, _arg_reason) }
              fn r#getDataCallList(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getDataCallList(_arg_serial) }
              fn r#getSlicingConfig(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getSlicingConfig(_arg_serial) }
              fn r#releasePduSessionId(&self, _arg_serial: i32, _arg_id: i32) -> binder::Result<()> { self.0.r#releasePduSessionId(_arg_serial, _arg_id) }
              fn r#responseAcknowledgement(&self) -> binder::Result<()> { self.0.r#responseAcknowledgement() }
              fn r#setDataAllowed(&self, _arg_serial: i32, _arg_allow: bool) -> binder::Result<()> { self.0.r#setDataAllowed(_arg_serial, _arg_allow) }
              fn r#setDataProfile(&self, _arg_serial: i32, _arg_profiles: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo]) -> binder::Result<()> { self.0.r#setDataProfile(_arg_serial, _arg_profiles) }
              fn r#setDataThrottling(&self, _arg_serial: i32, _arg_dataThrottlingAction: crate::mangled::_7_android_8_hardware_5_radio_4_data_20_DataThrottlingAction, _arg_completionDurationMillis: i64) -> binder::Result<()> { self.0.r#setDataThrottling(_arg_serial, _arg_dataThrottlingAction, _arg_completionDurationMillis) }
              fn r#setInitialAttachApn(&self, _arg_serial: i32, _arg_dataProfileInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo>) -> binder::Result<()> { self.0.r#setInitialAttachApn(_arg_serial, _arg_dataProfileInfo) }
              fn r#setResponseFunctions(&self, _arg_radioDataResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_18_IRadioDataResponse>, _arg_radioDataIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_20_IRadioDataIndication>) -> binder::Result<()> { self.0.r#setResponseFunctions(_arg_radioDataResponse, _arg_radioDataIndication) }
              fn r#setupDataCall(&self, _arg_serial: i32, _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo, _arg_roamingAllowed: bool, _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason, _arg_addresses: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_11_LinkAddress], _arg_dnses: &[String], _arg_pduSessionId: i32, _arg_sliceInfo: Option<&crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>, _arg_matchAllRuleAllowed: bool) -> binder::Result<()> { self.0.r#setupDataCall(_arg_serial, _arg_accessNetwork, _arg_dataProfileInfo, _arg_roamingAllowed, _arg_reason, _arg_addresses, _arg_dnses, _arg_pduSessionId, _arg_sliceInfo, _arg_matchAllRuleAllowed) }
              fn r#startHandover(&self, _arg_serial: i32, _arg_callId: i32) -> binder::Result<()> { self.0.r#startHandover(_arg_serial, _arg_callId) }
              fn r#startKeepalive(&self, _arg_serial: i32, _arg_keepalive: &crate::mangled::_7_android_8_hardware_5_radio_4_data_16_KeepaliveRequest) -> binder::Result<()> { self.0.r#startKeepalive(_arg_serial, _arg_keepalive) }
              fn r#stopKeepalive(&self, _arg_serial: i32, _arg_sessionHandle: i32) -> binder::Result<()> { self.0.r#stopKeepalive(_arg_serial, _arg_sessionHandle) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioData, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#allocatePduSessionId => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#allocatePduSessionId(_arg_serial);
                  Ok(())
                }
                transactions::r#cancelHandover => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_callId: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#cancelHandover(_arg_serial, _arg_callId);
                  Ok(())
                }
                transactions::r#deactivateDataCall => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_cid: i32 = _aidl_data.read()?;
                  let _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#deactivateDataCall(_arg_serial, _arg_cid, _arg_reason);
                  Ok(())
                }
                transactions::r#getDataCallList => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDataCallList(_arg_serial);
                  Ok(())
                }
                transactions::r#getSlicingConfig => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSlicingConfig(_arg_serial);
                  Ok(())
                }
                transactions::r#releasePduSessionId => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_id: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#releasePduSessionId(_arg_serial, _arg_id);
                  Ok(())
                }
                transactions::r#responseAcknowledgement => {
                  let _aidl_return = _aidl_service.r#responseAcknowledgement();
                  Ok(())
                }
                transactions::r#setDataAllowed => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_allow: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setDataAllowed(_arg_serial, _arg_allow);
                  Ok(())
                }
                transactions::r#setDataProfile => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_profiles: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setDataProfile(_arg_serial, &_arg_profiles);
                  Ok(())
                }
                transactions::r#setDataThrottling => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_dataThrottlingAction: crate::mangled::_7_android_8_hardware_5_radio_4_data_20_DataThrottlingAction = _aidl_data.read()?;
                  let _arg_completionDurationMillis: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setDataThrottling(_arg_serial, _arg_dataThrottlingAction, _arg_completionDurationMillis);
                  Ok(())
                }
                transactions::r#setInitialAttachApn => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_dataProfileInfo: Option<crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setInitialAttachApn(_arg_serial, _arg_dataProfileInfo.as_ref());
                  Ok(())
                }
                transactions::r#setResponseFunctions => {
                  let _arg_radioDataResponse: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_18_IRadioDataResponse> = _aidl_data.read()?;
                  let _arg_radioDataIndication: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_4_data_20_IRadioDataIndication> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setResponseFunctions(&_arg_radioDataResponse, &_arg_radioDataIndication);
                  Ok(())
                }
                transactions::r#setupDataCall => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_accessNetwork: crate::mangled::_7_android_8_hardware_5_radio_13_AccessNetwork = _aidl_data.read()?;
                  let _arg_dataProfileInfo: crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo = _aidl_data.read()?;
                  let _arg_roamingAllowed: bool = _aidl_data.read()?;
                  let _arg_reason: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataRequestReason = _aidl_data.read()?;
                  let _arg_addresses: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_11_LinkAddress> = _aidl_data.read()?;
                  let _arg_dnses: Vec<String> = _aidl_data.read()?;
                  let _arg_pduSessionId: i32 = _aidl_data.read()?;
                  let _arg_sliceInfo: Option<crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo> = _aidl_data.read()?;
                  let _arg_matchAllRuleAllowed: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setupDataCall(_arg_serial, _arg_accessNetwork, &_arg_dataProfileInfo, _arg_roamingAllowed, _arg_reason, &_arg_addresses, &_arg_dnses, _arg_pduSessionId, _arg_sliceInfo.as_ref(), _arg_matchAllRuleAllowed);
                  Ok(())
                }
                transactions::r#startHandover => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_callId: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#startHandover(_arg_serial, _arg_callId);
                  Ok(())
                }
                transactions::r#startKeepalive => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_keepalive: crate::mangled::_7_android_8_hardware_5_radio_4_data_16_KeepaliveRequest = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#startKeepalive(_arg_serial, &_arg_keepalive);
                  Ok(())
                }
                transactions::r#stopKeepalive => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_sessionHandle: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#stopKeepalive(_arg_serial, _arg_sessionHandle);
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
             pub use super::r#IRadioData as _7_android_8_hardware_5_radio_4_data_10_IRadioData;
            }
          }
          pub mod IRadioDataIndication {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioDataIndication["android.hardware.radio.data.IRadioDataIndication"] {
                native: BnRadioDataIndication(on_transact),
                proxy: BpRadioDataIndication {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioDataIndicationAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioDataIndication: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.data.IRadioDataIndication" }
              fn r#dataCallListChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dcList: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<()>;
              fn r#keepaliveStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<()>;
              fn r#pcoData(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pco: &crate::mangled::_7_android_8_hardware_5_radio_4_data_11_PcoDataInfo) -> binder::Result<()>;
              fn r#unthrottleApn(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo) -> binder::Result<()>;
              fn r#slicingConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioDataIndicationDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioDataIndicationDefaultRef) -> IRadioDataIndicationDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioDataIndicationAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.data.IRadioDataIndication" }
              fn r#dataCallListChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dcList: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> std::future::Ready<binder::Result<()>>;
              fn r#keepaliveStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> std::future::Ready<binder::Result<()>>;
              fn r#pcoData(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pco: &crate::mangled::_7_android_8_hardware_5_radio_4_data_11_PcoDataInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#unthrottleApn(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#slicingConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioDataIndicationAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.data.IRadioDataIndication" }
              async fn r#dataCallListChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dcList: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<()>;
              async fn r#keepaliveStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<()>;
              async fn r#pcoData(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pco: &crate::mangled::_7_android_8_hardware_5_radio_4_data_11_PcoDataInfo) -> binder::Result<()>;
              async fn r#unthrottleApn(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo) -> binder::Result<()>;
              async fn r#slicingConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<()>;
            }
            impl BnRadioDataIndication {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioDataIndication>
              where
                T: IRadioDataIndicationAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioDataIndication for Wrapper<T, R>
                where
                  T: IRadioDataIndicationAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#dataCallListChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dcList: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#dataCallListChanged(_arg_type, _arg_dcList))
                  }
                  fn r#keepaliveStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#keepaliveStatus(_arg_type, _arg_status))
                  }
                  fn r#pcoData(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pco: &crate::mangled::_7_android_8_hardware_5_radio_4_data_11_PcoDataInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#pcoData(_arg_type, _arg_pco))
                  }
                  fn r#unthrottleApn(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#unthrottleApn(_arg_type, _arg_dataProfileInfo))
                  }
                  fn r#slicingConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#slicingConfigChanged(_arg_type, _arg_slicingConfig))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioDataIndicationDefault: Send + Sync {
              fn r#dataCallListChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dcList: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#keepaliveStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#pcoData(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pco: &crate::mangled::_7_android_8_hardware_5_radio_4_data_11_PcoDataInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#unthrottleApn(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#slicingConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#dataCallListChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#keepaliveStatus: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#pcoData: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#unthrottleApn: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#slicingConfigChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioDataIndicationDefaultRef = Option<std::sync::Arc<dyn IRadioDataIndicationDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioDataIndicationDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "6d7a86008ea4fe79ced2a86b526a92618eb4c84a";
            impl BpRadioDataIndication {
              fn build_parcel_dataCallListChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dcList: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_dcList)?;
                Ok(aidl_data)
              }
              fn read_response_dataCallListChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dcList: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#dataCallListChanged(_arg_type, _arg_dcList);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_keepaliveStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_status)?;
                Ok(aidl_data)
              }
              fn read_response_keepaliveStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#keepaliveStatus(_arg_type, _arg_status);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_pcoData(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pco: &crate::mangled::_7_android_8_hardware_5_radio_4_data_11_PcoDataInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_pco)?;
                Ok(aidl_data)
              }
              fn read_response_pcoData(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pco: &crate::mangled::_7_android_8_hardware_5_radio_4_data_11_PcoDataInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#pcoData(_arg_type, _arg_pco);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_unthrottleApn(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_dataProfileInfo)?;
                Ok(aidl_data)
              }
              fn read_response_unthrottleApn(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#unthrottleApn(_arg_type, _arg_dataProfileInfo);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_slicingConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_slicingConfig)?;
                Ok(aidl_data)
              }
              fn read_response_slicingConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#slicingConfigChanged(_arg_type, _arg_slicingConfig);
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
            impl IRadioDataIndication for BpRadioDataIndication {
              fn r#dataCallListChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dcList: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_dataCallListChanged(_arg_type, _arg_dcList)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#dataCallListChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_dataCallListChanged(_arg_type, _arg_dcList, _aidl_reply)
              }
              fn r#keepaliveStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_keepaliveStatus(_arg_type, _arg_status)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#keepaliveStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_keepaliveStatus(_arg_type, _arg_status, _aidl_reply)
              }
              fn r#pcoData(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pco: &crate::mangled::_7_android_8_hardware_5_radio_4_data_11_PcoDataInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_pcoData(_arg_type, _arg_pco)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#pcoData, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_pcoData(_arg_type, _arg_pco, _aidl_reply)
              }
              fn r#unthrottleApn(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_unthrottleApn(_arg_type, _arg_dataProfileInfo)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#unthrottleApn, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_unthrottleApn(_arg_type, _arg_dataProfileInfo, _aidl_reply)
              }
              fn r#slicingConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_slicingConfigChanged(_arg_type, _arg_slicingConfig)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#slicingConfigChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_slicingConfigChanged(_arg_type, _arg_slicingConfig, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioDataIndicationAsync<P> for BpRadioDataIndication {
              fn r#dataCallListChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dcList: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_dataCallListChanged(_arg_type, _arg_dcList) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#dataCallListChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_dataCallListChanged(_arg_type, _arg_dcList, _aidl_reply))
              }
              fn r#keepaliveStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_keepaliveStatus(_arg_type, _arg_status) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#keepaliveStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_keepaliveStatus(_arg_type, _arg_status, _aidl_reply))
              }
              fn r#pcoData(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pco: &crate::mangled::_7_android_8_hardware_5_radio_4_data_11_PcoDataInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_pcoData(_arg_type, _arg_pco) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#pcoData, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_pcoData(_arg_type, _arg_pco, _aidl_reply))
              }
              fn r#unthrottleApn(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_unthrottleApn(_arg_type, _arg_dataProfileInfo) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#unthrottleApn, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_unthrottleApn(_arg_type, _arg_dataProfileInfo, _aidl_reply))
              }
              fn r#slicingConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_slicingConfigChanged(_arg_type, _arg_slicingConfig) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#slicingConfigChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_slicingConfigChanged(_arg_type, _arg_slicingConfig, _aidl_reply))
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
            impl IRadioDataIndication for binder::binder_impl::Binder<BnRadioDataIndication> {
              fn r#dataCallListChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dcList: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<()> { self.0.r#dataCallListChanged(_arg_type, _arg_dcList) }
              fn r#keepaliveStatus(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<()> { self.0.r#keepaliveStatus(_arg_type, _arg_status) }
              fn r#pcoData(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_pco: &crate::mangled::_7_android_8_hardware_5_radio_4_data_11_PcoDataInfo) -> binder::Result<()> { self.0.r#pcoData(_arg_type, _arg_pco) }
              fn r#unthrottleApn(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_dataProfileInfo: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo) -> binder::Result<()> { self.0.r#unthrottleApn(_arg_type, _arg_dataProfileInfo) }
              fn r#slicingConfigChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<()> { self.0.r#slicingConfigChanged(_arg_type, _arg_slicingConfig) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioDataIndication, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#dataCallListChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_dcList: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#dataCallListChanged(_arg_type, &_arg_dcList);
                  Ok(())
                }
                transactions::r#keepaliveStatus => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_status: crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#keepaliveStatus(_arg_type, &_arg_status);
                  Ok(())
                }
                transactions::r#pcoData => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_pco: crate::mangled::_7_android_8_hardware_5_radio_4_data_11_PcoDataInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#pcoData(_arg_type, &_arg_pco);
                  Ok(())
                }
                transactions::r#unthrottleApn => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_dataProfileInfo: crate::mangled::_7_android_8_hardware_5_radio_4_data_15_DataProfileInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#unthrottleApn(_arg_type, &_arg_dataProfileInfo);
                  Ok(())
                }
                transactions::r#slicingConfigChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_slicingConfig: crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#slicingConfigChanged(_arg_type, &_arg_slicingConfig);
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
             pub use super::r#IRadioDataIndication as _7_android_8_hardware_5_radio_4_data_20_IRadioDataIndication;
            }
          }
          pub mod IRadioDataResponse {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioDataResponse["android.hardware.radio.data.IRadioDataResponse"] {
                native: BnRadioDataResponse(on_transact),
                proxy: BpRadioDataResponse {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioDataResponseAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioDataResponse: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.data.IRadioDataResponse" }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#allocatePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_id: i32) -> binder::Result<()>;
              fn r#cancelHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#deactivateDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#getDataCallListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<()>;
              fn r#getSlicingConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<()>;
              fn r#releasePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setDataAllowedResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setDataProfileResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setDataThrottlingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setInitialAttachApnResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setupDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult) -> binder::Result<()>;
              fn r#startHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#startKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<()>;
              fn r#stopKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioDataResponseDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioDataResponseDefaultRef) -> IRadioDataResponseDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioDataResponseAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.data.IRadioDataResponse" }
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#allocatePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_id: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#cancelHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#deactivateDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getDataCallListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> std::future::Ready<binder::Result<()>>;
              fn r#getSlicingConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> std::future::Ready<binder::Result<()>>;
              fn r#releasePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setDataAllowedResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setDataProfileResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setDataThrottlingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setInitialAttachApnResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setupDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult) -> std::future::Ready<binder::Result<()>>;
              fn r#startHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#startKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> std::future::Ready<binder::Result<()>>;
              fn r#stopKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioDataResponseAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.data.IRadioDataResponse" }
              async fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#allocatePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_id: i32) -> binder::Result<()>;
              async fn r#cancelHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#deactivateDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#getDataCallListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<()>;
              async fn r#getSlicingConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<()>;
              async fn r#releasePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setDataAllowedResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setDataProfileResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setDataThrottlingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setInitialAttachApnResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setupDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult) -> binder::Result<()>;
              async fn r#startHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#startKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<()>;
              async fn r#stopKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
            }
            impl BnRadioDataResponse {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioDataResponse>
              where
                T: IRadioDataResponseAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioDataResponse for Wrapper<T, R>
                where
                  T: IRadioDataResponseAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#acknowledgeRequest(_arg_serial))
                  }
                  fn r#allocatePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_id: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#allocatePduSessionIdResponse(_arg_info, _arg_id))
                  }
                  fn r#cancelHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#cancelHandoverResponse(_arg_info))
                  }
                  fn r#deactivateDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#deactivateDataCallResponse(_arg_info))
                  }
                  fn r#getDataCallListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getDataCallListResponse(_arg_info, _arg_dcResponse))
                  }
                  fn r#getSlicingConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSlicingConfigResponse(_arg_info, _arg_slicingConfig))
                  }
                  fn r#releasePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#releasePduSessionIdResponse(_arg_info))
                  }
                  fn r#setDataAllowedResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setDataAllowedResponse(_arg_info))
                  }
                  fn r#setDataProfileResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setDataProfileResponse(_arg_info))
                  }
                  fn r#setDataThrottlingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setDataThrottlingResponse(_arg_info))
                  }
                  fn r#setInitialAttachApnResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setInitialAttachApnResponse(_arg_info))
                  }
                  fn r#setupDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setupDataCallResponse(_arg_info, _arg_dcResponse))
                  }
                  fn r#startHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#startHandoverResponse(_arg_info))
                  }
                  fn r#startKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#startKeepaliveResponse(_arg_info, _arg_status))
                  }
                  fn r#stopKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#stopKeepaliveResponse(_arg_info))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioDataResponseDefault: Send + Sync {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#allocatePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_id: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#cancelHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#deactivateDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDataCallListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSlicingConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#releasePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setDataAllowedResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setDataProfileResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setDataThrottlingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setInitialAttachApnResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setupDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#startHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#startKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#stopKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#acknowledgeRequest: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#allocatePduSessionIdResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#cancelHandoverResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#deactivateDataCallResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#getDataCallListResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#getSlicingConfigResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#releasePduSessionIdResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#setDataAllowedResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#setDataProfileResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#setDataThrottlingResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#setInitialAttachApnResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#setupDataCallResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#startHandoverResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#startKeepaliveResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#stopKeepaliveResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioDataResponseDefaultRef = Option<std::sync::Arc<dyn IRadioDataResponseDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioDataResponseDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "6d7a86008ea4fe79ced2a86b526a92618eb4c84a";
            impl BpRadioDataResponse {
              fn build_parcel_acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_acknowledgeRequest(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#acknowledgeRequest(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_allocatePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_id: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_id)?;
                Ok(aidl_data)
              }
              fn read_response_allocatePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_id: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#allocatePduSessionIdResponse(_arg_info, _arg_id);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_cancelHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_cancelHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#cancelHandoverResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_deactivateDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_deactivateDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#deactivateDataCallResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getDataCallListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_dcResponse)?;
                Ok(aidl_data)
              }
              fn read_response_getDataCallListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDataCallListResponse(_arg_info, _arg_dcResponse);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSlicingConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_slicingConfig)?;
                Ok(aidl_data)
              }
              fn read_response_getSlicingConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSlicingConfigResponse(_arg_info, _arg_slicingConfig);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_releasePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_releasePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#releasePduSessionIdResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setDataAllowedResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setDataAllowedResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setDataAllowedResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setDataProfileResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setDataProfileResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setDataProfileResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setDataThrottlingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setDataThrottlingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setDataThrottlingResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setInitialAttachApnResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setInitialAttachApnResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setInitialAttachApnResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setupDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_dcResponse)?;
                Ok(aidl_data)
              }
              fn read_response_setupDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setupDataCallResponse(_arg_info, _arg_dcResponse);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_startHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_startHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#startHandoverResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_startKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_status)?;
                Ok(aidl_data)
              }
              fn read_response_startKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#startKeepaliveResponse(_arg_info, _arg_status);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_stopKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_stopKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioDataResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#stopKeepaliveResponse(_arg_info);
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
            impl IRadioDataResponse for BpRadioDataResponse {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_acknowledgeRequest(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeRequest, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_acknowledgeRequest(_arg_serial, _aidl_reply)
              }
              fn r#allocatePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_id: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_allocatePduSessionIdResponse(_arg_info, _arg_id)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#allocatePduSessionIdResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_allocatePduSessionIdResponse(_arg_info, _arg_id, _aidl_reply)
              }
              fn r#cancelHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_cancelHandoverResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#cancelHandoverResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_cancelHandoverResponse(_arg_info, _aidl_reply)
              }
              fn r#deactivateDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_deactivateDataCallResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#deactivateDataCallResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_deactivateDataCallResponse(_arg_info, _aidl_reply)
              }
              fn r#getDataCallListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getDataCallListResponse(_arg_info, _arg_dcResponse)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDataCallListResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDataCallListResponse(_arg_info, _arg_dcResponse, _aidl_reply)
              }
              fn r#getSlicingConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSlicingConfigResponse(_arg_info, _arg_slicingConfig)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSlicingConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSlicingConfigResponse(_arg_info, _arg_slicingConfig, _aidl_reply)
              }
              fn r#releasePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_releasePduSessionIdResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#releasePduSessionIdResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_releasePduSessionIdResponse(_arg_info, _aidl_reply)
              }
              fn r#setDataAllowedResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setDataAllowedResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDataAllowedResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setDataAllowedResponse(_arg_info, _aidl_reply)
              }
              fn r#setDataProfileResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setDataProfileResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDataProfileResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setDataProfileResponse(_arg_info, _aidl_reply)
              }
              fn r#setDataThrottlingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setDataThrottlingResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDataThrottlingResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setDataThrottlingResponse(_arg_info, _aidl_reply)
              }
              fn r#setInitialAttachApnResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setInitialAttachApnResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setInitialAttachApnResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setInitialAttachApnResponse(_arg_info, _aidl_reply)
              }
              fn r#setupDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setupDataCallResponse(_arg_info, _arg_dcResponse)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setupDataCallResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setupDataCallResponse(_arg_info, _arg_dcResponse, _aidl_reply)
              }
              fn r#startHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_startHandoverResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#startHandoverResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_startHandoverResponse(_arg_info, _aidl_reply)
              }
              fn r#startKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_startKeepaliveResponse(_arg_info, _arg_status)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#startKeepaliveResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_startKeepaliveResponse(_arg_info, _arg_status, _aidl_reply)
              }
              fn r#stopKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_stopKeepaliveResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#stopKeepaliveResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_stopKeepaliveResponse(_arg_info, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioDataResponseAsync<P> for BpRadioDataResponse {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_acknowledgeRequest(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#acknowledgeRequest, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_acknowledgeRequest(_arg_serial, _aidl_reply))
              }
              fn r#allocatePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_id: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_allocatePduSessionIdResponse(_arg_info, _arg_id) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#allocatePduSessionIdResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_allocatePduSessionIdResponse(_arg_info, _arg_id, _aidl_reply))
              }
              fn r#cancelHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_cancelHandoverResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#cancelHandoverResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_cancelHandoverResponse(_arg_info, _aidl_reply))
              }
              fn r#deactivateDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_deactivateDataCallResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#deactivateDataCallResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_deactivateDataCallResponse(_arg_info, _aidl_reply))
              }
              fn r#getDataCallListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getDataCallListResponse(_arg_info, _arg_dcResponse) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDataCallListResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getDataCallListResponse(_arg_info, _arg_dcResponse, _aidl_reply))
              }
              fn r#getSlicingConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSlicingConfigResponse(_arg_info, _arg_slicingConfig) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSlicingConfigResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSlicingConfigResponse(_arg_info, _arg_slicingConfig, _aidl_reply))
              }
              fn r#releasePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_releasePduSessionIdResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#releasePduSessionIdResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_releasePduSessionIdResponse(_arg_info, _aidl_reply))
              }
              fn r#setDataAllowedResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setDataAllowedResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDataAllowedResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setDataAllowedResponse(_arg_info, _aidl_reply))
              }
              fn r#setDataProfileResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setDataProfileResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDataProfileResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setDataProfileResponse(_arg_info, _aidl_reply))
              }
              fn r#setDataThrottlingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setDataThrottlingResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDataThrottlingResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setDataThrottlingResponse(_arg_info, _aidl_reply))
              }
              fn r#setInitialAttachApnResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setInitialAttachApnResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setInitialAttachApnResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setInitialAttachApnResponse(_arg_info, _aidl_reply))
              }
              fn r#setupDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setupDataCallResponse(_arg_info, _arg_dcResponse) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setupDataCallResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setupDataCallResponse(_arg_info, _arg_dcResponse, _aidl_reply))
              }
              fn r#startHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_startHandoverResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#startHandoverResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_startHandoverResponse(_arg_info, _aidl_reply))
              }
              fn r#startKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_startKeepaliveResponse(_arg_info, _arg_status) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#startKeepaliveResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_startKeepaliveResponse(_arg_info, _arg_status, _aidl_reply))
              }
              fn r#stopKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_stopKeepaliveResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#stopKeepaliveResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_stopKeepaliveResponse(_arg_info, _aidl_reply))
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
            impl IRadioDataResponse for binder::binder_impl::Binder<BnRadioDataResponse> {
              fn r#acknowledgeRequest(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#acknowledgeRequest(_arg_serial) }
              fn r#allocatePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_id: i32) -> binder::Result<()> { self.0.r#allocatePduSessionIdResponse(_arg_info, _arg_id) }
              fn r#cancelHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#cancelHandoverResponse(_arg_info) }
              fn r#deactivateDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#deactivateDataCallResponse(_arg_info) }
              fn r#getDataCallListResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &[crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult]) -> binder::Result<()> { self.0.r#getDataCallListResponse(_arg_info, _arg_dcResponse) }
              fn r#getSlicingConfigResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slicingConfig: &crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig) -> binder::Result<()> { self.0.r#getSlicingConfigResponse(_arg_info, _arg_slicingConfig) }
              fn r#releasePduSessionIdResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#releasePduSessionIdResponse(_arg_info) }
              fn r#setDataAllowedResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setDataAllowedResponse(_arg_info) }
              fn r#setDataProfileResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setDataProfileResponse(_arg_info) }
              fn r#setDataThrottlingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setDataThrottlingResponse(_arg_info) }
              fn r#setInitialAttachApnResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setInitialAttachApnResponse(_arg_info) }
              fn r#setupDataCallResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_dcResponse: &crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult) -> binder::Result<()> { self.0.r#setupDataCallResponse(_arg_info, _arg_dcResponse) }
              fn r#startHandoverResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#startHandoverResponse(_arg_info) }
              fn r#startKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_status: &crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus) -> binder::Result<()> { self.0.r#startKeepaliveResponse(_arg_info, _arg_status) }
              fn r#stopKeepaliveResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#stopKeepaliveResponse(_arg_info) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioDataResponse, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#acknowledgeRequest => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#acknowledgeRequest(_arg_serial);
                  Ok(())
                }
                transactions::r#allocatePduSessionIdResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_id: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#allocatePduSessionIdResponse(&_arg_info, _arg_id);
                  Ok(())
                }
                transactions::r#cancelHandoverResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#cancelHandoverResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#deactivateDataCallResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#deactivateDataCallResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#getDataCallListResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_dcResponse: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDataCallListResponse(&_arg_info, &_arg_dcResponse);
                  Ok(())
                }
                transactions::r#getSlicingConfigResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_slicingConfig: crate::mangled::_7_android_8_hardware_5_radio_4_data_13_SlicingConfig = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSlicingConfigResponse(&_arg_info, &_arg_slicingConfig);
                  Ok(())
                }
                transactions::r#releasePduSessionIdResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#releasePduSessionIdResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setDataAllowedResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setDataAllowedResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setDataProfileResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setDataProfileResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setDataThrottlingResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setDataThrottlingResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setInitialAttachApnResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setInitialAttachApnResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setupDataCallResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_dcResponse: crate::mangled::_7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setupDataCallResponse(&_arg_info, &_arg_dcResponse);
                  Ok(())
                }
                transactions::r#startHandoverResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#startHandoverResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#startKeepaliveResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_status: crate::mangled::_7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#startKeepaliveResponse(&_arg_info, &_arg_status);
                  Ok(())
                }
                transactions::r#stopKeepaliveResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#stopKeepaliveResponse(&_arg_info);
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
             pub use super::r#IRadioDataResponse as _7_android_8_hardware_5_radio_4_data_18_IRadioDataResponse;
            }
          }
          pub mod KeepaliveRequest {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#KeepaliveRequest {
              pub r#type: i32,
              pub r#sourceAddress: Vec<u8>,
              pub r#sourcePort: i32,
              pub r#destinationAddress: Vec<u8>,
              pub r#destinationPort: i32,
              pub r#maxKeepaliveIntervalMillis: i32,
              pub r#cid: i32,
            }
            pub const r#TYPE_NATT_IPV4: i32 = 0;
            pub const r#TYPE_NATT_IPV6: i32 = 1;
            impl Default for r#KeepaliveRequest {
              fn default() -> Self {
                Self {
                  r#type: 0,
                  r#sourceAddress: Default::default(),
                  r#sourcePort: 0,
                  r#destinationAddress: Default::default(),
                  r#destinationPort: 0,
                  r#maxKeepaliveIntervalMillis: 0,
                  r#cid: 0,
                }
              }
            }
            impl binder::Parcelable for r#KeepaliveRequest {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#type)?;
                  subparcel.write(&self.r#sourceAddress)?;
                  subparcel.write(&self.r#sourcePort)?;
                  subparcel.write(&self.r#destinationAddress)?;
                  subparcel.write(&self.r#destinationPort)?;
                  subparcel.write(&self.r#maxKeepaliveIntervalMillis)?;
                  subparcel.write(&self.r#cid)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#type = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sourceAddress = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sourcePort = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#destinationAddress = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#destinationPort = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxKeepaliveIntervalMillis = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cid = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#KeepaliveRequest);
            binder::impl_deserialize_for_parcelable!(r#KeepaliveRequest);
            impl binder::binder_impl::ParcelableMetadata for r#KeepaliveRequest {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.KeepaliveRequest" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#KeepaliveRequest as _7_android_8_hardware_5_radio_4_data_16_KeepaliveRequest;
            }
          }
          pub mod KeepaliveStatus {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#KeepaliveStatus {
              pub r#sessionHandle: i32,
              pub r#code: i32,
            }
            pub const r#CODE_ACTIVE: i32 = 0;
            pub const r#CODE_INACTIVE: i32 = 1;
            pub const r#CODE_PENDING: i32 = 2;
            impl Default for r#KeepaliveStatus {
              fn default() -> Self {
                Self {
                  r#sessionHandle: 0,
                  r#code: 0,
                }
              }
            }
            impl binder::Parcelable for r#KeepaliveStatus {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#sessionHandle)?;
                  subparcel.write(&self.r#code)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#sessionHandle = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#code = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#KeepaliveStatus);
            binder::impl_deserialize_for_parcelable!(r#KeepaliveStatus);
            impl binder::binder_impl::ParcelableMetadata for r#KeepaliveStatus {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.KeepaliveStatus" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#KeepaliveStatus as _7_android_8_hardware_5_radio_4_data_15_KeepaliveStatus;
            }
          }
          pub mod LinkAddress {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#LinkAddress {
              pub r#address: String,
              pub r#addressProperties: i32,
              pub r#deprecationTime: i64,
              pub r#expirationTime: i64,
            }
            pub const r#ADDRESS_PROPERTY_NONE: i32 = 0;
            pub const r#ADDRESS_PROPERTY_DEPRECATED: i32 = 32;
            impl Default for r#LinkAddress {
              fn default() -> Self {
                Self {
                  r#address: Default::default(),
                  r#addressProperties: 0,
                  r#deprecationTime: 0,
                  r#expirationTime: 0,
                }
              }
            }
            impl binder::Parcelable for r#LinkAddress {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#address)?;
                  subparcel.write(&self.r#addressProperties)?;
                  subparcel.write(&self.r#deprecationTime)?;
                  subparcel.write(&self.r#expirationTime)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#address = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#addressProperties = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#deprecationTime = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#expirationTime = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#LinkAddress);
            binder::impl_deserialize_for_parcelable!(r#LinkAddress);
            impl binder::binder_impl::ParcelableMetadata for r#LinkAddress {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.LinkAddress" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#LinkAddress as _7_android_8_hardware_5_radio_4_data_11_LinkAddress;
            }
          }
          pub mod NrQos {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#NrQos {
              pub r#fiveQi: i32,
              pub r#downlink: crate::mangled::_7_android_8_hardware_5_radio_4_data_12_QosBandwidth,
              pub r#uplink: crate::mangled::_7_android_8_hardware_5_radio_4_data_12_QosBandwidth,
              pub r#qfi: i8,
              pub r#averagingWindowMs: u16,
            }
            pub const r#FLOW_ID_RANGE_MIN: i8 = 1;
            pub const r#FLOW_ID_RANGE_MAX: i8 = 63;
            impl Default for r#NrQos {
              fn default() -> Self {
                Self {
                  r#fiveQi: 0,
                  r#downlink: Default::default(),
                  r#uplink: Default::default(),
                  r#qfi: 0,
                  r#averagingWindowMs: '\0' as u16,
                }
              }
            }
            impl binder::Parcelable for r#NrQos {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#fiveQi)?;
                  subparcel.write(&self.r#downlink)?;
                  subparcel.write(&self.r#uplink)?;
                  subparcel.write(&self.r#qfi)?;
                  subparcel.write(&self.r#averagingWindowMs)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#fiveQi = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#downlink = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#uplink = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#qfi = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#averagingWindowMs = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#NrQos);
            binder::impl_deserialize_for_parcelable!(r#NrQos);
            impl binder::binder_impl::ParcelableMetadata for r#NrQos {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.NrQos" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#NrQos as _7_android_8_hardware_5_radio_4_data_5_NrQos;
            }
          }
          pub mod OsAppId {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#OsAppId {
              pub r#osAppId: Vec<u8>,
            }
            impl Default for r#OsAppId {
              fn default() -> Self {
                Self {
                  r#osAppId: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#OsAppId {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#osAppId)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#osAppId = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#OsAppId);
            binder::impl_deserialize_for_parcelable!(r#OsAppId);
            impl binder::binder_impl::ParcelableMetadata for r#OsAppId {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.OsAppId" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#OsAppId as _7_android_8_hardware_5_radio_4_data_7_OsAppId;
            }
          }
          pub mod PcoDataInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#PcoDataInfo {
              pub r#cid: i32,
              pub r#bearerProto: String,
              pub r#pcoId: i32,
              pub r#contents: Vec<u8>,
            }
            impl Default for r#PcoDataInfo {
              fn default() -> Self {
                Self {
                  r#cid: 0,
                  r#bearerProto: Default::default(),
                  r#pcoId: 0,
                  r#contents: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#PcoDataInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cid)?;
                  subparcel.write(&self.r#bearerProto)?;
                  subparcel.write(&self.r#pcoId)?;
                  subparcel.write(&self.r#contents)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cid = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bearerProto = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#pcoId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#contents = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PcoDataInfo);
            binder::impl_deserialize_for_parcelable!(r#PcoDataInfo);
            impl binder::binder_impl::ParcelableMetadata for r#PcoDataInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.PcoDataInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PcoDataInfo as _7_android_8_hardware_5_radio_4_data_11_PcoDataInfo;
            }
          }
          pub mod PdpProtocolType {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              r#PdpProtocolType : [i32; 7] {
                r#UNKNOWN = -1,
                r#IP = 0,
                r#IPV6 = 1,
                r#IPV4V6 = 2,
                r#PPP = 3,
                r#NON_IP = 4,
                r#UNSTRUCTURED = 5,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PdpProtocolType as _7_android_8_hardware_5_radio_4_data_15_PdpProtocolType;
            }
          }
          pub mod PortRange {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#PortRange {
              pub r#start: i32,
              pub r#end: i32,
            }
            pub const r#PORT_RANGE_MIN: i32 = 20;
            pub const r#PORT_RANGE_MAX: i32 = 65535;
            impl Default for r#PortRange {
              fn default() -> Self {
                Self {
                  r#start: 0,
                  r#end: 0,
                }
              }
            }
            impl binder::Parcelable for r#PortRange {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#start)?;
                  subparcel.write(&self.r#end)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#start = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#end = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PortRange);
            binder::impl_deserialize_for_parcelable!(r#PortRange);
            impl binder::binder_impl::ParcelableMetadata for r#PortRange {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.PortRange" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PortRange as _7_android_8_hardware_5_radio_4_data_9_PortRange;
            }
          }
          pub mod Qos {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub enum r#Qos {
              Noinit(bool),
              Eps(crate::mangled::_7_android_8_hardware_5_radio_4_data_6_EpsQos),
              Nr(crate::mangled::_7_android_8_hardware_5_radio_4_data_5_NrQos),
            }
            impl Default for r#Qos {
              fn default() -> Self {
                Self::Noinit(false)
              }
            }
            impl binder::Parcelable for r#Qos {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::Noinit(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::Eps(v) => {
                    parcel.write(&1i32)?;
                    parcel.write(v)
                  }
                  Self::Nr(v) => {
                    parcel.write(&2i32)?;
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
                    let value: crate::mangled::_7_android_8_hardware_5_radio_4_data_6_EpsQos = parcel.read()?;
                    *self = Self::Eps(value);
                    Ok(())
                  }
                  2 => {
                    let value: crate::mangled::_7_android_8_hardware_5_radio_4_data_5_NrQos = parcel.read()?;
                    *self = Self::Nr(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#Qos);
            binder::impl_deserialize_for_parcelable!(r#Qos);
            impl binder::binder_impl::ParcelableMetadata for r#Qos {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.Qos" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                r#Tag : [i32; 3] {
                  r#noinit = 0,
                  r#eps = 1,
                  r#nr = 2,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Qos as _7_android_8_hardware_5_radio_4_data_3_Qos;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_5_radio_4_data_3_Qos_3_Tag;
            }
          }
          pub mod QosBandwidth {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#QosBandwidth {
              pub r#maxBitrateKbps: i32,
              pub r#guaranteedBitrateKbps: i32,
            }
            impl Default for r#QosBandwidth {
              fn default() -> Self {
                Self {
                  r#maxBitrateKbps: 0,
                  r#guaranteedBitrateKbps: 0,
                }
              }
            }
            impl binder::Parcelable for r#QosBandwidth {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#maxBitrateKbps)?;
                  subparcel.write(&self.r#guaranteedBitrateKbps)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#maxBitrateKbps = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#guaranteedBitrateKbps = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#QosBandwidth);
            binder::impl_deserialize_for_parcelable!(r#QosBandwidth);
            impl binder::binder_impl::ParcelableMetadata for r#QosBandwidth {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.QosBandwidth" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#QosBandwidth as _7_android_8_hardware_5_radio_4_data_12_QosBandwidth;
            }
          }
          pub mod QosFilter {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#QosFilter {
              pub r#localAddresses: Vec<String>,
              pub r#remoteAddresses: Vec<String>,
              pub r#localPort: Option<crate::mangled::_7_android_8_hardware_5_radio_4_data_9_PortRange>,
              pub r#remotePort: Option<crate::mangled::_7_android_8_hardware_5_radio_4_data_9_PortRange>,
              pub r#protocol: i8,
              pub r#tos: crate::mangled::_7_android_8_hardware_5_radio_4_data_22_QosFilterTypeOfService,
              pub r#flowLabel: crate::mangled::_7_android_8_hardware_5_radio_4_data_22_QosFilterIpv6FlowLabel,
              pub r#spi: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_QosFilterIpsecSpi,
              pub r#direction: i8,
              pub r#precedence: i32,
            }
            pub const r#DIRECTION_DOWNLINK: i8 = 0;
            pub const r#DIRECTION_UPLINK: i8 = 1;
            pub const r#DIRECTION_BIDIRECTIONAL: i8 = 2;
            pub const r#PROTOCOL_UNSPECIFIED: i8 = -1;
            pub const r#PROTOCOL_TCP: i8 = 6;
            pub const r#PROTOCOL_UDP: i8 = 17;
            pub const r#PROTOCOL_ESP: i8 = 50;
            pub const r#PROTOCOL_AH: i8 = 51;
            impl Default for r#QosFilter {
              fn default() -> Self {
                Self {
                  r#localAddresses: Default::default(),
                  r#remoteAddresses: Default::default(),
                  r#localPort: Default::default(),
                  r#remotePort: Default::default(),
                  r#protocol: 0,
                  r#tos: Default::default(),
                  r#flowLabel: Default::default(),
                  r#spi: Default::default(),
                  r#direction: 0,
                  r#precedence: 0,
                }
              }
            }
            impl binder::Parcelable for r#QosFilter {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#localAddresses)?;
                  subparcel.write(&self.r#remoteAddresses)?;
                  subparcel.write(&self.r#localPort)?;
                  subparcel.write(&self.r#remotePort)?;
                  subparcel.write(&self.r#protocol)?;
                  subparcel.write(&self.r#tos)?;
                  subparcel.write(&self.r#flowLabel)?;
                  subparcel.write(&self.r#spi)?;
                  subparcel.write(&self.r#direction)?;
                  subparcel.write(&self.r#precedence)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#localAddresses = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#remoteAddresses = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#localPort = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#remotePort = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#protocol = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#tos = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#flowLabel = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#spi = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#direction = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#precedence = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#QosFilter);
            binder::impl_deserialize_for_parcelable!(r#QosFilter);
            impl binder::binder_impl::ParcelableMetadata for r#QosFilter {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.QosFilter" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#QosFilter as _7_android_8_hardware_5_radio_4_data_9_QosFilter;
            }
          }
          pub mod QosFilterIpsecSpi {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub enum r#QosFilterIpsecSpi {
              Noinit(bool),
              Value(i32),
            }
            impl Default for r#QosFilterIpsecSpi {
              fn default() -> Self {
                Self::Noinit(false)
              }
            }
            impl binder::Parcelable for r#QosFilterIpsecSpi {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::Noinit(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::Value(v) => {
                    parcel.write(&1i32)?;
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
                    let value: i32 = parcel.read()?;
                    *self = Self::Value(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#QosFilterIpsecSpi);
            binder::impl_deserialize_for_parcelable!(r#QosFilterIpsecSpi);
            impl binder::binder_impl::ParcelableMetadata for r#QosFilterIpsecSpi {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.QosFilterIpsecSpi" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                r#Tag : [i32; 2] {
                  r#noinit = 0,
                  r#value = 1,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#QosFilterIpsecSpi as _7_android_8_hardware_5_radio_4_data_17_QosFilterIpsecSpi;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_5_radio_4_data_17_QosFilterIpsecSpi_3_Tag;
            }
          }
          pub mod QosFilterIpv6FlowLabel {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub enum r#QosFilterIpv6FlowLabel {
              Noinit(bool),
              Value(i32),
            }
            impl Default for r#QosFilterIpv6FlowLabel {
              fn default() -> Self {
                Self::Noinit(false)
              }
            }
            impl binder::Parcelable for r#QosFilterIpv6FlowLabel {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::Noinit(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::Value(v) => {
                    parcel.write(&1i32)?;
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
                    let value: i32 = parcel.read()?;
                    *self = Self::Value(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#QosFilterIpv6FlowLabel);
            binder::impl_deserialize_for_parcelable!(r#QosFilterIpv6FlowLabel);
            impl binder::binder_impl::ParcelableMetadata for r#QosFilterIpv6FlowLabel {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.QosFilterIpv6FlowLabel" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                r#Tag : [i32; 2] {
                  r#noinit = 0,
                  r#value = 1,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#QosFilterIpv6FlowLabel as _7_android_8_hardware_5_radio_4_data_22_QosFilterIpv6FlowLabel;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_5_radio_4_data_22_QosFilterIpv6FlowLabel_3_Tag;
            }
          }
          pub mod QosFilterTypeOfService {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub enum r#QosFilterTypeOfService {
              Noinit(bool),
              Value(i8),
            }
            impl Default for r#QosFilterTypeOfService {
              fn default() -> Self {
                Self::Noinit(false)
              }
            }
            impl binder::Parcelable for r#QosFilterTypeOfService {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::Noinit(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::Value(v) => {
                    parcel.write(&1i32)?;
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
                    let value: i8 = parcel.read()?;
                    *self = Self::Value(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#QosFilterTypeOfService);
            binder::impl_deserialize_for_parcelable!(r#QosFilterTypeOfService);
            impl binder::binder_impl::ParcelableMetadata for r#QosFilterTypeOfService {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.QosFilterTypeOfService" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                r#Tag : [i32; 2] {
                  r#noinit = 0,
                  r#value = 1,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#QosFilterTypeOfService as _7_android_8_hardware_5_radio_4_data_22_QosFilterTypeOfService;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_5_radio_4_data_22_QosFilterTypeOfService_3_Tag;
            }
          }
          pub mod QosSession {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#QosSession {
              pub r#qosSessionId: i32,
              pub r#qos: crate::mangled::_7_android_8_hardware_5_radio_4_data_3_Qos,
              pub r#qosFilters: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_9_QosFilter>,
            }
            impl Default for r#QosSession {
              fn default() -> Self {
                Self {
                  r#qosSessionId: 0,
                  r#qos: Default::default(),
                  r#qosFilters: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#QosSession {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#qosSessionId)?;
                  subparcel.write(&self.r#qos)?;
                  subparcel.write(&self.r#qosFilters)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#qosSessionId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#qos = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#qosFilters = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#QosSession);
            binder::impl_deserialize_for_parcelable!(r#QosSession);
            impl binder::binder_impl::ParcelableMetadata for r#QosSession {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.QosSession" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#QosSession as _7_android_8_hardware_5_radio_4_data_10_QosSession;
            }
          }
          pub mod RouteSelectionDescriptor {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#RouteSelectionDescriptor {
              pub r#precedence: i8,
              pub r#sessionType: crate::mangled::_7_android_8_hardware_5_radio_4_data_15_PdpProtocolType,
              pub r#sscMode: i8,
              pub r#sliceInfo: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>,
              pub r#dnn: Vec<String>,
            }
            pub const r#SSC_MODE_UNKNOWN: i8 = -1;
            pub const r#SSC_MODE_1: i8 = 1;
            pub const r#SSC_MODE_2: i8 = 2;
            pub const r#SSC_MODE_3: i8 = 3;
            impl Default for r#RouteSelectionDescriptor {
              fn default() -> Self {
                Self {
                  r#precedence: 0,
                  r#sessionType: Default::default(),
                  r#sscMode: 0,
                  r#sliceInfo: Default::default(),
                  r#dnn: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#RouteSelectionDescriptor {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#precedence)?;
                  subparcel.write(&self.r#sessionType)?;
                  subparcel.write(&self.r#sscMode)?;
                  subparcel.write(&self.r#sliceInfo)?;
                  subparcel.write(&self.r#dnn)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#precedence = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sessionType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sscMode = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sliceInfo = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#dnn = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#RouteSelectionDescriptor);
            binder::impl_deserialize_for_parcelable!(r#RouteSelectionDescriptor);
            impl binder::binder_impl::ParcelableMetadata for r#RouteSelectionDescriptor {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.RouteSelectionDescriptor" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#RouteSelectionDescriptor as _7_android_8_hardware_5_radio_4_data_24_RouteSelectionDescriptor;
            }
          }
          pub mod SetupDataCallResult {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#SetupDataCallResult {
              pub r#cause: crate::mangled::_7_android_8_hardware_5_radio_4_data_17_DataCallFailCause,
              pub r#suggestedRetryTime: i64,
              pub r#cid: i32,
              pub r#active: i32,
              pub r#type: crate::mangled::_7_android_8_hardware_5_radio_4_data_15_PdpProtocolType,
              pub r#ifname: String,
              pub r#addresses: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_11_LinkAddress>,
              pub r#dnses: Vec<String>,
              pub r#gateways: Vec<String>,
              pub r#pcscf: Vec<String>,
              pub r#mtuV4: i32,
              pub r#mtuV6: i32,
              pub r#defaultQos: crate::mangled::_7_android_8_hardware_5_radio_4_data_3_Qos,
              pub r#qosSessions: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_10_QosSession>,
              pub r#handoverFailureMode: i8,
              pub r#pduSessionId: i32,
              pub r#sliceInfo: Option<crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>,
              pub r#trafficDescriptors: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_17_TrafficDescriptor>,
            }
            pub const r#DATA_CONNECTION_STATUS_INACTIVE: i32 = 0;
            pub const r#DATA_CONNECTION_STATUS_DORMANT: i32 = 1;
            pub const r#DATA_CONNECTION_STATUS_ACTIVE: i32 = 2;
            pub const r#HANDOVER_FAILURE_MODE_LEGACY: i8 = 0;
            pub const r#HANDOVER_FAILURE_MODE_DO_FALLBACK: i8 = 1;
            pub const r#HANDOVER_FAILURE_MODE_NO_FALLBACK_RETRY_HANDOVER: i8 = 2;
            pub const r#HANDOVER_FAILURE_MODE_NO_FALLBACK_RETRY_SETUP_NORMAL: i8 = 3;
            impl Default for r#SetupDataCallResult {
              fn default() -> Self {
                Self {
                  r#cause: Default::default(),
                  r#suggestedRetryTime: 0,
                  r#cid: 0,
                  r#active: 0,
                  r#type: Default::default(),
                  r#ifname: Default::default(),
                  r#addresses: Default::default(),
                  r#dnses: Default::default(),
                  r#gateways: Default::default(),
                  r#pcscf: Default::default(),
                  r#mtuV4: 0,
                  r#mtuV6: 0,
                  r#defaultQos: Default::default(),
                  r#qosSessions: Default::default(),
                  r#handoverFailureMode: 0,
                  r#pduSessionId: 0,
                  r#sliceInfo: Default::default(),
                  r#trafficDescriptors: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#SetupDataCallResult {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cause)?;
                  subparcel.write(&self.r#suggestedRetryTime)?;
                  subparcel.write(&self.r#cid)?;
                  subparcel.write(&self.r#active)?;
                  subparcel.write(&self.r#type)?;
                  subparcel.write(&self.r#ifname)?;
                  subparcel.write(&self.r#addresses)?;
                  subparcel.write(&self.r#dnses)?;
                  subparcel.write(&self.r#gateways)?;
                  subparcel.write(&self.r#pcscf)?;
                  subparcel.write(&self.r#mtuV4)?;
                  subparcel.write(&self.r#mtuV6)?;
                  subparcel.write(&self.r#defaultQos)?;
                  subparcel.write(&self.r#qosSessions)?;
                  subparcel.write(&self.r#handoverFailureMode)?;
                  subparcel.write(&self.r#pduSessionId)?;
                  subparcel.write(&self.r#sliceInfo)?;
                  subparcel.write(&self.r#trafficDescriptors)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cause = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#suggestedRetryTime = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cid = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#active = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#type = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#ifname = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#addresses = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#dnses = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#gateways = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#pcscf = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mtuV4 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mtuV6 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#defaultQos = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#qosSessions = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#handoverFailureMode = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#pduSessionId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sliceInfo = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#trafficDescriptors = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SetupDataCallResult);
            binder::impl_deserialize_for_parcelable!(r#SetupDataCallResult);
            impl binder::binder_impl::ParcelableMetadata for r#SetupDataCallResult {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.SetupDataCallResult" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SetupDataCallResult as _7_android_8_hardware_5_radio_4_data_19_SetupDataCallResult;
            }
          }
          pub mod SliceInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#SliceInfo {
              pub r#sliceServiceType: i8,
              pub r#sliceDifferentiator: i32,
              pub r#mappedHplmnSst: i8,
              pub r#mappedHplmnSd: i32,
              pub r#status: i8,
            }
            pub const r#SERVICE_TYPE_NONE: i8 = 0;
            pub const r#SERVICE_TYPE_EMBB: i8 = 1;
            pub const r#SERVICE_TYPE_URLLC: i8 = 2;
            pub const r#SERVICE_TYPE_MIOT: i8 = 3;
            pub const r#STATUS_UNKNOWN: i8 = 0;
            pub const r#STATUS_CONFIGURED: i8 = 1;
            pub const r#STATUS_ALLOWED: i8 = 2;
            pub const r#STATUS_REJECTED_NOT_AVAILABLE_IN_PLMN: i8 = 3;
            pub const r#STATUS_REJECTED_NOT_AVAILABLE_IN_REG_AREA: i8 = 4;
            pub const r#STATUS_DEFAULT_CONFIGURED: i8 = 5;
            impl Default for r#SliceInfo {
              fn default() -> Self {
                Self {
                  r#sliceServiceType: 0,
                  r#sliceDifferentiator: 0,
                  r#mappedHplmnSst: 0,
                  r#mappedHplmnSd: 0,
                  r#status: 0,
                }
              }
            }
            impl binder::Parcelable for r#SliceInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#sliceServiceType)?;
                  subparcel.write(&self.r#sliceDifferentiator)?;
                  subparcel.write(&self.r#mappedHplmnSst)?;
                  subparcel.write(&self.r#mappedHplmnSd)?;
                  subparcel.write(&self.r#status)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#sliceServiceType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sliceDifferentiator = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mappedHplmnSst = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mappedHplmnSd = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#status = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SliceInfo);
            binder::impl_deserialize_for_parcelable!(r#SliceInfo);
            impl binder::binder_impl::ParcelableMetadata for r#SliceInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.SliceInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SliceInfo as _7_android_8_hardware_5_radio_4_data_9_SliceInfo;
            }
          }
          pub mod SlicingConfig {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#SlicingConfig {
              pub r#urspRules: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_8_UrspRule>,
              pub r#sliceInfo: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_9_SliceInfo>,
            }
            impl Default for r#SlicingConfig {
              fn default() -> Self {
                Self {
                  r#urspRules: Default::default(),
                  r#sliceInfo: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#SlicingConfig {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#urspRules)?;
                  subparcel.write(&self.r#sliceInfo)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#urspRules = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sliceInfo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SlicingConfig);
            binder::impl_deserialize_for_parcelable!(r#SlicingConfig);
            impl binder::binder_impl::ParcelableMetadata for r#SlicingConfig {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.SlicingConfig" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SlicingConfig as _7_android_8_hardware_5_radio_4_data_13_SlicingConfig;
            }
          }
          pub mod TrafficDescriptor {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#TrafficDescriptor {
              pub r#dnn: Option<String>,
              pub r#osAppId: Option<crate::mangled::_7_android_8_hardware_5_radio_4_data_7_OsAppId>,
            }
            impl Default for r#TrafficDescriptor {
              fn default() -> Self {
                Self {
                  r#dnn: Default::default(),
                  r#osAppId: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#TrafficDescriptor {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#dnn)?;
                  subparcel.write(&self.r#osAppId)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#dnn = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#osAppId = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#TrafficDescriptor);
            binder::impl_deserialize_for_parcelable!(r#TrafficDescriptor);
            impl binder::binder_impl::ParcelableMetadata for r#TrafficDescriptor {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.TrafficDescriptor" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#TrafficDescriptor as _7_android_8_hardware_5_radio_4_data_17_TrafficDescriptor;
            }
          }
          pub mod UrspRule {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            pub struct r#UrspRule {
              pub r#precedence: i32,
              pub r#trafficDescriptors: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_17_TrafficDescriptor>,
              pub r#routeSelectionDescriptor: Vec<crate::mangled::_7_android_8_hardware_5_radio_4_data_24_RouteSelectionDescriptor>,
            }
            impl Default for r#UrspRule {
              fn default() -> Self {
                Self {
                  r#precedence: 0,
                  r#trafficDescriptors: Default::default(),
                  r#routeSelectionDescriptor: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#UrspRule {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#precedence)?;
                  subparcel.write(&self.r#trafficDescriptors)?;
                  subparcel.write(&self.r#routeSelectionDescriptor)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#precedence = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#trafficDescriptors = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#routeSelectionDescriptor = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#UrspRule);
            binder::impl_deserialize_for_parcelable!(r#UrspRule);
            impl binder::binder_impl::ParcelableMetadata for r#UrspRule {
              fn get_descriptor() -> &'static str { "android.hardware.radio.data.UrspRule" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#UrspRule as _7_android_8_hardware_5_radio_4_data_8_UrspRule;
            }
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::hardware::radio::data::ApnAuthType::mangled::*;
  pub use super::aidl::android::hardware::radio::data::ApnTypes::mangled::*;
  pub use super::aidl::android::hardware::radio::data::DataCallFailCause::mangled::*;
  pub use super::aidl::android::hardware::radio::data::DataProfileInfo::mangled::*;
  pub use super::aidl::android::hardware::radio::data::DataRequestReason::mangled::*;
  pub use super::aidl::android::hardware::radio::data::DataThrottlingAction::mangled::*;
  pub use super::aidl::android::hardware::radio::data::EpsQos::mangled::*;
  pub use super::aidl::android::hardware::radio::data::IRadioData::mangled::*;
  pub use super::aidl::android::hardware::radio::data::IRadioDataIndication::mangled::*;
  pub use super::aidl::android::hardware::radio::data::IRadioDataResponse::mangled::*;
  pub use super::aidl::android::hardware::radio::data::KeepaliveRequest::mangled::*;
  pub use super::aidl::android::hardware::radio::data::KeepaliveStatus::mangled::*;
  pub use super::aidl::android::hardware::radio::data::LinkAddress::mangled::*;
  pub use super::aidl::android::hardware::radio::data::NrQos::mangled::*;
  pub use super::aidl::android::hardware::radio::data::OsAppId::mangled::*;
  pub use super::aidl::android::hardware::radio::data::PcoDataInfo::mangled::*;
  pub use super::aidl::android::hardware::radio::data::PdpProtocolType::mangled::*;
  pub use super::aidl::android::hardware::radio::data::PortRange::mangled::*;
  pub use super::aidl::android::hardware::radio::data::Qos::mangled::*;
  pub use super::aidl::android::hardware::radio::data::QosBandwidth::mangled::*;
  pub use super::aidl::android::hardware::radio::data::QosFilter::mangled::*;
  pub use super::aidl::android::hardware::radio::data::QosFilterIpsecSpi::mangled::*;
  pub use super::aidl::android::hardware::radio::data::QosFilterIpv6FlowLabel::mangled::*;
  pub use super::aidl::android::hardware::radio::data::QosFilterTypeOfService::mangled::*;
  pub use super::aidl::android::hardware::radio::data::QosSession::mangled::*;
  pub use super::aidl::android::hardware::radio::data::RouteSelectionDescriptor::mangled::*;
  pub use super::aidl::android::hardware::radio::data::SetupDataCallResult::mangled::*;
  pub use super::aidl::android::hardware::radio::data::SliceInfo::mangled::*;
  pub use super::aidl::android::hardware::radio::data::SlicingConfig::mangled::*;
  pub use super::aidl::android::hardware::radio::data::TrafficDescriptor::mangled::*;
  pub use super::aidl::android::hardware::radio::data::UrspRule::mangled::*;
  pub(crate) use android_hardware_radio::mangled::*;
}
