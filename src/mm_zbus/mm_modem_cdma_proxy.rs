//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.ModemCdma`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.
//! Source: `org.freedesktop.ModemManager1.Modem.ModemCdma.xml`.
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
    interface = "org.freedesktop.ModemManager1.Modem.ModemCdma",
    assume_defaults = true,
    gen_blocking = false
)]
trait ModemCdma {
    /// Activate method
    fn activate(&self, carrier_code: &str) -> zbus::Result<()>;

    /// ActivateManual method
    fn activate_manual(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// ActivationStateChanged signal
    #[dbus_proxy(signal)]
    fn activation_state_changed(
        &self,
        activation_state: u32,
        activation_error: u32,
        status_changes: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// ActivationState property
    #[dbus_proxy(property, name = "activation_state")]
    fn activation_state_prop(&self) -> zbus::Result<u32>;

    /// Cdma1xRegistrationState property
    #[dbus_proxy(property)]
    fn cdma1x_registration_state(&self) -> zbus::Result<u32>;

    /// Esn property
    #[dbus_proxy(property)]
    fn esn(&self) -> zbus::Result<String>;

    /// EvdoRegistrationState property
    #[dbus_proxy(property)]
    fn evdo_registration_state(&self) -> zbus::Result<u32>;

    /// Meid property
    #[dbus_proxy(property)]
    fn meid(&self) -> zbus::Result<String>;

    /// Nid property
    #[dbus_proxy(property)]
    fn nid(&self) -> zbus::Result<u32>;

    /// Sid property
    #[dbus_proxy(property)]
    fn sid(&self) -> zbus::Result<u32>;
}