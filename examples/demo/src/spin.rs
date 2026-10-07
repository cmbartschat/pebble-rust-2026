use core::{
    cell::{Cell, RefCell},
    time::Duration,
};

use alloc::{boxed::Box, rc::Rc, vec::Vec};
use pebble_sdk::{
    APP, ActionBarLayer, ActionButton, ActionMenu, ActionMenuAlign, ActionMenuLevel,
    ActionMenuLevelDisplayMode, Angle, Bitmap, Button, CompassHeading, GColor, GContext, GPoint,
    GRect, GSize, Layer, Mutex, TextLayer, Timer, Window,
    color::{
        GCOLOR_BLACK, GCOLOR_CLEAR, GCOLOR_GREEN, GCOLOR_RED, GCOLOR_SHOCKING_PINK,
        GCOLOR_VERY_LIGHT_BLUE, GCOLOR_WHITE, GCOLOR_YELLOW,
    },
    hex_color, resource_ids, sys, warn,
};

struct SpinState {
    goal_position: Angle,
    current_position: Angle,
}

resource_ids!(resource_ids);

static COLOR: Mutex<Cell<GColor>> = Mutex::new(Cell::new(GCOLOR_BLACK));

extern "C" fn fill_circle(_layer: *mut sys::Layer, ctx: *mut sys::GContext) {
    let mut ctx = unsafe { GContext::from_raw(ctx) }.unwrap();
    ctx.set_fill_color(COLOR.get());
    ctx.fill_circle(GPoint { x: 5, y: 5 }, 4);

    ctx.set_stroke_color(GCOLOR_BLACK);
    ctx.draw_circle(GPoint { x: 5, y: 5 }, 4);
}

pub fn spin() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(hex_color!("#fff"));
    let state = Rc::new(RefCell::new(SpinState {
        goal_position: Angle::from_degrees(0),
        current_position: Angle::from_degrees(0),
    }));

    let bounds = window.get_bounds();
    let center_x = bounds.origin.x + bounds.size.w / 2 - 20;
    let center_y = bounds.origin.y + bounds.size.h / 2;

    let mut center_layer = Layer::new(bounds).unwrap();
    window.add_child(&mut center_layer);

    let mut circle_layers = Vec::with_capacity(8);

    for _ in 0..8 {
        let mut layer = Layer::new(GRect::new(0, 0, 10, 10)).unwrap();
        layer.set_raw_update_handler(fill_circle);
        center_layer.add_child(&mut layer);
        circle_layers.push(layer);
    }

    let compass_layer_bounds = window.get_bounds().shrink(15);

    let compass_layer = {
        let mut layer = Layer::new(GRect::new(0, 0, 0, 0)).unwrap();
        layer.set_raw_update_handler(fill_circle);
        center_layer.add_child(&mut layer);
        layer
    };

    let update = {
        let state = state.clone();
        let window = window.downgrade();
        move || {
            if window.upgrade().is_none() {
                return false;
            }
            let mut state = state.borrow_mut();
            state.current_position = state
                .current_position
                .towards(state.goal_position, Angle::from_degrees(8));

            let x_offset = state.current_position.cos().scale(80) as i16;
            let y_offset = -state.current_position.sin().scale(80) as i16;

            for (i, layer) in circle_layers.iter_mut().enumerate() {
                let i = i as i16;
                layer.set_frame(GRect::new(
                    center_x + (x_offset * i) / 10,
                    center_y + (y_offset * i) / 10,
                    10,
                    10,
                ));
            }
            true
        }
    };

    {
        let mut action_bar = ActionBarLayer::new().unwrap();
        action_bar.add_to_window(&mut window);

        action_bar.set_icon(
            ActionButton::Up,
            Bitmap::from_resource(resource_ids::UP).unwrap(),
        );

        action_bar.set_icon(
            ActionButton::Down,
            Bitmap::from_resource(resource_ids::DOWN).unwrap(),
        );

        action_bar.set_icon(
            ActionButton::Select,
            Bitmap::from_resource(resource_ids::COLOR).unwrap(),
        );

        action_bar.set_click_provider({
            let state = state.clone();
            move |input| {
                let v1 = state.clone();
                input.single(
                    Button::Up,
                    move |_| {
                        let mut value = v1.borrow_mut();
                        value.goal_position += Angle::from_degrees(60);
                    },
                    None,
                );

                let v2 = state.clone();
                input.single(
                    Button::Down,
                    move |_| {
                        let mut value = v2.borrow_mut();
                        value.goal_position -= Angle::from_degrees(60);
                    },
                    None,
                );

                input.single(
                    Button::Select,
                    move |_| {
                        let mut level = ActionMenuLevel::new();
                        level.add_action("Black", move || COLOR.set(GCOLOR_BLACK));
                        level.add_action("White", move || COLOR.set(GCOLOR_WHITE));
                        let mut color_level = ActionMenuLevel::new();

                        color_level.add_action("Pink", move || COLOR.set(GCOLOR_SHOCKING_PINK));
                        color_level.add_action("Blue", move || COLOR.set(GCOLOR_VERY_LIGHT_BLUE));
                        color_level.add_action("Red", move || COLOR.set(GCOLOR_RED));
                        color_level.add_action("Yellow", move || COLOR.set(GCOLOR_YELLOW));
                        color_level.add_action("Green", move || COLOR.set(GCOLOR_GREEN));
                        color_level.set_display_mode(ActionMenuLevelDisplayMode::Thin);
                        level.add_child("More Colors...", color_level);
                        ActionMenu::begin(level)
                            .set_align(ActionMenuAlign::Center)
                            .open();
                    },
                    None,
                );
            }
        });
    }

    {
        let mut label_layer = TextLayer::new(GRect::new(10, 10, 100, 20)).unwrap();
        label_layer.set_text_c_str(c"spin");
        label_layer.set_background_color(GCOLOR_CLEAR);
        window.add_child(&mut label_layer);
    }

    window.set_appear_effect(Box::new(move || {
        let timer = Timer::repeat(Duration::from_millis(40), update.clone()).unwrap();
        let mut compass_layer = compass_layer.clone();
        let compass_callback = APP.compass.subscribe(Box::new(move |angle| {
            let angle = match angle {
                CompassHeading::Invalid | CompassHeading::Unavailable => {
                    warn!("compass unavailable/invalid");
                    compass_layer.set_bounds(GRect::new(0, 0, 0, 0));
                    return;
                }
                CompassHeading::Calibrating(angle) | CompassHeading::Calibrated(angle) => angle,
            };
            compass_layer.set_frame(GRect::new_on_circle(
                compass_layer_bounds,
                angle,
                GSize::new(10, 10),
            ));
        }));
        Box::new(move || {
            timer.cancel();
            compass_callback.cancel();
        })
    }));

    window
}
