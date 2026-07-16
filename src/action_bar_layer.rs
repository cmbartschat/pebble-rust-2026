use core::{cell::RefCell, ffi::c_void, pin::Pin, ptr::NonNull};

use alloc::{boxed::Box, rc::Rc};

use crate::{
    ClickConfigBuilder, GColor, Window,
    bitmap::Bitmap,
    input::{
        context::{InputContext, InputReceiver},
        handlers::global_click_config_handler,
    },
    sys,
    window::WeakWindow,
};

struct ActionBarLayerInner {
    raw: NonNull<sys::ActionBarLayer>,
    pub(crate) input_context: Pin<Box<InputContext>>,
    bitmap_up: Option<Bitmap>,
    bitmap_select: Option<Bitmap>,
    bitmap_down: Option<Bitmap>,
    attached_window: Option<WeakWindow>,
}

impl Drop for ActionBarLayerInner {
    fn drop(&mut self) {
        unsafe { sys::action_bar_layer_destroy(self.raw.as_ptr()) };
    }
}

#[derive(Clone)]
pub struct ActionBarLayer {
    handle: Rc<RefCell<ActionBarLayerInner>>,
}

impl ActionBarLayer {
    pub fn new() -> Option<Self> {
        unsafe {
            let raw = NonNull::new(sys::action_bar_layer_create())?;
            sys::action_bar_layer_set_click_config_provider(
                raw.as_ptr(),
                Some(global_click_config_handler),
            );

            let mut input_context = Box::new(InputContext::default());
            sys::action_bar_layer_set_context(
                raw.as_ptr(),
                input_context.as_mut() as *mut InputContext as *mut c_void,
            );

            let inner = ActionBarLayerInner {
                raw,
                input_context: Box::into_pin(input_context),
                bitmap_up: None,
                bitmap_select: None,
                bitmap_down: None,
                attached_window: None,
            };

            Some(Self {
                handle: Rc::new(RefCell::new(inner)),
            })
        }
    }

    pub fn add_to_window(&mut self, window: &mut Window) {
        let mut window_inner = window.handle.borrow_mut();
        self.inner_mut(|inner| {
            window_inner.add_action_bar_layer(inner.raw.as_ptr());
            inner.attached_window = Some(window.downgrade());
        });
        window_inner.retain_input_receiver(self.clone());
    }

    pub fn remove(&mut self) {
        let extra = self.clone();
        self.inner_mut(|inner| unsafe {
            sys::action_bar_layer_remove_from_window(inner.raw.as_ptr());
            if let Some(mut window) = inner.attached_window.take().and_then(|e| e.upgrade()) {
                window.remove_input_receiver(&extra);
            }
        });
    }

    fn inner_mut(&mut self, f: impl FnOnce(&mut ActionBarLayerInner)) {
        let mut inner = self.handle.borrow_mut();
        f(&mut inner);
    }

    pub fn set_background_color(&mut self, color: GColor) {
        self.inner_mut(|inner| {
            unsafe { sys::action_bar_layer_set_background_color(inner.raw.as_ptr(), color) };
        });
    }

    pub fn set_icon(&mut self, location: ActionButton, icon: Bitmap) {
        self.inner_mut(|inner| {
            unsafe {
                let (target, location) = match location {
                    ActionButton::Up => (&mut inner.bitmap_up, sys::ButtonId_BUTTON_ID_UP),
                    ActionButton::Select => {
                        (&mut inner.bitmap_select, sys::ButtonId_BUTTON_ID_SELECT)
                    }
                    ActionButton::Down => (&mut inner.bitmap_down, sys::ButtonId_BUTTON_ID_DOWN),
                };

                sys::action_bar_layer_set_icon_animated(
                    inner.raw.as_ptr(),
                    location,
                    icon.handle.borrow().raw.as_ptr(),
                    true,
                );

                *target = Some(icon);
            };
        });
    }

    pub fn set_click_provider(&mut self, builder: impl Fn(&mut ClickConfigBuilder) + 'static) {
        self.inner_mut(|inner| unsafe {
            inner.input_context.configure_click = Some(Box::new(builder));
            sys::action_bar_layer_set_click_config_provider(
                inner.raw.as_ptr(),
                Some(global_click_config_handler),
            );
        });
    }
}

impl InputReceiver for ActionBarLayer {
    fn get_id(&self) -> usize {
        self.handle.as_ptr() as usize
    }
}

pub enum ActionButton {
    Up,
    Select,
    Down,
}
