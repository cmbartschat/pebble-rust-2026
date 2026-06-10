use crate::sys::{self, GRect};

pub struct Layer {
    pub(crate) inner: *mut sys::Layer,
}

impl Layer {
    pub fn new(r: GRect) -> Result<Self, ()> {
        unsafe {
            let layer = sys::layer_create(r);
            if layer.is_null() {
                return Err(());
            }
            Ok(Self { inner: layer })
        }
    }

    pub fn add_child(&mut self, other: &Self) {
        unsafe { sys::layer_add_child(self.inner, other.inner) };
    }

    pub fn mark_dirty(&mut self) {
        unsafe { sys::layer_mark_dirty(self.inner) };
    }
}
