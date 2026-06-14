use core::{cell::RefCell, ffi::c_void, ptr::NonNull};

use alloc::{boxed::Box, rc::Rc};

use crate::{
    Layer,
    layer::{ChildLayer, LayerInner},
    sys::{self, WindowHandlers, window_destroy},
};

pub struct Window {
    pub(crate) raw: NonNull<sys::Window>,
    pub(crate) root_layer: Layer,
}

type Callback<'a> = Box<dyn FnMut() + 'a>;

struct WindowUserData<'a> {
    load_handler: Option<Callback<'a>>,
    appear_handler: Option<Callback<'a>>,
    disappear_handler: Option<Callback<'a>>,
    unload_handler: Option<Callback<'a>>,
}

impl Drop for Window {
    fn drop(&mut self) {
        // TODO(christoph): Free user handler, currently dangling
        unsafe { window_destroy(self.raw.as_ptr()) };
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

impl Window {
    pub fn new() -> Option<Self> {
        unsafe {
            let window = sys::window_create();

            let Some(layer) = LayerInner::from_ptr(sys::window_get_root_layer(window), false)
            else {
                sys::window_destroy(window);
                return None;
            };

            let res = Self {
                raw: NonNull::new(window)?,
                root_layer: Layer {
                    handle: Rc::new(RefCell::new(layer)),
                },
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
            Some(res)
        }
    }

    fn get_user_data<'b>(&'b mut self) -> &'b mut WindowUserData {
        unsafe {
            let ptr = core::mem::transmute::<*mut c_void, *mut WindowUserData>(
                sys::window_get_user_data(self.raw.as_ptr()),
            );
            ptr.as_mut().unwrap()
        }
    }

    pub fn set_load_handler<'a>(&'a mut self, callback: impl FnMut() + 'a) {
        self.get_user_data().load_handler = Some(Box::new(callback));
    }

    pub fn clear_load_handler(&mut self) {
        self.get_user_data().load_handler = None;
    }

    pub fn set_unload_handler<'a>(&'a mut self, callback: impl FnMut() + 'a) {
        self.get_user_data().unload_handler = Some(Box::new(callback));
    }

    pub fn clear_unload_handler(&mut self) {
        self.get_user_data().unload_handler = None;
    }

    pub fn set_appear_handler<'a>(&'a mut self, callback: impl FnMut() + 'a) {
        self.get_user_data().appear_handler = Some(Box::new(callback));
    }

    pub fn clear_appear_handler(&mut self) {
        self.get_user_data().appear_handler = None;
    }

    pub fn set_disappear_handler<'a>(&'a mut self, callback: impl FnMut() + 'a) {
        self.get_user_data().disappear_handler = Some(Box::new(callback));
    }

    pub fn clear_disappear_handler(&mut self) {
        self.get_user_data().disappear_handler = None;
    }

    pub fn set_background_color(&mut self, color: sys::GColor) {
        unsafe { sys::window_set_background_color(self.raw.as_ptr(), color) };
    }

    pub fn add_child<T>(&mut self, child: &mut T)
    where
        T: Clone + ChildLayer + 'static,
    {
        self.root_layer.add_child(child);
    }
}
