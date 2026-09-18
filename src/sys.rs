// These are expected and compatible, since they were generated from the system header.
#![allow(suspicious_runtime_symbol_definitions)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use bindings::*;
