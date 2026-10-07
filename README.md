> [!NOTE]
> Migrated to https://codeberg.org/pebble-rust/pebble-sdk

# Pebble Rust SDK

Rust bindings and safe rust wrappers for the Pebble Smartwatch SDK.

## Getting Started

- See [this project](https://github.com/cmbartschat/pebble-64cores) for a working watchface.
- See [the examples](./examples) for small example applications.
- See [this demo app](https://github.com/cmbartschat/pebble-rust-demo) for a larger demonstration.

To create a basic skeleton, you can use [`cargo pebble new`](https://codeberg.org/filmroellchen/cargo-pebble/). All in all:

```shell
uv tool install pebble-tool
pebble sdk install latest
# You can also use `cargo binstall`, which is faster
cargo install cargo-repebble
cargo install cargo-generate
cargo pebble new my-awesome-app
```

The information below is mainly relevant for manual setups, or if you want to understand the structure of the template and the examples. It is not important for running your first line of code :)

### Selecting the target platform (watch model)

`pebble-sdk` supports all [Pebble platforms](https://developer.repebble.com/guides/tools-and-resources/hardware-information/). Some features are only available on some platforms, and some functionality is a noop on certain platforms (e.g. setting touch handlers on platforms which don’t have touch input). For these reasons, it is required to set the target platform during compilation, or the build will error out. Note that compiling for more than one platform at once is _not_ possible. For multi-platform support, you have to compile your application multiple times, once for each target.

There are two main ways of selecting a platform to compile for:

- Via the crate features `aplite`, `basalt`, `chalk`, `diorite`, `emery`, `flint`, `gabbro`. These each specify a certain platform to be used. Again, specifying more than one of these features is not supported and may lead to unexpected results.
- Via the environment variable `PEBBLE_PLATFORM=<platform name>`.

### Setting a Global Allocator

`pebble-sdk` does not work without a global allocator. A lot of functionality needs to perform (usually small) allocations as part of its normal operation. There are two available allocators: a malloc-based one, selected via `malloc-allocator` (enabled by default), or one based on embedded-alloc, selected via `embedded-allocator`. The malloc allocator is easiest to use and recommended as a start. Simply add these lines to the top of your `lib.rs`:

```rust,ignore
use pebble_sdk::MallocAllocator;

#[global_allocator]
static ALLOCATOR: MallocAllocator = MallocAllocator;
```

If you use a different allocator, including a fully custom one, make sure to fully initialize it before calling into any library functionality, at the very start of your main function.

### Entry point and Panic Handling

`pebble-sdk` requires a C-like entry point to be declared:

```rust,ignore
#[unsafe(no_mangle)]
fn main() -> i32 {
    // Application logic goes here...
    0
}
```

Also, make sure to `use pebble_sdk as _;`, which ensures the panic handling machinery is present.

## Known Issues

- Using `format!()` or related Display traits frequently causes a crash, use `fmt!` to convert data to strings, or the log::info, etc macros for logging.
- Without some call to `APP.event_loop()`, the application is likely to not build properly or crash immediately on startup.
