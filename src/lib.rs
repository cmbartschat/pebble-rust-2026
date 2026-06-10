#![no_main]
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unnecessary_transmutes)]
#![allow(unsafe_op_in_unsafe_fn)]

use core::{panic::PanicInfo, ptr::null};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[unsafe(no_mangle)]
pub fn main() -> isize {
    unsafe {
        let window = window_create();
        let root_layer = window_get_root_layer(window);
        let text_layer = text_layer_create(GRect {
            origin: GPoint { x: 0, y: 0 },
            size: GSize { w: 200, h: 100 },
        });
        let font = fonts_get_system_font(FONT_KEY_GOTHIC_24.as_ptr());
        text_layer_set_font(text_layer, font);
        text_layer_set_text(text_layer, c"hello world".as_ptr());
        text_layer_set_background_color(text_layer, GColor8 { argb: 0xf0 });
        text_layer_set_text_color(text_layer, GColor8 { argb: 0xa0 });

        let text_layer_inner = text_layer_get_layer(text_layer);
        layer_add_child(root_layer, text_layer_inner);
        window_stack_push(window, true);

        let filename = c"rust-lib.c".as_ptr();
        app_log(200, filename, 1, c"window created".as_ptr());
        app_event_loop();
        app_log(200, filename, 2, c"event loop ended".as_ptr());
        window_destroy(window);
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn _close(_fd: i32) -> i32 {
    -1
}

#[unsafe(no_mangle)]
pub extern "C" fn _fstat(_fd: i32, stat: *mut u8) -> i32 {
    unsafe {
        *(stat.add(0) as *mut u16) = 0x2000;
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
    // logs "panicked at '$reason', src/main.rs:27:4" to the host stderr
    // writeln!(host_stderr, "{}", info).ok();

    loop {}
}
