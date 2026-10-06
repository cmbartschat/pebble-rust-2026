#![no_main]
#![no_std]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;

use pebble_rust_2026::{self as _, color::*, *};

#[global_allocator]
static ALLOCATOR: MallocAllocator = MallocAllocator;

#[unsafe(no_mangle)]
fn main() -> i32 {
    // Create a window with a custom layer for drawing graphics.
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);
    let mut custom_layer = Layer::new(window.get_bounds()).unwrap();
    window.add_child(&mut custom_layer);

    // Stores the position of the last touch.
    let last_touch_pos = Rc::new(RefCell::new(GPoint { x: 0, y: 0 }));

    {
        let last_touch_pos = last_touch_pos.clone();
        custom_layer.set_update_handler(move |_, mut ctx: GContext| {
            // Draw a green circle at the current position.
            ctx.set_fill_color(hex_color!("#000"));
            ctx.fill_circle(*last_touch_pos.clone().borrow(), 10);
        });
    }

    let graphics_callback = {
        let last_touch_pos = last_touch_pos.clone();
        // This first handler stores the touch start position for use by the circle drawing above.
        APP.touch.subscribe(move |touch| {
            if let TouchEvent::TouchDown(touch) = touch {
                last_touch_pos.replace(touch);
            }
            custom_layer.mark_dirty();
        })
    };

    // This second handler just logs touch events.
    let logging_callback = APP.touch.subscribe(move |touch| match touch {
        TouchEvent::TouchDown(gpoint) => {
            debug!("touch down at {} {}", gpoint.x, gpoint.y)
        }
        TouchEvent::TouchMove(gpoint) => {
            debug!("touch move at {} {}", gpoint.x, gpoint.y)
        }
        TouchEvent::TouchUp(gpoint) => {
            debug!("touch up at {} {}", gpoint.x, gpoint.y)
        }
    });

    window.set_click_provider(move |b| {
        // Remove the graphics handler when up is pressed.
        // This is just a demonstration; usually you want to unsubscribe from handlers only when destroying the current window.
        // See the effect code below.
        b.single(
            Button::Up,
            move |_| {
                graphics_callback.cancel();
            },
            None,
        );
    });

    window.set_load_effect(Box::new(move || {
        // Nothing to do during load.
        Box::new(move || {
            // During unload, unsubscribe from touch events.
            // Otherwise, the event handlers are leaked.
            graphics_callback.cancel();
            logging_callback.cancel();
        })
    }));

    APP.show(window);
    APP.event_loop();
    0
}
