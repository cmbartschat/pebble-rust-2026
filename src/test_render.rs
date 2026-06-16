extern crate alloc;

use core::cell::RefCell;
use core::str::FromStr;
use core::time::Duration;

use alloc::ffi::CString;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use cortex_m as _;
use critical_section::Mutex;

use crate::app::{APP, InboxSize};
use crate::bitmap::Bitmap;
use crate::color::{GCOLOR_BLACK, GCOLOR_BLUE_MOON, GCOLOR_GREEN, GCOLOR_RED};
use crate::font::SystemFont;
use crate::layer::{Layer, LayerCreateFailed};
use crate::log::log_c_str;
use crate::sys::{self, GBitmapFormat_GBitmapFormat1Bit};
use crate::sys::{GPoint, GRect, GSize};
use crate::text_layer::TextLayerCreateFailed;
use crate::timer::Timer;
use crate::window::{Window, WindowCreateFailed};
use crate::{BitmapLayer, GContext, TextLayer};

pub enum MultiError {
    WindowCreateFailed,
    LayerCreateFailed,
    TextLayerCreateFailed,
    MissingFont,
    MissingResource,
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

unsafe extern "C" fn render_with_bitmap(_layer: *mut sys::Layer, ctx: *mut sys::GContext) {
    let mut bitmap =
        Bitmap::new_empty(GSize { w: 50, h: 50 }, GBitmapFormat_GBitmapFormat1Bit).unwrap();

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
        let count = match borrow.try_borrow_mut() {
            Ok(c) => c,
            Err(_) => {
                log_c_str(c"failed to borrow:");
                return;
            }
        };

        let y = *count % (bounds.size.h);

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

pub fn test_render() -> Result<(), MultiError> {
    let Some(mut window) = Window::new() else {
        return Err(MultiError::WindowCreateFailed);
    };
    window.set_background_color(GCOLOR_BLUE_MOON);

    let Some(font) = SystemFont::Gothic24.load() else {
        return Err(MultiError::LayerCreateFailed);
    };

    let font = Rc::new(font);

    let text_layer1 = {
        let Some(font) = SystemFont::Gothic14Bold.load() else {
            return Err(MultiError::MissingFont);
        };
        let mut text_layer1 = TextLayer::new(GRect::new(0, 0, 32, 32))?;
        text_layer1.set_font(&Rc::new(font));
        text_layer1.set_text("layer1");
        text_layer1.set_background_color(GCOLOR_GREEN);
        text_layer1.set_text_color(GCOLOR_BLACK);
        window.add_child(&mut text_layer1);
        text_layer1
    };

    let mut bitmap_layer = {
        let Some(mut bitmap_layer) = BitmapLayer::new(GRect::new(40, 8, 16, 16)) else {
            return Err(MultiError::LayerCreateFailed);
        };
        window.add_child(&mut bitmap_layer);
        bitmap_layer
    };

    let mut custom_layer = Layer::new(GRect::new(64, 0, 32, 32))?;
    custom_layer.set_update_proc(render_with_bitmap);

    window.add_child(&mut custom_layer);

    window.show();

    let Some(hero_sheet) = Bitmap::from_resource(1) else {
        return Err(MultiError::MissingResource);
    };
    let hero_frames = [
        hero_sheet.extract(GRect::new(0, 16, 16, 16)).unwrap(),
        hero_sheet.extract(GRect::new(16, 16, 16, 16)).unwrap(),
        hero_sheet.extract(GRect::new(32, 16, 16, 16)).unwrap(),
        hero_sheet.extract(GRect::new(48, 16, 16, 16)).unwrap(),
    ];

    bitmap_layer.set_bitmap(&hero_frames[0]);

    let x = Rc::new(RefCell::new((custom_layer, window, font)));
    let y = x.clone();
    let mut count = 0;

    APP.set_tick_handler(sys::TimeUnits_SECOND_UNIT, move || {
        let mut data = x.borrow_mut();
        data.0.mark_dirty();

        critical_section::with(|cs| {
            let borrow = FRAME_COUNT.borrow(cs);
            let mut count = match borrow.try_borrow_mut() {
                Ok(c) => c,
                Err(_) => {
                    log_c_str(c"failed to borrow:");
                    return;
                }
            };

            *count = count.wrapping_add(10);
        });
    });

    Timer::repeat(Duration::from_millis(50), move || {
        count += 1;
        bitmap_layer.set_bitmap(&hero_frames[count % hero_frames.len()]);
        true
    });

    {
        let mut text_layer_cloned2 = text_layer1.clone();
        let mut start = 0;
        let mut text = String::new();
        let timer = Timer::repeat(Duration::from_secs(1), move || {
            start = (start + 1) % 10;
            text.clear();
            for _ in 0..start {
                text.push('x');
            }

            text_layer_cloned2.set_text(&text);
            true
        })
        .unwrap();

        Timer::once(Duration::from_secs(5), move || {
            timer.cancel();
        });
    }

    log_c_str(c"setting message handler.");

    APP.set_message_handler(|dict| {
        log_c_str(c"received message...");
        for tuple in dict.iter() {
            log_c_str(match tuple.value() {
                crate::dictionary::Value::Bytes(_) => c" - key of bytes",
                crate::dictionary::Value::CStr(_) => c" - key of str",
                crate::dictionary::Value::Uint(_) => c" - key of uint",
                crate::dictionary::Value::Int(_) => c" - key of int",
            });
        }
        log_c_str(c"done.");
    });

    APP.open_inbox(InboxSize::Half).unwrap();

    let sent_count = 0;
    Timer::repeat(Duration::from_secs(2), move || {
        if let Err(e) = APP.send_message(|builder| {
            builder.write_i8(10000 /* ECHO */, 1)?;
            builder.write_i32(10001 /* DATA1 */, 1)?;
            Ok(())
        }) {
            let s = format!("{e:?}");
            log_c_str(CString::from_str(&s).unwrap().as_c_str());
        }

        sent_count < 10
    });

    match APP.persist.read_bool(10000) {
        Some(true) => log_c_str(c"bool is true"),
        Some(false) => log_c_str(c"bool is false"),
        None => log_c_str(c"bool is unset"),
    };

    let mut v = false;
    Timer::repeat(Duration::from_secs(2), move || {
        v = !v;
        log_c_str(if v { c"storing true" } else { c"storing false" });
        APP.persist.write_bool(10000, v);
        true
    });

    APP.event_loop();
    APP.clear_tick_handler();

    y.borrow_mut().1.hide();

    Ok(())
}
