use core::{panic::PanicInfo, ptr::null};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use bindings::*;
