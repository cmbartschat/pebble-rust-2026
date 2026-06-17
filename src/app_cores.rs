use alloc::{format, rc::Rc, vec::Vec};
use core::{cell::RefCell, time::Duration};

use crate::{
    APP, Bitmap, BitmapLayer, GRect, SystemFont, TextLayer, Time, Timer, Window, color,
    dictionary::Value, log::log_c_str, sys::TimeUnits_MINUTE_UNIT,
};

extern crate alloc;

struct Core {
    level: u8,
    broken: bool,
    layer: BitmapLayer,
}

#[derive(PartialEq)]
enum Status {
    Loading,
    MissingConfig,
    Loaded,
}

struct State {
    status: Status,
    cycles: u32,
    cores: [Core; 64],
}

fn format_number(v: u32) -> (u32, Option<u32>, &'static str) {
    if v >= 10_000_000 {
        (v / 1_000_000, None, "M")
    } else if v >= 1000 * 1000 {
        (v / 1_000_000, Some((v % 1_000_000) / 100_1000), "M")
    } else if v >= 10_000 {
        (v / 1000, None, "K")
    } else if v >= 1000 {
        (v / 1000, Some((v % 1000) / 100), "K")
    } else {
        (v, None, "")
    }
}

pub fn run_cores() {
    let sprites = Bitmap::from_resource(1).unwrap();
    let core_sprites = (0..8)
        .map(|i| {
            (
                sprites.extract(GRect::new(i * 25, 0, 25, 25)).unwrap(),
                sprites.extract(GRect::new(i * 25, 25, 25, 25)).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut window = Window::new().unwrap();
    window.set_background_color(color::GCOLOR_OXFORD_BLUE);

    let state = {
        let mut core_index: i16 = 0;
        Rc::new(RefCell::new(State {
            status: Status::Loading,
            cycles: 0,
            cores: [(); 64].map(|_| {
                let y = core_index.div_euclid(8);
                let x = core_index.rem_euclid(8);
                core_index += 1;
                let mut layer = BitmapLayer::new(GRect::new(x * 25, y * 25, 25, 25)).unwrap();
                layer.set_bitmap(&core_sprites[7].1);
                window.add_child(&mut layer);
                Core {
                    level: 0,
                    broken: false,
                    layer,
                }
            }),
        }))
    };

    let font = Rc::new(SystemFont::Gothic28Bold.load().unwrap());

    let mut time_layer = TextLayer::new(GRect::new(4, 194, 192, 40)).unwrap();
    time_layer.set_font(&font);
    time_layer.set_text_color(color::GCOLOR_WHITE);
    time_layer.set_background_color(color::GCOLOR_CLEAR);
    window.add_child(&mut time_layer);

    let mut cycle_layer = TextLayer::new(GRect::new(104, 194, 192, 40)).unwrap();
    cycle_layer.set_font(&font);
    cycle_layer.set_text_color(color::GCOLOR_WHITE);
    cycle_layer.set_background_color(color::GCOLOR_CLEAR);
    window.add_child(&mut cycle_layer);

    let now = Time::now();
    log_c_str(c"got now");
    let local = now.to_local();
    log_c_str(c"got local");
    let formatted = local.to_string();
    log_c_str(c"got formatted");
    // log_c_str(formatted.as_c_str());
    // log_c_str(Time::now().to_local().to_string().as_c_str());
    // time_layer.set_text_bytes(Time::now().to_local().to_string().as_bytes());

    let mut update = move |state: &mut State| {
        log_c_str(c"update called");
        time_layer.set_text_bytes(Time::now().to_local().to_string().as_bytes());
        // time_layer.set_text("12:30");
        // log_c_str(Time::now().to_local().to_string().as_c_str());
        cycle_layer.set_text("cycles");

        match state.status {
            Status::Loading => {
                cycle_layer.set_text_c_str(c"loading");
            }
            Status::MissingConfig => cycle_layer.set_text_c_str(c"Needs config"),
            Status::Loaded => {
                cycle_layer.set_text_c_str(c"loaded");

                // let formatted = format_number(state.cycles);
                // let cycle_display = if let Some(decimal) = formatted.1 {
                //     format!("{}.{}{}", formatted.0, decimal, formatted.2)
                // } else {
                //     format!("{}{}", formatted.0, formatted.2)
                // };
                // cycle_layer.set_text(&cycle_display);

                // let mut cycle_text = String::new();
                // let formatted = format_number(state.cycles);
                // use core::fmt::Write as _;
                // write!(&mut cycle_text, "{}", formatted.0).unwrap();
                // cycle_text.write_fmt(Arg);

                // let cycle_display = if let Some(decimal) = formatted.1 {
                //     format!("{}.{}{}", formatted.0, decimal, formatted.2)
                // } else {
                //     format!("{}{}", formatted.0, formatted.2)
                // };
                // cycle_layer.set_text(&cycle_text);
            }
        }

        // cycle_layer.set_alignment(GTextAlignment_GTextAlignmentLeft);

        // log_c_str(c"setting alignment");
        // cycle_layer.set_alignment(2);
        // log_c_str(c"set alignment");

        state.cores.iter_mut().for_each(|c| {
            let sprite = if state.status == Status::Loaded {
                if c.level < 7 && c.broken {
                    &core_sprites[c.level as usize].1
                } else {
                    &core_sprites[c.level as usize].0
                }
            } else {
                &core_sprites[7].1
            };
            c.layer.set_bitmap(sprite);
        });
    };

    if true {
        // let state = state.bo();
        // core::mem::drop(state);
        // if let Ok(mut state) = state.try_borrow_mut() {
        //     update(&mut state);
        // } else {
        //     log_c_str(c"initial update failed to borrow mut");
        // }
        // let mut update = update.clone();
        update(&mut state.borrow_mut());

        let mut update = update.clone();
        let state = state.clone();
        APP.set_tick_handler(TimeUnits_MINUTE_UNIT, move || {
            update(&mut state.borrow_mut());
        });
    }

    APP.set_message_handler(move |d| {
        log_c_str(c"got message");
        match d.get(10003) {
            Some(Value::CStr(t)) if *t == c"RESET" => {
                log_c_str(c"received reset");

                let mut state = state.borrow_mut();
                state.status = Status::MissingConfig;
                update(&mut state);
            }
            Some(Value::CStr(t)) if *t == c"STATE" => {
                if let Some((level_data, broken_data)) = match (d.get(10004), d.get(10005)) {
                    (Some(Value::Bytes(level_value)), Some(Value::Bytes(broken_value))) => {
                        Some((level_value, broken_value))
                    }
                    _ => None,
                } {
                    log_c_str(c"received valid state");
                    let mut state = state.borrow_mut();
                    state.status = Status::Loaded;
                    level_data.iter().enumerate().for_each(|(i, level)| {
                        state.cores[i].level = *level;
                    });

                    broken_data.iter().enumerate().for_each(|(i, broken)| {
                        state.cores[i].broken = *broken != 0;
                    });

                    if let Some(cycles) = d.get(10006).and_then(|f| f.as_u32()) {
                        state.cycles = cycles;
                    }

                    update(&mut state);
                    log_c_str(c"finished update");
                } else {
                    log_c_str(c"received invalid state");
                }
            }
            _ => {}
        };
    });

    APP.open_inbox(crate::app::InboxSize::Half).unwrap();

    let request_update = || {
        log_c_str(c"requesting update");
        APP.send_message(|b| {
            b.write_cstr(10003, c"REFRESH")?;
            Ok(())
        })
        .is_ok()
    };

    request_update();

    Timer::repeat(Duration::from_mins(10), request_update);

    window.show();

    log_c_str(c"starting loop");

    APP.event_loop();

    log_c_str(c"finished loop");
}
