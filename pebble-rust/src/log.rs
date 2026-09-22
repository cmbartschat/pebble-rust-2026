use core::ffi::CStr;

use crate::sys;

/// Log a message to the application log, with C formatting.
/// See the [developer docs](https://developer.repebble.com/guides/debugging/debugging-with-app-logs) for important information on logging.
/// See [`fmt`](crate::fmt) for C formatting caveats.
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

/// Log a string to the application log.
/// See the [developer docs](https://developer.repebble.com/guides/debugging/debugging-with-app-logs) for important information on logging.
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

/// Log a C string to the application log.
/// See the [developer docs](https://developer.repebble.com/guides/debugging/debugging-with-app-logs) for important information on logging.
pub fn log_c_str(message: &CStr) {
    unsafe {
        sys::app_log(200, c"".as_ptr(), 1, c"%s".as_ptr(), message.as_ptr());
    };
}
