use crate::{
    service::{Callback, CallbackHandle, global_callback::GlobalCallbacks},
    sys,
};

/// Allows you to detect when the app gains or loses focus.
pub struct AppFocus;

static HANDLER: GlobalCallbacks<(bool,)> = GlobalCallbacks::new(|| unsafe {
    sys::app_focus_service_unsubscribe();
});

impl AppFocus {
    pub(crate) const fn new() -> Self {
        Self
    }

    /// Add a focus event handler.
    /// The callback function receives a boolean specifying whether the app has focus or not.
    /// The returned handle can be used to unsubscribe the callback from the events.
    pub fn subscribe(
        &'static self,
        handler: impl Into<Callback<(bool,)>>,
    ) -> CallbackHandle<'static, (bool,)> {
        let handle = HANDLER.add(handler.into());
        unsafe {
            // NOTE(christoph): Equivalent to sys::app_focus_service_subscribe
            sys::app_focus_service_subscribe_handlers(sys::AppFocusHandlers {
                will_focus: Some(global_will_focus_handler),
                did_focus: Some(global_did_focus_handler),
            });
        }
        handle
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
