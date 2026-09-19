use core::ffi::c_void;

use std_alloc::boxed::Box;

use crate::{GPoint, log_c_str, service::global_callback::GlobalCallback, sys};

pub struct Touch {
    callback: GlobalCallback<TouchEvent, ()>,
}

impl Touch {
    pub const fn new() -> Self {
        Self {
            callback: GlobalCallback::new(),
        }
    }

    pub fn is_enabled(&self) -> bool {
        #[cfg(not(any(
            platform = "aplite",
            platform = "basalt",
            platform = "chalk",
            platform = "diorite"
        )))]
        return unsafe { sys::touch_service_is_enabled() };

        #[allow(unreachable_code)]
        false
    }

    pub fn subscribe(&self, handler: Box<dyn FnMut(TouchEvent)>) {
        self.callback.set(handler);
        // No touch on these platforms, therefore subscribing to touch events is a noop.
        #[cfg(not(any(
            platform = "aplite",
            platform = "basalt",
            platform = "chalk",
            platform = "diorite"
        )))]
        {
            unsafe {
                sys::touch_service_subscribe(Some(global_touch_handler), self.callback.as_void());
            }
        }
    }

    pub fn unsubscribe(&self) {
        #[cfg(not(any(
            platform = "aplite",
            platform = "basalt",
            platform = "chalk",
            platform = "diorite"
        )))]
        {
            unsafe { sys::touch_service_unsubscribe() }
        }
        self.callback.clear()
    }
}

#[allow(unused)] // platforms which don’t have touch
extern "C" fn global_touch_handler(event: *const sys::TouchEvent, context: *mut c_void) {
    log_c_str(c"touch received");
    unsafe {
        let event = TouchEvent::try_from(event.as_ref().unwrap()).unwrap();
        GlobalCallback::<TouchEvent, ()>::dispatch_callback(context, event);
    }
}

pub enum TouchEvent {
    TouchDown(GPoint),
    TouchMove(GPoint),
    TouchUp(GPoint),
}

impl TryFrom<&sys::TouchEvent> for TouchEvent {
    type Error = ();
    fn try_from(value: &sys::TouchEvent) -> Result<Self, Self::Error> {
        let point = GPoint::new(value.x, value.y);
        Ok(match value.type_() {
            sys::TouchEventType_TouchEvent_Touchdown => Self::TouchDown(point),
            sys::TouchEventType_TouchEvent_Liftoff => Self::TouchUp(point),
            sys::TouchEventType_TouchEvent_PositionUpdate => Self::TouchMove(point),
            _ => return Err(()),
        })
    }
}
