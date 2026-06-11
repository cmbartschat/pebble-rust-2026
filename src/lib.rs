#![no_main]
#![no_std]
#![allow(unused)]
#![allow(clippy::empty_loop)]

extern crate alloc;

use core::panic::PanicInfo;

mod bitmap;
mod color;
mod context;
mod custom_alloc;
mod layer;
mod sys;
mod text_layer;
mod window;

use crate::bitmap::GBitmap;
use crate::color::{GCOLOR_BLUE_MOON, GCOLOR_RED, GCOLOR_WHITE};
use crate::sys::{GPoint, GRect, GSize};
use crate::window::Window;

use crate::text_layer::TextLayer;

#[unsafe(no_mangle)]
pub fn main() -> isize {
    let Ok(mut window) = Window::new() else {
        return -1;
    };

    window.set_background_color(GCOLOR_BLUE_MOON);

    let bounds = GRect {
        origin: GPoint { x: 10, y: 10 },
        size: GSize { w: 180, h: 100 },
    };

    let font = unsafe { sys::fonts_get_system_font(sys::FONT_KEY_GOTHIC_24.as_ptr()) };

    let Ok(mut text_layer) = TextLayer::new(bounds) else {
        return -1;
    };
    window.add_child(&text_layer.get_layer());

    text_layer.set_font(font);
    text_layer.set_text("Hello World.");
    text_layer.set_background_color(GCOLOR_RED);
    text_layer.set_text_color(GCOLOR_WHITE);

    // if let Ok(bitmap) = GBitmap::from_resource(0) {
    //     if let Ok(sub_bitmap) = bitmap.extract(GRect {
    //         origin: GPoint { x: 0, y: 0 },
    //         size: GSize { w: 20, h: 20 },
    //     }) {
    //         drop(bitmap);
    //         // sub_bitmap.draw(
    //         sub_bitmap.extract(GRect {
    //             origin: GPoint { x: 0, y: 0 },
    //             size: GSize { w: 20, h: 20 },
    //         });
    //     };
    // }

    window.push_animated();

    unsafe { sys::app_event_loop() };
    0
}

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
fn panic(_info: &PanicInfo) -> ! {
    // logs "panicked at '$reason', src/main.rs:27:4" to the host stderr
    // writeln!(host_stderr, "{}", info).ok();
    loop {}
}

#[global_allocator]
static ALLOC: crate::custom_alloc::Allocator = crate::custom_alloc::Allocator;

// #[alloc_error_handler]
// pub fn error_handler(_layout: core::alloc::Layout) -> ! {
//     loop {}
// }
