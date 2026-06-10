// use alloc::{boxed::Box, ffi::CString, vec, vec::Vec};
use core::{ffi::CStr, ptr::null};

use crate::{
    layer::Layer,
    sys::{self, GColor, GFont, GRect},
};

pub struct TextLayer {
    inner: *mut sys::TextLayer,
    text: [u8; 60],
}

impl TextLayer {
    pub fn new(r: GRect) -> Result<Self, ()> {
        unsafe {
            let layer = sys::text_layer_create(r);
            if layer.is_null() {
                return Err(());
            }
            Ok(Self {
                inner: layer,
                text: [0; 60],
                // text: CString::new(vec![]).unwrap(),
            })
        }
    }

    pub fn set_font(&mut self, font: GFont) {
        unsafe { sys::text_layer_set_font(self.inner, font) };
    }

    pub fn set_text(&mut self, text: &'static CStr) {
        // let bytes: Vec<u8> = text.bytes().collect();
        self.text[0] = 0x48;
        self.text[1] = 0x49;
        self.text[2] = 0x21;
        // self.text = CString::new(bytes).unwrap();
        unsafe { sys::text_layer_set_text(self.inner, self.text.as_ptr()) };
    }

    pub fn set_background_color(&mut self, color: GColor) {
        unsafe { sys::text_layer_set_background_color(self.inner, color) };
    }

    pub fn set_text_color(&mut self, color: GColor) {
        unsafe { sys::text_layer_set_text_color(self.inner, color) };
    }

    pub fn get_layer(&self) -> Layer {
        let layer = unsafe { sys::text_layer_get_layer(self.inner) };
        Layer { inner: layer }
    }
}
