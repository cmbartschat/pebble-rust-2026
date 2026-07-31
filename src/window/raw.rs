use core::{ffi::c_void, ptr::NonNull};

use alloc::boxed::Box;

use crate::{
    GColor, GRect,
    input::{context::InputContext, handlers::global_click_config_handler},
    sys,
    window::{callbacks, user_data::WindowUserData},
};

pub type Callback = Box<dyn FnMut() + 'static>;

pub(crate) struct WindowRaw {
    raw: NonNull<sys::Window>,
}

impl Drop for WindowRaw {
    fn drop(&mut self) {
        unsafe { sys::window_destroy(self.raw.as_ptr()) };
    }
}

impl WindowRaw {
    pub fn new() -> Option<Self> {
        let window = unsafe { sys::window_create() };

        let res = Self {
            raw: NonNull::new(window)?,
        };

        let handlers = sys::WindowHandlers {
            load: Some(callbacks::global_handle_load),
            appear: Some(callbacks::global_handle_appear),
            disappear: Some(callbacks::global_handle_disappear),
            unload: Some(callbacks::global_handle_unload),
        };
        unsafe { sys::window_set_window_handlers(window, handlers) };

        Some(res)
    }

    pub(crate) fn as_ptr_mut(&mut self) -> *mut sys::Window {
        self.raw.as_ptr()
    }

    fn as_ptr(&self) -> *const sys::Window {
        self.raw.as_ptr()
    }

    pub fn set_background_color(&mut self, color: GColor) {
        unsafe { sys::window_set_background_color(self.as_ptr_mut(), color) };
    }

    pub(crate) unsafe fn get_root_layer(&self) -> *mut sys::Layer {
        unsafe { sys::window_get_root_layer(self.as_ptr()) }
    }

    pub unsafe fn set_user_data(&mut self, data: *mut WindowUserData) {
        unsafe { sys::window_set_user_data(self.as_ptr_mut(), data as *mut c_void) };
    }

    pub(crate) fn is_equal(&self, other: *const sys::Window) -> bool {
        self.as_ptr() == other
    }

    pub(crate) unsafe fn set_click_context(&mut self, context: *mut InputContext) {
        unsafe {
            sys::window_set_click_config_provider_with_context(
                self.as_ptr_mut(),
                Some(global_click_config_handler),
                context as *mut c_void,
            );
        }
    }

    pub(crate) fn create_simple_menu_layer(
        &mut self,
        frame: GRect,
        options: &[sys::SimpleMenuSection],
        context: *mut c_void,
    ) -> *mut sys::SimpleMenuLayer {
        unsafe {
            sys::simple_menu_layer_create(
                frame,
                self.as_ptr_mut(),
                options.as_ptr(),
                options.len() as i32,
                context,
            )
        }
    }

    pub(crate) fn add_action_bar_layer(&mut self, layer: *mut sys::ActionBarLayer) {
        unsafe { sys::action_bar_layer_add_to_window(layer, self.as_ptr_mut()) };
    }

    pub(crate) fn set_scroll_layer_click_config(&mut self, layer: *mut sys::ScrollLayer) {
        unsafe { sys::scroll_layer_set_click_config_onto_window(layer, self.as_ptr_mut()) };
    }
}
