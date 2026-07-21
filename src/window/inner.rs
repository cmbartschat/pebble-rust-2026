use core::{cell::RefCell, ffi::c_void, pin::Pin};

use alloc::boxed::Box;

use crate::{
    ClickConfigBuilder, GColor, GRect, Layer, Mutex, MutexToken,
    effect::{Effect, EffectCallback},
    handle::new_handle,
    input::context::{InputContext, InputReceiver},
    layer::{ChildLayer, LayerInner},
    sys,
    window::raw::{Callback, WindowRaw, WindowUserData, WindowUserDataInner},
};

struct ConnectedInput {
    handle: Box<dyn InputReceiver>,
}

pub struct WindowInner {
    // incoming references
    root_layer: Layer,
    // window itself
    raw: super::raw::WindowRaw,
    // referenced by window
    user_data: Pin<Box<WindowUserData>>,
    input_context: Pin<Box<InputContext>>,
    connected_input: Option<ConnectedInput>,
}

impl WindowInner {
    pub fn new() -> Option<Self> {
        let raw = WindowRaw::new()?;
        let layer = unsafe { LayerInner::from_ptr(raw.get_root_layer(), false)? };

        let user_data = Box::pin(WindowUserData {
            inner: Mutex::new(RefCell::new(WindowUserDataInner {
                load_handler: None,
                appear_handler: None,
                disappear_handler: None,
                unload_handler: None,
                appear_effect: Effect::None,
                load_effect: Effect::None,
            })),
        });

        let input_context = Box::pin(InputContext::default());

        let mut res = WindowInner {
            connected_input: None,
            root_layer: Layer {
                handle: new_handle(layer),
            },
            raw,
            user_data,
            input_context,
        };

        unsafe {
            let user_data: &mut WindowUserData = &mut res.user_data;
            res.raw.set_user_data(user_data as *mut WindowUserData);
        }

        Some(res)
    }

    pub(crate) unsafe fn as_ptr_mut(&mut self) -> *mut sys::Window {
        self.raw.as_ptr_mut()
    }

    pub fn set_background_color(&mut self, color: GColor) {
        self.raw.set_background_color(color);
    }

    pub fn add_child<T>(&mut self, child: &mut T)
    where
        T: Clone + ChildLayer + 'static,
    {
        self.root_layer.add_child(child);
    }

    fn edit_context(&mut self, callback: impl FnOnce(&mut WindowUserDataInner)) {
        MutexToken::with(|t| {
            callback(&mut self.user_data.inner.borrow_mut(t));
        });
    }

    pub fn set_load_handler(&mut self, callback: Callback) {
        self.edit_context(|user_data| {
            user_data.load_handler = Some(Box::new(callback));
        });
    }

    pub fn clear_load_handler(&mut self) {
        self.edit_context(|user_data| {
            user_data.load_handler = None;
        });
    }

    pub fn set_unload_handler(&mut self, callback: Callback) {
        self.edit_context(|user_data| {
            user_data.unload_handler = Some(Box::new(callback));
        })
    }

    pub fn clear_unload_handler(&mut self) {
        self.edit_context(|user_data| {
            user_data.unload_handler = None;
        })
    }

    pub fn set_appear_handler(&mut self, callback: Callback) {
        self.edit_context(|user_data| {
            user_data.appear_handler = Some(Box::new(callback));
        })
    }

    pub fn clear_appear_handler(&mut self) {
        self.edit_context(|user_data| {
            user_data.appear_handler = None;
        });
    }

    pub fn set_disappear_handler(&mut self, callback: Callback) {
        self.edit_context(|user_data| {
            user_data.disappear_handler = Some(Box::new(callback));
        });
    }

    pub fn clear_disappear_handler(&mut self) {
        self.edit_context(|user_data| {
            user_data.disappear_handler = None;
        });
    }

    pub(crate) fn is_equal(&self, other: *const sys::Window) -> bool {
        self.raw.is_equal(other)
    }

    pub(crate) fn stack_push(&mut self, animated: bool) {
        self.raw.stack_push(animated);
    }

    fn refresh_input_handler(&mut self) {
        if self.connected_input.is_none() {
            unsafe {
                let input_context: &mut InputContext = &mut self.input_context;
                self.raw
                    .set_click_context(input_context as *mut InputContext)
            };
        }
    }

    pub fn set_click_provider(&mut self, configure: impl Fn(&mut ClickConfigBuilder) + 'static) {
        self.input_context.configure_click = Some(Box::new(configure));
        self.refresh_input_handler();
    }

    pub fn get_bounds(&self) -> GRect {
        self.root_layer.get_bounds()
    }

    pub(crate) fn retain_input_receiver(&mut self, receiver: impl InputReceiver + 'static) {
        self.connected_input = Some(ConnectedInput {
            handle: Box::new(receiver),
        })
    }

    pub(crate) fn remove_input_receiver(&mut self, receiver: &dyn InputReceiver) {
        if self
            .connected_input
            .as_ref()
            .is_some_and(|f| f.handle.get_id() == receiver.get_id())
        {
            self.connected_input = None;
            self.refresh_input_handler();
        }
    }

    pub(crate) fn create_simple_menu_layer(
        &mut self,
        frame: GRect,
        options: &[sys::SimpleMenuSection],
        context: *mut c_void,
    ) -> *mut sys::SimpleMenuLayer {
        self.raw.create_simple_menu_layer(frame, options, context)
    }

    pub(crate) fn add_action_bar_layer(&mut self, layer: *mut sys::ActionBarLayer) {
        self.raw.add_action_bar_layer(layer);
    }

    pub(crate) fn set_scroll_layer_click_config(&mut self, layer: *mut sys::ScrollLayer) {
        self.raw.set_scroll_layer_click_config(layer);
    }

    pub(crate) fn set_appear_effect(&mut self, callback: EffectCallback) {
        self.edit_context(|user_data| {
            user_data.appear_effect.set_callback(callback);
        });
    }

    pub(crate) fn set_load_effect(&mut self, callback: EffectCallback) {
        self.edit_context(|user_data| {
            user_data.load_effect.set_callback(callback);
        });
    }
}
