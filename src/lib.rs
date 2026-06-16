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
mod layer;
pub mod log;
mod persist;

mod bitmap_layer;
mod rect;
pub mod sys;
mod test_render;
mod text_layer;
mod timer;
mod window;

pub use crate::app::APP;
use crate::app_cores::run_cores;
pub use crate::app_message_result::AppMessageError;
pub use crate::bitmap::Bitmap;
pub use crate::bitmap_layer::BitmapLayer;
pub use crate::context::GContext;
pub use crate::font::{Font, SystemFont};
pub use crate::layer::Layer;
pub use crate::sys::{GPoint, GRect, GSize};
// use crate::test_render::test_render;
pub use crate::text_layer::TextLayer;
pub use crate::timer::Timer;
pub use crate::window::Window;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    // match test_render() {
    //     Ok(_) => 0,
    //     Err(_) => -1,
    // }
    run_cores();
    0
}
