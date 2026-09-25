use core::{ffi::CStr, ptr::NonNull};

use alloc::vec::Vec;

use crate::{
    GColor, GPoint, GRect, Layer, TextAlignment,
    font::Font,
    handle::{Handle, new_handle},
    layer::{ChildLayer, LayerInner},
    sys,
};

struct TextLayerInner {
    raw: NonNull<sys::TextLayer>,
    base_layer: Layer,
    font: Option<Font>,
    text_vec: Vec<u8>,
}

impl Drop for TextLayerInner {
    fn drop(&mut self) {
        unsafe { sys::text_layer_destroy(self.raw.as_ptr()) }
    }
}

/// A layer that displays text.
#[derive(Clone)]
pub struct TextLayer {
    handle: Handle<TextLayerInner>,
}

impl ChildLayer for TextLayer {
    fn remove_from_parent(&self) {
        self.handle.borrow_mut().base_layer.remove_from_parent();
    }

    fn id(&self) -> usize {
        self.handle.borrow().base_layer.id()
    }

    fn ptr_to_child_with(&mut self) -> *mut sys::Layer {
        self.handle.borrow_mut().base_layer.ptr_to_child_with()
    }

    fn record_new_parent(&self, parent: &Layer) {
        self.handle
            .borrow_mut()
            .base_layer
            .record_new_parent(parent);
    }
}

impl TextLayer {
    /// Create a new text layer with the given bounds.
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

    /// Set the font for this layer.
    pub fn set_font(&mut self, font: &Font) {
        self.inner_mut(|inner| {
            inner.font = Some(font.clone());
            unsafe {
                sys::text_layer_set_font(inner.raw.as_ptr(), font.handle.borrow().raw.as_ptr())
            };
        });
    }

    fn inner_mut(&mut self, f: impl FnOnce(&mut TextLayerInner)) {
        let mut inner = self.handle.borrow_mut();
        f(&mut inner);
    }

    /// Set the text for this layer.
    pub fn set_text(&mut self, text: &str) {
        self.inner_mut(|inner| {
            inner.text_vec.clear();
            inner.text_vec.reserve(text.len() + 1);
            inner.text_vec.extend(text.bytes());
            inner.text_vec.push(0);
            unsafe { sys::text_layer_set_text(inner.raw.as_ptr(), inner.text_vec.as_ptr()) };
        });
    }

    /// Set the raw text bytes (except the null terminator) for this layer.
    pub fn set_text_bytes(&mut self, text: &[u8]) {
        self.inner_mut(|inner| {
            inner.text_vec.clear();
            inner.text_vec.reserve(text.len() + 1);
            inner.text_vec.extend(text);
            inner.text_vec.push(0);
            unsafe { sys::text_layer_set_text(inner.raw.as_ptr(), inner.text_vec.as_ptr()) };
        });
    }

    /// Set the text for this layer via a C string.
    // Text lifetime must outlive this lifetime, since the C API does not copy the string.
    pub fn set_text_c_str<'s, 't: 's>(&'s mut self, text: &'t CStr) {
        self.inner_mut(|inner| {
            unsafe { sys::text_layer_set_text(inner.raw.as_ptr(), text.as_ptr()) };
            inner.text_vec.clear();
        });
    }

    /// Set the background color for this layer.
    pub fn set_background_color(&mut self, color: GColor) {
        self.inner_mut(|inner| {
            unsafe { sys::text_layer_set_background_color(inner.raw.as_ptr(), color) };
        });
    }

    /// Set the text color for this layer.
    pub fn set_text_color(&mut self, color: GColor) {
        self.inner_mut(|inner| {
            unsafe { sys::text_layer_set_text_color(inner.raw.as_ptr(), color) };
        });
    }

    /// Set the text’s alignment.
    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        self.inner_mut(|inner| {
            unsafe {
                sys::text_layer_set_text_alignment(
                    inner.raw.as_ptr(),
                    alignment as sys::GTextAlignment,
                )
            };
        });
    }

    /// Set the text bounding box.
    pub fn set_bounds(&mut self, bounds: GRect) {
        self.inner_mut(|inner| {
            inner.base_layer.set_bounds(bounds);
        });
    }

    /// Sets the frame (bounding box within parent) of the layer.
    pub fn set_frame(&mut self, frame: GRect) {
        self.inner_mut(|inner| {
            inner.base_layer.set_frame(frame);
        });
    }

    /// Returns whether clipping is enabled for this layer.
    pub fn is_clipping_enabled(&self) -> bool {
        self.handle.borrow().base_layer.is_clipping_enabled()
    }

    /// Enables/disables clipping.
    pub fn set_clipping_enabled(&mut self, clips: bool) {
        self.handle
            .borrow_mut()
            .base_layer
            .set_clipping_enabled(clips)
    }

    /// Returns whether the layer is hidden or not.
    pub fn is_hidden(&self) -> bool {
        self.handle.borrow().base_layer.is_hidden()
    }

    /// Hides/shows the layer.
    pub fn set_hidden(&mut self, hidden: bool) {
        self.handle.borrow_mut().base_layer.set_hidden(hidden)
    }

    /// Returns the bounds within the layer that are not obstructed by system UI.
    /// The associated overlay functionality is not available on Aplite, where this always returns the full bounds.
    pub fn get_unobstructed_bounds(&self) -> GRect {
        self.handle.borrow().base_layer.get_unobstructed_bounds()
    }

    /// Convert a point in layer coordinates to screen coordinates.
    pub fn convert_point_to_screen(&self, point: GPoint) -> GPoint {
        self.handle
            .borrow_mut()
            .base_layer
            .convert_point_to_screen(point)
    }

    /// Convert a rectangle in layer coordinates to screen coordinates.
    pub fn convert_rect_to_screen(&self, rect: GRect) -> GRect {
        self.handle.borrow().base_layer.convert_rect_to_screen(rect)
    }
}
