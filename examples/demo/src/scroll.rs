use pebble_sdk::{
    Button, ContentIndicatorConfig, ContentIndicatorDirection, GAlign, GRect, GSize, Layer,
    ScrollLayer, TextLayer, Window, hex_color, trace, warn,
};

pub fn scroll() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(hex_color!("#ffffff"));

    let mut scroll = ScrollLayer::new(window.get_bounds()).unwrap();

    let colors = [
        hex_color!("#f00"),
        hex_color!("#ff0"),
        hex_color!("#0f0"),
        hex_color!("#0ff"),
        hex_color!("#00f"),
        hex_color!("#f0f"),
        hex_color!("#f00"),
        hex_color!("#ff0"),
        hex_color!("#0f0"),
        hex_color!("#0ff"),
        hex_color!("#00f"),
        hex_color!("#f0f"),
        hex_color!("#f00"),
        hex_color!("#ff0"),
        hex_color!("#0f0"),
        hex_color!("#0ff"),
        hex_color!("#00f"),
        hex_color!("#f0f"),
        hex_color!("#f00"),
        hex_color!("#ff0"),
        hex_color!("#0f0"),
        hex_color!("#0ff"),
        hex_color!("#00f"),
        hex_color!("#f0f"),
    ];

    let mut offset_y = 0;
    let block_height = 60;
    for color in colors {
        let mut block = TextLayer::new(GRect::new(0, offset_y, 200, block_height)).unwrap();
        block.set_background_color(color);
        scroll.add_child(&mut block);
        offset_y += block_height;
    }

    let mut up_layer =
        Layer::new(GRect::new(0, 0, 16, 16).align(&window.get_bounds(), GAlign::Top)).unwrap();

    let mut down_layer =
        Layer::new(GRect::new(0, 0, 16, 16).align(&window.get_bounds(), GAlign::Bottom)).unwrap();

    window.add_child(&mut scroll);
    window.add_child(&mut up_layer);
    window.add_child(&mut down_layer);
    scroll.with_content_indicator(|indicator| {
        indicator
            .configure_direction(
                ContentIndicatorDirection::Up,
                ContentIndicatorConfig::basic(up_layer),
            )
            .unwrap();
        indicator
            .configure_direction(
                ContentIndicatorDirection::Down,
                ContentIndicatorConfig::basic(down_layer),
            )
            .unwrap();
        trace!("content indicator initialized");
    });

    scroll.set_content_size(GSize::new(200, offset_y));

    scroll.set_click_config_onto_window(&mut window);
    scroll.set_click_provider({
        let scroll = scroll.downgrade();
        move |c| {
            if true {
                c.single(
                    Button::Select,
                    {
                        let scroll = scroll.clone();
                        move |_| {
                            let Some(mut scroll) = scroll.upgrade() else {
                                warn!("Unexpected: Scroll failed to upgrade");
                                return;
                            };
                            let hidden = !scroll.get_paging_enabled();
                            scroll.set_paging_enabled(hidden);
                        }
                    },
                    None,
                );
            }
        }
    });

    window
}
