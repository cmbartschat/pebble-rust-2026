#[macro_export]
macro_rules! fmt {
    ($format: literal, $($arg:tt)*) => {
        {
            use alloc::{string::String, vec::Vec};
            const LEN: u32 = 300;
            let mut target = [0u8; LEN as usize];
            let written = $crate::sys::snprintf(
                target.as_mut_ptr(),
                LEN,
                $format.as_ptr(),
                $($arg)*,
            );
            if written < 0 {
                 None
            } else {
                let written = written as usize;
                if written > target.len() {
                     None
                } else {
                    let bytes = Vec::<u8>::from_iter(target.iter().take(written).copied());
                    String::from_utf8(bytes).ok()
                }
            }
        }
    }
}

#[macro_export]
macro_rules! fmt_c_str {
    ($format: literal, $($arg:tt)*) => {
        {
            use alloc::{ffi::CString, vec::Vec};

            const LEN: u32 = 300;
            let mut target = [0u8; LEN as usize];
            let written = $crate::sys::snprintf(
                target.as_mut_ptr(),
                LEN,
                $format.as_ptr(),
                $($arg)*,
            );
            if written < 0 {
                 None
            } else {
                let written = written as usize;
                if written >= target.len() {
                    None
                } else {
                    target[written] = 0;
                    let bytes = Vec::<u8>::from_iter(target.iter().take(written + 1).copied());
                    CString::from_vec_with_nul(bytes).ok()
                }
            }
        }
    }
}
