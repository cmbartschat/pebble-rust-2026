use alloc::boxed::Box;

use crate::sys;

fn get_heap_alloc(data: &core::ffi::CStr) -> *const core::ffi::c_char {
    let mut vec = alloc::vec::Vec::<core::ffi::c_char>::new();
    vec.extend(data.to_bytes());
    vec.push(0);
    let start = vec.as_ptr();
    core::mem::forget(vec);
    start as *const core::ffi::c_char
}

pub fn log_str(message: &str) {
    unsafe {
        // sys::app_log(
        //     sys::AppLogLevel_APP_LOG_LEVEL_ERROR as u8,
        //     get_heap_alloc(c"filename.c"),
        //     1,
        //     get_heap_alloc(CString::from(message.into())),
        // );
        let data: Box<[core::ffi::c_char; 3]> = Box::new([0x41, 0x41, 0x0]);
        sys::app_log(
            200,
            data.as_ptr(),
            0,
            data.as_ptr(),
            c"my message".as_ptr() as *const core::ffi::c_char,
        );
        core::mem::forget(data);
    };
}
