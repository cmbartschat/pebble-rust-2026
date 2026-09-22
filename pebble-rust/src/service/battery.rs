use alloc::boxed::Box;

use crate::{service::global_callback::GlobalCallback, sys};

/// Accessor for the battery charge state.
pub struct BatteryState(());

static HANDLER: GlobalCallback<BatteryChargeState, ()> = GlobalCallback::new();

/// The actual state of the battery charge.
/// This has fields for the battery percentage, whether the battery is being charged, and whether it is plugged in.
pub type BatteryChargeState = sys::BatteryChargeState;

impl BatteryState {
    pub(crate) const fn new() -> Self {
        Self(())
    }

    /// Subscribe to battery state events.
    /// The handler is a function that receives the battery charge state.
    pub fn subscribe(&self, handler: Box<dyn FnMut(BatteryChargeState)>) {
        HANDLER.set(handler);
        unsafe {
            sys::battery_state_service_subscribe(Some(global_battery_handler));
        }
    }

    /// Unsubscribe from battery state events.
    /// This overrides any previous handler that is subscribed to these events.
    pub fn unsubscribe(&self) {
        unsafe { sys::battery_state_service_unsubscribe() }
        HANDLER.clear()
    }

    /// Retrieve the current battery charge state.
    pub fn peek(&self) -> BatteryChargeState {
        unsafe { sys::battery_state_service_peek() }
    }
}

extern "C" fn global_battery_handler(event: sys::BatteryChargeState) {
    HANDLER.dispatch(event);
}
