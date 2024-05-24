use dbus::blocking::{Proxy, SyncConnection};
use std::time::Duration;

const DBUS_TIMEOUT_MS: u64 = 5000;

macro_rules! proxy {
    ($input:ident) => {
        $input.dbus_accessor.create_proxy()
    };
}

pub(crate) struct DBusAccessor<'a> {
    pub(crate) connection: &'a SyncConnection,
    pub(crate) bus: String,
    pub(crate) path: String,
}

impl<'a> DBusAccessor<'a> {
    pub(crate) fn new(connection: &'a SyncConnection, bus: &str, path: &str) -> Self {
        DBusAccessor {
            connection,
            bus: bus.to_owned(),
            path: path.to_owned(),
        }
    }
    pub(crate) fn create_proxy(&self) -> Proxy<'_, &SyncConnection> {
        self.connection.with_proxy(
            &self.bus,
            &self.path,
            Duration::from_millis(DBUS_TIMEOUT_MS),
        )
    }
}
