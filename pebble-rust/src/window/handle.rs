use alloc::rc::Rc;

use crate::{
    ClickConfigBuilder, GColor, GRect,
    effect::EffectCallback,
    handle::{Handle, WeakObject, new_handle},
    input::context::InputReceiver,
    layer::ChildLayer,
    service::{Callback, CallbackHandle},
    sys,
    window::inner::WindowInner,
};

/// The window represents the entire screen of the watch, and is the entry point for all graphics APIs.
/// An app can consist of a stack of windows.
/// See also the [Pebble documentation for windows](https://developer.repebble.com/docs/c/User_Interface/Window/).
#[derive(Clone)]
pub struct Window {
    pub(crate) handle: Handle<WindowInner>,
}

impl Window {
    /// Retrieves the current window.
    pub fn new() -> Option<Self> {
        Some(Self {
            handle: new_handle(WindowInner::new()?),
        })
    }

    /// Sets the background color of the window.
    pub fn set_background_color(&mut self, color: GColor) {
        self.handle.borrow_mut().set_background_color(color);
    }

    /// Add a child layer to the window.
    pub fn add_child<T>(&mut self, child: &mut T)
    where
        T: Clone + ChildLayer + 'static,
    {
        self.handle.borrow_mut().add_child(child);
    }

    /// Remove all child layers from the window.
    pub fn remove_child_layers(&mut self) {
        self.handle.borrow_mut().remove_child_layers();
    }

    /// Returns a weak pointer to the window, which may disappear when the window is dropped.
    pub fn downgrade(&self) -> WeakWindow {
        WeakWindow::from(Rc::downgrade(&self.handle))
    }

    /// Adds a handler for when the window is loaded, i.e. pushed to the screen while it is not loaded.
    pub fn add_load_handler(
        &mut self,
        callback: impl Into<Callback<(), ()>>,
    ) -> CallbackHandle<(), ()> {
        self.handle.borrow_mut().add_load_handler(callback)
    }

    /// Removes a load handler.
    pub fn clear_load_handler(&mut self, handle: CallbackHandle<()>) {
        self.handle.borrow_mut().remove_load_handler(handle);
    }

    /// Adds a handler for when the window is unloaded, i.e. no longer visible and uninitialized to free resources.
    pub fn add_unload_handler(&mut self, callback: impl Into<Callback<()>>) -> CallbackHandle<()> {
        self.handle.borrow_mut().add_unload_handler(callback)
    }

    /// Removes an unload handler.
    pub fn clear_unload_handler(&mut self, handle: CallbackHandle<()>) {
        self.handle.borrow_mut().remove_unload_handler(handle);
    }

    /// Adds a handler for when the window comes up on screen.
    pub fn add_appear_handler(&mut self, callback: impl Into<Callback<()>>) -> CallbackHandle<()> {
        self.handle.borrow_mut().add_appear_handler(callback)
    }

    /// Removes an appear handler.
    pub fn clear_appear_handler(&mut self, handle: CallbackHandle<()>) {
        self.handle.borrow_mut().remove_appear_handler(handle);
    }

    /// Adds a handler for when the window disappears from screen.
    pub fn add_disappear_handler(
        &mut self,
        callback: impl Into<Callback<()>>,
    ) -> CallbackHandle<()> {
        self.handle.borrow_mut().add_disappear_handler(callback)
    }

    /// Removes a disappear handler.
    pub fn clear_disappear_handler(&mut self, handle: CallbackHandle<()>) {
        self.handle.borrow_mut().remove_disappear_handler(handle);
    }

    /// Sets the click handlers.
    ///
    /// The callback function receives a mutable reference to a [`ClickConfigBuilder`].
    /// You can set handlers for specific click events on that builder.
    pub fn set_click_provider(&mut self, builder: impl Fn(&mut ClickConfigBuilder) + 'static) {
        self.handle.borrow_mut().set_click_provider(builder);
    }

    /// Returns the window’s bounds, which are the screen bounds.
    pub fn get_bounds(&self) -> GRect {
        self.handle.borrow().get_bounds()
    }

    pub(crate) fn retain_input_receiver(&mut self, receiver: impl InputReceiver + 'static) {
        self.handle.borrow_mut().retain_input_receiver(receiver);
    }

    pub(crate) fn remove_input_receiver(&mut self, receiver: &dyn InputReceiver) {
        self.handle.borrow_mut().remove_input_receiver(receiver);
    }

    /// Set the effect for the window appearing, see [`EffectCallback`].
    pub fn set_appear_effect(&mut self, callback: EffectCallback) {
        self.handle.borrow_mut().set_appear_effect(callback);
    }

    /// Set the effect for loading the window, see [`EffectCallback`].
    pub fn set_load_effect(&mut self, callback: EffectCallback) {
        self.handle.borrow_mut().set_load_effect(callback);
    }
}

impl PartialEq<*const sys::Window> for &Window {
    fn eq(&self, other: &*const sys::Window) -> bool {
        self.handle.borrow().is_equal(*other)
    }
}

impl From<Handle<WindowInner>> for Window {
    fn from(handle: Handle<WindowInner>) -> Self {
        Self { handle }
    }
}

/// A weak pointer to a window.
pub type WeakWindow = WeakObject<WindowInner, Window>;
