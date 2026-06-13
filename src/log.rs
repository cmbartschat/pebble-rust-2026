use core::ffi::CStr;

use crate::sys;

pub fn log_c_str(message: &CStr) {
    unsafe {
        sys::app_log(200, c"".as_ptr(), 1, c"%s".as_ptr(), message.as_ptr());
    };
}
