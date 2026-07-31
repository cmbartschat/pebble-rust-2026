use core::cell::RefCell;

use alloc::boxed::Box;

use crate::{effect::Effect, service::GlobalCallbackInner};

pub type Callback = Box<dyn FnMut() + 'static>;

pub(crate) struct WindowUserData {
    pub(crate) load_handler: RefCell<GlobalCallbackInner<Callback>>,
    pub(crate) appear_handler: RefCell<GlobalCallbackInner<Callback>>,
    pub(crate) disappear_handler: RefCell<GlobalCallbackInner<Callback>>,
    pub(crate) unload_handler: RefCell<GlobalCallbackInner<Callback>>,
    pub(crate) appear_effect: RefCell<Effect>,
    pub(crate) load_effect: RefCell<Effect>,
}

fn dispatch_handler(handler: &RefCell<GlobalCallbackInner<Callback>>) {
    let callback = { handler.borrow_mut().extract() };
    if let Some(mut callback) = callback {
        callback();
        handler.borrow_mut().restore(callback);
    }
}

impl WindowUserData {
    pub fn dispatch_load(&self) {
        dispatch_handler(&self.load_handler);
        self.load_effect.borrow_mut().mount();
    }

    pub fn dispatch_appear(&self) {
        dispatch_handler(&self.appear_handler);
        self.appear_effect.borrow_mut().mount();
    }

    pub fn dispatch_disappear(&self) {
        self.appear_effect.borrow_mut().unmount();
        dispatch_handler(&self.disappear_handler);
    }

    pub fn dispatch_unload(&self) {
        self.load_effect.borrow_mut().unmount();
        dispatch_handler(&self.unload_handler);
    }
}
