use core::ffi::c_void;

use crate::service::global_callback::{Callback, CallbackHandle};
#[allow(unused)]
use crate::{GRect, log_c_str, service::global_callback::GlobalCallbacks, sys};

/// Allows you to subscribe to changes to the unobstructed area.
pub struct UnobstructedArea {
    callback: GlobalCallbacks<GRect>,
}

impl UnobstructedArea {
    pub(crate) const fn new() -> Self {
        Self {
            callback: GlobalCallbacks::new(),
        }
    }

    /// Adds a handler for unobstructed area events.
    /// The handler function receives the new unobstructed area.
    /// This function is a noop on platforms without unobstructed area functionality.
    pub fn subscribe(&self, handler: impl Into<Callback<GRect>>) -> CallbackHandle<GRect> {
        let handle = self.callback.add(handler.into());
        #[cfg(not(platform = "aplite"))]
        unsafe {
            sys::unobstructed_area_service_subscribe(
                sys::UnobstructedAreaHandlers {
                    did_change: None,
                    change: None,
                    will_change: Some(global_unobstructed_area_handler),
                },
                self.callback.as_void(),
            );
        }
        handle
    }

    /// Removes the unobstructed area change handler.
    pub fn unsubscribe(&self, handle: CallbackHandle<GRect>) {
        self.callback.remove(handle);
        if self.callback.is_empty() {
            #[cfg(not(platform = "aplite"))]
            unsafe {
                sys::unobstructed_area_service_unsubscribe();
            }
        }
    }
}

#[allow(unused)] // platforms which don’t have unobstructed area logic
unsafe extern "C" fn global_unobstructed_area_handler(rect: GRect, context: *mut c_void) {
    log_c_str(c"unobstructed_area received");
    unsafe {
        GlobalCallbacks::<(GRect,)>::dispatch_callback(context, (rect,));
    }
}
