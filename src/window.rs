use core::{ffi::c_void, marker::PhantomData};

use alloc::boxed::Box;

use crate::{
    layer::Layer,
    sys::{self, WindowHandlers, window_destroy},
};

pub struct Window<'a> {
    pub(crate) inner: *mut sys::Window,
    _user_data: PhantomData<WindowUserData<'a>>,
}

type Callback<'a> = Box<dyn FnMut() + 'a>;

struct WindowUserData<'a> {
    load_handler: Option<Callback<'a>>,
    appear_handler: Option<Callback<'a>>,
    disappear_handler: Option<Callback<'a>>,
    unload_handler: Option<Callback<'a>>,
}

impl Drop for Window<'_> {
    fn drop(&mut self) {
        // TODO(christoph): Free user handler, currently dangling
        unsafe { window_destroy(self.inner) };
    }
}

#[unsafe(no_mangle)]
extern "C" fn global_handle_load(window: *mut sys::Window) {
    unsafe {
        let void_ptr = sys::window_get_user_data(window);
        let user_data_ptr = core::mem::transmute::<*mut c_void, *mut WindowUserData<'_>>(void_ptr);
        let Some(data) = user_data_ptr.as_mut() else {
            panic!("Window does not have a user data");
        };
        let Some(handler) = data.load_handler.as_mut() else {
            return;
        };
        handler();
    }
}

#[unsafe(no_mangle)]
extern "C" fn global_handle_unload(window: *mut sys::Window) {
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

pub struct WindowCreateFailed;

impl<'a> Window<'a> {
    pub fn new() -> Result<Self, WindowCreateFailed> {
        unsafe {
            let window = sys::window_create();
            if window.is_null() {
                return Err(WindowCreateFailed);
            }

            let res = Self {
                inner: window,
                _user_data: PhantomData,
            };

            let user_data = Box::new(WindowUserData {
                load_handler: None,
                unload_handler: None,
                appear_handler: None,
                disappear_handler: None,
            });

            sys::window_set_user_data(window, Box::into_raw(user_data) as *mut c_void);

            let handlers = WindowHandlers {
                load: Some(global_handle_load),
                appear: None,
                disappear: None,
                unload: Some(global_handle_unload),
            };
            sys::window_set_window_handlers(window, handlers);
            Ok(res)
        }
    }

    fn get_user_data<'b>(&'b mut self) -> &'b mut WindowUserData<'a> {
        unsafe {
            let ptr = core::mem::transmute::<*mut c_void, *mut WindowUserData<'a>>(
                sys::window_get_user_data(self.inner),
            );
            ptr.as_mut().unwrap()
        }
    }

    pub fn set_load_handler(&mut self, callback: impl FnMut() + 'a) {
        self.get_user_data().load_handler = Some(Box::new(callback));
    }

    pub fn clear_load_handler(&mut self) {
        self.get_user_data().load_handler = None;
    }

    pub fn set_unload_handler(&mut self, callback: impl FnMut() + 'a) {
        self.get_user_data().unload_handler = Some(Box::new(callback));
    }

    pub fn clear_unload_handler(&mut self) {
        self.get_user_data().unload_handler = None;
    }

    pub fn set_appear_handler(&mut self, callback: impl FnMut() + 'a) {
        self.get_user_data().appear_handler = Some(Box::new(callback));
    }

    pub fn clear_appear_handler(&mut self) {
        self.get_user_data().appear_handler = None;
    }

    pub fn set_disappear_handler(&mut self, callback: impl FnMut() + 'a) {
        self.get_user_data().disappear_handler = Some(Box::new(callback));
    }

    pub fn clear_disappear_handler(&mut self) {
        self.get_user_data().disappear_handler = None;
    }

    pub fn set_background_color(&mut self, color: sys::GColor) {
        unsafe { sys::window_set_background_color(self.inner, color) };
    }

    pub fn add_child(&mut self, other: &Layer) {
        unsafe { sys::layer_add_child(sys::window_get_root_layer(self.inner), other.inner) };
    }
}
