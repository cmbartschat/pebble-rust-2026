#![no_main]
#![no_std]

extern crate alloc;

use alloc::rc::Rc;
use core::{cell::RefCell, time::Duration};

use pebble_rust_2026::{self as _, color::*, *};
use rand::RngExt;

#[global_allocator]
static ALLOCATOR: MallocAllocator = MallocAllocator;

#[unsafe(no_mangle)]
fn main() -> i32 {
    // Create a window with a custom layer for drawing our graphics onto.
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);
    let mut custom_layer = Layer::new(window.get_bounds()).unwrap();
    window.add_child(&mut custom_layer);
    let bounds = window.get_bounds();

    // This position will be randomized whenever the user clicks the button.
    let circle_pos = Rc::new(RefCell::new(GPoint {
        x: bounds.size.w,
        y: bounds.size.h,
    }));
    // This angle will also be randomized.
    let angle = Rc::new(RefCell::new(Angle::from_degrees(0)));

    {
        let circle_pos = circle_pos.clone();
        let angle = angle.clone();
        let _ = custom_layer.add_update_handler(move |_, mut ctx: GContext| {
            // Draw a green circle at the current position.
            ctx.set_fill_color(hex_color!("#0a5"));
            ctx.fill_circle(*circle_pos.clone().borrow(), 25);

            // Draw a blue line similar to an analog clock hand at the current angle.
            let point_on_circle = GPoint::new_on_circle(bounds, *angle.clone().borrow());
            ctx.set_stroke_color(hex_color!("#0af"));
            ctx.set_stroke_width(6);
            ctx.draw_line(bounds.center_point(), point_on_circle);
        });
    }

    window.set_click_provider(move |b| {
        let mut custom_layer = custom_layer.clone();
        let mut custom_layer2 = custom_layer.clone();
        let circle_pos = circle_pos.clone();
        let angle = angle.clone();
        // When the user clicks select...
        b.single(
            Button::Select,
            move |_| {
                // ... generate a new random position anywhere on the screen.
                *circle_pos.clone().borrow_mut() = GPoint {
                    x: Rng.random_range(0..bounds.size.w),
                    y: Rng.random_range(0..bounds.size.h),
                };
                // Force a redraw.
                custom_layer.mark_dirty();
            },
            // Hold the button to trigger lots of circle movements!
            Some(Duration::from_millis(100)),
        );
        // Similarly, when the user clicks up...
        b.single(
            Button::Up,
            move |_| {
                // ... generate a new random angle.
                *angle.clone().borrow_mut() = Rng.random();
                // Force a redraw.
                custom_layer2.mark_dirty();
            },
            // Hold the button to trigger lots of angle movements!
            Some(Duration::from_millis(100)),
        );
    });

    APP.show(window);
    APP.event_loop();
    0
}
