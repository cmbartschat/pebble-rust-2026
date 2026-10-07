use core::{cell::RefCell, time::Duration};

use alloc::rc::Rc;
use pebble_sdk::{
    Button, ClickRecognizer, GPoint, GRect, TextLayer, Window,
    color::{GCOLOR_GREEN, GCOLOR_WHITE},
    log::info,
};

pub fn move_box() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);

    {
        let mut label_layer = TextLayer::new(GRect::new(10, 10, 100, 20)).unwrap();
        label_layer.set_text_c_str(c"move_box");
        window.add_child(&mut label_layer);
    }

    #[derive(Clone, Copy)]
    enum Direction {
        Horizontal,
        Vertical,
    }

    let max_bounds = window.get_bounds().shrink(10);
    let min_box_position = max_bounds.origin;
    let max_box_position = GPoint {
        x: max_bounds.origin.x + (max_bounds.size.w - 30),
        y: max_bounds.origin.y + (max_bounds.size.h - 30),
    };

    let box_frame = GRect::new(
        max_bounds.origin.x + max_bounds.size.w / 2 - 15,
        max_bounds.origin.y + max_bounds.size.h / 2 - 15,
        30,
        30,
    );

    let mut box_layer = TextLayer::new(box_frame).unwrap();
    box_layer.set_background_color(GCOLOR_GREEN);
    window.add_child(&mut box_layer);

    let mut direction_layer =
        TextLayer::new(GRect::new(10, max_bounds.size.h - 30, 100, 20)).unwrap();
    direction_layer.set_text_c_str(c"Horizontal");
    window.add_child(&mut direction_layer);

    let current_direction = Rc::new(RefCell::new(Direction::Horizontal));
    let box_frame = Rc::new(RefCell::new(box_frame));
    let handle_direction = {
        let current_direction = current_direction.clone();
        move |c: &ClickRecognizer| {
            let mut box_frame = box_frame.borrow_mut();
            info!("got input");
            let offset = 10 * if c.button() == Button::Up { 1 } else { -1 };
            match *current_direction.borrow() {
                Direction::Horizontal => {
                    box_frame.origin.x =
                        (box_frame.origin.x + offset).clamp(min_box_position.x, max_box_position.x);
                }
                Direction::Vertical => {
                    box_frame.origin.y =
                        (box_frame.origin.y - offset).clamp(min_box_position.y, max_box_position.y);
                }
            };
            box_layer.set_frame(*box_frame);
            box_layer.set_bounds(GRect::new(0, 0, 30, 30));
        }
    };

    let handle_swap = move |_: &ClickRecognizer| {
        let mut current = current_direction.borrow_mut();
        let new_direction = match *current {
            Direction::Horizontal => {
                direction_layer.set_text_c_str(c"Vertical");
                Direction::Vertical
            }
            Direction::Vertical => {
                direction_layer.set_text_c_str(c"Horizontal");
                Direction::Horizontal
            }
        };
        *current = new_direction;
    };

    window.set_click_provider(move |b| {
        b.single(Button::Up, handle_direction.clone(), None);
        b.single(Button::Down, handle_direction.clone(), None);
        b.single(
            Button::Select,
            handle_swap.clone(),
            Some(Duration::from_millis(400)),
        );
    });

    window
}
