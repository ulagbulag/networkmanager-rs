// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus;
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceWiMax {
    fn get_nsp_list(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn nsps(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn hw_address(&self) -> Result<String, dbus::Error>;
    fn center_frequency(&self) -> Result<u32, dbus::Error>;
    fn rssi(&self) -> Result<i32, dbus::Error>;
    fn cinr(&self) -> Result<i32, dbus::Error>;
    fn tx_power(&self) -> Result<i32, dbus::Error>;
    fn bsid(&self) -> Result<String, dbus::Error>;
    fn active_nsp(&self) -> Result<dbus::Path<'static>, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target = blocking::Connection>>
    OrgFreedesktopNetworkManagerDeviceWiMax for blocking::Proxy<'a, C>
{
    fn get_nsp_list(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        self.method_call(
            "org.freedesktop.NetworkManager.Device.WiMax",
            "GetNspList",
            (),
        )
        .and_then(|r: (Vec<dbus::Path<'static>>,)| Ok(r.0))
    }

    fn nsps(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.WiMax",
            "Nsps",
        )
    }

    fn hw_address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.WiMax",
            "HwAddress",
        )
    }

    fn center_frequency(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.WiMax",
            "CenterFrequency",
        )
    }

    fn rssi(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.WiMax",
            "Rssi",
        )
    }

    fn cinr(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.WiMax",
            "Cinr",
        )
    }

    fn tx_power(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.WiMax",
            "TxPower",
        )
    }

    fn bsid(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.WiMax",
            "Bsid",
        )
    }

    fn active_nsp(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.WiMax",
            "ActiveNsp",
        )
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceWiMaxPropertiesChanged {
    pub properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceWiMaxPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceWiMaxPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceWiMaxPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceWiMaxPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.WiMax";
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceWiMaxNspAdded {
    pub nsp: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceWiMaxNspAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.nsp, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceWiMaxNspAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceWiMaxNspAdded { nsp: i.read()? })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceWiMaxNspAdded {
    const NAME: &'static str = "NspAdded";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.WiMax";
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceWiMaxNspRemoved {
    pub nsp: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceWiMaxNspRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.nsp, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceWiMaxNspRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceWiMaxNspRemoved { nsp: i.read()? })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceWiMaxNspRemoved {
    const NAME: &'static str = "NspRemoved";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.WiMax";
}