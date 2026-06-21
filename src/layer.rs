use core::{cell::RefCell, ptr::NonNull};

use alloc::{
    boxed::Box,
    rc::{Rc, Weak},
    vec::Vec,
};

use crate::{
    color::GCOLOR_BLACK,
    context::GContext,
    handle::Handle,
    log::log_c_str,
    sys::{self, GRect},
};

pub trait ChildLayer {
    fn is_same(&self, other: &Layer) -> bool;
    fn set_parent(&mut self, other: &mut Layer);
    fn remove_from_parent(&self);
}

pub struct LayerInner {
    pub(crate) raw: NonNull<sys::Layer>,
    parent: Option<Weak<RefCell<LayerInner>>>,
    children: Vec<Box<dyn ChildLayer>>,
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
            owned,
        })
    }
}

#[derive(Clone)]
pub struct Layer {
    pub(crate) handle: Handle<LayerInner>,
}

impl ChildLayer for Layer {
    // fn base(&self) -> RefMut<LayerInner> {
    //     self.handle.borrow_mut()
    // }

    fn remove_from_parent(&self) {
        unsafe { sys::layer_remove_from_parent(self.as_ptr()) };

        let self_mut = self.handle.borrow_mut();

        let Some(parent_weak) = &self_mut.parent else {
            return;
        };

        let Some(parent_rc) = parent_weak.upgrade() else {
            return;
        };

        let mut parent_mut = parent_rc.borrow_mut();
        let Some(child_index) = parent_mut.children.iter().position(|e| e.is_same(self)) else {
            return;
        };

        parent_mut.children.swap_remove(child_index);
    }

    fn is_same(&self, other: &Layer) -> bool {
        self.handle.borrow().raw == other.handle.borrow().raw
    }

    fn set_parent(&mut self, parent: &mut Layer) {
        unsafe { sys::layer_add_child(parent.as_ptr(), self.as_ptr()) };
    }
}

#[unsafe(no_mangle)]
extern "C" fn global_layer_update_handler(_layer: *mut sys::Layer, ctx: *mut sys::GContext) {
    log_c_str(c"global_layer_update_handler");
    let Ok(mut ctx) = GContext::from_raw(ctx) else {
        return;
    };

    ctx.set_fill_color(GCOLOR_BLACK);
    ctx.fill_rect(GRect::new(50, 50, 250, 250));
}

pub struct LayerCreateFailed;

impl Layer {
    pub fn new(r: GRect) -> Result<Self, LayerCreateFailed> {
        unsafe {
            let layer = sys::layer_create(r);
            let Some(handle) = LayerInner::from_ptr(layer, true) else {
                return Err(LayerCreateFailed);
            };
            Ok(Self {
                handle: Rc::new(RefCell::new(handle)),
            })
        }
    }

    pub fn add_child<T>(&mut self, child: &mut T)
    where
        T: Clone + ChildLayer + 'static,
    {
        child.remove_from_parent();
        {
            let mut parent_mut = self.handle.borrow_mut();
            parent_mut.children.push(Box::new(child.clone()));
        }
        child.set_parent(self);
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
    unsafe fn as_ptr(&self) -> *mut sys::Layer {
        self.handle.borrow_mut().raw.as_ptr()
    }
}
