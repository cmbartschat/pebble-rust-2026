use core::ptr::null_mut;

use alloc::ffi::CString;

use crate::sys;

pub struct Time {
    value: sys::time_t,
}

impl Time {
    pub fn now() -> Self {
        unsafe {
            let value: sys::time_t = sys::time(null_mut());
            Self { value }
        }
    }

    pub fn to_local(&self) -> LocalTime {
        LocalTime {
            value: unsafe { sys::localtime(core::ptr::addr_of!(self.value)).read() },
        }
    }
}

pub struct LocalTime {
    value: sys::tm,
}

impl LocalTime {
    // pub fn from_time() -> Self {
    //     unsafe {
    //         let value: sys::time_t = sys::time(null_mut());
    //         Self { value }
    //     }
    // }

    pub fn second(&self) -> i32 {
        self.value.tm_sec
    }

    pub fn minute(&self) -> i32 {
        self.value.tm_min
    }

    pub fn hour(&self) -> i32 {
        self.value.tm_min
    }

    pub fn to_string(&self) -> CString {
        let mut buffer = [0; 50];
        let written = unsafe {
            sys::strftime(
                buffer.as_mut_ptr(),
                buffer.len(),
                if sys::clock_is_24h_style() {
                    c"%H:%M".as_ptr()
                } else {
                    c"%I:%M".as_ptr()
                },
                &self.value,
            )
        };
        if written == 0 {
            panic!("Time overflowed buffer");
        }
        CString::new(&buffer[0..written]).unwrap()
    }
}
