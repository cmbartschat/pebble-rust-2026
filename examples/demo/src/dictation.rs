use alloc::format;
use pebble_rust_2026::{Button, DictationSession, TextLayer, Window};

pub fn dictation() -> Window {
    let mut window = Window::new().unwrap();

    let mut text = TextLayer::new(window.get_bounds()).unwrap();
    window.add_child(&mut text);

    let mut text_for_callback = text.clone();
    if let Some(session) = DictationSession::new(move |result| match result {
        Ok(value) => text_for_callback.set_text(&format!("You said: {value}")),
        Err(_) => text_for_callback.set_text("There was an error."),
    }) {
        text.set_text("Press Select to Dictate");
        window.set_click_provider(move |b| {
            let mut session = session.clone();
            let mut text = text.clone();
            b.single(
                Button::Select,
                move |_| {
                    if session.start().is_err() {
                        text.set_text("Failed to start dictation.");
                    }
                },
                None,
            );
        });
    } else {
        text.set_text("Unable to create dictation session.");
    }

    window
}
