use pebble_sdk::{APP, GRect, TextLayer, Time, TimeUnits, Window, color::GCOLOR_WHITE, fmt};

pub fn time() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);

    let mut local_time_layer = TextLayer::new(GRect::new(0, 0, 200, 30)).unwrap();
    window.add_child(&mut local_time_layer);

    let mut utc_time_layer = TextLayer::new(GRect::new(0, 30, 200, 30)).unwrap();
    window.add_child(&mut utc_time_layer);

    let mut now_timestamp_layer = TextLayer::new(GRect::new(0, 60, 200, 30)).unwrap();
    window.add_child(&mut now_timestamp_layer);

    let mut converted_local_timestamp_layer = TextLayer::new(GRect::new(0, 90, 200, 30)).unwrap();
    window.add_child(&mut converted_local_timestamp_layer);

    let mut converted_utc_timestamp_layer = TextLayer::new(GRect::new(0, 120, 200, 30)).unwrap();
    window.add_child(&mut converted_utc_timestamp_layer);

    let mut battery_layer = TextLayer::new(GRect::new(0, 150, 200, 30)).unwrap();
    window.add_child(&mut battery_layer);

    let mut bluetooth_layer = TextLayer::new(GRect::new(0, 180, 200, 30)).unwrap();
    window.add_child(&mut bluetooth_layer);

    APP.set_tick_handler(TimeUnits::Second, move || {
        let now = Time::now();
        local_time_layer.set_text_bytes(now.to_local().format_hh_mm().as_bytes());
        utc_time_layer.set_text_bytes(now.to_utc().format_hh_mm().as_bytes());
        now_timestamp_layer.set_text(&fmt!("{}", now.epoch_seconds()));
        converted_local_timestamp_layer.set_text(&fmt!(
            "{}",
            Time::try_from(now.to_local()).unwrap().epoch_seconds()
        ));
        converted_utc_timestamp_layer.set_text(&fmt!(
            "{}",
            Time::try_from(now.to_utc()).unwrap().epoch_seconds()
        ));

        battery_layer.set_text(&{ fmt!("Battery: {}%", APP.battery_state.peek().charge_percent) });

        if APP.bluetooth_connection.peek() {
            bluetooth_layer.set_text_c_str(c"Connected to bluetooth");
        } else {
            bluetooth_layer.set_text_c_str(c"Not connected to bluetooth");
        }
    });

    window
}
