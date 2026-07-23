use alloc::boxed::Box;

use crate::{service::global_callback::GlobalCallback, sys};

pub struct BluetoothConnection;

static HANDLER: GlobalCallback<bool, ()> = GlobalCallback::new();

impl BluetoothConnection {
    pub const fn new() -> Self {
        Self
    }

    pub fn subscribe(&self, handler: Box<dyn FnMut(bool)>) {
        HANDLER.set(handler);
        unsafe {
            sys::bluetooth_connection_service_subscribe(Some(global_bluetooth_connection_handler));
        }
    }

    pub fn unsubscribe(&self) {
        unsafe { sys::bluetooth_connection_service_unsubscribe() }
        HANDLER.clear()
    }

    pub fn peek(&self) -> bool {
        unsafe { sys::bluetooth_connection_service_peek() }
    }
}

extern "C" fn global_bluetooth_connection_handler(event: bool) {
    HANDLER.dispatch(event);
}
