use core::cell::RefCell;
use cortex_m as _;
use critical_section::Mutex;

use alloc::{
    boxed::Box,
    ffi::CString,
    sync::Arc,
    vec::{self, Vec},
};

use crate::{
    log::log_str,
    sys::{self, TimeUnits},
    window::Window,
};

struct SyncWrapper<T>(RefCell<T>);
unsafe impl<T> Sync for SyncWrapper<T> {}

pub struct AppState {
    timer_callback: Option<Box<dyn FnMut() + 'static>>,
    filename: Option<Box<[u8; 60]>>,
}

static mut APP_STATE: Mutex<RefCell<AppState>> = Mutex::new(RefCell::new(AppState {
    timer_callback: None,
    filename: None,
}));

pub struct App;

pub static APP: App = App;

fn with_state<R>(func: impl FnOnce(&mut AppState) -> R) -> Option<R> {
    critical_section::with(|cs| {
        #[allow(static_mut_refs)]
        let mut borrowed_state = unsafe { APP_STATE.borrow(cs) };
        let Ok(mut ready_state) = borrowed_state.try_borrow_mut() else {
            log_str("Can't borrow APP_STATE");
            return None;
        };
        log_str("Borrowed APP_STATE");
        Some(func(&mut ready_state))
    })
}

extern "C" fn tick_handler(tick_time: *mut sys::tm, units_changed: TimeUnits) {
    with_state(|state| {
        let Some(mut callback) = state.timer_callback.as_mut() else {
            return;
        };
        callback();
    });
}

impl App {
    pub fn show_window(&self, window: &Window) {
        unsafe { sys::window_stack_push(window.inner, true) };
    }
    pub fn show_window_immediate(&self, window: &Window) {
        unsafe { sys::window_stack_push(window.inner, false) };
    }
    pub fn event_loop(&self) {
        unsafe { sys::app_event_loop() };
    }
    pub fn set_timer(&self, unit: TimeUnits, callback: impl FnMut() + 'static) {
        with_state(|state| unsafe {
            state.timer_callback = Some(Box::new(callback));
            sys::tick_timer_service_subscribe(unit, Some(tick_handler));
        });
    }
    pub fn clear_timer(&self) {
        with_state(|state| unsafe {
            sys::tick_timer_service_unsubscribe();
            state.timer_callback = None;
        });
    }
}
