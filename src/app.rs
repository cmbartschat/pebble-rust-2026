use core::{
    cell::{RefCell, UnsafeCell},
    ops::DerefMut,
};
use cortex_m as _;
use critical_section::Mutex;

use alloc::{
    boxed::Box,
    ffi::CString,
    sync::Arc,
    vec::{self, Vec},
};

use crate::{
    log::{log_num, log_str},
    sys::{self, TimeUnits},
    window::Window,
};

struct SyncWrapper<T>(RefCell<T>);
unsafe impl<T> Sync for SyncWrapper<T> {}

pub struct AppState {
    timer_callback: Option<Box<dyn FnMut() + 'static>>,
    filename: Option<Box<[u8; 60]>>,
}

static mut APP_STATE: Mutex<UnsafeCell<AppState>> = Mutex::new(UnsafeCell::new(AppState {
    timer_callback: None,
    filename: None,
}));

pub struct App;

pub static APP: App = App;

fn with_state<R>(func: impl FnOnce(&mut AppState) -> R) -> Option<R> {
    log_num(700);
    critical_section::with(|cs| {
        #[allow(static_mut_refs)]
        let borrowed_state = unsafe { APP_STATE.borrow(cs) };

        // {
        //     let state = borrowed_state.borrow();
        //     if (state.timer_callback.is_some()) {
        //         log_num(6700);
        //     } else {
        //         log_num(6710);
        //     }
        // }
        log_num(710);

        let ptr = unsafe { borrowed_state.get() };
        let maybe_ref = unsafe { ptr.as_mut() };
        let Some(b) = maybe_ref else {
            log_num(711);
            return None;
        };
        log_num(720);
        let res = Some(func(b));
        log_num(730);
        res
    })
}

extern "C" fn tick_handler(tick_time: *mut sys::tm, units_changed: TimeUnits) {
    log_num(5000);
    // with_state(|state| {
    //     // let Ok(mut borrowed_handler) = state.timer_callback.try_borrow_mut() else {
    //     //     log_str("badcall");
    //     //     return;
    //     // };

    //     let Some(mut callback) = state.timer_callback.as_mut() else {
    //         return;
    //     };
    //     log_num(5555);
    //     callback();
    // });
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
        log_num(900);
        with_state(|state| unsafe {
            log_num(902);
            // state.timer_callback = Some(Box::new(callback));
            log_num(904);
            sys::tick_timer_service_subscribe(unit, Some(tick_handler));
            log_num(910);
        });
        log_num(920);
    }
    pub fn clear_timer(&self) {
        with_state(|state| unsafe {
            sys::tick_timer_service_unsubscribe();
            state.timer_callback = None;
        });
    }
}
