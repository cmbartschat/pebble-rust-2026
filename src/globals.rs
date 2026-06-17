extern crate alloc;

use core::panic::PanicInfo;

use alloc::{ffi::CString, vec::Vec};

use crate::log::log_c_str;

#[unsafe(no_mangle)]
pub extern "C" fn _close(_fd: i32) -> i32 {
    -1
}

#[unsafe(no_mangle)]
pub extern "C" fn _fstat(_fd: i32, stat: *mut u8) -> i32 {
    unsafe {
        *(stat as *mut u16) = 0x2000;
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn _isatty(_fd: i32) -> i32 {
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn _lseek(_fd: i32, _offset: i32, _whence: i32) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn _read(_fd: i32, _buf: *mut u8, _len: i32) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn _write(_fd: i32, _buf: *const u8, len: i32) -> i32 {
    len
}

#[unsafe(no_mangle)]
pub extern "C" fn _exit(_status: i32) -> ! {
    log_c_str(c"_exit called");
    #[allow(clippy::empty_loop)]
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _kill(_pid: i32, _sig: i32) -> i32 {
    -1
}

#[unsafe(no_mangle)]
pub extern "C" fn _getpid() -> i32 {
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn _sbrk(_incr: i32) -> *mut u8 {
    usize::MAX as *mut u8
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.message().as_str() {
        Some(e) => {
            log_c_str(c"panic called, message:");
            let bytes: Vec<_> = e.bytes().collect::<Vec<_>>();
            let str = CString::new(bytes).unwrap_or(CString::from(c"failed"));
            log_c_str(str.as_c_str());
        }
        None => {
            log_c_str(c"panic called, no message");
        }
    };

    match info.location() {
        Some(l) => log_c_str(l.file_as_c_str()),
        None => log_c_str(c"no location"),
    }

    loop {}
}
