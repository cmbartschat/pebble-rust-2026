//! Automatically generated bindings to the Pebble C APIs.
//! See [the official documentation](https://developer.repebble.com/docs/c/) for details.

#![allow(
   // These are expected and compatible, since they were generated from the system header.
    suspicious_runtime_symbol_definitions,
    // Generated from C
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    //  unused,
    // Caused by bindgen
    clippy::all,
    unnecessary_transmutes,
    clippy::useless_transmute,
    unsafe_op_in_unsafe_fn,
    clippy::upper_case_acronyms,
    clippy::transmute_int_to_bool,
    clippy::ptr_offset_with_cast
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
