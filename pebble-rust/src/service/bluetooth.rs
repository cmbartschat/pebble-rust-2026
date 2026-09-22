use alloc::boxed::Box;

use crate::{service::global_callback::GlobalCallback, sys};

/// Allows you to detect when the app gains or loses its Bluetooth connection.
pub struct BluetoothConnection;

// TODO: Should be a property on BluetoothConnection.
static HANDLER: GlobalCallback<bool, ()> = GlobalCallback::new();

impl BluetoothConnection {
    pub(crate) const fn new() -> Self {
        Self
    }

    /// Set the Bluetooth connection event handler.
    /// The callback function receives a boolean specifying whether the app has a connection or not.
    pub fn subscribe(&self, handler: Box<dyn FnMut(bool)>) {
        HANDLER.set(handler);
        unsafe {
            sys::bluetooth_connection_service_subscribe(Some(global_bluetooth_connection_handler));
        }
    }

    /// Remove the Bluetooth connection event handler.
    pub fn unsubscribe(&self) {
        unsafe { sys::bluetooth_connection_service_unsubscribe() }
        HANDLER.clear()
    }

    /// Returns whether the app currently has a Bluetooth connection.
    pub fn peek(&self) -> bool {
        unsafe { sys::bluetooth_connection_service_peek() }
    }
}

extern "C" fn global_bluetooth_connection_handler(event: bool) {
    HANDLER.dispatch(event);
}
