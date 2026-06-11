use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory

    // let sdk_root = std::env::var("PEBBLE_SDK_PATH").unwrap_or_else(|_| {
    //     format!(
    //         "{}/Library/Application Support/Pebble SDK/SDKs/4.9.169/sdk-core/pebble/basalt",
    //         std::env::var("HOME").unwrap()
    //     )
    // });

    let sdk_root = std::env::var("PEBBLE_SDK_PATH").unwrap_or_else(|_| {
        format!(
            "{}/Library/Application Support/Pebble SDK/SDKs/4.9.169/sdk-core/pebble/basalt",
            std::env::var("HOME").unwrap()
        )
    });

    // println!("cargo:rustc-link-search={}/lib", sdk_root);
    // println!("cargo:rustc-link-lib=pebble");

    // let toolchain1 = "/Users/cmb/Library/Application Support/Pebble SDK/SDKs/4.9.169/toolchain/arm-none-eabi/arm-none-eabi/include/ssp/";
    let toolchain2 = "/Users/cmb/Library/Application Support/Pebble SDK/SDKs/4.9.169/toolchain/arm-none-eabi/arm-none-eabi/include";

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}/include", sdk_root))
        // .clang_arg(format!("-I{}", toolchain1))
        .clang_arg(format!("-I{}", toolchain2))
        .clang_arg(
            "--sysroot=/Users/cmb/Library/Application Support/Pebble SDK/SDKs/4.9.169/toolchain/arm-none-eabi/arm-none-eabi"
        )
        // .clang_arg("-D__stdio_h")
        .clang_arg("-D_TIME_H_")
        //  .blocklist_type("tm")  
        .clang_arg("-I/Users/cmb/repo/pebble-rust-2026")
        .clang_arg("--target=thumbv7m-none-eabi")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut generated_bindings: Vec<u8> = r#"
#[allow(clippy::all)]
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
"#
    .bytes()
    .collect();

    generated_bindings.reserve(1_000_000);

    bindings
        .write(Box::new(&mut generated_bindings))
        .expect("Failed to write");

    generated_bindings.push(b'}');

    std::fs::write(out_path.join("bindings.rs"), generated_bindings)
        .expect("Couldn't write bindings!");
}
