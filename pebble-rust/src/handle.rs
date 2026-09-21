use core::cell::RefCell;

use alloc::rc::{Rc, Weak};

pub type Handle<T> = Rc<RefCell<T>>;
pub type WeakHandle<T> = Weak<RefCell<T>>;

pub(crate) fn new_handle<T>(s: T) -> Handle<T> {
    Rc::new(RefCell::new(s))
}
