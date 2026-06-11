#!/bin/bash


cargo build --release
rm -rf tmp
mkdir tmp
cd tmp
/Users/cmb/Library/Application\ Support/Pebble\ SDK/SDKs/4.9.169/toolchain/arm-none-eabi/bin/arm-none-eabi-ar x "../target/thumbv7m-none-eabi/release/libpebble_rust_2026.a"


# cp target/thumbv7m-none-eabi/release/pebble-rust-2026 tmp/pebble-rust-2026.o
