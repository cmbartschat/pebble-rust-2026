use crate::{
    service::{Callback, CallbackHandle, global_callback::GlobalCallbacks},
    sys,
};

/// Accessor for the battery charge state.
pub struct BatteryState(());

static HANDLER: GlobalCallbacks<(BatteryChargeState,), ()> = GlobalCallbacks::new();

/// The actual state of the battery charge.
/// This has fields for the battery percentage, whether the battery is being charged, and whether it is plugged in.
pub type BatteryChargeState = sys::BatteryChargeState;

impl BatteryState {
    pub(crate) const fn new() -> Self {
        Self(())
    }

    /// Subscribe to battery state events.
    /// The handler is a function that receives the battery charge state.
    pub fn subscribe(
        &self,
        handler: impl Into<Callback<(BatteryChargeState,)>>,
    ) -> CallbackHandle<(BatteryChargeState,)> {
        let handle = HANDLER.add(handler);
        unsafe {
            sys::battery_state_service_subscribe(Some(global_battery_handler));
        }
        handle
    }

    /// Unsubscribe the handler from battery state events.
    pub fn unsubscribe(&self, handle: CallbackHandle<(BatteryChargeState,)>) {
        HANDLER.remove(handle);
        if HANDLER.is_empty() {
            unsafe {
                sys::battery_state_service_unsubscribe();
            }
        }
    }

    /// Retrieve the current battery charge state.
    pub fn peek(&self) -> BatteryChargeState {
        unsafe { sys::battery_state_service_peek() }
    }
}

extern "C" fn global_battery_handler(event: sys::BatteryChargeState) {
    HANDLER.dispatch((event,));
}
