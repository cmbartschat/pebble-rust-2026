use core::{ffi::c_void, time::Duration};

use alloc::{boxed::Box, rc::Rc};

use crate::{
    handle::{Handle, WeakHandle, new_handle},
    log_c_str,
    raw_timer::RawTimer,
};

struct OnceContext {
    callback: Option<Box<dyn FnOnce()>>,
    handle: WeakHandle<Option<RawTimer>>,
}

impl OnceContext {
    pub fn dispatch(mut self) {
        if let Some(handle) = self.handle.upgrade() {
            handle.take();
        }
        let Some(callback) = self.callback.take() else {
            log_c_str(c"Unexpected: OnceContext dispatch missing callback");
            return;
        };

        callback();
    }
}

struct RepeatContext {
    frequency: Duration,
    callback: Box<dyn FnMut() -> bool>,
    handle: WeakHandle<Option<RawTimer>>,
}

enum RepeatState {
    Continuing,
    Stopped,
}

impl RepeatContext {
    pub fn dispatch(&mut self) -> RepeatState {
        let handle = self.handle.upgrade();

        if let Some(handle) = handle {
            handle.take();
        }

        let should_repeat = self.callback.as_mut()();
        if !should_repeat {
            return RepeatState::Stopped;
        }

        let handle = self.handle.upgrade();
        let new_timer: Option<RawTimer> = RawTimer::start(
            self.frequency,
            self as *mut RepeatContext as *mut c_void,
            Some(global_timer_repeat_handler),
            drop_timer_repeat_context,
        );

        if new_timer.is_none() {
            log_c_str(c"Unexpected: repeated timer failed to repeat");
            return RepeatState::Stopped;
        }

        if let Some(handle) = handle {
            handle.replace(new_timer);
        }

        RepeatState::Continuing
    }
}

#[derive(Clone)]
pub struct Timer {
    handle: Handle<Option<RawTimer>>,
}

impl Timer {
    pub fn once(delay: Duration, callback: impl FnOnce() + 'static) -> Option<Self> {
        let handle = new_handle(None);
        let context = Box::new(OnceContext {
            callback: Some(Box::new(callback)),
            handle: Rc::downgrade(&handle),
        });

        let raw = RawTimer::start(
            delay,
            Box::into_raw(context) as *mut c_void,
            Some(global_timer_once_handler),
            drop_timer_once_context,
        )?;

        *handle.borrow_mut() = Some(raw);
        Some(Self { handle })
    }

    pub fn repeat<F>(frequency: Duration, callback: F) -> Option<Self>
    where
        F: FnMut() -> bool + 'static,
    {
        let handle = new_handle(None);
        let context = Box::new(RepeatContext {
            callback: Box::new(callback),
            handle: Rc::downgrade(&handle),
            frequency,
        });

        let raw = RawTimer::start(
            frequency,
            Box::into_raw(context) as *mut c_void,
            Some(global_timer_repeat_handler),
            drop_timer_repeat_context,
        )?;

        *handle.borrow_mut() = Some(raw);
        Some(Self { handle })
    }

    pub fn cancel(self) {
        if let Some(inner) = self.handle.borrow_mut().take() {
            inner.cancel();
        }
    }
}

extern "C" fn global_timer_once_handler(data: *mut c_void) {
    unsafe {
        let data = Box::from_raw(data as *mut OnceContext);
        data.dispatch();
    }
}

fn drop_timer_once_context(data: *mut c_void) {
    drop(unsafe { Box::from_raw(data as *mut OnceContext) });
}

extern "C" fn global_timer_repeat_handler(data: *mut c_void) {
    match unsafe {
        let Some(data) = (data as *mut RepeatContext).as_mut() else {
            log_c_str(c"Unexpected: Repeat handler called without context");
            return;
        };
        data.dispatch()
    } {
        RepeatState::Continuing => {}
        RepeatState::Stopped => drop_timer_repeat_context(data),
    }
}

fn drop_timer_repeat_context(data: *mut c_void) {
    drop(unsafe { Box::from_raw(data as *mut RepeatContext) });
}
