use core::{cell::RefCell, ffi::c_void, pin::Pin, ptr::NonNull};

use alloc::{boxed::Box, rc::Rc};

use crate::{
    ClickConfigBuilder, GColor, Window,
    bitmap::Bitmap,
    handle::WeakHandle,
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

/// Vertical, bar-shaped control widget on the right edge of the window.
/// See the [Pebble developer documentation](https://developer.repebble.com/docs/c/User_Interface/Layers/ActionBarLayer/) for more.
/// This is not a regular `ChildLayer` and must be added to the window using [`Self::add_to_window`].
#[derive(Clone)]
pub struct ActionBarLayer {
    handle: Rc<RefCell<ActionBarLayerInner>>,
}

impl ActionBarLayer {
    /// Create a new action bar layer.
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

    /// Add the action bar layer to the window.
    /// Since this is a special layer, [`Window::add_child`] does not work.
    pub fn add_to_window(&mut self, window: &mut Window) {
        let mut window_inner = window.handle.borrow_mut();
        self.inner_mut(|inner| {
            window_inner.add_action_bar_layer(inner.raw.as_ptr());
            inner.attached_window = Some(window.downgrade());
        });
        window_inner.retain_input_receiver(self.clone());
    }

    /// Remove the layer from the window it is currently attached to.
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

    /// Set the background color of the action bar.
    pub fn set_background_color(&mut self, color: GColor) {
        self.inner_mut(|inner| {
            unsafe { sys::action_bar_layer_set_background_color(inner.raw.as_ptr(), color) };
        });
    }

    /// Set one of the icons of the action bar.
    /// `location` determines which one, each corresponding to one of the possible action buttons, see [`ActionButton`].
    pub fn set_icon(&mut self, location: ActionButton, icon: Bitmap) {
        self.inner_mut(|inner| {
            unsafe {
                sys::action_bar_layer_set_icon_animated(
                    inner.raw.as_ptr(),
                    location as u8,
                    icon.handle.borrow().raw.as_ptr(),
                    true,
                );

                let target = match location {
                    ActionButton::Up => &mut inner.bitmap_up,
                    ActionButton::Select => &mut inner.bitmap_select,
                    ActionButton::Down => &mut inner.bitmap_down,
                };

                *target = Some(icon);
            };
        });
    }

    /// Set the click provider that configures button click callbacks on this layer.
    /// See [`Window::set_click_provider`].
    pub fn set_click_provider(&mut self, builder: impl Fn(&mut ClickConfigBuilder) + 'static) {
        self.inner_mut(|inner| unsafe {
            inner.input_context.configure_click = Some(Box::new(builder));
            sys::action_bar_layer_set_click_config_provider(
                inner.raw.as_ptr(),
                Some(global_click_config_handler),
            );
        });
    }

    /// Returns a non-owning reference to this layer.
    pub fn downgrade(&self) -> WeakActionBarLayer {
        WeakActionBarLayer::from(self)
    }
}

impl InputReceiver for ActionBarLayer {
    fn get_id(&self) -> usize {
        self.handle.as_ptr() as usize
    }
}

/// The possible action buttons that can be used to trigger actions via an [`ActionBarLayer`].
#[repr(u8)]
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum ActionButton {
    /// The up button.
    Up = sys::ButtonId_BUTTON_ID_UP,
    /// The select (center) button.
    Select = sys::ButtonId_BUTTON_ID_SELECT,
    /// The down button.
    Down = sys::ButtonId_BUTTON_ID_DOWN,
}

/// A non-owning reference to an [`ActionBarLayer`].
#[derive(Clone)]
pub struct WeakActionBarLayer {
    handle: WeakHandle<ActionBarLayerInner>,
}

impl WeakActionBarLayer {
    /// Upgrade this to a strong reference.
    /// If the layer is already gone, returns None.
    pub fn upgrade(&self) -> Option<ActionBarLayer> {
        Some(ActionBarLayer {
            handle: self.handle.upgrade()?,
        })
    }
}

impl From<&ActionBarLayer> for WeakActionBarLayer {
    fn from(value: &ActionBarLayer) -> Self {
        Self {
            handle: Rc::downgrade(&value.handle),
        }
    }
}
