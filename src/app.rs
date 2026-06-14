use core::cell::UnsafeCell;

use alloc::boxed::Box;

use crate::{
    log::log_c_str,
    sys::{self, TimeUnits},
    window::Window,
};

pub struct AppState {
    timer_callback: Option<Box<dyn FnMut() + 'static>>,
}

pub struct App;

pub static APP: App = App;

static mut APP_STATE: UnsafeCell<AppState> = UnsafeCell::new(AppState {
    timer_callback: None,
});

unsafe fn with_state<R>(func: impl FnOnce(&mut AppState) -> R) -> Option<R> {
    #[allow(static_mut_refs)]
    let state_ref = unsafe { APP_STATE.get().as_mut() };
    Some(func(state_ref?))
}

#[unsafe(no_mangle)]
extern "C" fn tick_handler(_tick_time: *mut sys::tm, _units_changed: TimeUnits) {
    unsafe {
        with_state(|state| {
            let Some(callback) = state.timer_callback.as_mut() else {
                log_c_str(c"No tick handler associated");
                return;
            };
            callback();
        });
    }
}

impl App {
    pub fn show_window(&self, window: &Window) {
        unsafe { sys::window_stack_push(window.raw.as_ptr(), true) };
    }
    pub fn show_window_immediate(&self, window: &Window) {
        unsafe { sys::window_stack_push(window.raw.as_ptr(), false) };
    }
    pub fn event_loop(&self) {
        unsafe { sys::app_event_loop() };
    }
    pub fn set_timer(&self, unit: TimeUnits, callback: impl FnMut() + 'static) {
        unsafe {
            with_state(|state| {
                state.timer_callback = Some(Box::new(callback));
                sys::tick_timer_service_subscribe(unit, Some(tick_handler));
            });
        };
    }
    pub fn clear_timer(&self) {
        unsafe {
            with_state(|state| {
                sys::tick_timer_service_unsubscribe();
                state.timer_callback = None;
            });
        }
    }
}
