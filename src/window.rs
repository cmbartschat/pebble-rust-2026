use crate::{
    layer::Layer,
    sys::{self, GColor8, window_set_background_color},
};

pub struct Window {
    inner: *mut sys::Window,
}

impl Window {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let window = sys::window_create();
            if window.is_null() {
                return Err(());
            }
            window_set_background_color(window, GColor8 { argb: 0b11001100 });
            Ok(Self { inner: window })
        }
    }

    pub fn push(&mut self) {
        unsafe { sys::window_stack_push(self.inner, false) };
    }

    pub fn push_animated(&mut self) {
        unsafe { sys::window_stack_push(self.inner, true) };
    }

    pub fn add_child(&mut self, other: &Layer) {
        unsafe { sys::layer_add_child(sys::window_get_root_layer(self.inner), other.inner) };
    }
}
