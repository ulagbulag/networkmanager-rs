// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceMacsec {
    fn parent(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn sci(&self) -> Result<u64, dbus::Error>;
    fn icv_length(&self) -> Result<u8, dbus::Error>;
    fn cipher_suite(&self) -> Result<u64, dbus::Error>;
    fn window(&self) -> Result<u32, dbus::Error>;
    fn encoding_sa(&self) -> Result<u8, dbus::Error>;
    fn validation(&self) -> Result<String, dbus::Error>;
    fn encrypt(&self) -> Result<bool, dbus::Error>;
    fn protect(&self) -> Result<bool, dbus::Error>;
    fn include_sci(&self) -> Result<bool, dbus::Error>;
    fn es(&self) -> Result<bool, dbus::Error>;
    fn scb(&self) -> Result<bool, dbus::Error>;
    fn replay_protect(&self) -> Result<bool, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target = blocking::SyncSyncConnection>>
    OrgFreedesktopNetworkManagerDeviceMacsec for blocking::Proxy<'a, C>
{
    fn parent(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "Parent",
        )
    }

    fn sci(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "Sci",
        )
    }

    fn icv_length(&self) -> Result<u8, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "IcvLength",
        )
    }

    fn cipher_suite(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "CipherSuite",
        )
    }

    fn window(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "Window",
        )
    }

    fn encoding_sa(&self) -> Result<u8, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "EncodingSa",
        )
    }

    fn validation(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "Validation",
        )
    }

    fn encrypt(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "Encrypt",
        )
    }

    fn protect(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "Protect",
        )
    }

    fn include_sci(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "IncludeSci",
        )
    }

    fn es(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "Es",
        )
    }

    fn scb(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "Scb",
        )
    }

    fn replay_protect(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Macsec",
            "ReplayProtect",
        )
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceMacsecPropertiesChanged {
    pub properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceMacsecPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceMacsecPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceMacsecPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceMacsecPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Macsec";
}
