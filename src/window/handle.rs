use alloc::rc::Rc;

use crate::{
    handle::{Handle, WeakHandle, new_handle},
    layer::ChildLayer,
    sys::{self},
    window::inner::WindowInner,
};

pub struct Window {
    pub(crate) handle: Handle<WindowInner>,
}

impl Window {
    pub fn new() -> Option<Self> {
        Some(Self {
            handle: new_handle(WindowInner::new()?),
        })
    }

    pub fn set_background_color(&mut self, color: sys::GColor) {
        self.handle.borrow_mut().set_background_color(color);
    }

    pub fn add_child<T>(&mut self, child: &mut T)
    where
        T: Clone + ChildLayer + 'static,
    {
        self.handle.borrow_mut().add_child(child);
    }

    pub fn retain(&self) -> Window {
        Self {
            handle: self.handle.clone(),
        }
    }

    pub fn downgrade(&self) -> WeakWindow {
        WeakWindow::from(self)
    }

    pub(crate) fn is_equal(&self, other: *const sys::Window) -> bool {
        self.handle.borrow().is_equal(other)
    }

    pub fn set_load_handler(&mut self, callback: impl FnMut() + 'static) {
        self.handle.borrow_mut().set_load_handler(callback);
    }

    pub fn clear_load_handler(&mut self) {
        self.handle.borrow_mut().clear_load_handler();
    }

    pub fn set_unload_handler(&mut self, callback: impl FnMut() + 'static) {
        self.handle.borrow_mut().set_unload_handler(callback);
    }

    pub fn clear_unload_handler(&mut self) {
        self.handle.borrow_mut().clear_unload_handler();
    }

    pub fn set_appear_handler(&mut self, callback: impl FnMut() + 'static) {
        self.handle.borrow_mut().set_appear_handler(callback);
    }

    pub fn clear_appear_handler(&mut self) {
        self.handle.borrow_mut().clear_appear_handler();
    }

    pub fn set_disappear_handler(&mut self, callback: impl FnMut() + 'static) {
        self.handle.borrow_mut().set_disappear_handler(callback);
    }

    pub fn clear_disappear_handler(&mut self) {
        self.handle.borrow_mut().clear_disappear_handler();
    }
}

#[derive(Clone)]
pub struct WeakWindow {
    handle: WeakHandle<WindowInner>,
}

impl WeakWindow {
    pub fn from(window: &Window) -> Self {
        Self {
            handle: Rc::downgrade(&window.handle),
        }
    }
    pub fn upgrade(&mut self) -> Option<Window> {
        Some(Window {
            handle: self.handle.upgrade()?,
        })
    }
}
