#![no_main]
#![no_std]

extern crate alloc;

mod app;
mod bitmap;
pub mod color;
mod context;
mod custom_alloc;
mod font;
mod globals;
mod layer;
mod log;
// mod node;
mod rect;
pub mod sys;
mod test_render;
mod text_layer;
mod window;

pub use crate::app::APP;
pub use crate::context::GContext;
pub use crate::layer::Layer;
pub use crate::sys::{GPoint, GRect, GSize};
use crate::test_render::test_render;
pub use crate::text_layer::TextLayer;
pub use crate::window::Window;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    match test_render() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
