use core::time::Duration;

use alloc::{boxed::Box, vec::Vec};
use pebble_rust_2026::{
    Bitmap, BitmapLayer, GRect, Timer, Window, color::GCOLOR_WHITE, resource_ids,
};

resource_ids!(resource_ids);

pub fn bitmaps() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);
    let bounds = window.get_bounds();
    let hero = Bitmap::from_resource(resource_ids::HERO).unwrap();

    let x = bounds.size.w / 2 - 8;
    let center_y = bounds.size.h / 2 - 8;
    let spacing = 40;

    let mut animations = Vec::new();

    // Walking
    {
        let frames = [
            hero.extract(GRect::new(0, 16, 16, 16)).unwrap(),
            hero.extract(GRect::new(16, 16, 16, 16)).unwrap(),
            hero.extract(GRect::new(32, 16, 16, 16)).unwrap(),
            hero.extract(GRect::new(48, 16, 16, 16)).unwrap(),
            hero.extract(GRect::new(64, 16, 16, 16)).unwrap(),
            hero.extract(GRect::new(80, 16, 16, 16)).unwrap(),
        ];
        let mut layer = BitmapLayer::new(GRect::new(x, center_y - spacing, 16, 16)).unwrap();
        window.add_child(&mut layer);
        layer.set_bitmap(&frames[0]);
        animations.push((frames, layer));
    }

    // Swimming
    {
        let frames = [
            hero.extract(GRect::new(0, 48, 16, 16)).unwrap(),
            hero.extract(GRect::new(16, 48, 16, 16)).unwrap(),
            hero.extract(GRect::new(32, 48, 16, 16)).unwrap(),
            hero.extract(GRect::new(48, 48, 16, 16)).unwrap(),
            hero.extract(GRect::new(64, 48, 16, 16)).unwrap(),
            hero.extract(GRect::new(80, 48, 16, 16)).unwrap(),
        ];
        let mut layer = BitmapLayer::new(GRect::new(x, center_y, 16, 16)).unwrap();
        window.add_child(&mut layer);
        layer.set_bitmap(&frames[0]);
        animations.push((frames, layer));
    }

    // Fighting
    {
        let frames = [
            hero.extract(GRect::new(0, 64, 16, 16)).unwrap(),
            hero.extract(GRect::new(0, 64, 16, 16)).unwrap(),
            hero.extract(GRect::new(0, 64, 16, 16)).unwrap(),
            hero.extract(GRect::new(0, 64, 16, 16)).unwrap(),
            hero.extract(GRect::new(16, 64, 16, 16)).unwrap(),
            hero.extract(GRect::new(32, 64, 16, 16)).unwrap(),
        ];
        let mut layer = BitmapLayer::new(GRect::new(x, center_y + spacing, 16, 16)).unwrap();
        window.add_child(&mut layer);
        layer.set_bitmap(&frames[0]);
        animations.push((frames, layer));
    }

    window.set_appear_effect(Box::new(move || {
        let timers: Vec<Timer> = animations
            .iter()
            .map(|(frames, layer)| {
                let frames = frames.clone();
                let mut layer = layer.clone();
                let mut frame = 0;
                Timer::repeat(Duration::from_millis(100), move || {
                    frame = (frame + 1) % frames.len();
                    layer.set_bitmap(&frames[frame]);
                    true
                })
                .unwrap()
            })
            .collect();

        Box::new(|| {
            timers.into_iter().for_each(|e| e.cancel());
        })
    }));

    window
}
