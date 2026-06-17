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
mod time;

mod bitmap_layer;
mod rect;
pub mod sys;
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
pub use crate::time::Time;
pub use crate::timer::Timer;
pub use crate::window::Window;

// fn stack_depth_test(depth: u8, v: &mut u8) {
//     // log_c_str(c"stack_depth_test");

//     let mut data = [0u8; 100];

//     data[3] = *v;

//     if depth > 0 {
//         stack_depth_test(depth - 1, &mut data[3]);
//         data[3] += 1;
//     };

//     *v = data[3];
// }

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    // for depth in [0, 1] {
    //     let mut res = 0;
    //     stack_depth_test(depth, &mut res);
    //     log_c_str(c"loop done.");

    //     if res != depth {
    //         log_c_str(c" - bad");
    //         return -1;
    //         // panic!("Bad data")
    //     }

    //     log_c_str(c" - good");
    // }
    // let mut res = 0;
    // stack_depth_test(15, &mut res);

    // match test_render() {
    //     Ok(_) => 0,
    //     Err(_) => -1,
    // }
    run_cores();
    0
}
