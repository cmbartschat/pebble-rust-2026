use alloc::boxed::Box;

use crate::{
    Time,
    service::global_callback::GlobalCallback,
    status_code::{StatusError, parse_status_result},
    sys::{self, WakeupId},
};

/// Allows you to configure watch wakeup and retrieve information about the last wakeup that occurred.
pub struct Wakeup;

static HANDLER: GlobalCallback<WakeupEvent, ()> = GlobalCallback::new();

/// Possible errors that can happen when scheduling a wakeup.
#[derive(Copy, Clone, PartialEq)]
#[repr(i8)]
pub enum WakeupSchedulingError {
    /// The wakeup is too close to an existing one.
    /// There must be a distance of at least 1 minute between wakeups.
    TooCloseToExisting = StatusError::Range as i8,
    /// The wakeup you tried to schedule lies in the past.
    InThePast = StatusError::InvalidArgument as i8,
    /// The app has already scheduled 8 wakeups, more are not possible.
    MaximumWakeupsReached = StatusError::OutOfResources as i8,
    /// An internal unknown error.
    Internal = StatusError::Internal as i8,
}

impl From<StatusError> for WakeupSchedulingError {
    fn from(value: StatusError) -> Self {
        match value {
            StatusError::InvalidArgument => Self::InThePast,
            StatusError::Range => Self::TooCloseToExisting,
            StatusError::OutOfResources => Self::MaximumWakeupsReached,
            _ => Self::Internal,
        }
    }
}

impl Wakeup {
    pub(crate) const fn new() -> Self {
        Self
    }

    /// Schedule a wakeup at the specified time for the specified reason.
    /// Every app can schedule up to 8 wakeup events ([`WakeupSchedulingError::MaximumWakeupsReached`]).
    /// Also, you cannot schedule a wakeup event within 1 minute of another ([`WakeupSchedulingError::TooCloseToExisting`]).
    pub fn schedule(
        &self,
        time: Time,
        reason: i32,
    ) -> Result<PendingWakeup, WakeupSchedulingError> {
        let res = unsafe { sys::wakeup_schedule(time.epoch_seconds(), reason, false) };
        parse_status_result(res.min(0))?;
        Ok(PendingWakeup { time, id: res })
    }

    /// Cancel all wakeup events.
    pub fn cancel_all(&self) {
        unsafe { sys::wakeup_cancel_all() };
    }

    /// Return the event that launched the app, if the app was launched by a wakeup event.
    pub fn get_launch_event(&self) -> Option<WakeupEvent> {
        let mut id = 0;
        let mut reason = 0;
        if unsafe { sys::wakeup_get_launch_event(&mut id, &mut reason) } {
            Some(WakeupEvent { reason, id })
        } else {
            None
        }
    }
    /// Sets or overrides a handler for wakeup events.
    /// When the app is woken up, the handler is called with the [`WakeupEvent`].
    pub fn subscribe(&self, handler: Box<dyn FnMut(WakeupEvent)>) {
        HANDLER.set(handler);
        unsafe {
            sys::wakeup_service_subscribe(Some(global_wakeup_handler));
        }
    }

    /// Removes the handler for wakeup events.
    pub fn unsubscribe(&self) {
        unsafe { sys::wakeup_service_subscribe(None) }
        HANDLER.clear()
    }
}

extern "C" fn global_wakeup_handler(id: sys::WakeupId, reason: i32) {
    let event = WakeupEvent { reason, id };
    HANDLER.dispatch(event);
}

/// A wakeup event.
pub struct WakeupEvent {
    /// The raw wakeup reason.
    /// This is the value that was specified in the [`Wakeup::schedule`] function.
    pub reason: i32,
    /// Which wakeup triggered this event.
    pub id: WakeupId,
}

/// A pending wakeup that was scheduled by [`Wakeup::schedule`].
/// You can drop this structure and the event will still occur, but you won’t be able to cancel it.
pub struct PendingWakeup {
    /// At which time the wakeup should occur.
    pub time: Time,
    /// The wakeup ID.
    id: WakeupId,
}

impl PendingWakeup {
    /// Cancel the wakeup event.
    pub fn cancel(self) {
        unsafe { sys::wakeup_cancel(self.id) };
    }

    /// Check whether the wakeup is still scheduled.
    pub fn is_still_scheduled(&self) -> bool {
        unsafe { sys::wakeup_query(self.id, core::ptr::null_mut()) }
    }
}
