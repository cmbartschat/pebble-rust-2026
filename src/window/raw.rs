use core::{cell::RefCell, ffi::c_void, ptr::NonNull};

use alloc::boxed::Box;

use crate::{
    APP, GColor, GRect, Mutex, MutexToken,
    effect::Effect,
    input::{context::InputContext, handlers::global_click_config_handler},
    sys,
};

pub type Callback = Box<dyn FnMut() + 'static>;

pub(crate) struct WindowUserDataInner {
    pub(crate) load_handler: Option<Callback>,
    pub(crate) appear_handler: Option<Callback>,
    pub(crate) disappear_handler: Option<Callback>,
    pub(crate) unload_handler: Option<Callback>,
    pub(crate) appear_effect: Effect,
    pub(crate) load_effect: Effect,
}

pub(crate) struct WindowUserData {
    pub(crate) inner: Mutex<RefCell<WindowUserDataInner>>,
}

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
            load: Some(global_handle_load),
            appear: Some(global_handle_appear),
            disappear: Some(global_handle_disappear),
            unload: Some(global_handle_unload),
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

    pub(crate) fn stack_push(&mut self, animated: bool) {
        unsafe { sys::window_stack_push(self.as_ptr_mut(), animated) };
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

extern "C" fn global_handle_load(window: *mut sys::Window) {
    unsafe {
        let void_ptr = sys::window_get_user_data(window);
        let user_data_ptr = void_ptr as *mut WindowUserData;
        let Some(data) = user_data_ptr.as_mut() else {
            panic!("Window does not have a user data");
        };
        MutexToken::with(|t| {
            let mut data = data.inner.borrow_mut(t);
            if let Some(handler) = &mut data.load_handler {
                handler();
            };
            data.load_effect.mount();
        });
    }
}

extern "C" fn global_handle_appear(window: *mut sys::Window) {
    unsafe {
        let void_ptr = sys::window_get_user_data(window);
        let user_data_ptr = void_ptr as *mut WindowUserData;
        let Some(data) = user_data_ptr.as_mut() else {
            panic!("Window does not have a user data");
        };

        MutexToken::with(|t| {
            let mut data = data.inner.borrow_mut(t);
            if let Some(handler) = &mut data.appear_handler {
                handler();
            };
            data.appear_effect.mount();
        });
    }
}

extern "C" fn global_handle_disappear(window: *mut sys::Window) {
    unsafe {
        let void_ptr = sys::window_get_user_data(window);
        let user_data_ptr = void_ptr as *mut WindowUserData;
        let Some(data) = user_data_ptr.as_mut() else {
            panic!("Window does not have a user data");
        };
        MutexToken::with(|t| {
            let mut data = data.inner.borrow_mut(t);
            data.appear_effect.unmount();
            if let Some(handler) = &mut data.disappear_handler {
                handler();
            };
        });
    }
}

extern "C" fn global_handle_unload(window: *mut sys::Window) {
    unsafe {
        let void_ptr = sys::window_get_user_data(window);
        let user_data_ptr = void_ptr as *mut WindowUserData;
        let Some(data) = user_data_ptr.as_mut() else {
            panic!("Window does not have a user data");
        };
        MutexToken::with(|t| {
            let mut data = data.inner.borrow_mut(t);
            data.load_effect.unmount();
            if let Some(handler) = &mut data.unload_handler {
                handler();
            };
        });
    }
    APP.notify_unload(window);
}
