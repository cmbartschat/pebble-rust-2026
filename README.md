# Pebble Rust (2026 edition)

Rust bindings and safe rust wrappers for the Pebble Smartwatch SDK.

## Getting Started

See [this project](https://github.com/cmbartschat/pebble-64cores) for a working example.

## Known Issues

- Some memory is leaked in windows and cancelled timers
- The global allocator doesn't respect alignment, so alignments about 8 bytes may be misaligned
- Only Emery (Pebble Time 2) is supported
