use core::pin::Pin;

use alloc::boxed::Box;

use crate::{
    Layer,
    handle::new_handle,
    layer::{ChildLayer, LayerInner},
    sys::{self},
    window::raw::{WindowRaw, WindowUserData},
};

pub struct WindowInner {
    // incoming references
    root_layer: Layer,
    // window itself
    raw: super::raw::WindowRaw,
    user_data: Pin<Box<WindowUserData>>,
}

impl WindowInner {
    pub fn new() -> Option<Self> {
        let raw = WindowRaw::new()?;
        let layer = unsafe { LayerInner::from_ptr(raw.get_root_layer(), false)? };

        let user_data = Box::pin(WindowUserData {
            load_handler: None,
            appear_handler: None,
            disappear_handler: None,
            unload_handler: None,
        });

        let mut res = WindowInner {
            root_layer: Layer {
                handle: new_handle(layer),
            },
            raw,
            user_data,
        };

        unsafe {
            let user_data: &mut WindowUserData = &mut res.user_data;
            res.raw.set_user_data(user_data as *mut WindowUserData);
        }

        Some(res)
    }

    pub fn set_background_color(&mut self, color: sys::GColor) {
        self.raw.set_background_color(color);
    }

    pub fn add_child<T>(&mut self, child: &mut T)
    where
        T: Clone + ChildLayer + 'static,
    {
        self.root_layer.add_child(child);
    }

    pub fn set_load_handler(&mut self, callback: impl FnMut() + 'static) {
        self.user_data.load_handler = Some(Box::new(callback));
    }

    pub fn clear_load_handler(&mut self) {
        self.user_data.load_handler = None;
    }

    pub fn set_unload_handler(&mut self, callback: impl FnMut() + 'static) {
        self.user_data.unload_handler = Some(Box::new(callback));
    }

    pub fn clear_unload_handler(&mut self) {
        self.user_data.unload_handler = None;
    }

    pub fn set_appear_handler(&mut self, callback: impl FnMut() + 'static) {
        self.user_data.appear_handler = Some(Box::new(callback));
    }

    pub fn clear_appear_handler(&mut self) {
        self.user_data.appear_handler = None;
    }

    pub fn set_disappear_handler(&mut self, callback: impl FnMut() + 'static) {
        self.user_data.disappear_handler = Some(Box::new(callback));
    }

    pub fn clear_disappear_handler(&mut self) {
        self.user_data.disappear_handler = None;
    }

    pub(crate) fn is_equal(&self, other: *const sys::Window) -> bool {
        self.raw.is_equal(other)
    }

    pub(crate) fn stack_push(&mut self, animated: bool) {
        self.raw.stack_push(animated);
    }

    pub(crate) fn stack_remove(&mut self, animated: bool) {
        self.raw.stack_remove(animated);
    }
}
