// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceGeneric {
    fn hw_address(&self) -> Result<String, dbus::Error>;
    fn type_description(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target = blocking::SyncSyncConnection>>
    OrgFreedesktopNetworkManagerDeviceGeneric for blocking::Proxy<'a, C>
{
    fn hw_address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Generic",
            "HwAddress",
        )
    }

    fn type_description(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Generic",
            "TypeDescription",
        )
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceGenericPropertiesChanged {
    pub properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceGenericPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceGenericPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceGenericPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceGenericPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Generic";
}
