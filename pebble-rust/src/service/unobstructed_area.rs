use core::ffi::c_void;

use alloc::boxed::Box;

use crate::{GRect, log_c_str, service::global_callback::GlobalCallback, sys};

pub struct UnobstructedArea {
    callback: GlobalCallback<GRect, ()>,
}

impl UnobstructedArea {
    pub const fn new() -> Self {
        Self {
            callback: GlobalCallback::new(),
        }
    }

    pub fn subscribe(&self, handler: Box<dyn FnMut(GRect)>) {
        self.callback.set(handler);
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
    }

    pub fn unsubscribe(&self) {
        unsafe { sys::unobstructed_area_service_unsubscribe() }
        self.callback.clear()
    }
}

unsafe extern "C" fn global_unobstructed_area_handler(rect: GRect, context: *mut c_void) {
    log_c_str(c"unobstructed_area received");
    unsafe {
        GlobalCallback::<GRect, ()>::dispatch_callback(context, rect);
    }
}
