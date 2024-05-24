// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceStatistics {
    fn refresh_rate_ms(&self) -> Result<u32, dbus::Error>;
    fn set_refresh_rate_ms(&self, value: u32) -> Result<(), dbus::Error>;
    fn tx_bytes(&self) -> Result<u64, dbus::Error>;
    fn rx_bytes(&self) -> Result<u64, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target = blocking::SyncConnection>>
    OrgFreedesktopNetworkManagerDeviceStatistics for blocking::Proxy<'a, C>
{
    fn refresh_rate_ms(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Statistics",
            "RefreshRateMs",
        )
    }

    fn tx_bytes(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Statistics",
            "TxBytes",
        )
    }

    fn rx_bytes(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Statistics",
            "RxBytes",
        )
    }

    fn set_refresh_rate_ms(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.freedesktop.NetworkManager.Device.Statistics",
            "RefreshRateMs",
            value,
        )
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
    pub properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(
            OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
                properties: i.read()?,
            },
        )
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Statistics";
}
