use core::ptr::NonNull;

use alloc::{boxed::Box, rc::Rc, vec::Vec};

use crate::{
    GContext, GRect,
    handle::{Handle, WeakHandle, new_handle},
    log_c_str, sys,
};

pub struct LayerContext {
    back_to_self: WeakHandle<LayerInner>,
}

pub trait ChildLayer {
    fn id(&self) -> usize;
    fn ptr_to_child_with(&mut self) -> *mut sys::Layer;
    fn record_new_parent(&self, parent: &Layer);
    fn remove_from_parent(&self);
}

pub struct LayerInner {
    pub(crate) raw: NonNull<sys::Layer>,
    parent: Option<WeakHandle<LayerInner>>,
    children: Vec<Box<dyn ChildLayer>>,
    render: Option<Box<dyn Fn(Layer, GContext)>>,
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
            render: None,
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
    pub fn new(frame: GRect) -> Option<Self> {
        unsafe {
            // Same as sys::layer_create
            let layer = sys::layer_create_with_data(frame, size_of::<Option<LayerContext>>());
            let handle = LayerInner::from_ptr(layer, true)?;
            let handle = new_handle(handle);
            let context = (sys::layer_get_data(layer) as *mut Option<LayerContext>).as_mut()?;
            *context = Some(LayerContext {
                back_to_self: Rc::downgrade(&handle),
            });
            Some(Self { handle })
        }
    }

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

    pub fn mark_dirty(&mut self) {
        unsafe { sys::layer_mark_dirty(self.as_ptr()) };
    }

    pub fn set_bounds(&mut self, bounds: GRect) {
        unsafe { sys::layer_set_bounds(self.as_ptr(), bounds) };
    }

    pub fn set_update_proc(
        &mut self,
        proc: unsafe extern "C" fn(layer: *mut sys::Layer, ctx: *mut sys::GContext),
    ) {
        unsafe { sys::layer_set_update_proc(self.as_ptr(), Some(proc)) };
    }

    pub fn set_update(&mut self, callback: Box<dyn Fn(Layer, GContext)>) {
        self.handle.borrow_mut().render = Some(callback);
        self.set_update_proc(global_layer_update_handler);
    }

    unsafe fn as_ptr(&self) -> *mut sys::Layer {
        self.handle.borrow_mut().raw.as_ptr()
    }

    pub fn get_bounds(&self) -> GRect {
        unsafe { sys::layer_get_bounds(self.as_ptr()) }
    }

    pub fn get_frame(&self) -> GRect {
        unsafe { sys::layer_get_frame(self.as_ptr()) }
    }

    pub fn set_frame(&mut self, frame: GRect) {
        unsafe { sys::layer_set_frame(self.as_ptr(), frame) }
    }

    pub fn remove(&mut self) {
        ChildLayer::remove_from_parent(self);
    }
}

extern "C" fn global_layer_update_handler(layer: *mut sys::Layer, ctx: *mut sys::GContext) {
    let ptr = unsafe { (sys::layer_get_data(layer) as *mut LayerContext).as_ref() };
    let Some(inner_ref) = ptr.as_ref() else {
        log_c_str(c"Unexpected: Layer data is null");
        return;
    };
    let Some(ctx) = GContext::from_raw(ctx) else {
        log_c_str(c"Unexpected: Layer context is null");
        return;
    };
    let Some(inner_ref) = inner_ref.back_to_self.upgrade() else {
        log_c_str(c"Unexpected: Layer inner is destroyed");
        return;
    };
    let layer = inner_ref.borrow();
    let Some(callback) = &layer.render else {
        log_c_str(c"Unexpected: Layer has no render function");
        return;
    };

    callback(
        Layer {
            handle: inner_ref.clone(),
        },
        ctx,
    );
}
