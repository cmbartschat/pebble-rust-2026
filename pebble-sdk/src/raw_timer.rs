use core::{ffi::c_void, ptr::NonNull, time::Duration};

use crate::sys;

pub struct RawTimer {
    ptr: NonNull<sys::AppTimer>,
    data: *mut c_void,
    cleanup: fn(*mut c_void),
}

impl RawTimer {
    pub fn start(
        duration: Duration,
        data: *mut c_void,
        callback: sys::AppTimerCallback,
        cleanup: fn(*mut c_void),
    ) -> Option<Self> {
        let ptr = unsafe {
            sys::app_timer_register(
                duration.as_millis().min(u32::MAX as u128) as u32,
                callback,
                data,
            )
        };
        match NonNull::new(ptr) {
            Some(ptr) => Some(Self { ptr, data, cleanup }),
            None => {
                cleanup(data);
                None
            }
        }
    }

    pub fn cancel(self) {
        unsafe {
            sys::app_timer_cancel(self.ptr.as_ptr());
            (self.cleanup)(self.data)
        }
    }
}
