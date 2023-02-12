#![allow(non_camel_case_types)]

#[cfg(not(any(target_os = "android", feature = "test")))]
compile_error!("ndk-sys only supports compiling for Android");

#[cfg(all(
    any(target_os = "android", feature = "test"),
    any(target_arch = "arm", target_arch = "armv7")
))]
include!("bindings_arm.rs");

#[cfg(all(any(target_os = "android", feature = "test"), target_arch = "aarch64"))]
include!("bindings_aarch64.rs");

#[cfg(all(any(target_os = "android", feature = "test"), target_arch = "x86"))]
include!("bindings_x86.rs");

#[cfg(all(any(target_os = "android", feature = "test"), target_arch = "x86_64"))]
include!("bindings_x86_64.rs");
