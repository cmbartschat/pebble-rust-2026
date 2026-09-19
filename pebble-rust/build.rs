use std::env;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

const SUPPORTED_TARGETS: &[&str] = &[
    "thumbv8m.main-none-eabi",
    "thumbv7m-none-eabi",
    "thumbv7em-none-eabi",
];

enum Platform {
    Aplite,
    Basalt,
    Chalk,
    Diorite,
    Emery,
    Flint,
    Gabbro,
}

impl FromStr for Platform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "aplite" => Self::Aplite,
            "basalt" => Self::Basalt,
            "chalk" => Self::Chalk,
            "diorite" => Self::Diorite,
            "emery" => Self::Emery,
            "flint" => Self::Flint,
            "gabbro" => Self::Gabbro,
            _ => return Err(()),
        })
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Aplite => "aplite",
            Self::Basalt => "basalt",
            Self::Chalk => "chalk",
            Self::Diorite => "diorite",
            Self::Emery => "emery",
            Self::Flint => "flint",
            Self::Gabbro => "gabbro",
        })
    }
}

impl Platform {
    pub fn clang_target(&self) -> &'static str {
        match self {
            Self::Aplite => "thumbv7m-none-eabi",
            Self::Basalt | Self::Chalk | Self::Diorite | Self::Flint => "thumbv7m-none-eabi",
            Self::Gabbro | Self::Emery => "thumbv8m.main-none-eabi",
        }
    }
}

fn main() {
    // Check build target
    let target = env::var("TARGET").unwrap();
    if !SUPPORTED_TARGETS.contains(&target.as_str()) {
        println!(
            "cargo::error=Only the Rust targets {SUPPORTED_TARGETS:?} are supported by pebble_rust_2026."
        );
        panic!();
    }

    // Check which Pebble platform we’re being compiled for, which is set either by crate features or PEBBLE_PLATFORM env var.
    let pebble_platform_env = env::var("PEBBLE_PLATFORM");
    let platform_cfg = env::var("CARGO_CFG_PLATFORM");
    let features = env::var("CARGO_CFG_FEATURE").unwrap();
    let mut features = features.split(",");
    // println!("cargo::warning={features:?} {pebble_platform_env:?}");
    let actual_platform: Option<Platform> = pebble_platform_env
        .ok()
        .and_then(|env| env.parse().ok())
        .or_else(|| features.find_map(|f| f.parse().ok()))
        .or_else(|| platform_cfg.ok().and_then(|c| c.parse().ok()));
    let Some(platform) = actual_platform else {
        println!(
            "cargo::error=Did not receive a valid Pebble platform to compile for. \
            Either set the PEBBLE_PLATFORM environment variable, or specify one of the crate features."
        );
        panic!();
    };
    println!("cargo::rustc-cfg=platform=\"{platform}\"");

    let include_path = String::from_utf8(
        std::process::Command::new("pebble")
            .args(["sdk", "include-path", &platform.to_string()])
            .output()
            .expect("Could not locate Pebble SDK paths, make sure pebble-tool is installed (uv tool install pebble-tool)")
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
        .clang_arg(format!("--target={}", platform.clang_target()))
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

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=PEBBLE_PLATFORM");
}
