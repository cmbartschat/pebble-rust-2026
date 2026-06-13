#![no_main]
#![no_std]

extern crate alloc;

mod app;
mod bitmap;
pub mod color;
mod context;
mod custom_alloc;
mod globals;
mod layer;
mod log;
mod rect;
pub mod sys;
mod text_layer;
mod window;

pub use crate::app::APP;
use crate::color::{GCOLOR_BLUE, GCOLOR_BLUE_MOON, GCOLOR_GREEN, GCOLOR_RED, GCOLOR_WHITE};
pub use crate::context::GContext;
pub use crate::layer::Layer;
pub use crate::sys::{GPoint, GRect, GSize};
pub use crate::window::Window;

use crate::text_layer::TextLayer;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let Ok(mut window) = Window::new() else {
        return -1;
    };

    window.set_background_color(GCOLOR_BLUE_MOON);

    let _bounds = GRect {
        origin: GPoint { x: 10, y: 10 },
        size: GSize { w: 180, h: 100 },
    };

    let font = unsafe { sys::fonts_get_system_font(sys::FONT_KEY_GOTHIC_24.as_ptr()) };

    let Ok(mut text_layer1) = TextLayer::new(GRect::new(10, 60, 180, 100)) else {
        return -1;
    };
    window.add_child(&text_layer1.get_layer());

    text_layer1.set_font(font);
    text_layer1.set_text("text_layer1");
    text_layer1.set_background_color(GCOLOR_GREEN);
    text_layer1.set_text_color(GCOLOR_WHITE);

    let Ok(mut text_layer2) = TextLayer::new(GRect::new(10, 160, 180, 100)) else {
        return -1;
    };
    window.add_child(&text_layer2.get_layer());

    text_layer2.set_font(font);
    text_layer2.set_text("text_layer2");
    text_layer2.set_background_color(GCOLOR_BLUE);
    text_layer2.set_text_color(GCOLOR_WHITE);

    let Ok(mut text_layer3) = TextLayer::new(GRect::new(0, 0, 75, 75)) else {
        return -1;
    };
    window.add_child(&text_layer3.get_layer());
    text_layer3.set_font(font);
    text_layer3.set_text("text_layer3, plus Something a bit longer");
    text_layer3.set_background_color(GCOLOR_RED);
    text_layer3.set_text_color(GCOLOR_WHITE);

    let Ok(mut custom_layer) = Layer::new(GRect::new(50, 50, 100, 100)) else {
        return -1;
    };
    window.add_child(&custom_layer);

    let Ok(mut child_text_layer) = TextLayer::new(GRect::new(0, 0, 75, 75)) else {
        return -1;
    };
    custom_layer.add_child(&child_text_layer.get_layer());
    child_text_layer.set_font(font);
    child_text_layer.set_text("child_text_layer");
    child_text_layer.set_background_color(GCOLOR_GREEN);
    child_text_layer.set_text_color(GCOLOR_WHITE);

    APP.show_window(&window);

    APP.event_loop();

    APP.clear_timer();

    0
}
