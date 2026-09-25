use core::{ffi::c_void, pin::Pin, ptr::NonNull};

use alloc::{boxed::Box, rc::Rc};

use crate::{
    ClickConfigBuilder, ClickRecognizer, ContentIndicator, GPoint, GRect, GSize, Layer, Window,
    handle::{Handle, WeakObject, new_handle},
    input::{
        context::{InputContext, InputReceiver},
        handlers::global_click_config_handler,
    },
    layer::{ChildLayer, LayerInner},
    sys,
    window::WeakWindow,
};

struct ScrollLayerRaw {
    raw: NonNull<sys::ScrollLayer>,
}

impl From<NonNull<sys::ScrollLayer>> for ScrollLayerRaw {
    fn from(value: NonNull<sys::ScrollLayer>) -> Self {
        Self { raw: value }
    }
}

impl ScrollLayerRaw {
    const fn as_ptr(&self) -> *const sys::ScrollLayer {
        self.raw.as_ptr()
    }

    const fn as_ptr_mut(&mut self) -> *mut sys::ScrollLayer {
        self.raw.as_ptr()
    }
}

impl Drop for ScrollLayerRaw {
    fn drop(&mut self) {
        unsafe { sys::scroll_layer_destroy(self.raw.as_ptr()) }
    }
}

struct ScrollLayerInner {
    content_indicator: Option<ContentIndicator>,
    shadow_hidden: bool,
    base_layer: Layer,
    raw: ScrollLayerRaw,
    pub(crate) input_context: Pin<Box<InputContext>>,
    attached_window: Option<WeakWindow>,
}

impl ScrollLayerInner {
    const fn as_ptr(&self) -> *const sys::ScrollLayer {
        self.raw.as_ptr()
    }

    const fn as_ptr_mut(&mut self) -> *mut sys::ScrollLayer {
        self.raw.as_ptr_mut()
    }

    fn get_or_create_content_indicator(&mut self) -> Option<&mut ContentIndicator> {
        if self.content_indicator.is_none() {
            self.content_indicator = ContentIndicator::from_ptr(
                unsafe { sys::scroll_layer_get_content_indicator(self.as_ptr_mut()) },
                false,
            );
        }
        self.content_indicator.as_mut()
    }
}

/// A layer that extends beyond the screen, and can be scrolled up and down by the user.
#[derive(Clone)]
pub struct ScrollLayer {
    handle: Handle<ScrollLayerInner>,
}

impl ChildLayer for ScrollLayer {
    fn remove_from_parent(&self) {
        self.handle.borrow_mut().base_layer.remove_from_parent();
        let mut inner = self.handle.borrow_mut();
        if let Some(mut window) = inner.attached_window.take().and_then(|e| e.upgrade()) {
            window.remove_input_receiver(self);
        }
    }

    fn id(&self) -> usize {
        self.handle.borrow().base_layer.id()
    }

    fn ptr_to_child_with(&mut self) -> *mut sys::Layer {
        self.handle.borrow_mut().base_layer.ptr_to_child_with()
    }

    fn record_new_parent(&self, parent: &Layer) {
        self.handle
            .borrow_mut()
            .base_layer
            .record_new_parent(parent);
    }
}

impl ScrollLayer {
    /// Creates a new layer with the given bounds.
    pub fn new(r: GRect) -> Option<Self> {
        unsafe {
            let raw = NonNull::new(sys::scroll_layer_create(r))?;

            let base = LayerInner::from_ptr(sys::scroll_layer_get_layer(raw.as_ptr()), false);
            let Some(base_layer) = base else {
                sys::scroll_layer_destroy(raw.as_ptr());
                return None;
            };

            let mut input_context = Box::new(InputContext::default());
            sys::scroll_layer_set_context(
                raw.as_ptr(),
                input_context.as_mut() as *mut InputContext as *mut c_void,
            );

            sys::scroll_layer_set_callbacks(
                raw.as_ptr(),
                sys::ScrollLayerCallbacks {
                    click_config_provider: Some(global_click_config_handler),
                    content_offset_changed_handler: None,
                },
            );

            Some(Self {
                handle: new_handle(ScrollLayerInner {
                    content_indicator: None,
                    shadow_hidden: false,
                    raw: raw.into(),
                    base_layer: Layer {
                        handle: new_handle(base_layer),
                    },
                    input_context: Box::into_pin(input_context),
                    attached_window: None,
                }),
            })
        }
    }

    fn inner_mut<T>(&mut self, f: impl FnOnce(&mut ScrollLayerInner) -> T) -> T {
        let mut inner = self.handle.borrow_mut();
        f(&mut inner)
    }

    /// Returns the frame (bounding box within parent) of the layer.
    pub fn get_frame(&self) -> GRect {
        self.handle.borrow().base_layer.get_frame()
    }

    /// Sets the frame (bounding box within parent) of the layer.
    pub fn set_frame(&mut self, frame: GRect) {
        unsafe { sys::scroll_layer_set_frame(self.as_ptr_mut(), frame) };
    }

    fn as_ptr(&self) -> *const sys::ScrollLayer {
        self.handle.borrow().as_ptr()
    }

    fn as_ptr_mut(&self) -> *mut sys::ScrollLayer {
        self.handle.borrow_mut().as_ptr_mut()
    }

    /// Adds a child layer to this scroll layer.
    pub fn add_child<T>(&mut self, child: &mut T)
    where
        T: Clone + ChildLayer + 'static,
    {
        self.inner_mut(|inner| {
            child.record_new_parent(&inner.base_layer);
            inner.base_layer.handle.borrow_mut().retain_child(child);
            unsafe {
                sys::scroll_layer_add_child(inner.as_ptr_mut(), child.ptr_to_child_with());
            }
        });
    }

    /// Returns the content size of this scroll layer, which is intended to be larger than the screen size.
    pub fn get_content_size(&self) -> GSize {
        unsafe { sys::scroll_layer_get_content_size(self.as_ptr()) }
    }

    /// Sets the content size of this scroll layer, which is intended to be larger than the screen size.
    pub fn set_content_size(&mut self, size: GSize) {
        unsafe { sys::scroll_layer_set_content_size(self.as_ptr_mut(), size) };
    }

    /// Modifies the click configuration of the window such that the up and down buttons correctly scroll this layer.
    /// You need to call this function before using the layer in order to get the expected interactive behavior.
    pub fn set_click_config_onto_window(&mut self, window: &mut Window) {
        let extra = self.clone();
        self.inner_mut(|f| {
            f.attached_window = Some(window.downgrade());
            let mut window_inner = window.handle.borrow_mut();
            window_inner.set_scroll_layer_click_config(f.as_ptr_mut());
            window_inner.retain_input_receiver(extra);
        })
    }

    /// Returns the visibility of the scroll layer shadow.
    pub fn is_shadow_hidden(&mut self) -> bool {
        unsafe { sys::scroll_layer_get_shadow_hidden(self.handle.borrow().raw.as_ptr()) }
    }

    /// Hides/shows the scroll layer shadow.
    pub fn set_shadow_hidden(&mut self, hidden: bool) {
        self.inner_mut(|inner| {
            unsafe { sys::scroll_layer_set_shadow_hidden(inner.as_ptr_mut(), hidden) };
            inner.shadow_hidden = hidden;
        })
    }

    /// Returns whether paging is enabled, i.e. a single scroll action (pressing a button) scrolls the layer by an entire screen.
    /// Disabled by default.
    pub fn get_paging_enabled(&self) -> bool {
        unsafe { sys::scroll_layer_get_paging(self.as_ptr_mut()) }
    }

    /// Sets whether paging is enabled, i.e. a single scroll action (pressing a button) scrolls the layer by an entire screen.
    /// Disabled by default.
    pub fn set_paging_enabled(&mut self, enabled: bool) {
        self.inner_mut(|inner| {
            unsafe { sys::scroll_layer_set_paging(inner.as_ptr_mut(), enabled) };
            if !enabled {
                unsafe {
                    sys::scroll_layer_set_shadow_hidden(inner.as_ptr_mut(), inner.shadow_hidden)
                };
            }
        })
    }

    /// Returns the delta by which the content is currently offset due to scrolling.
    pub fn get_content_offset(&self) -> GPoint {
        unsafe { sys::scroll_layer_get_content_offset(self.as_ptr_mut()) }
    }

    pub(crate) fn _set_content_offset(&mut self, point: GPoint, animated: bool) {
        unsafe { sys::scroll_layer_set_content_offset(self.as_ptr_mut(), point, animated) };
    }

    /// Sets the content offset, and plays the scrolling animation to visually move the contents to this position.
    pub fn set_content_offset(&mut self, point: GPoint) {
        self._set_content_offset(point, true);
    }

    /// Sets the content offset and bypasses the scrolling animation.
    pub fn set_content_offset_immediate(&mut self, point: GPoint) {
        self._set_content_offset(point, false);
    }

    /// Sets the click configuration callback for this layer.
    /// See [`ClickConfigBuilder`] for details.
    pub fn set_click_provider(&mut self, builder: impl Fn(&mut ClickConfigBuilder) + 'static) {
        self.inner_mut(|inner| {
            inner.input_context.configure_click = Some(Box::new(builder));
        });
    }

    /// Calls the default up click handler.
    /// This can be used from custom up click handlers to also trigger the default behavior.
    pub fn up_click_handler(&mut self, click: &ClickRecognizer) {
        unsafe {
            sys::scroll_layer_scroll_up_click_handler(click.raw, self.as_ptr_mut() as *mut c_void)
        };
    }

    /// Calls the default down click handler.
    /// This can be used from custom down click handlers to also trigger the default behavior.
    pub fn down_click_handler(&mut self, click: &ClickRecognizer) {
        unsafe {
            sys::scroll_layer_scroll_down_click_handler(
                click.raw,
                self.as_ptr_mut() as *mut c_void,
            );
        }
    }

    /// Remove all child layers of the scroll layer.
    pub fn remove_child_layers(&mut self) {
        self.handle.borrow_mut().base_layer.remove_child_layers();
    }

    /// Modify the [`ContentIndicator`] for this layer.
    /// This takes a function which is called with a mutable reference to the content indicator.
    pub fn with_content_indicator(&mut self, f: impl FnOnce(&mut ContentIndicator)) {
        self.inner_mut(|inner| {
            if let Some(indicator) = inner.get_or_create_content_indicator() {
                f(indicator);
            }
        })
    }

    /// Downgrade to a weak handle.
    #[allow(private_interfaces)]
    pub fn downgrade(&self) -> WeakScrollLayer {
        WeakScrollLayer::from(Rc::downgrade(&self.handle))
    }

    /// Returns whether this layer is hidden.
    pub fn is_hidden(&self) -> bool {
        self.handle.borrow().base_layer.is_hidden()
    }

    /// Hides/unhides the layer.
    pub fn set_hidden(&mut self, hidden: bool) {
        self.handle.borrow_mut().base_layer.set_hidden(hidden)
    }
}

impl InputReceiver for ScrollLayer {
    fn get_id(&self) -> usize {
        self.handle.as_ptr() as usize
    }
}

impl From<Handle<ScrollLayerInner>> for ScrollLayer {
    fn from(handle: Handle<ScrollLayerInner>) -> Self {
        Self { handle }
    }
}

/// Weak handle to a [`ScrollLayer`].
#[allow(private_interfaces)]
pub type WeakScrollLayer = WeakObject<ScrollLayerInner, ScrollLayer>;
