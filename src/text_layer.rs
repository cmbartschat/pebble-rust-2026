use core::{ffi::CStr, ptr::NonNull};

use alloc::{rc::Rc, vec::Vec};

use crate::{
    Layer,
    font::Font,
    handle::{Handle, new_handle},
    layer::{ChildLayer, LayerInner},
    sys::{self, GColor, GRect, GTextAlignment},
};

struct TextLayerInner {
    raw: NonNull<sys::TextLayer>,
    base_layer: Layer,
    font: Option<Rc<Font>>,
    text_vec: Vec<u8>,
}

#[derive(Clone)]
pub struct TextLayer {
    handle: Handle<TextLayerInner>,
}

impl ChildLayer for TextLayer {
    fn remove_from_parent(&self) {
        self.handle.borrow_mut().base_layer.remove_from_parent();
    }

    fn is_same(&self, other: &Layer) -> bool {
        self.handle.borrow().base_layer.is_same(other)
    }

    fn set_parent(&mut self, other: &mut Layer) {
        self.handle.borrow_mut().base_layer.set_parent(other);
    }
}

impl TextLayer {
    pub fn new(r: GRect) -> Option<Self> {
        unsafe {
            let raw = NonNull::new(sys::text_layer_create(r))?;

            let base = LayerInner::from_ptr(sys::text_layer_get_layer(raw.as_ptr()), false);
            let Some(base_layer) = base else {
                sys::text_layer_destroy(raw.as_ptr());
                return None;
            };

            Some(Self {
                handle: new_handle(TextLayerInner {
                    raw,
                    base_layer: Layer {
                        handle: new_handle(base_layer),
                    },
                    text_vec: Vec::new(),
                    font: None,
                }),
            })
        }
    }

    pub fn set_font(&mut self, font: &Rc<Font>) {
        self.inner_mut(|inner| {
            inner.font = Some(font.clone());
            unsafe { sys::text_layer_set_font(inner.raw.as_ptr(), font.raw.as_ptr()) };
        });
    }

    fn inner_mut(&mut self, f: impl FnOnce(&mut TextLayerInner)) {
        let mut inner = self.handle.borrow_mut();
        f(&mut inner);
    }

    pub fn set_text(&mut self, text: &str) {
        self.inner_mut(|inner| {
            inner.text_vec.clear();
            inner.text_vec.reserve(text.len() + 1);
            inner.text_vec.extend(text.bytes());
            inner.text_vec.push(0);
            unsafe { sys::text_layer_set_text(inner.raw.as_ptr(), inner.text_vec.as_ptr()) };
        });
    }

    pub fn set_text_bytes(&mut self, text: &[u8]) {
        self.inner_mut(|inner| {
            inner.text_vec.clear();
            inner.text_vec.reserve(text.len() + 1);
            inner.text_vec.extend(text);
            inner.text_vec.push(0);
            unsafe { sys::text_layer_set_text(inner.raw.as_ptr(), inner.text_vec.as_ptr()) };
        });
    }

    pub fn set_text_c_str(&mut self, text: &'static CStr) {
        self.inner_mut(|inner| {
            unsafe { sys::text_layer_set_text(inner.raw.as_ptr(), text.as_ptr()) };
            inner.text_vec.clear();
        });
    }

    pub fn set_background_color(&mut self, color: GColor) {
        self.inner_mut(|inner| {
            unsafe { sys::text_layer_set_background_color(inner.raw.as_ptr(), color) };
        });
    }

    pub fn set_text_color(&mut self, color: GColor) {
        self.inner_mut(|inner| {
            unsafe { sys::text_layer_set_text_color(inner.raw.as_ptr(), color) };
        });
    }

    pub fn set_alignment(&mut self, alignment: GTextAlignment) {
        self.inner_mut(|inner| {
            unsafe { sys::text_layer_set_text_alignment(inner.raw.as_ptr(), alignment) };
        });
    }

    pub fn set_bounds(&mut self, bounds: GRect) {
        self.inner_mut(|inner| {
            inner.base_layer.set_bounds(bounds);
        });
    }

    pub fn set_frame(&mut self, frame: GRect) {
        self.inner_mut(|inner| {
            inner.base_layer.set_frame(frame);
        });
    }
}
