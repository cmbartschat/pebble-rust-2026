# Pebble Rust (2026 edition)

Rust bindings and safe rust wrappers for the Pebble Smartwatch SDK.

## Getting Started

- See [this project](https://github.com/cmbartschat/pebble-64cores) for a working watchface.
- See [this demo app](https://github.com/cmbartschat/pebble-rust-demo) for demonstrations of additional features.

## Known Issues

- Some memory is leaked if a timer is cancelled before it fires
- The memory allocator doesn't respect alignment larger than 4, e.g. 8 byte alignment for u64, so things might misbehave there
- Only Emery (Pebble Time 2) is supported
- Using `format!()` or related Display traits causes a crash
