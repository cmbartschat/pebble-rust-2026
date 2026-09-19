use std::env;
use std::path::PathBuf;

fn main() {
    let include_path = String::from_utf8(
        std::process::Command::new("pebble")
            .args(["sdk", "include-path", "emery"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    let bindings = bindgen::Builder::default()
        .header("headers/entry.h")
        .clang_arg(format!("-I{}", include_path.trim_ascii()))
        .clang_arg(format!(
            "--sysroot={}/../../../../toolchain/arm-none-eabi/arm-none-eabi",
            include_path.trim_ascii()
        ))
        .clang_arg("-D_TIME_H_")
        .clang_arg("-Iheaders")
        .clang_arg("--target=thumbv8m.main-none-eabi")
        .clang_arg("-fshort-enums")
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

    bindings.write(Box::new(&mut bindings_handle)).unwrap();
}
