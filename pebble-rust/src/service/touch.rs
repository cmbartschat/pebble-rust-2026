use core::ffi::c_void;

use alloc::boxed::Box;

use crate::{GPoint, log_c_str, service::global_callback::GlobalCallback, sys};

/// Allows you to subscribe to touch events.
pub struct Touch {
    callback: GlobalCallback<TouchEvent, ()>,
}

impl Touch {
    pub(crate) const fn new() -> Self {
        Self {
            callback: GlobalCallback::new(),
        }
    }

    /// Returns whether touch is enabled.
    #[allow(clippy::missing_const_for_fn)] // only possible on non-touch platforms
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

    /// Sets or overwrites the handler for touch events.
    /// The handler function receives the touch event.
    /// This function is a noop on platforms without touch.
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

    /// Removes the touch event handler.
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

/// The different kinds of touch event.
pub enum TouchEvent {
    /// Touch started at the given coordinate.
    TouchDown(GPoint),
    /// Touch continued, and is currently at the given coordinate.
    TouchMove(GPoint),
    /// Touch ended at the given coordinate.
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
