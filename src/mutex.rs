use core::cell::{Cell, Ref, RefCell, RefMut};

use cortex_m as _;
use critical_section::CriticalSection as Token;

type InnerMutex<T> = critical_section::Mutex<T>;

pub struct Mutex<T>(InnerMutex<T>);

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self(InnerMutex::new(value))
    }

    pub fn as_ref<'a>(&'a self, token: MutexToken<'a>) -> &'a T {
        self.0.borrow(token.0)
    }
}

unsafe impl<T> Sync for Mutex<T> {}
unsafe impl<T> Send for Mutex<T> {}

impl<T> Mutex<Cell<T>>
where
    T: Copy,
{
    pub fn get(&self) -> T {
        MutexToken::with(|t| self.as_ref(t).get())
    }

    pub fn set(&self, value: T) {
        MutexToken::with(|t| self.as_ref(t).set(value))
    }
}

impl<T> Mutex<RefCell<T>> {
    pub fn borrow<'a>(&'a self, token: MutexToken<'a>) -> Ref<'a, T> {
        self.as_ref(token).borrow()
    }

    pub fn borrow_mut<'a>(&'a self, token: MutexToken<'a>) -> RefMut<'a, T> {
        self.as_ref(token).borrow_mut()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MutexToken<'a>(Token<'a>);

impl<'a> MutexToken<'a> {
    pub fn with<R>(handler: impl FnOnce(MutexToken) -> R) -> R {
        critical_section::with(|c| handler(MutexToken(c)))
    }
}
