use core::{
    cell::RefCell,
    ffi::{self, c_void},
    ptr::null_mut,
};

use alloc::{boxed::Box, vec::Vec};

use crate::{
    Window,
    app_message_result::{AppMessageResult, app_message_result_from_raw},
    dictionary::{DictionaryBuilder, DictionaryView},
    log::log_c_str,
    persist::Persist,
    sys::{self},
};

type InboxReceivedCallback = Option<Box<dyn FnMut(&mut DictionaryView) + 'static>>;

pub struct AppState {
    timer_callback: Option<Box<dyn FnMut() + 'static>>,
    inbox_received_callback: InboxReceivedCallback,
    visible_windows: Vec<Window>,
}

pub struct App {
    pub persist: Persist,
}

pub static APP: App = App { persist: Persist };

static mut APP_STATE: RefCell<AppState> = RefCell::new(AppState {
    timer_callback: None,
    inbox_received_callback: None,
    visible_windows: Vec::new(),
});

extern "C" fn global_message_handler(
    message: *mut sys::DictionaryIterator,
    _data: *mut ffi::c_void,
) {
    let Some(mut message) = DictionaryView::from_raw(message) else {
        log_c_str(c"Unexpected null message in inbox");
        return;
    };
    unsafe {
        with_state(|state| {
            if let Some(callback) = state.inbox_received_callback.as_mut() {
                callback(&mut message);
            } else {
                log_c_str(c"global_message_handler has no callback to call");
            }
        });
    }
}

unsafe fn with_state<R>(func: impl FnOnce(&mut AppState) -> R) -> R {
    #[allow(static_mut_refs)]
    let mut state_ref = unsafe { APP_STATE.borrow_mut() };
    func(&mut state_ref)
}

#[unsafe(no_mangle)]
extern "C" fn tick_handler(_tick_time: *mut sys::tm, _units_changed: sys::TimeUnits) {
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

extern "C" fn global_inbox_dropped_handler(_: u32, _: *mut c_void) {
    log_c_str(c"inbox_dropped");
}

extern "C" fn global_outbox_failed_handler(
    _iterator: *mut sys::DictionaryIterator,
    reason: sys::AppMessageResult,
    _context: *mut ::core::ffi::c_void,
) {
    log_c_str(c"outbox failed");
    if let Err(err) = app_message_result_from_raw(reason) {
        match err {
            crate::AppMessageError::AlreadyReleased => todo!(),
            crate::AppMessageError::AppNotRunning => log_c_str(c"  reason: not running"),
            crate::AppMessageError::BufferOverflow => todo!(),
            crate::AppMessageError::Busy => log_c_str(c"  reason: busy"),
            crate::AppMessageError::CallbackAlreadyRegistered => todo!(),
            crate::AppMessageError::CallbackNotRegistered => todo!(),
            crate::AppMessageError::Closed => log_c_str(c"  reason: closed"),
            crate::AppMessageError::InternalError => todo!(),
            crate::AppMessageError::InvalidArgs => todo!(),
            crate::AppMessageError::InvalidState => todo!(),
            crate::AppMessageError::NotConnected => log_c_str(c"  reason: not connected"),
            crate::AppMessageError::OutOfMemory => todo!(),
            crate::AppMessageError::SendRejected => todo!(),
            crate::AppMessageError::SendTimeout => log_c_str(c"  reason: timeout"),
            crate::AppMessageError::Unknown => todo!(),
        };
    }
}

extern "C" fn global_outbox_sent_handler(
    _iterator: *mut sys::DictionaryIterator,
    _context: *mut ::core::ffi::c_void,
) {
    // log_c_str(c"global_outbox_sent_handler");
}

impl App {
    pub fn event_loop(&self) {
        unsafe { sys::app_event_loop() };
    }
    pub fn set_tick_handler(&self, unit: sys::TimeUnits, callback: impl FnMut() + 'static) {
        unsafe {
            with_state(|state| {
                state.timer_callback = Some(Box::new(callback));
                sys::tick_timer_service_subscribe(unit, Some(tick_handler));
            });
        };
    }
    pub fn clear_tick_handler(&self) {
        unsafe {
            with_state(|state| {
                sys::tick_timer_service_unsubscribe();
                state.timer_callback = None;
            });
        }
    }

    pub fn set_message_handler(&self, callback: impl FnMut(&mut DictionaryView) + 'static) {
        unsafe {
            with_state(|state| {
                state.inbox_received_callback = Some(Box::new(callback));
                sys::app_message_register_inbox_received(Some(global_message_handler));
            });
        }
    }

    pub fn clear_message_handler(&self) {
        unsafe {
            with_state(|state| {
                state.inbox_received_callback = None;
                sys::app_message_register_inbox_received(None);
            });
        }
    }

    pub fn open_message(&self) {
        unsafe {
            with_state(|state| {
                sys::tick_timer_service_unsubscribe();
                state.timer_callback = None;
            });
        }
    }

    pub fn open_inbox(&self, size: InboxSize) -> AppMessageResult<()> {
        let (inbox_size, outbox_size) = match size {
            InboxSize::Exact { inbox, outbox } => (inbox, outbox),
            InboxSize::Max => unsafe {
                (
                    sys::app_message_inbox_size_maximum(),
                    sys::app_message_outbox_size_maximum(),
                )
            },
            InboxSize::Half => unsafe {
                (
                    sys::app_message_inbox_size_maximum() >> 1,
                    sys::app_message_outbox_size_maximum() >> 1,
                )
            },
            InboxSize::Quarter => unsafe {
                (
                    sys::app_message_inbox_size_maximum() >> 2,
                    sys::app_message_outbox_size_maximum() >> 2,
                )
            },
        };

        unsafe {
            sys::app_message_register_inbox_dropped(Some(global_inbox_dropped_handler));
            sys::app_message_register_outbox_failed(Some(global_outbox_failed_handler));
            sys::app_message_register_outbox_sent(Some(global_outbox_sent_handler));
        }

        app_message_result_from_raw(unsafe { sys::app_message_open(inbox_size, outbox_size) })?;

        Ok(())
    }

    pub fn send_message(
        &self,
        builder_callback: impl FnOnce(&mut DictionaryBuilder) -> AppMessageResult<()>,
    ) -> AppMessageResult<()> {
        unsafe {
            let mut b = null_mut::<sys::DictionaryIterator>();
            app_message_result_from_raw(sys::app_message_outbox_begin(&mut b))?;
            let Some(mut dict) = DictionaryBuilder::from_ptr(b) else {
                log_c_str(c"outbox begin gave back null.");
                return Err(crate::AppMessageError::SendRejected);
            };
            builder_callback(&mut dict)?;
            app_message_result_from_raw(sys::app_message_outbox_send())?;
        }
        Ok(())
    }

    fn show_inner(&self, window: Window, animated: bool) {
        unsafe {
            with_state(|state| {
                window.handle.borrow_mut().stack_push(animated);
                state.visible_windows.push(window.retain());
            });
        }
    }

    pub fn show(&self, window: Window) {
        self.show_inner(window, true);
    }

    pub fn show_immediate(&self, window: Window) {
        self.show_inner(window, false);
    }

    fn hide_inner(&self, window: Window, animated: bool) {
        window.handle.borrow_mut().stack_remove(animated);
    }

    pub fn hide(&self, window: Window) {
        self.hide_inner(window, true);
    }

    pub fn hide_immediate(&self, window: Window) {
        self.hide_inner(window, false);
    }

    pub(crate) fn notify_unload(&self, window: *const sys::Window) {
        unsafe {
            with_state(|state| {
                state.visible_windows.retain(|f| !f.is_equal(window));
            })
        }
    }
}

#[derive(Copy, Clone)]
pub enum InboxSize {
    Exact { inbox: u32, outbox: u32 },
    Max,
    Half,
    Quarter,
}
