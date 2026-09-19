//! Automatically generated bindings to the Pebble C APIs.
//! See [the official documentation](https://developer.repebble.com/docs/c/) for details.

// These are expected and compatible, since they were generated from the system header.
#![allow(suspicious_runtime_symbol_definitions, clippy::missing_const_for_fn)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use bindings::*;
