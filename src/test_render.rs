extern crate alloc;
use core::fmt::Write;

use core::cell::RefCell;

use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec::Vec;
use cortex_m as _;
use critical_section::Mutex;

use crate::app::APP;
use crate::bitmap::GBitmap;
use crate::color::{GCOLOR_BLUE_MOON, GCOLOR_GREEN, GCOLOR_ORANGE, GCOLOR_RED, GCOLOR_WHITE};
use crate::font::SystemFont;
use crate::layer::{Layer, LayerCreateFailed};
use crate::log::log_c_str;
use crate::sys::{self, GBitmapFormat_GBitmapFormat1Bit, TimeUnits_SECOND_UNIT};
use crate::sys::{GPoint, GRect, GSize};
use crate::text_layer::TextLayerCreateFailed;
use crate::window::{Window, WindowCreateFailed};
use crate::{GContext, TextLayer};

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

impl From<TextLayerCreateFailed> for MultiError {
    fn from(_: TextLayerCreateFailed) -> Self {
        MultiError::TextLayerCreateFailed
    }
}

static FRAME_COUNT: Mutex<RefCell<i16>> = Mutex::new(RefCell::new(0));

struct Foo;
impl core::fmt::Display for Foo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("foo")
    }
}

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
    log_c_str(c"starting critical");
    critical_section::with(|cs| {
        log_c_str(c"inside critical");
        let borrow = FRAME_COUNT.borrow(cs);
        let mut count = match borrow.try_borrow_mut() {
            Ok(c) => c,
            Err(e) => {
                log_c_str(c"failed to borrow:");
                // return;
                let mut v = String::with_capacity(1000);
                log_c_str(c"created string with 1000 capacity.");
                v.push_str(" [ ");
                // Does pushing a static string crash?
                v.push_str("hello");
                log_c_str(c"push_str ok");

                // Does formatting a simple literal crash?
                write!(&mut v, "test").ok();
                log_c_str(c"write literal ok");

                // Does formatting a number crash?
                if write!(&mut v, "{}", 42u32).is_ok() {
                    log_c_str(c"write number ok");
                }

                let foo = Foo;
                if write!(&mut v, "{foo}").is_ok() {
                    log_c_str(c"write foo ok");
                };

                // Does Display for &str specifically crash?
                if write!(&mut v, "{}", "hello world").is_ok() {
                    log_c_str(c"write str ok");
                }

                // // Does formatting the actual error crash?
                // if write!(&mut v, "{e}").is_ok() {
                //     log_c_str(c"write error ok");
                // } else {
                //     log_c_str(c"write error no good");
                // }

                // if let Err(e) = write!(&mut v, "{e}") {
                //     log_c_str(c"Failed to write borrow error");
                //     // v.push_str("failed to write: ");
                //     // if write!(&mut v, "{e}").is_err() {
                //     //     v.push_str("and the error.");
                //     // };
                // };
                // v.push_str("something...");
                v.push_str(" ] ");
                log_c_str(c"created string.");
                // write!(&mut v, "{}", e);
                log_c_str(c"formatted");
                let bytes: Vec<_> = v.bytes().collect::<Vec<_>>();
                log_c_str(c"got bytes");
                let str = CString::new(bytes).unwrap_or(CString::from(c"no loc?"));
                log_c_str(c"trying to print");
                log_c_str(str.as_c_str());
                return;
            }
        };
        let y = *count % (bounds.size.h);
        *count = count.wrapping_add(10);

        log_c_str(c"borrowed");

        ctx.set_stroke_color(GCOLOR_ORANGE);
        ctx.set_stroke_width(3);
        ctx.draw_line(
            GPoint {
                x: bounds.origin.x,
                y: 4,
            },
            GPoint {
                x: bounds.origin.x + bounds.size.w,
                y: 4,
            },
        );

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
        log_c_str(c"assigning");
        log_c_str(c"done");
    });
    log_c_str(c"outside critical");
}

pub fn test_render() -> Result<(), MultiError> {
    let mut window = Window::new()?;

    window.set_background_color(GCOLOR_BLUE_MOON);

    let _bounds = GRect {
        origin: GPoint { x: 10, y: 10 },
        size: GSize { w: 180, h: 100 },
    };

    let Some(font) = SystemFont::Gothic24.load() else {
        return Err(MultiError::LayerCreateFailed);
    };

    let mut text_layer1 = TextLayer::new(GRect::new(10, 60, 180, 100))?;
    window.add_child(&text_layer1.get_layer());

    text_layer1.set_font(font);
    text_layer1.set_text("text_layer1");
    text_layer1.set_background_color(GCOLOR_GREEN);
    text_layer1.set_text_color(GCOLOR_WHITE);

    // let mut text_layer2 = TextLayer::new(GRect::new(10, 160, 180, 100))?;

    // window.add_child(&text_layer2.get_layer());

    // text_layer2.set_font(font);
    // text_layer2.set_text("text_layer2");
    // text_layer2.set_background_color(GCOLOR_BLUE);
    // text_layer2.set_text_color(GCOLOR_WHITE);

    // let mut text_layer3 = TextLayer::new(GRect::new(0, 0, 75, 75))?;
    // window.add_child(&text_layer3.get_layer());
    // text_layer3.set_font(font);
    // text_layer3.set_text("text_layer3, plus Something a bit longer");
    // text_layer3.set_background_color(GCOLOR_RED);
    // text_layer3.set_text_color(GCOLOR_WHITE);

    let mut custom_layer = Layer::new(GRect::new(50, 50, 100, 100))?;
    // custom_layer.add_child(&child_text_layer.get_layer());
    custom_layer.set_update_proc(render_with_bitmap);
    window.add_child(&custom_layer);

    APP.set_timer(TimeUnits_SECOND_UNIT, move || {
        custom_layer.mark_dirty();
    });

    // let mut child_text_layer = TextLayer::new(GRect::new(0, 0, 75, 75))?;
    // child_text_layer.set_font(font);
    // child_text_layer.set_text("child_text_layer");
    // child_text_layer.set_background_color(GCOLOR_GREEN);
    // child_text_layer.set_text_color(GCOLOR_WHITE);

    APP.show_window(&window);
    APP.event_loop();
    APP.clear_timer();

    Ok(())
}
