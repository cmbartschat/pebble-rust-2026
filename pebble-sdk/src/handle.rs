use core::{cell::RefCell, marker::PhantomData};

use alloc::rc::{Rc, Weak};

/// Reference-counted handle to an inner value.
pub type Handle<T> = Rc<RefCell<T>>;
/// Weak handle equivalent to some [`Handle`].
pub type WeakHandle<T> = Weak<RefCell<T>>;

pub(crate) fn new_handle<T>(s: T) -> Handle<T> {
    Rc::new(RefCell::new(s))
}

/// Generic weak pointer.
pub struct WeakObject<Inner, Handled> {
    handle: WeakHandle<Inner>,
    _outer: PhantomData<Handled>,
}

impl<Handled, Inner> WeakObject<Inner, Handled>
where
    Handled: From<Handle<Inner>>,
{
    /// Upgrades this function to a strong pointer.
    /// Returns none if the value is already gone.
    pub fn upgrade(&self) -> Option<Handled> {
        Some(Handled::from(self.handle.upgrade()?))
    }
}

impl<Handled, Inner> From<WeakHandle<Inner>> for WeakObject<Inner, Handled> {
    fn from(handle: WeakHandle<Inner>) -> Self {
        Self {
            handle,
            _outer: PhantomData,
        }
    }
}

impl<Handled, Inner> Clone for WeakObject<Inner, Handled> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            _outer: PhantomData,
        }
    }
}
