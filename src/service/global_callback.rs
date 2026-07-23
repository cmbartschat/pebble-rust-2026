use core::{cell::RefCell, ffi::c_void, ptr::addr_of};

use alloc::boxed::Box;

use crate::{Mutex, MutexToken};

type Callback<P, T> = Box<dyn FnMut(P) -> T>;

struct GlobalCallbackInner<P, T> {
    callback: Option<Callback<P, T>>,
    configured: bool,
}

impl<P, T> GlobalCallbackInner<P, T> {
    pub const fn new() -> Self {
        Self {
            callback: None,
            configured: false,
        }
    }
    pub fn set(&mut self, callback: Option<Callback<P, T>>) {
        self.callback = callback;
        self.configured = true;
    }

    pub fn extract(&mut self) -> Option<Callback<P, T>> {
        self.configured = false;
        self.callback.take()
    }

    pub fn restore(&mut self, callback: Callback<P, T>) {
        if self.configured {
            self.configured = false;
            return;
        }
        self.callback = Some(callback);
    }
}

pub struct GlobalCallback<P, T> {
    inner: Mutex<RefCell<GlobalCallbackInner<P, T>>>,
}

impl<P, T> GlobalCallback<P, T> {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(RefCell::new(GlobalCallbackInner::new())),
        }
    }

    pub fn set(&self, callback: Callback<P, T>) {
        MutexToken::with(|t| {
            self.inner.borrow_mut(t).set(Some(callback));
        });
    }

    pub fn clear(&self) {
        MutexToken::with(|t| {
            self.inner.borrow_mut(t).set(None);
        });
    }

    pub unsafe fn as_void(&self) -> *mut c_void {
        addr_of!(self.inner) as *const c_void as *mut c_void
    }

    fn dispatch_on(mutex: &Mutex<RefCell<GlobalCallbackInner<P, T>>>, data: P) -> Option<T> {
        MutexToken::with(|t| {
            let mut callback = {
                match mutex.borrow_mut(t).extract() {
                    Some(e) => e,
                    None => return None,
                }
            };

            let res = callback(data);

            mutex.borrow_mut(t).restore(callback);

            Some(res)
        })
    }

    pub unsafe fn dispatch_callback(context: *mut c_void, data: P) -> Option<T> {
        let mutex =
            (unsafe { (context as *mut Mutex<RefCell<GlobalCallbackInner<P, T>>>).as_ref() })?;

        Self::dispatch_on(mutex, data)
    }

    pub(crate) fn dispatch(&self, data: P) -> Option<T> {
        Self::dispatch_on(&self.inner, data)
    }
}
