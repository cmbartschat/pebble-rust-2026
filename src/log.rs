use alloc::{boxed::Box, slice};

use crate::sys;

fn get_heap_alloc(data: &str) -> Box<[core::ffi::c_char]> {
    data.bytes()
        .map(|b| b as core::ffi::c_char)
        .take(6)
        .chain(core::iter::once(0))
        .collect::<alloc::vec::Vec<_>>()
        .into_boxed_slice()
}

pub fn log_str(message: &str) {
    unsafe {
        let filename = get_heap_alloc("idk.c");
        let format = get_heap_alloc("%s");
        let message = get_heap_alloc(message);
        // sys::app_log(200, filename.as_ptr(), 1, format.as_ptr(), message.as_ptr());
        // core::mem::forget((filename, format, message));
    };
}

pub fn log_num(v: core::ffi::c_int) {
    unsafe {
        let filename = get_heap_alloc("num.c");
        let format = get_heap_alloc("%s");
        let message = get_heap_alloc("");
        sys::app_log(200, filename.as_ptr(), v, format.as_ptr(), message.as_ptr());
        // core::mem::forget((filename, format, message));
    };
}
