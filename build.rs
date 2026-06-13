use std::env;
use std::io::Write as _;
use std::path::PathBuf;

fn main() {
    let home = std::env::var("HOME").unwrap();
    let sdk_root = format!("{home}/Library/Application Support/Pebble SDK/SDKs/4.9.169");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{sdk_root}/sdk-core/pebble/emery/include"))
        .clang_arg(format!(
            "--sysroot={sdk_root}/toolchain/arm-none-eabi/arm-none-eabi"
        ))
        .clang_arg("-D_TIME_H_")
        .clang_arg("-I/Users/cmb/repo/pebble-rust-2026")
        .clang_arg("--target=thumbv7em-none-eabi")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let output_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = output_dir.join("bindings.rs");

    std::fs::create_dir_all(output_dir).unwrap();

    let mut bindings_handle = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(bindings_path)
        .expect("Failed to open bindings_path");

    bindings_handle
        .write_all(
            b"#[allow(clippy::all)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
#[allow(unnecessary_transmutes)]
#[allow(clippy::useless_transmute)]
#[allow(unsafe_op_in_unsafe_fn)]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::transmute_int_to_bool)]
#[allow(clippy::ptr_offset_with_cast)]

mod bindings {
",
        )
        .unwrap();

    bindings.write(Box::new(&mut bindings_handle)).unwrap();
    bindings_handle.write_all(b"}\n").unwrap();
}
