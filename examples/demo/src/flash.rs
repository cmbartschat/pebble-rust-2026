use core::time::Duration;

use alloc::boxed::Box;
use pebble_rust_2026::{
    StatusBarLayer, StatusBarSeparatorMode, Timer, Window,
    color::{
        GCOLOR_BLUE, GCOLOR_GREEN, GCOLOR_ORANGE, GCOLOR_PURPLE, GCOLOR_RED, GCOLOR_WHITE,
        GCOLOR_YELLOW,
    },
    hex_color,
};

pub fn flash() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(hex_color!("#ffffff"));

    {
        let mut status_bar = StatusBarLayer::new().unwrap();
        status_bar.set_separator_mode(StatusBarSeparatorMode::Dotted);
        status_bar.set_colors(GCOLOR_BLUE, GCOLOR_WHITE);
        window.add_child(&mut status_bar);
    }

    let mut color_index: usize = 0;
    let colors = [
        GCOLOR_BLUE,
        GCOLOR_RED,
        GCOLOR_ORANGE,
        GCOLOR_YELLOW,
        GCOLOR_PURPLE,
        GCOLOR_GREEN,
    ];

    window.set_background_color(colors[0]);

    let window_ref = window.downgrade();

    window.set_appear_effect(Box::new(move || {
        let window_ref = window_ref.clone();
        let timer = Timer::repeat(Duration::from_secs(1), move || {
            let Some(mut window) = window_ref.upgrade() else {
                return false;
            };
            color_index = (color_index + 1) % colors.len();
            window.set_background_color(colors[color_index]);
            true
        })
        .unwrap();

        Box::new(move || {
            timer.cancel();
        })
    }));

    window
}
