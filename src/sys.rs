#![no_main]
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unnecessary_transmutes)]
#![allow(clippy::useless_transmute)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::transmute_int_to_bool)]

use core::{panic::PanicInfo, ptr::null};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
