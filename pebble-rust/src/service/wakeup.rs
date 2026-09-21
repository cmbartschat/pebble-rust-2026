use alloc::boxed::Box;

use crate::{
    Time,
    service::global_callback::GlobalCallback,
    status_code::{StatusError, parse_status_result},
    sys,
};

pub struct Wakeup;

static HANDLER: GlobalCallback<WakeupEvent, ()> = GlobalCallback::new();

impl Wakeup {
    pub const fn new() -> Self {
        Self
    }

    pub fn schedule(&self, time: Time, reason: i32) -> Result<(), StatusError> {
        let res = unsafe { sys::wakeup_schedule(time.epoch_seconds(), reason, false) };
        parse_status_result(res.min(0) as i8).map(|_| ())
    }

    pub fn cancel(&self, wakeup_id: i32) {
        unsafe { sys::wakeup_cancel(wakeup_id) };
    }

    pub fn cancel_all(&self) {
        unsafe { sys::wakeup_cancel_all() };
    }

    pub fn get_wakeup(&self, wakeup_id: i32) -> Option<PendingWakeup> {
        let mut timestamp: i32 = 0;
        if unsafe { sys::wakeup_query(wakeup_id, &mut timestamp) } {
            Some(PendingWakeup {
                time: Time::from_epoch_seconds(timestamp),
            })
        } else {
            None
        }
    }

    pub fn get_launch_event(&self) -> Option<WakeupEvent> {
        let mut id = 0;
        let mut reason = 0;
        if unsafe { sys::wakeup_get_launch_event(&mut id, &mut reason) } {
            Some(WakeupEvent { reason })
        } else {
            None
        }
    }
    pub fn subscribe(&self, handler: Box<dyn FnMut(WakeupEvent)>) {
        HANDLER.set(handler);
        unsafe {
            sys::wakeup_service_subscribe(Some(global_wakeup_handler));
        }
    }

    pub fn unsubscribe(&self) {
        unsafe { sys::wakeup_service_subscribe(None) }
        HANDLER.clear()
    }
}

extern "C" fn global_wakeup_handler(_id: sys::WakeupId, reason: i32) {
    let event = WakeupEvent { reason };
    HANDLER.dispatch(event);
}

pub struct WakeupEvent {
    pub reason: i32,
}

pub struct PendingWakeup {
    pub time: Time,
}
