#![no_main]
#![no_std]

extern crate alloc;

mod app;
mod app_cores;
mod app_message_result;
mod bitmap;
pub mod color;
mod context;
mod custom_alloc;
mod dictionary;
mod font;
mod globals;
mod key;
mod layer;
mod log;
mod persist;
mod time;

mod bitmap_layer;
mod rect;
pub mod sys;
mod text_layer;
mod timer;
mod window;

pub use crate::app::APP;
pub use crate::app::InboxSize;
use crate::app_cores::run_cores;
pub use crate::app_message_result::AppMessageError;
pub use crate::bitmap::Bitmap;
pub use crate::bitmap_layer::BitmapLayer;
pub use crate::context::GContext;
pub use crate::custom_alloc::Allocator;
pub use crate::dictionary::{DictionaryBuilder, DictionaryView, Tuple, Value};
pub use crate::font::{Font, SystemFont};
pub use crate::layer::Layer;
pub use crate::log::log_c_str;
pub use crate::sys::{GPoint, GRect, GSize};
pub use crate::text_layer::TextLayer;
pub use crate::time::Time;
pub use crate::timer::Timer;
pub use crate::window::Window;
pub use proc::*;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    run_cores();
    0
}
