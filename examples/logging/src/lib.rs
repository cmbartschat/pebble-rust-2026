#![no_main]
#![no_std]

extern crate alloc;

use pebble_rust_2026::{self as _, log::*, *};
use ufmt::derive::uDebug;

#[global_allocator]
static ALLOCATOR: MallocAllocator = MallocAllocator;

#[derive(uDebug)]
struct MyStruct {
    x: i32,
    y: u32,
}

#[unsafe(no_mangle)]
fn main() -> i32 {
    // You can log at the following five levels.
    // Try using `PEBBLE_LOG=debug cargo pebble build`,
    // and try all of `trace, debug, info, warn, error` for the PEBBLE_LOG environment variable.
    // Also try `PEBBLE_LOG=off`!
    // You’ll see that this variable changes which levels are allowed to be logged,
    // and it in fact also entirely removes the unused logging code.
    // This makes your application very efficient in release mode,
    // while allowing you to keep as many debug!()’s as you want for development.
    trace!("Tracing the application flow...");
    debug!("Here’s some debug information!");
    info!("And more information.");
    warn!("Hmm, something may be off...");
    error!("Oh no, something went wrong!");

    // You can also use basic `format!`-like formatting, utilizing ufmt: https://docs.rs/ufmt/
    debug!("There are {} bugs in my code :(", 42);
    info!("There are {} bugs in my code", "some");
    info!("I have a cool struct: {:?}", MyStruct { x: 1, y: 1700 });

    let mut window = Window::new().unwrap();
    let mut layer = TextLayer::new(window.get_bounds()).unwrap();

    let output = fmt!("I have some numbers here:\n{}, {}, {}", 1, 2, 3);
    error!("output is {} bytes", output.len());

    layer.set_text(output.as_ref());
    window.add_child(&mut layer);

    APP.show(window);

    APP.event_loop();
    0
}
