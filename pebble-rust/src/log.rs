use core::ffi::CStr;

use crate::sys;

#[macro_export]
macro_rules! log_fmt {
    ($format: literal, $($arg:tt)*) => {
        {
            $crate::sys::app_log(
                200,
                c"".as_ptr(),
                1,
                $format.as_ptr(),
                $($arg)*,
            );
        }
    }
}

pub fn log_str(message: &str) {
    unsafe {
        sys::app_log(
            200,
            c"".as_ptr(),
            1,
            c"%.*s".as_ptr(),
            message.len() as u32,
            message.as_ptr(),
        );
    };
}

pub fn log_c_str(message: &CStr) {
    unsafe {
        sys::app_log(200, c"".as_ptr(), 1, c"%s".as_ptr(), message.as_ptr());
    };
}
