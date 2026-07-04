use core::{cell::RefCell, ffi::c_void, ptr::NonNull, time::Duration};

use alloc::{boxed::Box, rc::Rc};

use crate::sys;

extern "C" fn global_timer_handler(target: *mut c_void) {
    unsafe {
        let callback = Box::from_raw(target as *mut Box<dyn FnOnce()>);
        callback();
    }
}

struct TimerInner {
    raw: NonNull<sys::AppTimer>,
}

impl TimerInner {
    unsafe fn from_ptr(ptr: *mut sys::AppTimer) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(ptr)?,
        })
    }

    fn cancel(self) {
        unsafe {
            sys::app_timer_cancel(self.raw.as_ptr());
        };
    }
}

#[derive(Clone)]
pub struct Timer {
    handle: Rc<RefCell<Option<TimerInner>>>,
}

impl Timer {
    fn from_inner(timer: TimerInner) -> Option<Self> {
        Some(Self {
            handle: Rc::new(RefCell::new(Some(timer))),
        })
    }

    fn once_inner(delay: Duration, callback: impl FnOnce() + 'static) -> Option<TimerInner> {
        let callback: Box<Box<dyn FnOnce()>> = Box::new(Box::new(callback));
        unsafe {
            let ptr = sys::app_timer_register(
                delay.as_millis().min(u32::MAX as u128) as u32,
                Some(global_timer_handler),
                Box::into_raw(callback) as *mut c_void,
            );

            TimerInner::from_ptr(ptr)
        }
    }

    pub fn once(delay: Duration, callback: impl FnOnce() + 'static) -> Option<Self> {
        Self::from_inner(Self::once_inner(delay, callback)?)
    }

    pub fn repeat(
        frequency: Duration,
        user_callback: impl FnMut() -> bool + 'static,
    ) -> Option<Self> {
        let _update_loop_ref = Rc::new(RefCell::<Box<dyn FnMut()>>::new(Box::new(|| {})));

        let res = Self {
            handle: Rc::new(RefCell::new(None)),
        };

        fn schedule_next(
            me: Timer,
            frequency: Duration,
            mut user_callback: impl FnMut() -> bool + 'static,
        ) {
            let new_inner = Timer::once_inner(frequency, {
                let me = me.clone();
                move || {
                    if !user_callback() {
                        return;
                    }
                    schedule_next(me, frequency, user_callback);
                }
            });
            *me.handle.borrow_mut() = new_inner;
        }

        schedule_next(res.clone(), frequency, user_callback);

        Some(res)
    }

    pub fn cancel(self) {
        if let Some(inner) = self.handle.borrow_mut().take() {
            inner.cancel();
        }
    }
}
