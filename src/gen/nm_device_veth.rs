// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceVeth {
    fn peer(&self) -> Result<dbus::Path<'static>, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target = blocking::SyncSyncConnection>>
    OrgFreedesktopNetworkManagerDeviceVeth for blocking::Proxy<'a, C>
{
    fn peer(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Veth",
            "Peer",
        )
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceVethPropertiesChanged {
    pub properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceVethPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceVethPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceVethPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceVethPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Veth";
}
