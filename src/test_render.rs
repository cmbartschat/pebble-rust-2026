// extern crate alloc;

// use core::cell::RefCell;

// use alloc::rc::Rc;
// use cortex_m as _;
// use critical_section::Mutex;

// use crate::app::APP;
// use crate::bitmap::GBitmap;
// use crate::color::{GCOLOR_BLUE, GCOLOR_BLUE_MOON, GCOLOR_GREEN, GCOLOR_RED, GCOLOR_WHITE};
// use crate::font::SystemFont;
// use crate::layer::{Layer, LayerCreateFailed};
// use crate::log::log_c_str;
// use crate::sys::{self, GBitmapFormat_GBitmapFormat1Bit};
// use crate::sys::{GPoint, GRect, GSize};
// use crate::text_layer::TextLayerCreateFailed;
// use crate::window::{Window, WindowCreateFailed};
// use crate::{GContext, TextLayer};

// #[allow(clippy::enum_variant_names)]
// pub enum MultiError {
//     WindowCreateFailed,
//     LayerCreateFailed,
//     TextLayerCreateFailed,
// }

// impl From<WindowCreateFailed> for MultiError {
//     fn from(_: WindowCreateFailed) -> Self {
//         MultiError::WindowCreateFailed
//     }
// }

// impl From<LayerCreateFailed> for MultiError {
//     fn from(_: LayerCreateFailed) -> Self {
//         MultiError::LayerCreateFailed
//     }
// }

// impl From<TextLayerCreateFailed> for MultiError {
//     fn from(_: TextLayerCreateFailed) -> Self {
//         MultiError::TextLayerCreateFailed
//     }
// }

// static FRAME_COUNT: Mutex<RefCell<i16>> = Mutex::new(RefCell::new(0));

// unsafe extern "C" fn render_with_bitmap(_layer: *mut sys::Layer, ctx: *mut sys::GContext) {
//     let mut bitmap =
//         GBitmap::new_empty(GSize { w: 50, h: 50 }, GBitmapFormat_GBitmapFormat1Bit).unwrap();

//     let bounds = bitmap.get_bounds();
//     let data = bitmap.get_data().unwrap();

//     for (i, x) in data.iter_mut().enumerate() {
//         *x = (i % 256) as u8;
//     }

//     let Ok(mut ctx) = GContext::from_raw(ctx) else {
//         log_c_str(c"invalid context");
//         return;
//     };

//     ctx.set_fill_color(GCOLOR_RED);
//     ctx.fill_rect(bounds.expand(3));
//     ctx.draw_bitmap(&bitmap, bounds);
//     critical_section::with(|cs| {
//         let borrow = FRAME_COUNT.borrow(cs);
//         let mut count = match borrow.try_borrow_mut() {
//             Ok(c) => c,
//             Err(_) => {
//                 log_c_str(c"failed to borrow:");
//                 return;
//             }
//         };

//         let y = *count % (bounds.size.h);
//         *count = count.wrapping_add(10);

//         ctx.draw_line(
//             GPoint {
//                 x: bounds.origin.x,
//                 y,
//             },
//             GPoint {
//                 x: bounds.origin.x + bounds.size.w,
//                 y,
//             },
//         );
//     });
// }

// pub fn test_render() -> Result<(), MultiError> {
//     let Some(mut window) = Window::new() else {
//         return Err(MultiError::WindowCreateFailed);
//     };
//     window.set_background_color(GCOLOR_BLUE_MOON);

//     let Some(font) = SystemFont::Gothic24.load() else {
//         return Err(MultiError::LayerCreateFailed);
//     };

//     let mut text_layer1 = TextLayer::new(GRect::new(10, 60, 180, 100))?;
//     text_layer1.set_font(&font);
//     text_layer1.set_text("text_layer1");
//     text_layer1.set_background_color(GCOLOR_GREEN);
//     text_layer1.set_text_color(GCOLOR_WHITE);
//     {
//         text_layer1.get_layer_mut().add_to_window(&window);
//     }

//     let mut text_layer2 = TextLayer::new(GRect::new(10, 160, 180, 100))?;
//     text_layer2.set_font(&font);
//     text_layer2.set_text("text_layer2");
//     text_layer2.set_background_color(GCOLOR_BLUE);
//     text_layer2.set_text_color(GCOLOR_WHITE);
//     {
//         text_layer2.get_layer_mut().add_to_window(&window);
//     }

//     let mut text_layer3 = TextLayer::new(GRect::new(0, 0, 75, 75))?;
//     text_layer3.set_font(&font);
//     text_layer3.set_text("text_layer3, plus Something a bit longer");
//     text_layer3.set_background_color(GCOLOR_RED);
//     text_layer3.set_text_color(GCOLOR_WHITE);
//     {
//         text_layer3.get_layer_mut().add_to_window(&window);
//     }

//     let mut custom_layer = Layer::new(GRect::new(50, 50, 100, 100))?;
//     custom_layer.set_update_proc(render_with_bitmap);

//     let mut child_text_layer = TextLayer::new(GRect::new(0, 35, 75, 75))?;
//     child_text_layer.set_font(&font);
//     child_text_layer.set_text("child_text_layer");
//     child_text_layer.set_background_color(GCOLOR_GREEN);
//     child_text_layer.set_text_color(GCOLOR_WHITE);

//     custom_layer.add_child(child_text_layer);

//     custom_layer.add_to_window(&window);

//     let mut x = Rc::new(RefCell::new((custom_layer, window, font)));
//     let mut y = x.clone();
//     APP.set_timer(sys::TimeUnits_SECOND_UNIT, move || {
//         let mut data = x.borrow_mut();
//         data.0.mark_dirty();
//     });

//     APP.show_window(&window);

//     APP.event_loop();
//     APP.clear_timer();

//     Ok(())
// }

extern crate alloc;

use core::cell::RefCell;

use alloc::rc::Rc;
use cortex_m as _;
use critical_section::Mutex;

use crate::GContext;
use crate::app::APP;
use crate::bitmap::GBitmap;
use crate::color::{GCOLOR_BLUE, GCOLOR_BLUE_MOON, GCOLOR_RED};
use crate::font::SystemFont;
use crate::layer::{Layer, LayerCreateFailed};
use crate::log::log_c_str;
use crate::sys::{self, GBitmapFormat_GBitmapFormat1Bit};
use crate::sys::{GPoint, GRect, GSize};
use crate::window::{Window, WindowCreateFailed};

#[allow(clippy::enum_variant_names)]
pub enum MultiError {
    WindowCreateFailed,
    LayerCreateFailed,
    TextLayerCreateFailed,
}

impl From<WindowCreateFailed> for MultiError {
    fn from(_: WindowCreateFailed) -> Self {
        MultiError::WindowCreateFailed
    }
}

impl From<LayerCreateFailed> for MultiError {
    fn from(_: LayerCreateFailed) -> Self {
        MultiError::LayerCreateFailed
    }
}

static FRAME_COUNT: Mutex<RefCell<i16>> = Mutex::new(RefCell::new(0));

unsafe extern "C" fn render_with_bitmap(_layer: *mut sys::Layer, ctx: *mut sys::GContext) {
    let mut bitmap =
        GBitmap::new_empty(GSize { w: 50, h: 50 }, GBitmapFormat_GBitmapFormat1Bit).unwrap();

    let bounds = bitmap.get_bounds();
    let data = bitmap.get_data().unwrap();

    for (i, x) in data.iter_mut().enumerate() {
        *x = (i % 256) as u8;
    }

    let Ok(mut ctx) = GContext::from_raw(ctx) else {
        log_c_str(c"invalid context");
        return;
    };

    ctx.set_fill_color(GCOLOR_RED);
    ctx.fill_rect(bounds.expand(3));
    ctx.draw_bitmap(&bitmap, bounds);
    critical_section::with(|cs| {
        let borrow = FRAME_COUNT.borrow(cs);
        let mut count = match borrow.try_borrow_mut() {
            Ok(c) => c,
            Err(_) => {
                log_c_str(c"failed to borrow:");
                return;
            }
        };

        let y = *count % (bounds.size.h);
        *count = count.wrapping_add(10);

        ctx.draw_line(
            GPoint {
                x: bounds.origin.x,
                y,
            },
            GPoint {
                x: bounds.origin.x + bounds.size.w,
                y,
            },
        );
    });
}

unsafe extern "C" fn render_square(_layer: *mut sys::Layer, ctx: *mut sys::GContext) {
    let Ok(mut ctx) = GContext::from_raw(ctx) else {
        log_c_str(c"invalid context");
        return;
    };

    ctx.set_fill_color(GCOLOR_BLUE);
    ctx.fill_rect(GRect::new(5, 5, 5, 5));
}

pub fn test_render() -> Result<(), MultiError> {
    let Some(mut window) = Window::new() else {
        return Err(MultiError::WindowCreateFailed);
    };
    window.set_background_color(GCOLOR_BLUE_MOON);

    let Some(font) = SystemFont::Gothic24.load() else {
        return Err(MultiError::LayerCreateFailed);
    };

    let mut custom_layer = Layer::new(GRect::new(50, 50, 100, 100))?;
    custom_layer.set_update_proc(render_with_bitmap);

    let mut custom_child = Layer::new(GRect::new(2, 3, 15, 15))?;
    custom_layer.add_child(&mut custom_child);
    custom_child.set_update_proc(render_square);

    window.add_child(&mut custom_layer);

    APP.show_window(&window);

    let mut x = Rc::new(RefCell::new((custom_layer, window, font)));
    let mut y = x.clone();
    APP.set_timer(sys::TimeUnits_SECOND_UNIT, move || {
        let mut data = x.borrow_mut();
        data.0.mark_dirty();
    });

    APP.event_loop();
    APP.clear_timer();

    Ok(())
}
