//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Signal`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.
//! Source: `org.freedesktop.ModemManager1.Modem.Signal.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(
    default_service = "org.freedesktop.ModemManager1",
    interface = "org.freedesktop.ModemManager1.Modem.Signal",
    assume_defaults = true,
    gen_blocking = false
)]
trait Signal {
    /// Setup method
    fn setup(&self, rate: u32) -> zbus::Result<()>;

    /// SetupThresholds method
    fn setup_thresholds(
        &self,
        settings: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Cdma property
    #[dbus_proxy(property)]
    fn cdma(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// ErrorRateThreshold property
    #[dbus_proxy(property)]
    fn error_rate_threshold(&self) -> zbus::Result<bool>;

    /// Evdo property
    #[dbus_proxy(property)]
    fn evdo(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Gsm property
    #[dbus_proxy(property)]
    fn gsm(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Lte property
    #[dbus_proxy(property)]
    fn lte(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Nr5g property
    #[dbus_proxy(property)]
    fn nr5g(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Rate property
    #[dbus_proxy(property)]
    fn rate(&self) -> zbus::Result<u32>;

    /// RssiThreshold property
    #[dbus_proxy(property)]
    fn rssi_threshold(&self) -> zbus::Result<u32>;

    /// Umts property
    #[dbus_proxy(property)]
    fn umts(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;
}
