use dbus::blocking::{Proxy, SyncSyncConnection};
use std::time::Duration;

const DBUS_TIMEOUT_MS: u64 = 5000;

macro_rules! proxy {
    ($input:ident) => {
        $input.dbus_accessor.create_proxy()
    };
}

pub(crate) struct DBusAccessor<'a> {
    pub(crate) connection: &'a SyncSyncConnection,
    pub(crate) bus: String,
    pub(crate) path: String,
}

impl<'a> DBusAccessor<'a> {
    pub(crate) fn new(connection: &'a SyncSyncConnection, bus: &str, path: &str) -> Self {
        DBusAccessor {
            connection,
            bus: bus.to_owned(),
            path: path.to_owned(),
        }
    }
    pub(crate) fn create_proxy(&self) -> Proxy<'_, &SyncSyncConnection> {
        self.connection.with_proxy(
            &self.bus,
            &self.path,
            Duration::from_millis(DBUS_TIMEOUT_MS),
        )
    }
}
