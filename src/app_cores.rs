use core::{cell::RefCell, time::Duration};

use alloc::{rc::Rc, vec::Vec};

use crate::{
    APP, Bitmap, BitmapLayer, GRect, SystemFont, TextLayer, Timer, Window, color,
    dictionary::Value, log::log_c_str, sys::GTextAlignment_GTextAlignmentRight,
};

extern crate alloc;

struct Core {
    level: u8,
    broken: bool,
    layer: BitmapLayer,
}

struct State {
    loaded: bool,
    cycles: usize,
    cores: [Core; 64],
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
            loaded: false,
            cycles: 0,
            cores: [(); 64].map(|_| {
                let y = core_index.div_euclid(8);
                let x = core_index.rem_euclid(8);
                core_index += 1;
                let mut layer = BitmapLayer::new(GRect::new(x * 25, y * 25, 25, 25)).unwrap();
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

    let mut cycle_layer = TextLayer::new(GRect::new(0, 200, 200, 28)).unwrap();
    cycle_layer.set_font(&font);
    cycle_layer.set_text_color(color::GCOLOR_WHITE);
    cycle_layer.set_background_color(color::GCOLOR_CLEAR);
    // cycle_layer.set_alignment(GTextAlignment_GTextAlignmentRight);
    window.add_child(&mut cycle_layer);

    let mut time_layer = TextLayer::new(GRect::new(0, 200, 200, 28)).unwrap();
    time_layer.set_font(&font);
    time_layer.set_text_color(color::GCOLOR_WHITE);
    time_layer.set_background_color(color::GCOLOR_CLEAR);
    window.add_child(&mut time_layer);

    let mut update = move |state: &mut State| {
        time_layer.set_text("time");
        cycle_layer.set_text("cycles");
        // log_c_str(c"setting alignment");
        // cycle_layer.set_alignment(2);
        // log_c_str(c"set alignment");

        state.cores.iter_mut().for_each(|c| {
            let sprite = if state.loaded {
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

    {
        update(&mut state.borrow_mut());
    }

    APP.set_message_handler(move |d| {
        log_c_str(c"got message");
        match d.get(10003) {
            Some(Value::CStr(t)) if *t == c"RESET" => {
                log_c_str(c"received reset");

                let mut state = state.borrow_mut();
                state.loaded = false;
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
                    state.loaded = true;
                    level_data.iter().enumerate().for_each(|(i, level)| {
                        state.cores[i].level = *level;
                    });

                    broken_data.iter().enumerate().for_each(|(i, broken)| {
                        state.cores[i].broken = *broken != 0;
                    });

                    update(&mut state);
                } else {
                    log_c_str(c"received invalid state");
                }
            }
            _ => {}
        };
    });

    APP.open_inbox(crate::app::InboxSize::Half).unwrap();

    // let request_update = || {
    //     log_c_str(c"requesting update");
    //     APP.send_message(|b| {
    //         b.write_cstr(10003, c"REFRESH")?;
    //         Ok(())
    //     })
    //     .is_ok()
    // };

    // request_update();

    // Timer::repeat(Duration::from_mins(10), request_update);

    window.show();

    APP.event_loop();
}
