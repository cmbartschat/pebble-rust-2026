use crate::{
    service::{Callback, CallbackHandle, global_callback::GlobalCallbacks},
    sys,
};

/// Allows you to detect when the app gains or loses focus.
pub struct AppFocus;

static HANDLER: GlobalCallbacks<(bool,), ()> = GlobalCallbacks::new();

impl AppFocus {
    pub(crate) const fn new() -> Self {
        Self
    }

    /// Add a focus event handler.
    /// The callback function receives a boolean specifying whether the app has focus or not.
    pub fn subscribe(&self, handler: impl Into<Callback<(bool,)>>) -> CallbackHandle<(bool,)> {
        let handle = HANDLER.add(handler);
        unsafe {
            // NOTE(christoph): Equivalent to sys::app_focus_service_subscribe
            sys::app_focus_service_subscribe_handlers(sys::AppFocusHandlers {
                will_focus: Some(global_will_focus_handler),
                did_focus: Some(global_did_focus_handler),
            });
        }
        handle
    }

    /// Remove the focus event handler.
    pub fn unsubscribe(&self, handle: CallbackHandle<(bool,)>) {
        HANDLER.remove(handle);
    }
}

extern "C" fn global_will_focus_handler(focused: bool) {
    if focused {
        HANDLER.dispatch((focused,));
    }
}

extern "C" fn global_did_focus_handler(focused: bool) {
    if !focused {
        HANDLER.dispatch((focused,));
    }
}
