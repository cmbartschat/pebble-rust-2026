use core::{
    cell::RefCell,
    ffi::{self, c_void},
    ptr::null_mut,
};

use alloc::{boxed::Box, vec::Vec};

use crate::{
    TimeUnits, Window,
    app_message_result::{AppMessageResult, app_message_result_from_raw},
    dictionary::{DictionaryBuilder, DictionaryView},
    log::log_c_str,
    service, sys,
};

type InboxReceivedCallback = Option<Box<dyn FnMut(&mut DictionaryView) + 'static>>;

pub(crate) struct AppState {
    timer_callback: Option<Box<dyn FnMut() + 'static>>,
    inbox_received_callback: InboxReceivedCallback,
    visible_windows: Vec<Window>,
}

/// The Pebble app, see [`APP`].
#[non_exhaustive]
pub struct App {
    /// See [`Persist`](crate::persist::Persist).
    pub persist: crate::persist::Persist,
    /// See [`Touch`](service::Touch).
    pub touch: service::Touch,
    /// See [`UnobstructedArea`](service::UnobstructedArea).
    pub unobstructed_area: service::UnobstructedArea,
    /// See [`BatteryState`](service::BatteryState).
    pub battery_state: service::BatteryState,
    /// See [`Compass`](service::Compass).
    pub compass: service::Compass,
    /// See [`BluetoothConnection`](service::BluetoothConnection).
    pub bluetooth_connection: service::BluetoothConnection,
    /// See [`Accelerometer`](service::Accelerometer).
    pub accelerometer: service::Accelerometer,
    /// See [`AppFocus`](service::AppFocus).
    pub focus: service::AppFocus,
    /// See [`Wakeup`](service::Wakeup).
    pub wakeup: service::Wakeup,
}

/// The Pebble app.
/// This singleton contains most global application functionality.
pub static APP: App = App {
    persist: crate::persist::Persist,
    touch: service::Touch::new(),
    unobstructed_area: service::UnobstructedArea::new(),
    battery_state: service::BatteryState::new(),
    compass: service::Compass::new(),
    bluetooth_connection: service::BluetoothConnection::new(),
    accelerometer: service::Accelerometer::new(),
    focus: service::AppFocus::new(),
    wakeup: service::Wakeup::new(),
};

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

extern "C" fn global_inbox_dropped_handler(_: u16, _: *mut c_void) {
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

#[allow(clippy::missing_const_for_fn)]
extern "C" fn global_outbox_sent_handler(
    _iterator: *mut sys::DictionaryIterator,
    _context: *mut ::core::ffi::c_void,
) {
    // log_c_str(c"global_outbox_sent_handler");
}

impl App {
    /// Run the app event loop.
    /// Not calling this function may lead your application to crash, or to fail to link.
    pub fn event_loop(&self) {
        unsafe { sys::app_event_loop() };
    }
    /// Set or override the global tick handler.
    /// The handler is called every specified time unit, see [`TimeUnits`].
    pub fn set_tick_handler(&self, unit: TimeUnits, callback: impl FnMut() + 'static) {
        unsafe {
            with_state(|state| {
                state.timer_callback = Some(Box::new(callback));
                sys::tick_timer_service_subscribe(unit.bits(), Some(tick_handler));
            });
        };
    }
    /// Clear the current tick handler.
    pub fn clear_tick_handler(&self) {
        unsafe {
            with_state(|state| {
                sys::tick_timer_service_unsubscribe();
                state.timer_callback = None;
            });
        }
    }

    /// Set or override the app message handler.
    /// It receives a [`DictionaryView`].
    pub fn set_message_handler(&self, callback: impl FnMut(&mut DictionaryView) + 'static) {
        unsafe {
            with_state(|state| {
                state.inbox_received_callback = Some(Box::new(callback));
                sys::app_message_register_inbox_received(Some(global_message_handler));
            });
        }
    }

    /// Clear the app message handler.
    pub fn clear_message_handler(&self) {
        unsafe {
            with_state(|state| {
                state.inbox_received_callback = None;
                sys::app_message_register_inbox_received(None);
            });
        }
    }

    /// Open the app message inbox.
    /// This causes the app message handler to be invoked for received messages.
    /// The size specifies the required inbox memory sizes, see [`InboxSize`].
    // TODO: Manual specification of the inbox size is awkward.
    //       Maybe we should always request the maximum.
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

    /// Send an app message.
    /// The given callback will be invoked with a mutable reference to a [`DictionaryBuilder`] that can then be modified to set up the message.
    /// Afterwards, the message is sent.
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
                let window_ptr = { window.handle.borrow_mut().as_ptr_mut() };
                sys::window_stack_push(window_ptr, animated);
                state.visible_windows.push(window);
            });
        }
    }

    /// Show the specified window.
    pub fn show(&self, window: Window) {
        self.show_inner(window, true);
    }

    /// Show the specified window without playing animations.
    pub fn show_immediate(&self, window: Window) {
        self.show_inner(window, false);
    }

    fn hide_inner(&self, window: &mut Window, animated: bool) {
        unsafe {
            let window = window.handle.borrow_mut().as_ptr_mut();
            sys::window_stack_remove(window, animated)
        };
    }

    /// Hide the specified window.
    pub fn hide(&self, window: &mut Window) {
        self.hide_inner(window, true);
    }

    /// Hide the specified window without playing animations.
    pub fn hide_immediate(&self, window: &mut Window) {
        self.hide_inner(window, false);
    }

    pub(crate) fn notify_unload(&self, window: *const sys::Window) {
        unsafe {
            with_state(|state| {
                state.visible_windows.retain(|f| f != window);
            })
        }
    }
}

/// The requested data size of the inbox.
/// This is used for [`App::open_inbox`].
#[derive(Copy, Clone)]
pub enum InboxSize {
    /// Request exactly these data sizes from the C API.
    /// Using this variant is not recommended.
    Exact {
        /// Size of the inbox buffer in bytes.
        inbox: u32,
        /// Size of the outbox buffer in bytes.
        outbox: u32,
    },
    /// Request the maximum possible size.
    Max,
    /// Request half of the maximum possible size.
    Half,
    /// Request a quarter of the maximum possible size.
    Quarter,
}
