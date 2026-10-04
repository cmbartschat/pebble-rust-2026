use core::{cell::RefCell, ffi::c_void, ptr::addr_of};

use alloc::boxed::Box;
use intrusive_collections::{XorLinkedList, XorLinkedListLink, intrusive_adapter};

use crate::{Mutex, MutexToken};

/// Internal callback storage.
/// You don’t need to interact with this type directly; all [`FnMut`] implementors can be converted to it.
pub struct Callback<Args> {
    /// Intrusive linked list connection in the global callback list
    link: XorLinkedListLink,
    /// Actual function.
    /// Unfortunately due to intrusive_collections limitations, we can’t remove the Box here.
    function: Box<dyn FnMut(Args)>,
    /// Symbolic ID, so that handles can refer to it.
    /// Using IDs avoids overhead from Rc, RefCell, Mutex etc.,
    /// since we know that the handles cannot (and are not supposed to) mutate the callback’s inner data.
    id: usize,
}

// TODO: A lot of this would be less annoying if Rust:
// - allowed FnMut<ArgumentTuple, Result> syntax (unstable)
// - allowed us to be less careful with nonoverlapping trait implementations (aka. specialization)

/// Only for internal use, since this doesn’t set the index.
impl<Arg, F> From<F> for Callback<(Arg,)>
where
    F: FnMut(Arg) + 'static,
{
    fn from(mut value: F) -> Self {
        Self {
            link: XorLinkedListLink::new(),
            function: Box::new(move |(a,)| value(a)),
            id: 0,
        }
    }
}
impl<Arg1, Arg2, F> From<F> for Callback<(Arg1, Arg2)>
where
    F: FnMut(Arg1, Arg2) + 'static,
{
    fn from(mut value: F) -> Self {
        Self {
            link: XorLinkedListLink::new(),
            function: Box::new(move |(a, b)| value(a, b)),
            id: 0,
        }
    }
}

impl<Args> Callback<Args> {
    fn call(&mut self, args: Args) {
        (self.function)(args)
    }
}

intrusive_adapter!(LinkedCallback<Args> = Box<Callback<Args>>:
    Callback<Args> { link => XorLinkedListLink }
);

/// Handle to a subscribed callback.
///
/// This handle can later be used to remove the callback again via [`CallbackHandle::cancel`].
/// This is very important if you frequently add and remove callbacks, such as in sub-windows.
/// If you do not remove a callback after you don’t use it anymore, it will remain leaked in memory.
// Implementation note: when using arguments with non-static lifetime (usually temporary shared references),
// the handle type should use 'static to reduce borrow checker issues.
// Thanks to covariance, the conversion `for<'a> 'a: 'static` is always possible.
#[must_use = "ignoring a callback handle leads to memory leaks"]
// It is sound to copy the handle, since unsubscribing from a handle that is no longer subscribed is a noop.
// The only possible issue would arise if the user carefully subscribed and unsubscribed to 2^32 events in a very specific order,
// and then tried to unsubscribe from this event again, at which point the unsubscription will hit an unrelated handler.
// This is clearly a degenerate case that should not happen in practice, and it is still sound (just a logic bug).
#[derive(Clone, Copy)]
// Lifetime parameter is the lifetime of the parent. In practice this is almost always 'static.
pub struct CallbackHandle<'a, Args> {
    id: usize,
    parent: &'a GlobalCallbacks<Args>,
}

impl<'a, Args> CallbackHandle<'a, Args> {
    /// Removes this callback from its event, so it will no longer be triggered.
    /// The callback is dropped.
    pub fn cancel(self) {
        self.parent.remove(self);
    }
}

pub(crate) struct GlobalCallbacksInner<Args> {
    callbacks: XorLinkedList<LinkedCallback<Args>>,
    /// Callback that is invoked when the list is empty.
    /// This is intended to remove the C API handler.
    /// We could make this a struct generic parameter (`Fn()`), but that would probably polymorphize all of the logic below even more, costing lots of code size.
    /// In many cases `Args` is `()`, so the code is actually not duplicated too much.
    /// On the other hand, this is just a pointer, aka. 4 bytes in the data segment per global callback instance (of which there are approximately 20-30).
    empty_callback: fn(),
}

impl<Args> GlobalCallbacksInner<Args> {
    pub const fn new(empty_callback: fn()) -> Self {
        Self {
            callbacks: XorLinkedList::new(LinkedCallback::new()),
            empty_callback,
        }
    }

    pub fn add<'a>(
        &mut self,
        callback: Callback<Args>,
        parent: &'a GlobalCallbacks<Args>,
    ) -> CallbackHandle<'a, Args> {
        let mut callback = Box::new(callback);
        let new_id = self
            .callbacks
            .back()
            .get()
            .map(|b| b.id.wrapping_add(1))
            .unwrap_or_default();
        callback.id = new_id;
        self.callbacks.push_back(callback);
        CallbackHandle { id: new_id, parent }
    }

    fn remove(&mut self, id: usize) {
        let mut cursor = self.callbacks.cursor_mut();
        cursor.move_next();
        while !cursor.is_null() {
            let matches_id = cursor
                .get()
                .map(|callback| callback.id == id)
                .unwrap_or(false);
            if matches_id {
                cursor.remove();
                break;
            }
            cursor.move_next();
        }
        if self.is_empty() {
            (self.empty_callback)();
        }
    }

    pub fn clear(&mut self) {
        self.callbacks.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.callbacks.is_empty()
    }
}

pub struct GlobalCallbacks<P> {
    inner: Mutex<RefCell<GlobalCallbacksInner<P>>>,
}

impl<P> GlobalCallbacks<P> {
    pub const fn new(empty_callback: fn()) -> Self {
        Self {
            inner: Mutex::new(RefCell::new(GlobalCallbacksInner::new(empty_callback))),
        }
    }

    /// Note that this function copies the reference into the callback handle.
    /// Most users should use a `&'static self` explicitly, to force the user to input a static reference, so that the callback handle is also valid for 'static.
    ///
    pub fn add<'a>(&'a self, callback: Callback<P>) -> CallbackHandle<'a, P> {
        MutexToken::with(|t| self.inner.borrow_mut(t).add(callback, self))
    }

    fn remove(&self, handle: CallbackHandle<P>) {
        MutexToken::with(|t| {
            self.inner.borrow_mut(t).remove(handle.id);
        });
    }

    #[allow(unused)] // currently no users, might be useful in the future
    pub fn clear(&self) {
        MutexToken::with(|t| {
            self.inner.borrow_mut(t).clear();
        });
    }

    #[allow(unused, clippy::missing_const_for_fn)]
    pub unsafe fn as_void(&self) -> *mut c_void {
        addr_of!(self.inner) as *const c_void as *mut c_void
    }

    fn dispatch_on(mutex: &Mutex<RefCell<GlobalCallbacksInner<P>>>, data: P)
    where
        P: Clone,
    {
        MutexToken::with(|t| {
            let mut callbacks = mutex.borrow_mut(t);
            let mut cursor = callbacks.callbacks.cursor_mut();
            cursor.move_next();
            while !cursor.is_null() {
                let mut callback = cursor.remove().expect("callback must exist");
                callback.call(data.clone());
                cursor.insert_before(callback);
            }
        });
    }

    pub(crate) unsafe fn dispatch_callback(context: *mut c_void, data: P) -> Option<()>
    where
        P: Clone,
    {
        let mutex =
            (unsafe { (context as *mut Mutex<RefCell<GlobalCallbacksInner<P>>>).as_ref() })?;

        Self::dispatch_on(mutex, data);
        Some(())
    }

    pub(crate) fn dispatch(&self, data: P)
    where
        P: Clone,
    {
        Self::dispatch_on(&self.inner, data)
    }
}

/// Callback storage when there is only one event handler.
/// F is always `dyn FnMut(Args)`, but we can’t say that here or risk making lifetimes in `Args` invariant.
/// If you don’t need to worry about higher-ranked trait bounds (HRTBs), aka. `for<'a> FnMut(&'a SomeArgument)`, just use [`SingleCallbackFn`].
/// (See [here](https://doc.rust-lang.org/stable/nomicon/hrtb.html) for more information.)
pub struct SingleCallback<F: ?Sized> {
    /// Actual function.
    function: Mutex<RefCell<Option<Box<F>>>>,
}

impl<F: ?Sized> SingleCallback<F> {
    pub fn set(&self, value: Box<F>) {
        MutexToken::with(|token| {
            *self.function.borrow_mut(token) = Some(value);
        });
    }

    pub fn clear(&self) {
        MutexToken::with(|token| {
            *self.function.borrow_mut(token) = None;
        });
    }

    pub const fn new() -> Self {
        Self {
            function: Mutex::new(RefCell::new(None)),
        }
    }

    // Complex dispatch implementation using HRTB.
    // In practice F should be a type erased `dyn FnMut` but we can’t tell the compiler about that.
    #[allow(clippy::extra_unused_lifetimes)] // clearer this way
    pub fn dispatch<Args>(&self, args: Args)
    where
        F: for<'a> FnMut(Args),
    {
        let function = MutexToken::with(|token| self.function.borrow_mut(token).take());
        if let Some(mut function) = function {
            function(args);
            MutexToken::with(|token| {
                *self.function.borrow_mut(token) = Some(function);
            });
        }
    }
}

/// More convenient way to plug a FnMut into [`SingleCallback`].
/// Note that this type alias almost definitely breaks the HRTB on `dispatch`,
/// so it cannot be used if your function takes references of any kind.
pub type SingleCallbackFn<Args> = SingleCallback<dyn FnMut(Args) + 'static>;
