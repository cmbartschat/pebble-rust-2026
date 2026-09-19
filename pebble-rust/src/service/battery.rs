use alloc::boxed::Box;

use crate::{service::global_callback::GlobalCallback, sys};

pub struct BatteryState;

static HANDLER: GlobalCallback<BatteryChargeState, ()> = GlobalCallback::new();

pub type BatteryChargeState = sys::BatteryChargeState;

impl BatteryState {
    pub(crate) const fn new() -> Self {
        Self
    }

    pub fn subscribe(&self, handler: Box<dyn FnMut(BatteryChargeState)>) {
        HANDLER.set(handler);
        unsafe {
            sys::battery_state_service_subscribe(Some(global_battery_handler));
        }
    }

    pub fn unsubscribe(&self) {
        unsafe { sys::battery_state_service_unsubscribe() }
        HANDLER.clear()
    }

    pub fn peek(&self) -> BatteryChargeState {
        unsafe { sys::battery_state_service_peek() }
    }
}

extern "C" fn global_battery_handler(event: sys::BatteryChargeState) {
    HANDLER.dispatch(event);
}
