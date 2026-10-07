use alloc::boxed::Box;
use pebble_sdk::{APP, Button, GColor, TextLayer, Window, color, fmt, hex_color, warn};

static COLORS: [GColor; 3] = [hex_color!("#ff0"), hex_color!("#f0f"), hex_color!("#0ff")];

pub fn nested_window() -> Window {
    _nested_window(0)
}

fn _nested_window(i: usize) -> Window {
    let mut window = Window::new().unwrap();
    let bounds = window.get_bounds();
    window.set_background_color(color::GCOLOR_BLACK);

    let fill = {
        let window = window.downgrade();
        move || {
            let Some(mut window) = window.upgrade() else {
                warn!("fill has no window to use");
                return;
            };

            let mut text_layer = TextLayer::new(bounds).unwrap();
            text_layer.set_background_color(COLORS[i % COLORS.len()]);
            text_layer.set_text(&fmt!("Level {}", i));

            window.add_child(&mut text_layer);
        }
    };

    let clear = {
        let window = window.downgrade();
        move || {
            let Some(mut window) = window.upgrade() else {
                return;
            };
            window.set_background_color(color::GCOLOR_BLACK);
            window.remove_child_layers();
        }
    };

    window.set_appear_effect(Box::new(move || {
        fill();

        let clear = clear.clone();

        Box::new(move || {
            clear();
        })
    }));

    window.set_click_provider(move |b| {
        b.single(
            Button::Select,
            move |_| {
                APP.show(_nested_window(i + 1));
            },
            None,
        );
    });

    window
}
