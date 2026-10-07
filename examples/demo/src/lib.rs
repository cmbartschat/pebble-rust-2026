#![no_main]
#![no_std]

extern crate alloc;

use pebble_sdk::MallocAllocator;

#[global_allocator]
static ALLOCATOR: MallocAllocator = MallocAllocator;

mod app;
mod bitmaps;
mod dictation;
mod draw_commands;
mod flash;
mod heap;
mod move_box;
mod nested_window;
mod scroll;
mod sensors;
mod spin;
mod time;
mod windows;

use crate::app::run_app;
use pebble_sdk as _;

#[unsafe(no_mangle)]
fn main() -> i32 {
    run_app();
    0
}
