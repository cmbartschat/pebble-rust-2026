use alloc::boxed::Box;
use pebble_rust_2026::{APP, SimpleMenuItem, SimpleMenuLayer, SimpleMenuSection, Window, info};

use crate::windows::WINDOWS;

pub fn run_app() {
    let mut menu_window = Window::new().unwrap();

    let mut options = SimpleMenuSection::new_untitled();
    WINDOWS.iter().for_each(|f| {
        options.push(SimpleMenuItem::new(f.0, None, None, move || {
            APP.show(f.1());
        }));
    });

    let mut menu = SimpleMenuLayer::new(
        menu_window.get_bounds(),
        &mut menu_window,
        Box::new([options]),
    )
    .unwrap();
    menu_window.add_child(&mut menu);

    APP.show(menu_window);

    APP.event_loop();

    info!("finished loop");
}
