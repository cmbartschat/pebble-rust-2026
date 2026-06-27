use core::{cell::RefCell, ffi::c_void, pin::Pin, ptr::NonNull};

use alloc::{boxed::Box, rc::Rc};

use crate::{
    Layer, Window, bitmap::Bitmap, input::{context::InputContext, handlers::global_click_config_handler}, layer::{ChildLayer, LayerInner}, sys::{
        self, ButtonId_BUTTON_ID_DOWN, ButtonId_BUTTON_ID_SELECT, ButtonId_BUTTON_ID_UP, GColor,
    }
};

#[derive(Default)]
struct ActionButton {
    icon: Option<Bitmap>,
    // click_handler: Option<Box<dyn FnMut() + 'static>>,
}

struct ActionBarLayerInner {
    raw: NonNull<sys::ActionBarLayer>,
    base_layer: Layer,
    pub(crate) input_context: Pin<Box<InputContext>>,
    button_up: ActionButton,
    button_select: ActionButton,
    button_down: ActionButton,
}

impl Drop for ActionBarLayerInner {
    fn drop(&mut self) {
        unsafe { sys::action_bar_layer_destroy(self.raw.as_ptr()) };
        // let _ = unsafe { Box::from_raw(self.context) };
    }
}

#[derive(Clone)]
pub struct ActionBarLayer {
    handle: Rc<RefCell<ActionBarLayerInner>>,
}

impl ChildLayer for ActionBarLayer {
    fn remove_from_parent(&self) {
        self.handle.borrow_mut().base_layer.remove_from_parent();
    }

    fn is_same(&self, other: &Layer) -> bool {
        self.handle.borrow().base_layer.is_same(other)
    }

    fn set_parent(&mut self, other: &mut Layer) {
        self.handle.borrow_mut().base_layer.set_parent(other);
    }
}

impl ActionBarLayer {
    pub fn new() -> Option<Self> {
        unsafe {
            let raw = NonNull::new(sys::action_bar_layer_create())?;
            let base = LayerInner::from_ptr(sys::action_bar_layer_get_layer(raw.as_ptr()), false);
            let Some(base_layer) = base else {
                sys::action_bar_layer_destroy(raw.as_ptr());
                return None;
            };

            // sys::action_bar_layer_set_context(raw.as_ptr(), context);
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
                button_up: Default::default(),
                button_select: Default::default(),
                button_down: Default::default(),
                base_layer: Layer {
                    handle: Rc::new(RefCell::new(base_layer)),
                },
            };

            Some(Self {
                handle: Rc::new(RefCell::new(inner)),
            })
        }
    }

    pub fn add_to_window(&mut self, window: &mut Window) {
        self.inner_mut(|inner| unsafe {
            sys::action_bar_layer_add_to_window(inner.raw.as_ptr(), window.raw.as_ptr());
        });
    }

    pub fn remove(&mut self) {
        self.inner_mut(|inner| unsafe {
            sys::action_bar_layer_remove_from_window(inner.raw.as_ptr());
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

    pub fn set_icon(&mut self, location: IconLocation, icon: Bitmap) {
        self.inner_mut(|inner| {
            unsafe {
                let (target, location) = match location {
                    IconLocation::Up => (&mut inner.button_up.icon, ButtonId_BUTTON_ID_UP),
                    IconLocation::Select => {
                        (&mut inner.button_select.icon, ButtonId_BUTTON_ID_SELECT)
                    }
                    IconLocation::Down => (&mut inner.button_down.icon, ButtonId_BUTTON_ID_DOWN),
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

    pub fn configure_input(&mut self, builder: impl FnMut(&mut ClickConfigBuilder) + 'static) {
        self.inner_mut(|inner| unsafe {
            inner.input_context.configure_click = Some(Box::new(builder));
            sys::action_bar_layer_set_click_config_provider(
                inner.raw.as_ptr(),
                Some(global_click_config_handler),
            );
        });
    }
}

// extern "C" fn global_input_handler(recognizer: *mut c_void, context: *mut c_void) {
//     let context = unsafe { (context as *mut ActionBarContext).as_mut() }.unwrap();
//     log_c_str(c"global_input_handler");
// }

pub enum IconLocation {
    Up,
    Select,
    Down,
}

// pub struct ActionLayerInput {
//     click: Box<dyn Fn(&ClickRecognizer) + 'static>,
//     hold: Box<dyn Fn(&ClickRecognizer) + 'static>,
//     repeat: Box<dyn Fn(&ClickRecognizer) + 'static>,
// }

// impl Default for ActionLayerInput {
//     fn default() -> Self {
//         Self {}
//     }
// }
