use core::{mem::swap, ptr::NonNull};

use alloc::{boxed::Box, rc::Rc, vec::Vec};

use crate::{
    GContext, GPoint, GRect,
    handle::{Handle, WeakHandle, new_handle},
    log_c_str,
    service::GlobalCallbackInner,
    sys,
};

pub struct LayerContext {
    back_to_self: WeakHandle<LayerInner>,
}

/// Common functionality of child layers.
// This trait is partially public, so we hide our implementation details with #[doc(hidden)]
pub trait ChildLayer {
    /// Returns the layer ID.
    fn id(&self) -> usize;
    /// Returns the layer pointer.
    #[doc(hidden)]
    fn ptr_to_child_with(&mut self) -> *mut sys::Layer;
    /// Changes the parent for the layer.
    #[doc(hidden)]
    fn record_new_parent(&self, parent: &Layer);
    /// Removes this layer from the parent.
    fn remove_from_parent(&self);
}

pub struct LayerInner {
    pub(crate) raw: NonNull<sys::Layer>,
    parent: Option<WeakHandle<LayerInner>>,
    children: Vec<Box<dyn ChildLayer>>,
    render: GlobalCallbackInner<Box<dyn FnMut(Layer, GContext)>>,
    owned: bool,
}

impl Drop for LayerInner {
    fn drop(&mut self) {
        self.children.iter().for_each(|f| f.remove_from_parent());
        if self.owned {
            unsafe { sys::layer_destroy(self.raw.as_ptr()) };
        }
    }
}

impl LayerInner {
    pub(crate) unsafe fn from_ptr(ptr: *mut sys::Layer, owned: bool) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(ptr)?,
            parent: None,
            children: Vec::new(),
            render: GlobalCallbackInner::new(),
            owned,
        })
    }

    pub(crate) fn release_child<T>(&mut self, child: &T)
    where
        T: Clone + ChildLayer + 'static,
    {
        let Some(child_index) = self.children.iter().position(|e| e.id() == child.id()) else {
            return;
        };
        self.children.swap_remove(child_index);
    }

    pub(crate) fn retain_child<T>(&mut self, child: &mut T)
    where
        T: Clone + ChildLayer + 'static,
    {
        if self.children.iter().any(|e| e.id() == child.id()) {
            return;
        };
        self.children.push(Box::new(child.clone()));
    }
}

/// A graphics layer within a window.
/// This is the most generic layer, suitable for manual drawing with [`GContext`].
/// There are several more specialized layer types, like [`crate::BitmapLayer`] and [`crate::TextLayer`].
#[derive(Clone)]
pub struct Layer {
    pub(crate) handle: Handle<LayerInner>,
}

impl ChildLayer for Layer {
    fn remove_from_parent(&self) {
        unsafe { sys::layer_remove_from_parent(self.as_ptr()) };

        let mut inner = self.handle.borrow_mut();

        if let Some(Some(parent_rc)) = inner.parent.take().map(|f| f.upgrade()) {
            // NOTE(christoph): If remove_from_parent is being called from the Layer's Drop impl,
            // missing the reference would be expected. Otherwise an invariant is being violated.
            parent_rc.borrow_mut().release_child(self);
        };
    }

    fn id(&self) -> usize {
        self.handle.borrow().raw.as_ptr() as usize
    }

    fn ptr_to_child_with(&mut self) -> *mut sys::Layer {
        self.handle.borrow_mut().raw.as_ptr()
    }

    fn record_new_parent(&self, parent: &Layer) {
        self.remove_from_parent();
        self.handle.borrow_mut().parent = Some(Rc::downgrade(&parent.handle))
    }
}

impl Layer {
    /// Create a new layer with the given bounds.
    pub fn new(frame: GRect) -> Option<Self> {
        unsafe {
            // Same as sys::layer_create
            // SAFETY(?): There do not seem to be any requirements on the arguments to this function.
            let layer = sys::layer_create_with_data(frame, size_of::<Option<LayerContext>>());
            // SAFETY: This function returns None if the pointer was null.
            let handle = LayerInner::from_ptr(layer, true)?;
            let handle = new_handle(handle);
            // SAFETY: LayerInner::from_ptr ensured that the layer pointer is not null.
            let context = (sys::layer_get_data(layer) as *mut Option<LayerContext>).as_mut()?;
            // SAFETY: The context pointer is valid, since `layer` is valid.
            // NOTE: Since the C API does not guarantee alignment of the context pointer, we are required to use write_unaligned here.
            core::ptr::write_unaligned(
                context,
                Some(LayerContext {
                    back_to_self: Rc::downgrade(&handle),
                }),
            );
            Some(Self { handle })
        }
    }

    /// Add a child layer.
    pub fn add_child<T>(&mut self, child: &mut T)
    where
        T: Clone + ChildLayer + 'static,
    {
        child.record_new_parent(self);
        {
            let mut inner = self.handle.borrow_mut();
            inner.retain_child(child);
            unsafe { sys::layer_add_child(inner.raw.as_ptr(), child.ptr_to_child_with()) };
        }
    }

    /// Mark the layer as dirty, forcing a redraw.
    /// This is the preferred way to manually update a layer.
    pub fn mark_dirty(&mut self) {
        unsafe { sys::layer_mark_dirty(self.as_ptr()) };
    }

    /// Set the boundaries of the layer (in parent coordinates).
    pub fn set_bounds(&mut self, bounds: GRect) {
        unsafe { sys::layer_set_bounds(self.as_ptr(), bounds) };
    }

    fn _set_update_handler(
        &mut self,
        proc: Option<unsafe extern "C" fn(layer: *mut sys::Layer, ctx: *mut sys::GContext)>,
        callback: Option<Box<dyn FnMut(Layer, GContext)>>,
    ) {
        let mut inner = self.handle.borrow_mut();
        unsafe { sys::layer_set_update_proc(inner.raw.as_ptr(), proc) };
        unsafe { sys::layer_mark_dirty(inner.raw.as_ptr()) };
        inner.render.set(callback);
    }

    /// Set a raw update handler.
    /// In most cases you should use [`Self::set_update_handler`] instead.
    pub fn set_raw_update_handler(
        &mut self,
        proc: unsafe extern "C" fn(layer: *mut sys::Layer, ctx: *mut sys::GContext),
    ) {
        self._set_update_handler(Some(proc), None);
    }

    /// Set the handler for layer updates.
    /// The callback receives this layer, as well as a graphics context that can be drawn to to set the visual contents of the layer.
    pub fn set_update_handler(&mut self, callback: Box<dyn FnMut(Layer, GContext)>) {
        self._set_update_handler(Some(global_layer_update_handler), Some(callback));
    }

    /// Remove the layer update handler.
    pub fn clear_update_handler(&mut self) {
        self._set_update_handler(None, None);
    }

    unsafe fn as_ptr(&self) -> *mut sys::Layer {
        self.handle.borrow_mut().raw.as_ptr()
    }

    /// Returns the boundaries of the layer.
    pub fn get_bounds(&self) -> GRect {
        unsafe { sys::layer_get_bounds(self.as_ptr()) }
    }

    /// Returns the frame (bounding box within parent) of the layer.
    pub fn get_frame(&self) -> GRect {
        unsafe { sys::layer_get_frame(self.as_ptr()) }
    }

    /// Sets the frame (bounding box within parent) of the layer.
    pub fn set_frame(&mut self, frame: GRect) {
        unsafe { sys::layer_set_frame(self.as_ptr(), frame) }
    }

    /// Returns whether clipping is enabled for this layer.
    pub fn is_clipping_enabled(&self) -> bool {
        unsafe { sys::layer_get_clips(self.as_ptr()) }
    }

    /// Enables/disables clipping.
    pub fn set_clipping_enabled(&mut self, clips: bool) {
        unsafe { sys::layer_set_clips(self.as_ptr(), clips) }
    }

    /// Returns whether the layer is hidden or not.
    pub fn is_hidden(&self) -> bool {
        unsafe { sys::layer_get_hidden(self.as_ptr()) }
    }

    /// Hides/shows the layer.
    pub fn set_hidden(&mut self, hidden: bool) {
        unsafe { sys::layer_set_hidden(self.as_ptr(), hidden) }
    }

    /// Returns the bounds within the layer that are not obstructed by system UI.
    /// The associated overlay functionality is not available on Aplite, where this always returns the full bounds.
    pub fn get_unobstructed_bounds(&self) -> GRect {
        #[cfg(not(platform = "aplite"))]
        unsafe {
            sys::layer_get_unobstructed_bounds(self.as_ptr())
        }
        #[cfg(platform = "aplite")]
        self.get_bounds()
    }

    /// Convert a point in layer coordinates to screen coordinates.
    pub fn convert_point_to_screen(&self, point: GPoint) -> GPoint {
        unsafe { sys::layer_convert_point_to_screen(self.as_ptr(), point) }
    }

    /// Convert a rectangle in layer coordinates to screen coordinates.
    pub fn convert_rect_to_screen(&self, rect: GRect) -> GRect {
        unsafe { sys::layer_convert_rect_to_screen(self.as_ptr(), rect) }
    }

    /// Remove all children of this layer.
    pub fn remove_child_layers(&mut self) {
        let children = {
            let mut inner = self.handle.borrow_mut();
            let mut empty = Vec::new();
            swap(&mut inner.children, &mut empty);
            empty
        };

        children.iter().for_each(|f| f.remove_from_parent());
    }
}

extern "C" fn global_layer_update_handler(layer: *mut sys::Layer, ctx: *mut sys::GContext) {
    let ptr = unsafe { (sys::layer_get_data(layer) as *mut LayerContext).as_ref() };
    let Some(inner_ref) = ptr.as_ref() else {
        log_c_str(c"Unexpected: Layer data is null");
        return;
    };
    let ctx = unsafe { GContext::from_raw(ctx) };
    let Some(ctx) = ctx else {
        log_c_str(c"Unexpected: Layer context is null");
        return;
    };
    let Some(inner_ref) = inner_ref.back_to_self.upgrade() else {
        log_c_str(c"Unexpected: Layer inner is destroyed");
        return;
    };

    let callback = {
        let mut layer = inner_ref.borrow_mut();
        layer.render.extract()
    };

    match callback {
        Some(mut callback) => {
            callback(
                Layer {
                    handle: inner_ref.clone(),
                },
                ctx,
            );

            let mut layer = inner_ref.borrow_mut();
            layer.render.restore(callback);
        }
        None => {
            log_c_str(c"Unexpected: Layer has no render function");
        }
    }
}
