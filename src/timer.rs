use core::{cell::RefCell, ffi::c_void, ptr::NonNull, time::Duration};

use alloc::{boxed::Box, rc::Rc};

use crate::{log::log_c_str, sys};

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
        mut user_callback: impl FnMut() -> bool + 'static,
    ) -> Option<Self> {
        let update_loop_ref = Rc::new(RefCell::<Box<dyn FnMut()>>::new(Box::new(|| {})));

        let res = Self {
            handle: Rc::new(RefCell::new(None)),
        };

        let handle_ref = res.handle.clone();
        let update_loop_ref_inner = update_loop_ref.clone();
        let update = Box::new(move || {
            if !user_callback() {
                return;
            }
            let inner = update_loop_ref_inner.clone();

            if let Some(new_inner) = Timer::once_inner(frequency, move || {
                inner.borrow_mut()();
            }) {
                handle_ref.borrow_mut().replace(new_inner);
            } else {
                log_c_str(c"repeating timer failed to schedule");
            }
        });

        {
            *update_loop_ref.borrow_mut() = update;
        }

        res.handle
            .borrow_mut()
            .replace(Timer::once_inner(frequency, move || {
                update_loop_ref.borrow_mut()();
            })?);

        Some(res)
    }

    pub fn cancel(self) {
        if let Some(inner) = self.handle.borrow_mut().take() {
            inner.cancel();
        }
    }
}
