use core::{cell::RefCell, ffi::c_void, marker::PhantomData, ptr::addr_of};

use alloc::boxed::Box;
use intrusive_collections::{XorLinkedList, XorLinkedListLink, intrusive_adapter};

use crate::{Mutex, MutexToken, log_c_str};

/// Internal callback storage.
/// You don’t need to interact with this type directly; all [`FnMut`] implementors can be converted to it.
pub struct Callback<Args, Result = ()> {
    /// Intrusive linked list connection in the global callback list
    link: XorLinkedListLink,
    /// Actual function.
    /// Unfortunately due to intrusive_collections limitations, we can’t remove the Box here.
    function: Box<dyn FnMut(Args) -> Result>,
    /// Symbolic ID, so that handles can refer to it.
    /// Using IDs avoids overhead from Rc, RefCell, Mutex etc.,
    /// since we know that the handles cannot (and are not supposed to) mutate the callback’s inner data.
    id: usize,
}

// TODO: A lot of this would be less annoying if Rust:
// - allowed FnMut<ArgumentTuple, Result> syntax (unstable)
// - allowed us to be less careful with nonoverlapping trait implementations (aka. specialization)

/// Only for internal use, since this doesn’t set the index.
impl<Arg, Result, F> From<F> for Callback<(Arg,), Result>
where
    F: FnMut(Arg) -> Result + 'static,
{
    fn from(mut value: F) -> Self {
        Self {
            link: XorLinkedListLink::new(),
            function: Box::new(move |(a,)| value(a)),
            id: 0,
        }
    }
}
impl<Arg1, Arg2, Result, F> From<F> for Callback<(Arg1, Arg2), Result>
where
    F: FnMut(Arg1, Arg2) -> Result + 'static,
{
    fn from(mut value: F) -> Self {
        Self {
            link: XorLinkedListLink::new(),
            function: Box::new(move |(a, b)| value(a, b)),
            id: 0,
        }
    }
}

impl<Args, Result> Callback<Args, Result> {
    fn call(&mut self, args: Args) -> Result {
        (self.function)(args)
    }
}

intrusive_adapter!(LinkedCallback<Args, Result> = Box<Callback<Args, Result>>:
    Callback<Args, Result> { link => XorLinkedListLink }
);

/// Handle to a subscribed callback.
///
/// This handle can later be used to remove the callback again.
/// This is very important if you frequently add and remove callbacks, such as in sub-windows.
/// If you do not remove a callback after you don’t use it anymore, it will remain leaked in memory.
// Implementation note: when using arguments with non-static lifetime (usually temporary shared references),
// the handle type should use 'static to reduce borrow checker issues.
// Thanks to covariance, the conversion `for<'a> 'a: 'static` is always possible.
#[must_use = "ignoring a callback handle leads to memory leaks"]
pub struct CallbackHandle<Args, Result = ()> {
    id: usize,
    phantom: PhantomData<(Args, Result)>,
}

pub(crate) struct GlobalCallbacksInner<Args, Result> {
    callbacks: XorLinkedList<LinkedCallback<Args, Result>>,
}

impl<Args, Result> GlobalCallbacksInner<Args, Result> {
    pub const fn new() -> Self {
        Self {
            callbacks: XorLinkedList::new(LinkedCallback::new()),
        }
    }

    pub fn add(
        &mut self,
        callback: impl Into<Callback<Args, Result>>,
    ) -> CallbackHandle<Args, Result> {
        let mut callback = Box::new(callback.into());
        let new_id = self
            .callbacks
            .back()
            .get()
            .map(|b| b.id.wrapping_add(1))
            .unwrap_or_default();
        callback.id = new_id;
        self.callbacks.push_back(callback.into());
        CallbackHandle {
            id: new_id,
            phantom: PhantomData,
        }
    }

    pub fn remove(&mut self, handle: CallbackHandle<Args, Result>) {
        let mut cursor = self.callbacks.cursor_mut();
        cursor.move_next();
        while !cursor.is_null() {
            let matches_id = cursor
                .get()
                .map(|callback| callback.id == handle.id)
                .unwrap_or(false);
            if matches_id {
                log_c_str(c"found handler to remove");
                cursor.remove();
            } else {
                cursor.move_next();
            }
        }
    }

    pub fn clear(&mut self) {
        self.callbacks.clear();
    }
}

pub struct GlobalCallbacks<P, T> {
    inner: Mutex<RefCell<GlobalCallbacksInner<P, T>>>,
}

impl<P, T> GlobalCallbacks<P, T> {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(RefCell::new(GlobalCallbacksInner::new())),
        }
    }

    pub fn add(&self, callback: impl Into<Callback<P, T>>) -> CallbackHandle<P, T> {
        MutexToken::with(|t| self.inner.borrow_mut(t).add(callback))
    }

    pub fn remove(&self, handle: CallbackHandle<P, T>) {
        MutexToken::with(|t| {
            self.inner.borrow_mut(t).remove(handle);
        });
    }

    #[allow(unused)] // currently no users, might be useful in the future
    pub fn clear(&self) {
        MutexToken::with(|t| {
            self.inner.borrow_mut(t).clear();
        });
    }

    #[allow(unused)]
    pub unsafe fn as_void(&self) -> *mut c_void {
        addr_of!(self.inner) as *const c_void as *mut c_void
    }

    fn dispatch_on(mutex: &Mutex<RefCell<GlobalCallbacksInner<P, T>>>, data: P) -> Option<T>
    where
        P: Clone,
    {
        let mut result = None;
        MutexToken::with(|t| {
            let mut callbacks = mutex.borrow_mut(t);
            let mut cursor = callbacks.callbacks.cursor_mut();
            cursor.move_next();
            while !cursor.is_null() {
                let mut callback = cursor.remove().expect("callback must exist");
                let return_value = callback.call(data.clone());
                result = Some(return_value);
                cursor.insert_after(callback);
            }
        });
        result
    }

    pub(crate) unsafe fn dispatch_callback(context: *mut c_void, data: P) -> Option<T>
    where
        P: Clone,
    {
        let mutex =
            (unsafe { (context as *mut Mutex<RefCell<GlobalCallbacksInner<P, T>>>).as_ref() })?;

        Self::dispatch_on(mutex, data)
    }

    pub(crate) fn dispatch(&self, data: P) -> Option<T>
    where
        P: Clone,
    {
        Self::dispatch_on(&self.inner, data)
    }
}
