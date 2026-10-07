use alloc::{boxed::Box, vec::Vec};
use pebble_rust_2026::{
    APP, AccelerometerAxis, AccelerometerData, AccelerometerSamplingRate, BatteryChargeState,
    GRect, TextLayer, Window, color::GCOLOR_WHITE, fmt, hex_color,
};

pub fn sensors() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);

    let mut offset = 0;

    let mut update_battery = {
        let mut layer = TextLayer::new(GRect::new(0, offset, 200, 30)).unwrap();
        window.add_child(&mut layer);
        offset += 30;
        move |charge_percent: u8| {
            layer.set_text(&fmt!("Battery: {}%", charge_percent));
        }
    };

    let mut update_bluetooth = {
        let mut layer = TextLayer::new(GRect::new(0, offset, 200, 30)).unwrap();
        window.add_child(&mut layer);
        offset += 30;
        move |connected: bool| {
            if connected {
                layer.set_text_c_str(c"Bluetooth: Connected");
            } else {
                layer.set_text_c_str(c"Bluetooth: Not Connected");
            }
        }
    };

    let mut update_focus = {
        let mut layer = TextLayer::new(GRect::new(0, offset, 200, 30)).unwrap();
        offset += 30;
        window.add_child(&mut layer);
        move |focused: bool| {
            if focused {
                layer.set_text_c_str(c"Focused: true");
            } else {
                layer.set_text_c_str(c"Focused: false");
            }
        }
    };

    let mut update_accel = {
        let mut layer = TextLayer::new(GRect::new(0, offset, 200, 30)).unwrap();
        window.add_child(&mut layer);
        move |data: &[AccelerometerData]| {
            if let Some(data) = data.last() {
                let x = data.x;
                let y = data.y;
                let z = data.z;
                layer.set_text(&fmt!("Accel: ({}, {}, {})", x, y, z));
            } else {
                layer.set_text_c_str(c"Accelerometer: Unavailable");
            }
        }
    };

    let weak_window = window.downgrade();
    window.set_appear_effect(Box::new(move || {
        update_battery(APP.battery_state.peek().charge_percent);
        update_bluetooth(APP.bluetooth_connection.peek());
        let data: Vec<_> = APP.accelerometer.peek().into_iter().collect();
        update_accel(&data);

        let battery_callback = APP.battery_state.subscribe({
            let mut update_battery = update_battery.clone();
            move |b: BatteryChargeState| update_battery(b.charge_percent)
        });
        let bluetooth_callback = APP
            .bluetooth_connection
            .subscribe(Box::new(update_bluetooth.clone()));
        update_focus(true); // Assumed

        let accel_tap_callback = APP.accelerometer.subscribe_to_tap(Box::new({
            let weak_window = weak_window.clone();
            move |axis| {
                let Some(mut window) = weak_window.upgrade() else {
                    return;
                };
                let color = match axis {
                    AccelerometerAxis::PosX => hex_color!("#f0f"),
                    AccelerometerAxis::PosY => hex_color!("#ff0"),
                    AccelerometerAxis::PosZ => hex_color!("#0ff"),
                    AccelerometerAxis::NegX => hex_color!("#f00"),
                    AccelerometerAxis::NegY => hex_color!("#0f0"),
                    AccelerometerAxis::NegZ => hex_color!("#00f"),
                    AccelerometerAxis::Unspecified => hex_color!("#fa0"),
                };
                window.set_background_color(color);
            }
        }));
        let accel_callback = APP.accelerometer.subscribe(Box::new(update_accel.clone()));
        APP.accelerometer
            .set_sampling_rate(AccelerometerSamplingRate::Hz10);
        let focus_callback = APP.focus.subscribe(Box::new(update_focus.clone()));

        Box::new(move || {
            battery_callback.cancel();
            bluetooth_callback.cancel();
            accel_callback.cancel();
            accel_tap_callback.cancel();
            focus_callback.cancel();
        })
    }));

    window
}
