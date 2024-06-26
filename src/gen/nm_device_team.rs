// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceTeam {
    fn hw_address(&self) -> Result<String, dbus::Error>;
    fn carrier(&self) -> Result<bool, dbus::Error>;
    fn slaves(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn config(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target = blocking::SyncSyncConnection>>
    OrgFreedesktopNetworkManagerDeviceTeam for blocking::Proxy<'a, C>
{
    fn hw_address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Team",
            "HwAddress",
        )
    }

    fn carrier(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Team",
            "Carrier",
        )
    }

    fn slaves(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Team",
            "Slaves",
        )
    }

    fn config(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Team",
            "Config",
        )
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceTeamPropertiesChanged {
    pub properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceTeamPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceTeamPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceTeamPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceTeamPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Team";
}
