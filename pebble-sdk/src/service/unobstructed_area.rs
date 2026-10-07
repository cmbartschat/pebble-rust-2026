use core::ffi::c_void;

#[allow(unused)]
use crate::{GRect, service::global_callback::GlobalCallbacks, sys};
use crate::{
    service::global_callback::{Callback, CallbackHandle},
    trace,
};

/// Allows you to subscribe to changes to the unobstructed area.
pub struct UnobstructedArea {
    callback: GlobalCallbacks<(GRect,)>,
}

impl UnobstructedArea {
    pub(crate) const fn new() -> Self {
        Self {
            callback: GlobalCallbacks::new(|| {
                #[cfg(not(platform = "aplite"))]
                unsafe {
                    sys::unobstructed_area_service_unsubscribe();
                }
            }),
        }
    }

    /// Adds a handler for unobstructed area events.
    /// The handler function receives the new unobstructed area.
    /// This function is a noop on platforms without unobstructed area functionality.
    /// The returned handle can be used to unsubscribe the callback from the events.
    pub fn subscribe(
        &'static self,
        handler: impl Into<Callback<(GRect,)>>,
    ) -> CallbackHandle<'static, (GRect,)> {
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
}

#[allow(unused)] // platforms which don’t have unobstructed area logic
unsafe extern "C" fn global_unobstructed_area_handler(rect: GRect, context: *mut c_void) {
    trace!("unobstructed_area received");
    unsafe {
        GlobalCallbacks::<(GRect,)>::dispatch_callback(context, (rect,));
    }
}
