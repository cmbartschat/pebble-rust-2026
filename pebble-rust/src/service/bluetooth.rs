use crate::{
    service::{Callback, CallbackHandle, global_callback::GlobalCallbacks},
    sys,
};

/// Allows you to detect when the app gains or loses its Bluetooth connection.
pub struct BluetoothConnection;

static HANDLER: GlobalCallbacks<(bool,), ()> = GlobalCallbacks::new();

impl BluetoothConnection {
    pub(crate) const fn new() -> Self {
        Self
    }

    /// Add a Bluetooth connection event handler.
    /// The callback function receives a boolean specifying whether the app has a connection or not.
    pub fn subscribe(&self, handler: impl Into<Callback<(bool,)>>) -> CallbackHandle<(bool,)> {
        let handle = HANDLER.add(handler);
        unsafe {
            sys::bluetooth_connection_service_subscribe(Some(global_bluetooth_connection_handler));
        }
        handle
    }

    /// Remove a Bluetooth connection event handler.
    pub fn unsubscribe(&self, handle: CallbackHandle<(bool,)>) {
        HANDLER.remove(handle);
        if HANDLER.is_empty() {
            unsafe {
                sys::bluetooth_connection_service_unsubscribe();
            }
        }
    }

    /// Returns whether the app currently has a Bluetooth connection.
    pub fn peek(&self) -> bool {
        unsafe { sys::bluetooth_connection_service_peek() }
    }
}

extern "C" fn global_bluetooth_connection_handler(event: bool) {
    HANDLER.dispatch((event,));
}
