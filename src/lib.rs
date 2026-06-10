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
        printf(null());
    }
    0
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // logs "panicked at '$reason', src/main.rs:27:4" to the host stderr
    // writeln!(host_stderr, "{}", info).ok();

    loop {}
}
