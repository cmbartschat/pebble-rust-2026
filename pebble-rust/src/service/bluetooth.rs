use crate::{
    service::{Callback, CallbackHandle, global_callback::GlobalCallbacks},
    sys,
};

/// Allows you to detect when the app gains or loses its Bluetooth connection.
pub struct BluetoothConnection;

static HANDLER: GlobalCallbacks<(bool,)> = GlobalCallbacks::new(|| unsafe {
    sys::bluetooth_connection_service_unsubscribe();
});

impl BluetoothConnection {
    pub(crate) const fn new() -> Self {
        Self
    }

    /// Add a Bluetooth connection event handler.
    /// The callback function receives a boolean specifying whether the app has a connection or not.
    /// The returned handle can be used to unsubscribe the callback from the events.
    pub fn subscribe(
        &'static self,
        handler: impl Into<Callback<(bool,)>>,
    ) -> CallbackHandle<'static, (bool,)> {
        let handle = HANDLER.add(handler.into());
        unsafe {
            sys::bluetooth_connection_service_subscribe(Some(global_bluetooth_connection_handler));
        }
        handle
    }

    /// Returns whether the app currently has a Bluetooth connection.
    pub fn peek(&self) -> bool {
        unsafe { sys::bluetooth_connection_service_peek() }
    }
}

extern "C" fn global_bluetooth_connection_handler(event: bool) {
    HANDLER.dispatch((event,));
}
