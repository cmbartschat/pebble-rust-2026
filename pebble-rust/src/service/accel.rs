use core::slice;

use alloc::boxed::Box;

use crate::{log_c_str, log_fmt, service::global_callback::GlobalCallback, sys};

pub struct Accel;

#[derive(Copy, Clone, PartialEq, Hash)]
pub enum AccelAxis {
    PosX,
    PosY,
    PosZ,
    NegX,
    NegY,
    NegZ,
}

pub type AccelData = sys::AccelData;
pub type AccelRawData = sys::AccelRawData;

#[repr(u8)]
pub enum AccelSamplingRate {
    Hz10 = sys::AccelSamplingRate_ACCEL_SAMPLING_10HZ,
    Hz25 = sys::AccelSamplingRate_ACCEL_SAMPLING_25HZ,
    Hz50 = sys::AccelSamplingRate_ACCEL_SAMPLING_50HZ,
    Hz100 = sys::AccelSamplingRate_ACCEL_SAMPLING_100HZ,
}

static TAP_HANDLER: GlobalCallback<AccelAxis, ()> = GlobalCallback::new();
static DATA_HANDLER: GlobalCallback<&[sys::AccelData], ()> = GlobalCallback::new();
static RAW_HANDLER: GlobalCallback<&sys::AccelRawData, ()> = GlobalCallback::new();

type AccelHandler = Box<dyn FnMut(&[sys::AccelData])>;

impl Accel {
    pub const fn new() -> Self {
        Self
    }

    pub fn peek(&self) -> Option<sys::AccelData> {
        let mut data = sys::AccelData {
            x: 0,
            y: 0,
            z: 0,
            did_vibrate: false,
            timestamp: 0,
        };
        let res = unsafe { sys::accel_service_peek(&mut data) };
        if res == 0 { Some(data) } else { None }
    }

    pub fn set_sampling_rate(&self, rate: AccelSamplingRate) {
        unsafe { sys::accel_service_set_sampling_rate(rate as u8) };
    }

    pub fn set_samples_per_update(&self, mut rate: u32) {
        if rate > 25 {
            log_c_str(c"Unexpected: set_samples_per_update should be 0-25");
            rate = 25;
        }
        unsafe { sys::accel_service_set_samples_per_update(rate) };
    }

    pub fn subscribe(&self, samples_per_update: u32, handler: AccelHandler) {
        DATA_HANDLER.set(handler);
        unsafe {
            sys::accel_data_service_subscribe(samples_per_update, Some(global_accel_data_handler));
        }
    }

    pub fn unsubscribe(&self) {
        unsafe { sys::accel_data_service_unsubscribe() };
        DATA_HANDLER.clear()
    }

    pub fn tap_subscribe(&self, handler: Box<dyn FnMut(AccelAxis)>) {
        TAP_HANDLER.set(handler);
        unsafe {
            sys::accel_tap_service_subscribe(Some(global_accel_tap_handler));
        }
    }

    pub fn tap_unsubscribe(&self) {
        unsafe { sys::accel_tap_service_unsubscribe() };
        TAP_HANDLER.clear()
    }

    pub fn raw_subscribe(
        &self,
        samples_per_update: u32,
        handler: Box<dyn FnMut(&sys::AccelRawData)>,
    ) {
        RAW_HANDLER.set(handler);
        unsafe {
            sys::accel_raw_data_service_subscribe(
                samples_per_update,
                Some(global_accel_raw_data_handler),
            );
        }
    }
}

extern "C" fn global_accel_data_handler(data: *mut sys::AccelData, num_samples: u32) {
    let slice = unsafe { slice::from_raw_parts(data, num_samples as usize) };
    DATA_HANDLER.dispatch(slice);
}

extern "C" fn global_accel_tap_handler(axis: sys::AccelAxisType, direction: i32) {
    unsafe { log_fmt!(c"tap info: %d, %i", axis as u32, direction) };

    let axis = match (axis, direction) {
        (sys::AccelAxisType_ACCEL_AXIS_X, -1) => AccelAxis::NegX,
        (sys::AccelAxisType_ACCEL_AXIS_X, 1) => AccelAxis::PosX,
        (sys::AccelAxisType_ACCEL_AXIS_Y, -1) => AccelAxis::NegY,
        (sys::AccelAxisType_ACCEL_AXIS_Y, 1) => AccelAxis::PosY,
        (sys::AccelAxisType_ACCEL_AXIS_Z, -1) => AccelAxis::NegZ,
        (sys::AccelAxisType_ACCEL_AXIS_Z, 1) => AccelAxis::PosZ,
        _ => {
            return;
        }
    };

    TAP_HANDLER.dispatch(axis);
    todo!()
}

extern "C" fn global_accel_raw_data_handler(
    data: *mut sys::AccelRawData,
    _something: u32,
    _timestamp_ms: u64,
) {
    unsafe {
        RAW_HANDLER.dispatch(data.as_ref().unwrap());
    }
}
