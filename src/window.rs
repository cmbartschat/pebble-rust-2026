use core::marker::PhantomPinned;

use alloc::boxed::Box;

use crate::{
    layer::Layer,
    sys::{self, GColor8, WindowHandlers, window_destroy, window_set_background_color},
};

pub struct Window<'a> {
    pub(crate) inner: *mut sys::Window,
    user_data: WindowUserData<'a>,
    _pin: PhantomPinned,
}

type Callback<'a> = Box<dyn FnMut() + 'a>;

struct WindowUserData<'a> {
    load_handler: Option<Callback<'a>>,
    // appear_handler: Option<Box<dyn FnMut()>>,
    // disappear_handler: Option<Box<dyn FnMut()>>,
    unload_handler: Option<Callback<'a>>,
}

impl Drop for Window<'_> {
    fn drop(&mut self) {
        unsafe { window_destroy(self.inner) };
    }
}

unsafe extern "C" fn global_handle_load(window: *mut sys::Window) {
    unsafe {
        let context = sys::window_get_user_data(window);
        if context.is_null() {
            return;
        }
        let Some(data) = (context as *mut WindowUserData).as_mut() else {
            return;
        };
        let Some(handler) = data.load_handler.as_mut() else {
            return;
        };
        handler();
    }
}

unsafe extern "C" fn global_handle_unload(window: *mut sys::Window) {
    unsafe {
        let context = sys::window_get_user_data(window);
        if context.is_null() {
            return;
        }
        let Some(data) = (context as *mut WindowUserData).as_mut() else {
            return;
        };
        let Some(handler) = data.unload_handler.as_mut() else {
            return;
        };
        handler();
    }
}

impl<'a> Window<'a> {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let window = sys::window_create();
            if window.is_null() {
                return Err(());
            }

            let res = Self {
                inner: window,
                user_data: WindowUserData {
                    load_handler: None,
                    // appear_handler: None,
                    // disappear_handler: None,
                    unload_handler: None,
                },
                _pin: PhantomPinned,
            };

            sys::window_set_window_handlers(
                window,
                WindowHandlers {
                    load: Some(global_handle_load),
                    appear: None,
                    disappear: None,
                    unload: Some(global_handle_unload),
                },
            );

            Ok(res)
        }
    }

    pub fn set_load_handler(&mut self, callback: impl FnMut() + 'a) {
        self.user_data.load_handler = Some(Box::new(callback));
    }
    pub fn clear_load_handler(&mut self) {
        self.user_data.load_handler = None;
    }

    pub fn set_unload_handler(&mut self, callback: impl FnMut() + 'a) {
        self.user_data.unload_handler = Some(Box::new(callback));
    }

    pub fn clear_unload_handler(&mut self) {
        self.user_data.unload_handler = None;
    }

    pub fn set_background_color(&mut self, color: sys::GColor) {
        unsafe { sys::window_set_background_color(self.inner, color) };
    }

    pub fn add_child(&mut self, other: &Layer) {
        unsafe { sys::layer_add_child(sys::window_get_root_layer(self.inner), other.inner) };
    }
}
