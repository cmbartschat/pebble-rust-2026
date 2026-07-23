use alloc::boxed::Box;

use crate::{service::global_callback::GlobalCallback, sys};

pub struct AppFocus;

static HANDLER: GlobalCallback<bool, ()> = GlobalCallback::new();

impl AppFocus {
    pub const fn new() -> Self {
        Self
    }

    pub fn subscribe(&self, handler: Box<dyn FnMut(bool)>) {
        HANDLER.set(handler);
        unsafe {
            // NOTE(christoph): Equivalent to sys::app_focus_service_subscribe
            sys::app_focus_service_subscribe_handlers(sys::AppFocusHandlers {
                will_focus: Some(global_will_focus_handler),
                did_focus: Some(global_did_focus_handler),
            });
        }
    }

    pub fn unsubscribe(&self) {
        unsafe { sys::app_focus_service_unsubscribe() }
        HANDLER.clear()
    }
}

extern "C" fn global_will_focus_handler(focused: bool) {
    if focused {
        HANDLER.dispatch(focused);
    }
}

extern "C" fn global_did_focus_handler(focused: bool) {
    if !focused {
        HANDLER.dispatch(focused);
    }
}
