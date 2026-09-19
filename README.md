# Pebble Rust (2026 edition)

Rust bindings and safe rust wrappers for the Pebble Smartwatch SDK.

## Getting Started

- See [this project](https://github.com/cmbartschat/pebble-64cores) for a working watchface.
- See [this demo app](https://github.com/cmbartschat/pebble-rust-demo) for demonstrations of additional features.

### Selecting the target platform (watch model)

`pebble-rust-2026` supports all [Pebble platforms](https://developer.repebble.com/guides/tools-and-resources/hardware-information/). Some features are only available on some platforms, and some functionality is a noop on certain platforms (e.g. setting touch handlers on platforms which don’t have touch input). For these reasons, it is required to set the target platform during compilation, or the build will error out. Note that compiling for more than one platform at once is _not_ possible. For multi-platform support, you have to compile your application multiple times, once for each target.

There are two main ways of selecting a platform to compile for:

- Via the crate features `aplite`, `basalt`, `chalk`, `diorite`, `emery`, `flint`, `gabbro`. These each specify a certain platform to be used. Again, specifying more than one of these features is not supported and may lead to unexpected results.
- Via the environment variable `PEBBLE_PLATFORM=<platform name>`.

## Known Issues

- Using `format!()` or related Display traits frequently causes a crash, use `fmt` or `log_fmt` to convert data to strings.
