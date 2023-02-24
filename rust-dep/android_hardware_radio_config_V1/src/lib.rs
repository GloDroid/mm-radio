#![feature(custom_inner_attributes)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
pub use binder;
pub mod aidl {
    pub mod android {
        pub mod hardware {
            pub mod radio {
                pub mod config {
                    pub mod IRadioConfig {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioConfig["android.hardware.radio.config.IRadioConfig"] {
                native: BnRadioConfig(on_transact),
                proxy: BpRadioConfig {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioConfigAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioConfig: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.config.IRadioConfig" }
              fn r#getHalDeviceCapabilities(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getNumOfLiveModems(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getPhoneCapability(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#getSimSlotsStatus(&self, _arg_serial: i32) -> binder::Result<()>;
              fn r#setNumOfLiveModems(&self, _arg_serial: i32, _arg_numOfLiveModems: i8) -> binder::Result<()>;
              fn r#setPreferredDataModem(&self, _arg_serial: i32, _arg_modemId: i8) -> binder::Result<()>;
              fn r#setResponseFunctions(&self, _arg_radioConfigResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_20_IRadioConfigResponse>, _arg_radioConfigIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_22_IRadioConfigIndication>) -> binder::Result<()>;
              fn r#setSimSlotsMapping(&self, _arg_serial: i32, _arg_slotMap: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_15_SlotPortMapping]) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioConfigDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioConfigDefaultRef) -> IRadioConfigDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioConfigAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.config.IRadioConfig" }
              fn r#getHalDeviceCapabilities(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getNumOfLiveModems(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getPhoneCapability(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#getSimSlotsStatus(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>>;
              fn r#setNumOfLiveModems(&self, _arg_serial: i32, _arg_numOfLiveModems: i8) -> std::future::Ready<binder::Result<()>>;
              fn r#setPreferredDataModem(&self, _arg_serial: i32, _arg_modemId: i8) -> std::future::Ready<binder::Result<()>>;
              fn r#setResponseFunctions(&self, _arg_radioConfigResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_20_IRadioConfigResponse>, _arg_radioConfigIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_22_IRadioConfigIndication>) -> std::future::Ready<binder::Result<()>>;
              fn r#setSimSlotsMapping(&self, _arg_serial: i32, _arg_slotMap: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_15_SlotPortMapping]) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioConfigAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.config.IRadioConfig" }
              async fn r#getHalDeviceCapabilities(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getNumOfLiveModems(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getPhoneCapability(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#getSimSlotsStatus(&self, _arg_serial: i32) -> binder::Result<()>;
              async fn r#setNumOfLiveModems(&self, _arg_serial: i32, _arg_numOfLiveModems: i8) -> binder::Result<()>;
              async fn r#setPreferredDataModem(&self, _arg_serial: i32, _arg_modemId: i8) -> binder::Result<()>;
              async fn r#setResponseFunctions(&self, _arg_radioConfigResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_20_IRadioConfigResponse>, _arg_radioConfigIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_22_IRadioConfigIndication>) -> binder::Result<()>;
              async fn r#setSimSlotsMapping(&self, _arg_serial: i32, _arg_slotMap: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_15_SlotPortMapping]) -> binder::Result<()>;
            }
            impl BnRadioConfig {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioConfig>
              where
                T: IRadioConfigAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioConfig for Wrapper<T, R>
                where
                  T: IRadioConfigAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#getHalDeviceCapabilities(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getHalDeviceCapabilities(_arg_serial))
                  }
                  fn r#getNumOfLiveModems(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getNumOfLiveModems(_arg_serial))
                  }
                  fn r#getPhoneCapability(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getPhoneCapability(_arg_serial))
                  }
                  fn r#getSimSlotsStatus(&self, _arg_serial: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSimSlotsStatus(_arg_serial))
                  }
                  fn r#setNumOfLiveModems(&self, _arg_serial: i32, _arg_numOfLiveModems: i8) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setNumOfLiveModems(_arg_serial, _arg_numOfLiveModems))
                  }
                  fn r#setPreferredDataModem(&self, _arg_serial: i32, _arg_modemId: i8) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setPreferredDataModem(_arg_serial, _arg_modemId))
                  }
                  fn r#setResponseFunctions(&self, _arg_radioConfigResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_20_IRadioConfigResponse>, _arg_radioConfigIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_22_IRadioConfigIndication>) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setResponseFunctions(_arg_radioConfigResponse, _arg_radioConfigIndication))
                  }
                  fn r#setSimSlotsMapping(&self, _arg_serial: i32, _arg_slotMap: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_15_SlotPortMapping]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setSimSlotsMapping(_arg_serial, _arg_slotMap))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioConfigDefault: Send + Sync {
              fn r#getHalDeviceCapabilities(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getNumOfLiveModems(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getPhoneCapability(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSimSlotsStatus(&self, _arg_serial: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setNumOfLiveModems(&self, _arg_serial: i32, _arg_numOfLiveModems: i8) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setPreferredDataModem(&self, _arg_serial: i32, _arg_modemId: i8) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setResponseFunctions(&self, _arg_radioConfigResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_20_IRadioConfigResponse>, _arg_radioConfigIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_22_IRadioConfigIndication>) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setSimSlotsMapping(&self, _arg_serial: i32, _arg_slotMap: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_15_SlotPortMapping]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#getHalDeviceCapabilities: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION;
              pub const r#getNumOfLiveModems: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#getPhoneCapability: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#getSimSlotsStatus: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#setNumOfLiveModems: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#setPreferredDataModem: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#setResponseFunctions: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#setSimSlotsMapping: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioConfigDefaultRef = Option<std::sync::Arc<dyn IRadioConfigDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioConfigDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "dd9c3f8e21930f9b4c46a4125bd5f5cec90318ec";
            impl BpRadioConfig {
              fn build_parcel_getHalDeviceCapabilities(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getHalDeviceCapabilities(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfig>::getDefaultImpl() {
                    return _aidl_default_impl.r#getHalDeviceCapabilities(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getNumOfLiveModems(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getNumOfLiveModems(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfig>::getDefaultImpl() {
                    return _aidl_default_impl.r#getNumOfLiveModems(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getPhoneCapability(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getPhoneCapability(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfig>::getDefaultImpl() {
                    return _aidl_default_impl.r#getPhoneCapability(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSimSlotsStatus(&self, _arg_serial: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                Ok(aidl_data)
              }
              fn read_response_getSimSlotsStatus(&self, _arg_serial: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfig>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSimSlotsStatus(_arg_serial);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setNumOfLiveModems(&self, _arg_serial: i32, _arg_numOfLiveModems: i8) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_numOfLiveModems)?;
                Ok(aidl_data)
              }
              fn read_response_setNumOfLiveModems(&self, _arg_serial: i32, _arg_numOfLiveModems: i8, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfig>::getDefaultImpl() {
                    return _aidl_default_impl.r#setNumOfLiveModems(_arg_serial, _arg_numOfLiveModems);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setPreferredDataModem(&self, _arg_serial: i32, _arg_modemId: i8) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(&_arg_modemId)?;
                Ok(aidl_data)
              }
              fn read_response_setPreferredDataModem(&self, _arg_serial: i32, _arg_modemId: i8, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfig>::getDefaultImpl() {
                    return _aidl_default_impl.r#setPreferredDataModem(_arg_serial, _arg_modemId);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setResponseFunctions(&self, _arg_radioConfigResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_20_IRadioConfigResponse>, _arg_radioConfigIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_22_IRadioConfigIndication>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_radioConfigResponse)?;
                aidl_data.write(_arg_radioConfigIndication)?;
                Ok(aidl_data)
              }
              fn read_response_setResponseFunctions(&self, _arg_radioConfigResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_20_IRadioConfigResponse>, _arg_radioConfigIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_22_IRadioConfigIndication>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfig>::getDefaultImpl() {
                    return _aidl_default_impl.r#setResponseFunctions(_arg_radioConfigResponse, _arg_radioConfigIndication);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setSimSlotsMapping(&self, _arg_serial: i32, _arg_slotMap: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_15_SlotPortMapping]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_serial)?;
                aidl_data.write(_arg_slotMap)?;
                Ok(aidl_data)
              }
              fn read_response_setSimSlotsMapping(&self, _arg_serial: i32, _arg_slotMap: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_15_SlotPortMapping], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfig>::getDefaultImpl() {
                    return _aidl_default_impl.r#setSimSlotsMapping(_arg_serial, _arg_slotMap);
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
            impl IRadioConfig for BpRadioConfig {
              fn r#getHalDeviceCapabilities(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getHalDeviceCapabilities(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getHalDeviceCapabilities, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getHalDeviceCapabilities(_arg_serial, _aidl_reply)
              }
              fn r#getNumOfLiveModems(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getNumOfLiveModems(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getNumOfLiveModems, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getNumOfLiveModems(_arg_serial, _aidl_reply)
              }
              fn r#getPhoneCapability(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getPhoneCapability(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getPhoneCapability, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getPhoneCapability(_arg_serial, _aidl_reply)
              }
              fn r#getSimSlotsStatus(&self, _arg_serial: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSimSlotsStatus(_arg_serial)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSimSlotsStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSimSlotsStatus(_arg_serial, _aidl_reply)
              }
              fn r#setNumOfLiveModems(&self, _arg_serial: i32, _arg_numOfLiveModems: i8) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setNumOfLiveModems(_arg_serial, _arg_numOfLiveModems)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNumOfLiveModems, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setNumOfLiveModems(_arg_serial, _arg_numOfLiveModems, _aidl_reply)
              }
              fn r#setPreferredDataModem(&self, _arg_serial: i32, _arg_modemId: i8) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setPreferredDataModem(_arg_serial, _arg_modemId)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setPreferredDataModem, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setPreferredDataModem(_arg_serial, _arg_modemId, _aidl_reply)
              }
              fn r#setResponseFunctions(&self, _arg_radioConfigResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_20_IRadioConfigResponse>, _arg_radioConfigIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_22_IRadioConfigIndication>) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setResponseFunctions(_arg_radioConfigResponse, _arg_radioConfigIndication)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setResponseFunctions(_arg_radioConfigResponse, _arg_radioConfigIndication, _aidl_reply)
              }
              fn r#setSimSlotsMapping(&self, _arg_serial: i32, _arg_slotMap: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_15_SlotPortMapping]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setSimSlotsMapping(_arg_serial, _arg_slotMap)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSimSlotsMapping, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setSimSlotsMapping(_arg_serial, _arg_slotMap, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioConfigAsync<P> for BpRadioConfig {
              fn r#getHalDeviceCapabilities(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getHalDeviceCapabilities(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getHalDeviceCapabilities, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getHalDeviceCapabilities(_arg_serial, _aidl_reply))
              }
              fn r#getNumOfLiveModems(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getNumOfLiveModems(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getNumOfLiveModems, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getNumOfLiveModems(_arg_serial, _aidl_reply))
              }
              fn r#getPhoneCapability(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getPhoneCapability(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getPhoneCapability, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getPhoneCapability(_arg_serial, _aidl_reply))
              }
              fn r#getSimSlotsStatus(&self, _arg_serial: i32) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSimSlotsStatus(_arg_serial) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSimSlotsStatus, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSimSlotsStatus(_arg_serial, _aidl_reply))
              }
              fn r#setNumOfLiveModems(&self, _arg_serial: i32, _arg_numOfLiveModems: i8) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setNumOfLiveModems(_arg_serial, _arg_numOfLiveModems) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNumOfLiveModems, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setNumOfLiveModems(_arg_serial, _arg_numOfLiveModems, _aidl_reply))
              }
              fn r#setPreferredDataModem(&self, _arg_serial: i32, _arg_modemId: i8) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setPreferredDataModem(_arg_serial, _arg_modemId) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setPreferredDataModem, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setPreferredDataModem(_arg_serial, _arg_modemId, _aidl_reply))
              }
              fn r#setResponseFunctions(&self, _arg_radioConfigResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_20_IRadioConfigResponse>, _arg_radioConfigIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_22_IRadioConfigIndication>) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setResponseFunctions(_arg_radioConfigResponse, _arg_radioConfigIndication) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setResponseFunctions, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setResponseFunctions(_arg_radioConfigResponse, _arg_radioConfigIndication, _aidl_reply))
              }
              fn r#setSimSlotsMapping(&self, _arg_serial: i32, _arg_slotMap: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_15_SlotPortMapping]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setSimSlotsMapping(_arg_serial, _arg_slotMap) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSimSlotsMapping, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setSimSlotsMapping(_arg_serial, _arg_slotMap, _aidl_reply))
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
            impl IRadioConfig for binder::binder_impl::Binder<BnRadioConfig> {
              fn r#getHalDeviceCapabilities(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getHalDeviceCapabilities(_arg_serial) }
              fn r#getNumOfLiveModems(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getNumOfLiveModems(_arg_serial) }
              fn r#getPhoneCapability(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getPhoneCapability(_arg_serial) }
              fn r#getSimSlotsStatus(&self, _arg_serial: i32) -> binder::Result<()> { self.0.r#getSimSlotsStatus(_arg_serial) }
              fn r#setNumOfLiveModems(&self, _arg_serial: i32, _arg_numOfLiveModems: i8) -> binder::Result<()> { self.0.r#setNumOfLiveModems(_arg_serial, _arg_numOfLiveModems) }
              fn r#setPreferredDataModem(&self, _arg_serial: i32, _arg_modemId: i8) -> binder::Result<()> { self.0.r#setPreferredDataModem(_arg_serial, _arg_modemId) }
              fn r#setResponseFunctions(&self, _arg_radioConfigResponse: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_20_IRadioConfigResponse>, _arg_radioConfigIndication: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_22_IRadioConfigIndication>) -> binder::Result<()> { self.0.r#setResponseFunctions(_arg_radioConfigResponse, _arg_radioConfigIndication) }
              fn r#setSimSlotsMapping(&self, _arg_serial: i32, _arg_slotMap: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_15_SlotPortMapping]) -> binder::Result<()> { self.0.r#setSimSlotsMapping(_arg_serial, _arg_slotMap) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioConfig, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#getHalDeviceCapabilities => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getHalDeviceCapabilities(_arg_serial);
                  Ok(())
                }
                transactions::r#getNumOfLiveModems => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getNumOfLiveModems(_arg_serial);
                  Ok(())
                }
                transactions::r#getPhoneCapability => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getPhoneCapability(_arg_serial);
                  Ok(())
                }
                transactions::r#getSimSlotsStatus => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSimSlotsStatus(_arg_serial);
                  Ok(())
                }
                transactions::r#setNumOfLiveModems => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_numOfLiveModems: i8 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setNumOfLiveModems(_arg_serial, _arg_numOfLiveModems);
                  Ok(())
                }
                transactions::r#setPreferredDataModem => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_modemId: i8 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setPreferredDataModem(_arg_serial, _arg_modemId);
                  Ok(())
                }
                transactions::r#setResponseFunctions => {
                  let _arg_radioConfigResponse: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_20_IRadioConfigResponse> = _aidl_data.read()?;
                  let _arg_radioConfigIndication: binder::Strong<dyn crate::mangled::_7_android_8_hardware_5_radio_6_config_22_IRadioConfigIndication> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setResponseFunctions(&_arg_radioConfigResponse, &_arg_radioConfigIndication);
                  Ok(())
                }
                transactions::r#setSimSlotsMapping => {
                  let _arg_serial: i32 = _aidl_data.read()?;
                  let _arg_slotMap: Vec<crate::mangled::_7_android_8_hardware_5_radio_6_config_15_SlotPortMapping> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setSimSlotsMapping(_arg_serial, &_arg_slotMap);
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
             pub use super::r#IRadioConfig as _7_android_8_hardware_5_radio_6_config_12_IRadioConfig;
            }
          }
                    pub mod IRadioConfigIndication {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioConfigIndication["android.hardware.radio.config.IRadioConfigIndication"] {
                native: BnRadioConfigIndication(on_transact),
                proxy: BpRadioConfigIndication {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioConfigIndicationAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioConfigIndication: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.config.IRadioConfigIndication" }
              fn r#simSlotsStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioConfigIndicationDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioConfigIndicationDefaultRef) -> IRadioConfigIndicationDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioConfigIndicationAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.config.IRadioConfigIndication" }
              fn r#simSlotsStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioConfigIndicationAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.config.IRadioConfigIndication" }
              async fn r#simSlotsStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<()>;
            }
            impl BnRadioConfigIndication {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioConfigIndication>
              where
                T: IRadioConfigIndicationAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioConfigIndication for Wrapper<T, R>
                where
                  T: IRadioConfigIndicationAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#simSlotsStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#simSlotsStatusChanged(_arg_type, _arg_slotStatus))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioConfigIndicationDefault: Send + Sync {
              fn r#simSlotsStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#simSlotsStatusChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioConfigIndicationDefaultRef = Option<std::sync::Arc<dyn IRadioConfigIndicationDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioConfigIndicationDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "dd9c3f8e21930f9b4c46a4125bd5f5cec90318ec";
            impl BpRadioConfigIndication {
              fn build_parcel_simSlotsStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_type)?;
                aidl_data.write(_arg_slotStatus)?;
                Ok(aidl_data)
              }
              fn read_response_simSlotsStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfigIndication>::getDefaultImpl() {
                    return _aidl_default_impl.r#simSlotsStatusChanged(_arg_type, _arg_slotStatus);
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
            impl IRadioConfigIndication for BpRadioConfigIndication {
              fn r#simSlotsStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_simSlotsStatusChanged(_arg_type, _arg_slotStatus)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#simSlotsStatusChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_simSlotsStatusChanged(_arg_type, _arg_slotStatus, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioConfigIndicationAsync<P> for BpRadioConfigIndication {
              fn r#simSlotsStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_simSlotsStatusChanged(_arg_type, _arg_slotStatus) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#simSlotsStatusChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_simSlotsStatusChanged(_arg_type, _arg_slotStatus, _aidl_reply))
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
            impl IRadioConfigIndication for binder::binder_impl::Binder<BnRadioConfigIndication> {
              fn r#simSlotsStatusChanged(&self, _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<()> { self.0.r#simSlotsStatusChanged(_arg_type, _arg_slotStatus) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioConfigIndication, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#simSlotsStatusChanged => {
                  let _arg_type: crate::mangled::_7_android_8_hardware_5_radio_19_RadioIndicationType = _aidl_data.read()?;
                  let _arg_slotStatus: Vec<crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#simSlotsStatusChanged(_arg_type, &_arg_slotStatus);
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
             pub use super::r#IRadioConfigIndication as _7_android_8_hardware_5_radio_6_config_22_IRadioConfigIndication;
            }
          }
                    pub mod IRadioConfigResponse {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IRadioConfigResponse["android.hardware.radio.config.IRadioConfigResponse"] {
                native: BnRadioConfigResponse(on_transact),
                proxy: BpRadioConfigResponse {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IRadioConfigResponseAsync,
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IRadioConfigResponse: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.config.IRadioConfigResponse" }
              fn r#getHalDeviceCapabilitiesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_modemReducedFeatureSet1: bool) -> binder::Result<()>;
              fn r#getNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_numOfLiveModems: i8) -> binder::Result<()>;
              fn r#getPhoneCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_phoneCapability: &crate::mangled::_7_android_8_hardware_5_radio_6_config_15_PhoneCapability) -> binder::Result<()>;
              fn r#getSimSlotsStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<()>;
              fn r#setNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setPreferredDataModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#setSimSlotsMappingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IRadioConfigResponseDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IRadioConfigResponseDefaultRef) -> IRadioConfigResponseDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
            }
            pub trait IRadioConfigResponseAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.config.IRadioConfigResponse" }
              fn r#getHalDeviceCapabilitiesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_modemReducedFeatureSet1: bool) -> std::future::Ready<binder::Result<()>>;
              fn r#getNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_numOfLiveModems: i8) -> std::future::Ready<binder::Result<()>>;
              fn r#getPhoneCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_phoneCapability: &crate::mangled::_7_android_8_hardware_5_radio_6_config_15_PhoneCapability) -> std::future::Ready<binder::Result<()>>;
              fn r#getSimSlotsStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> std::future::Ready<binder::Result<()>>;
              fn r#setNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setPreferredDataModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#setSimSlotsMappingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IRadioConfigResponseAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.radio.config.IRadioConfigResponse" }
              async fn r#getHalDeviceCapabilitiesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_modemReducedFeatureSet1: bool) -> binder::Result<()>;
              async fn r#getNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_numOfLiveModems: i8) -> binder::Result<()>;
              async fn r#getPhoneCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_phoneCapability: &crate::mangled::_7_android_8_hardware_5_radio_6_config_15_PhoneCapability) -> binder::Result<()>;
              async fn r#getSimSlotsStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<()>;
              async fn r#setNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setPreferredDataModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
              async fn r#setSimSlotsMappingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()>;
            }
            impl BnRadioConfigResponse {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRadioConfigResponse>
              where
                T: IRadioConfigResponseAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IRadioConfigResponse for Wrapper<T, R>
                where
                  T: IRadioConfigResponseAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#getHalDeviceCapabilitiesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_modemReducedFeatureSet1: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getHalDeviceCapabilitiesResponse(_arg_info, _arg_modemReducedFeatureSet1))
                  }
                  fn r#getNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_numOfLiveModems: i8) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getNumOfLiveModemsResponse(_arg_info, _arg_numOfLiveModems))
                  }
                  fn r#getPhoneCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_phoneCapability: &crate::mangled::_7_android_8_hardware_5_radio_6_config_15_PhoneCapability) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getPhoneCapabilityResponse(_arg_info, _arg_phoneCapability))
                  }
                  fn r#getSimSlotsStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#getSimSlotsStatusResponse(_arg_info, _arg_slotStatus))
                  }
                  fn r#setNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setNumOfLiveModemsResponse(_arg_info))
                  }
                  fn r#setPreferredDataModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setPreferredDataModemResponse(_arg_info))
                  }
                  fn r#setSimSlotsMappingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setSimSlotsMappingResponse(_arg_info))
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
            }
            pub trait IRadioConfigResponseDefault: Send + Sync {
              fn r#getHalDeviceCapabilitiesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_modemReducedFeatureSet1: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_numOfLiveModems: i8) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getPhoneCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_phoneCapability: &crate::mangled::_7_android_8_hardware_5_radio_6_config_15_PhoneCapability) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSimSlotsStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setPreferredDataModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setSimSlotsMappingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#getHalDeviceCapabilitiesResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION;
              pub const r#getNumOfLiveModemsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#getPhoneCapabilityResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#getSimSlotsStatusResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#setNumOfLiveModemsResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#setPreferredDataModemResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#setSimSlotsMappingResponse: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IRadioConfigResponseDefaultRef = Option<std::sync::Arc<dyn IRadioConfigResponseDefault>>;
            use lazy_static::lazy_static;
            lazy_static! {
              static ref DEFAULT_IMPL: std::sync::Mutex<IRadioConfigResponseDefaultRef> = std::sync::Mutex::new(None);
            }
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "dd9c3f8e21930f9b4c46a4125bd5f5cec90318ec";
            impl BpRadioConfigResponse {
              fn build_parcel_getHalDeviceCapabilitiesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_modemReducedFeatureSet1: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_modemReducedFeatureSet1)?;
                Ok(aidl_data)
              }
              fn read_response_getHalDeviceCapabilitiesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_modemReducedFeatureSet1: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfigResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getHalDeviceCapabilitiesResponse(_arg_info, _arg_modemReducedFeatureSet1);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_numOfLiveModems: i8) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(&_arg_numOfLiveModems)?;
                Ok(aidl_data)
              }
              fn read_response_getNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_numOfLiveModems: i8, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfigResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getNumOfLiveModemsResponse(_arg_info, _arg_numOfLiveModems);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getPhoneCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_phoneCapability: &crate::mangled::_7_android_8_hardware_5_radio_6_config_15_PhoneCapability) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_phoneCapability)?;
                Ok(aidl_data)
              }
              fn read_response_getPhoneCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_phoneCapability: &crate::mangled::_7_android_8_hardware_5_radio_6_config_15_PhoneCapability, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfigResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getPhoneCapabilityResponse(_arg_info, _arg_phoneCapability);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getSimSlotsStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                aidl_data.write(_arg_slotStatus)?;
                Ok(aidl_data)
              }
              fn read_response_getSimSlotsStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfigResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSimSlotsStatusResponse(_arg_info, _arg_slotStatus);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfigResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setNumOfLiveModemsResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setPreferredDataModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setPreferredDataModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfigResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setPreferredDataModemResponse(_arg_info);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_setSimSlotsMappingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setSimSlotsMappingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IRadioConfigResponse>::getDefaultImpl() {
                    return _aidl_default_impl.r#setSimSlotsMappingResponse(_arg_info);
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
            impl IRadioConfigResponse for BpRadioConfigResponse {
              fn r#getHalDeviceCapabilitiesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_modemReducedFeatureSet1: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getHalDeviceCapabilitiesResponse(_arg_info, _arg_modemReducedFeatureSet1)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getHalDeviceCapabilitiesResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getHalDeviceCapabilitiesResponse(_arg_info, _arg_modemReducedFeatureSet1, _aidl_reply)
              }
              fn r#getNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_numOfLiveModems: i8) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getNumOfLiveModemsResponse(_arg_info, _arg_numOfLiveModems)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getNumOfLiveModemsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getNumOfLiveModemsResponse(_arg_info, _arg_numOfLiveModems, _aidl_reply)
              }
              fn r#getPhoneCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_phoneCapability: &crate::mangled::_7_android_8_hardware_5_radio_6_config_15_PhoneCapability) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getPhoneCapabilityResponse(_arg_info, _arg_phoneCapability)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getPhoneCapabilityResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getPhoneCapabilityResponse(_arg_info, _arg_phoneCapability, _aidl_reply)
              }
              fn r#getSimSlotsStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_getSimSlotsStatusResponse(_arg_info, _arg_slotStatus)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSimSlotsStatusResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSimSlotsStatusResponse(_arg_info, _arg_slotStatus, _aidl_reply)
              }
              fn r#setNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setNumOfLiveModemsResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNumOfLiveModemsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setNumOfLiveModemsResponse(_arg_info, _aidl_reply)
              }
              fn r#setPreferredDataModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setPreferredDataModemResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setPreferredDataModemResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setPreferredDataModemResponse(_arg_info, _aidl_reply)
              }
              fn r#setSimSlotsMappingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setSimSlotsMappingResponse(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSimSlotsMappingResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setSimSlotsMappingResponse(_arg_info, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IRadioConfigResponseAsync<P> for BpRadioConfigResponse {
              fn r#getHalDeviceCapabilitiesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_modemReducedFeatureSet1: bool) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getHalDeviceCapabilitiesResponse(_arg_info, _arg_modemReducedFeatureSet1) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getHalDeviceCapabilitiesResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getHalDeviceCapabilitiesResponse(_arg_info, _arg_modemReducedFeatureSet1, _aidl_reply))
              }
              fn r#getNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_numOfLiveModems: i8) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getNumOfLiveModemsResponse(_arg_info, _arg_numOfLiveModems) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getNumOfLiveModemsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getNumOfLiveModemsResponse(_arg_info, _arg_numOfLiveModems, _aidl_reply))
              }
              fn r#getPhoneCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_phoneCapability: &crate::mangled::_7_android_8_hardware_5_radio_6_config_15_PhoneCapability) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getPhoneCapabilityResponse(_arg_info, _arg_phoneCapability) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getPhoneCapabilityResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getPhoneCapabilityResponse(_arg_info, _arg_phoneCapability, _aidl_reply))
              }
              fn r#getSimSlotsStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_getSimSlotsStatusResponse(_arg_info, _arg_slotStatus) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSimSlotsStatusResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_getSimSlotsStatusResponse(_arg_info, _arg_slotStatus, _aidl_reply))
              }
              fn r#setNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setNumOfLiveModemsResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setNumOfLiveModemsResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setNumOfLiveModemsResponse(_arg_info, _aidl_reply))
              }
              fn r#setPreferredDataModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setPreferredDataModemResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setPreferredDataModemResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setPreferredDataModemResponse(_arg_info, _aidl_reply))
              }
              fn r#setSimSlotsMappingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> std::future::Ready<binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setSimSlotsMappingResponse(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return std::future::ready(Err(err)),
                };
                let _aidl_reply = self.binder.submit_transact(transactions::r#setSimSlotsMappingResponse, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                std::future::ready(self.read_response_setSimSlotsMappingResponse(_arg_info, _aidl_reply))
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
            impl IRadioConfigResponse for binder::binder_impl::Binder<BnRadioConfigResponse> {
              fn r#getHalDeviceCapabilitiesResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_modemReducedFeatureSet1: bool) -> binder::Result<()> { self.0.r#getHalDeviceCapabilitiesResponse(_arg_info, _arg_modemReducedFeatureSet1) }
              fn r#getNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_numOfLiveModems: i8) -> binder::Result<()> { self.0.r#getNumOfLiveModemsResponse(_arg_info, _arg_numOfLiveModems) }
              fn r#getPhoneCapabilityResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_phoneCapability: &crate::mangled::_7_android_8_hardware_5_radio_6_config_15_PhoneCapability) -> binder::Result<()> { self.0.r#getPhoneCapabilityResponse(_arg_info, _arg_phoneCapability) }
              fn r#getSimSlotsStatusResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo, _arg_slotStatus: &[crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus]) -> binder::Result<()> { self.0.r#getSimSlotsStatusResponse(_arg_info, _arg_slotStatus) }
              fn r#setNumOfLiveModemsResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setNumOfLiveModemsResponse(_arg_info) }
              fn r#setPreferredDataModemResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setPreferredDataModemResponse(_arg_info) }
              fn r#setSimSlotsMappingResponse(&self, _arg_info: &crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo) -> binder::Result<()> { self.0.r#setSimSlotsMappingResponse(_arg_info) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IRadioConfigResponse, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#getHalDeviceCapabilitiesResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_modemReducedFeatureSet1: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getHalDeviceCapabilitiesResponse(&_arg_info, _arg_modemReducedFeatureSet1);
                  Ok(())
                }
                transactions::r#getNumOfLiveModemsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_numOfLiveModems: i8 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getNumOfLiveModemsResponse(&_arg_info, _arg_numOfLiveModems);
                  Ok(())
                }
                transactions::r#getPhoneCapabilityResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_phoneCapability: crate::mangled::_7_android_8_hardware_5_radio_6_config_15_PhoneCapability = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getPhoneCapabilityResponse(&_arg_info, &_arg_phoneCapability);
                  Ok(())
                }
                transactions::r#getSimSlotsStatusResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _arg_slotStatus: Vec<crate::mangled::_7_android_8_hardware_5_radio_6_config_13_SimSlotStatus> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSimSlotsStatusResponse(&_arg_info, &_arg_slotStatus);
                  Ok(())
                }
                transactions::r#setNumOfLiveModemsResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setNumOfLiveModemsResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setPreferredDataModemResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setPreferredDataModemResponse(&_arg_info);
                  Ok(())
                }
                transactions::r#setSimSlotsMappingResponse => {
                  let _arg_info: crate::mangled::_7_android_8_hardware_5_radio_17_RadioResponseInfo = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setSimSlotsMappingResponse(&_arg_info);
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
             pub use super::r#IRadioConfigResponse as _7_android_8_hardware_5_radio_6_config_20_IRadioConfigResponse;
            }
          }
                    pub mod PhoneCapability {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            #[derive(Default)]
            pub struct r#PhoneCapability {
              pub r#maxActiveData: i8,
              pub r#maxActiveInternetData: i8,
              pub r#isInternetLingeringSupported: bool,
              pub r#logicalModemIds: Vec<u8>,
            }

            impl binder::Parcelable for r#PhoneCapability {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#maxActiveData)?;
                  subparcel.write(&self.r#maxActiveInternetData)?;
                  subparcel.write(&self.r#isInternetLingeringSupported)?;
                  subparcel.write(&self.r#logicalModemIds)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#maxActiveData = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxActiveInternetData = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#isInternetLingeringSupported = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#logicalModemIds = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PhoneCapability);
            binder::impl_deserialize_for_parcelable!(r#PhoneCapability);
            impl binder::binder_impl::ParcelableMetadata for r#PhoneCapability {
              fn get_descriptor() -> &'static str { "android.hardware.radio.config.PhoneCapability" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PhoneCapability as _7_android_8_hardware_5_radio_6_config_15_PhoneCapability;
            }
          }
                    pub mod SimPortInfo {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            #[derive(Default)]
            pub struct r#SimPortInfo {
              pub r#iccId: String,
              pub r#logicalSlotId: i32,
              pub r#portActive: bool,
            }
            
            impl binder::Parcelable for r#SimPortInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#iccId)?;
                  subparcel.write(&self.r#logicalSlotId)?;
                  subparcel.write(&self.r#portActive)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#iccId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#logicalSlotId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#portActive = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SimPortInfo);
            binder::impl_deserialize_for_parcelable!(r#SimPortInfo);
            impl binder::binder_impl::ParcelableMetadata for r#SimPortInfo {
              fn get_descriptor() -> &'static str { "android.hardware.radio.config.SimPortInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SimPortInfo as _7_android_8_hardware_5_radio_6_config_11_SimPortInfo;
            }
          }
                    pub mod SimSlotStatus {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            #[derive(Default)]
            pub struct r#SimSlotStatus {
              pub r#cardState: i32,
              pub r#atr: String,
              pub r#eid: String,
              pub r#portInfo: Vec<crate::mangled::_7_android_8_hardware_5_radio_6_config_11_SimPortInfo>,
            }
            
            impl binder::Parcelable for r#SimSlotStatus {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#cardState)?;
                  subparcel.write(&self.r#atr)?;
                  subparcel.write(&self.r#eid)?;
                  subparcel.write(&self.r#portInfo)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#cardState = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#atr = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#eid = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#portInfo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SimSlotStatus);
            binder::impl_deserialize_for_parcelable!(r#SimSlotStatus);
            impl binder::binder_impl::ParcelableMetadata for r#SimSlotStatus {
              fn get_descriptor() -> &'static str { "android.hardware.radio.config.SimSlotStatus" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SimSlotStatus as _7_android_8_hardware_5_radio_6_config_13_SimSlotStatus;
            }
          }
                    pub mod SlotPortMapping {
            #![forbid(unsafe_code)]
            #![rustfmt::skip]
            #[derive(Debug)]
            #[derive(Default)]
            #[derive(Clone)]
            pub struct r#SlotPortMapping {
              pub r#physicalSlotId: i32,
              pub r#portId: i32,
            }
            
            impl binder::Parcelable for r#SlotPortMapping {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#physicalSlotId)?;
                  subparcel.write(&self.r#portId)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#physicalSlotId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#portId = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SlotPortMapping);
            binder::impl_deserialize_for_parcelable!(r#SlotPortMapping);
            impl binder::binder_impl::ParcelableMetadata for r#SlotPortMapping {
              fn get_descriptor() -> &'static str { "android.hardware.radio.config.SlotPortMapping" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SlotPortMapping as _7_android_8_hardware_5_radio_6_config_15_SlotPortMapping;
            }
          }
                }
            }
        }
    }
}
pub mod mangled {
    pub use super::aidl::android::hardware::radio::config::IRadioConfig::mangled::*;
    pub use super::aidl::android::hardware::radio::config::IRadioConfigIndication::mangled::*;
    pub use super::aidl::android::hardware::radio::config::IRadioConfigResponse::mangled::*;
    pub use super::aidl::android::hardware::radio::config::PhoneCapability::mangled::*;
    pub use super::aidl::android::hardware::radio::config::SimPortInfo::mangled::*;
    pub use super::aidl::android::hardware::radio::config::SimSlotStatus::mangled::*;
    pub use super::aidl::android::hardware::radio::config::SlotPortMapping::mangled::*;
    pub(crate) use android_hardware_radio::mangled::*;
}
