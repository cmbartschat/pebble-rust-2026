use pebble_sdk::Window;

type WindowInfo = (&'static str, fn() -> Window);

pub static WINDOWS: [WindowInfo; 11] = [
    ("Heap", crate::heap::heap),
    ("Dictation", crate::dictation::dictation),
    ("Sensors", crate::sensors::sensors),
    ("Nested", crate::nested_window::nested_window),
    ("Scroll", crate::scroll::scroll),
    ("Time", crate::time::time),
    ("Spin", crate::spin::spin),
    ("Flash", crate::flash::flash),
    ("Move Box", crate::move_box::move_box),
    ("Draw Commands", crate::draw_commands::draw_commands),
    ("Bitmaps", crate::bitmaps::bitmaps),
];
