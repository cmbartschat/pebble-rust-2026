use alloc::vec::Vec;

use crate::{
    font::Font,
    layer::Layer,
    sys::{self, GColor, GRect},
};

pub struct TextLayerCreateFailed;

pub struct TextLayer {
    inner: *mut sys::TextLayer,
    text_vec: Vec<u8>,
}

impl TextLayer {
    pub fn new(r: GRect) -> Result<Self, TextLayerCreateFailed> {
        unsafe {
            let layer = sys::text_layer_create(r);
            if layer.is_null() {
                return Err(TextLayerCreateFailed);
            }

            Ok(Self {
                inner: layer,
                text_vec: alloc::vec![],
            })
        }
    }

    pub fn set_font(&mut self, font: Font) {
        unsafe { sys::text_layer_set_font(self.inner, font.inner) };
    }

    pub fn set_text(&mut self, text: &str) {
        self.text_vec.clear();
        self.text_vec.reserve(text.len() + 1);
        self.text_vec.extend(text.bytes());
        self.text_vec.push(0);
        unsafe { sys::text_layer_set_text(self.inner, self.text_vec.as_ptr()) };
    }

    pub fn set_background_color(&mut self, color: GColor) {
        unsafe { sys::text_layer_set_background_color(self.inner, color) };
    }

    pub fn set_text_color(&mut self, color: GColor) {
        unsafe { sys::text_layer_set_text_color(self.inner, color) };
    }

    pub fn get_layer(&self) -> Layer {
        let layer = unsafe { sys::text_layer_get_layer(self.inner) };
        Layer {
            inner: layer,
            owned: false,
        }
    }
}
