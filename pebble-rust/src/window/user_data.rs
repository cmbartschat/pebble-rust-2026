use core::cell::RefCell;

use crate::{effect::Effect, service::GlobalCallbacks};

pub(crate) struct WindowUserData {
    pub(crate) load_handler: GlobalCallbacks<()>,
    pub(crate) appear_handler: GlobalCallbacks<()>,
    pub(crate) disappear_handler: GlobalCallbacks<()>,
    pub(crate) unload_handler: GlobalCallbacks<()>,
    pub(crate) appear_effect: RefCell<Effect>,
    pub(crate) load_effect: RefCell<Effect>,
}

fn dispatch_handler(handler: &GlobalCallbacks<()>) {
    handler.dispatch(());
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
