use core::{ffi::CStr, str::FromStr};

use alloc::{ffi::CString, string::String, vec::Vec};

use crate::sys;

pub fn manual_fmt(_message: &CStr) -> Option<String> {
    const LEN: u32 = 300;
    let mut target = [0u8; LEN as usize];

    let written_length = unsafe { sys::snprintf(target.as_mut_ptr(), LEN, c"v1: %d".as_ptr(), 69) };
    if written_length < 0 {
        return Some(String::from("less than 0"));
    }
    if (written_length as u32) > LEN {
        return Some(String::from("not enough space"));
    }
    let bytes = Vec::<u8>::from_iter(target.into_iter().take(written_length as usize));
    String::from_utf8(bytes).ok()
}

#[macro_export]
macro_rules! fmt {
    ($format: literal, $($arg:tt)*) => {
        {
            const LEN: u32 = 300;
            let mut target = [0u8; LEN as usize];
            $crate::sys::snprintf(
                target.as_mut_ptr(),
                LEN,
                $format.as_ptr(),
                $($arg)*,
            );
        }
    }
}

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

// macro_rules! fmt {
//     ($($arg:tt)*) => {
//         $crate::__export::must_use({
//             $crate::fmt::format($crate::__export::format_args!($($arg)*))
//         })
//     }
// }

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
