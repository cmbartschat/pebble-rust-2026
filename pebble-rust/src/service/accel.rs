use core::{cell::RefCell, slice};

use alloc::boxed::Box;
use critical_section::Mutex;

use crate::{log_c_str, service::global_callback::GlobalCallback, sys};

/// Accessor for the acceleration data.
/// See the member functions for details.
pub struct Acceleration {
    samples_per_update: Mutex<RefCell<u32>>,
}

/// Specifies one of the three possible acceleration axes, both in positive and negative directions.
/// As specified in the [Pebble documentation](https://developer.repebble.com/docs/c/Foundation/Event_Service/AccelerometerService/#AccelRawData):
/// - X is towards the right of the watch.
/// - Y is towards the top of the watch.
/// - Z is vertically out of the watch screen.
#[derive(Copy, Clone, PartialEq, Hash)]
pub enum AccelAxis {
    /// Positive X, towards the right of the watch.
    PosX,
    /// Positive Y, vertically out of the watch screen.
    PosY,
    /// Positive Z, towards the top of the watch.
    PosZ,
    /// Negative X, towards the left of the watch.
    NegX,
    /// Positive Y, vertically into the watch screen (or out of the watch underside).
    NegY,
    /// Positive Z, towards the bottom of the watch.
    NegZ,
}

/// An accelerometer sample, including timestamp and vibration rumble status.
pub type TimedAccelerationData = sys::AccelData;
/// Simple acceleration data, only including values for the three acceleration axes.
pub type AccelerationData = sys::AccelRawData;

/// The possible acceleration data sampling rates.
/// All sampling rates are in Hz, or samples per second.
#[derive(Clone, Copy, PartialEq, Hash)]
#[repr(u8)]
#[non_exhaustive]
pub enum AccelSamplingRate {
    /// 10 Hz.
    Hz10 = sys::AccelSamplingRate_ACCEL_SAMPLING_10HZ,
    /// 25 Hz.
    Hz25 = sys::AccelSamplingRate_ACCEL_SAMPLING_25HZ,
    /// 50 Hz.
    Hz50 = sys::AccelSamplingRate_ACCEL_SAMPLING_50HZ,
    /// 100 Hz.
    Hz100 = sys::AccelSamplingRate_ACCEL_SAMPLING_100HZ,
}

static TAP_HANDLER: GlobalCallback<AccelAxis, ()> = GlobalCallback::new();
static DATA_HANDLER: GlobalCallback<&[sys::AccelData], ()> = GlobalCallback::new();
static RAW_HANDLER: GlobalCallback<&sys::AccelRawData, ()> = GlobalCallback::new();

/// The handler for acceleration events.
/// This is a function that takes in a slice of [`TimedAccelerationData`].
pub type AccelerationHandler = Box<dyn FnMut(&[TimedAccelerationData])>;

impl Acceleration {
    pub(crate) const fn new() -> Self {
        Self {
            samples_per_update: Mutex::new(RefCell::new(1)),
        }
    }

    /// Retrieve the current acceleration data.
    /// Returns None if the current data cannot be retrieved,
    /// either when the accelerometer is not running, or when you are subscribed to acceleration events.
    /// Since you cannot unsubscribe from raw events, this function becomes unavailable once
    pub fn peek(&self) -> Option<TimedAccelerationData> {
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

    /// Set the accelerometer sampling rate.
    pub fn set_sampling_rate(&self, rate: AccelSamplingRate) {
        unsafe { sys::accel_service_set_sampling_rate(rate as u8) };
    }

    /// Set the number of accelerometer samples that are buffered before the handler is called with all of them.
    /// More than 25 is not possible.
    pub fn set_samples_per_update(&self, mut samples_per_update: u32) {
        if samples_per_update > 25 {
            log_c_str(c"Unexpected: samples_per_update should be 0-25");
            samples_per_update = 25;
        }
        critical_section::with(|cs| {
            *self.samples_per_update.borrow_ref_mut(cs) = samples_per_update
        });
        // SAFETY: samples_per_update must be <=25, which we guarantee above.
        unsafe { sys::accel_service_set_samples_per_update(samples_per_update) };
    }

    /// Return the number of accelerometer samples that are buffered before the handler is called with all of them.
    pub fn get_samples_per_update(&self) -> u32 {
        critical_section::with(|cs| *self.samples_per_update.borrow_ref(cs))
    }

    /// Subscribe to acceleration events.
    /// To configure the sample rate, or the number of samples per update, use [`Self::set_sampling_rate`] and [`Self::set_samples_per_update`], which is also possible after the handler has been already set.
    /// This overrides any previous handler that is subscribed to these events.
    pub fn subscribe(&self, handler: AccelerationHandler) {
        DATA_HANDLER.set(handler);
        unsafe {
            sys::accel_data_service_subscribe(
                self.get_samples_per_update(),
                Some(global_accel_data_handler),
            );
        }
    }

    /// Unsubscribe from acceleration events.
    pub fn unsubscribe(&self) {
        unsafe { sys::accel_data_service_unsubscribe() };
        DATA_HANDLER.clear()
    }

    /// Subscribe to tap events.
    /// These are emitted whenever the watch is tapped or shaken along an axis.
    /// The handler is a function that receives the acceleration axis as its only argument.
    /// This overrides any previous handler that is subscribed to these events.
    pub fn subscribe_to_tap(&self, handler: Box<dyn FnMut(AccelAxis)>) {
        TAP_HANDLER.set(handler);
        unsafe {
            sys::accel_tap_service_subscribe(Some(global_accel_tap_handler));
        }
    }

    /// Unsubscribe from tap events.
    pub fn unsubscribe_from_tap(&self) {
        unsafe { sys::accel_tap_service_unsubscribe() };
        TAP_HANDLER.clear()
    }

    /// Subscribe to raw acceleration events.
    /// These events omit the timestamp and the vibration information.
    /// To configure the sample rate, or the number of samples per update, use [`Self::set_sampling_rate`] and [`Self::set_samples_per_update`], which is also possible after the handler has been already set.
    /// This overrides any previous handler that is subscribed to these events.
    ///
    /// Note: You cannot unsubscribe from raw events, since this functionality is unfortunately not available from the C API.
    pub fn subscribe_to_raw(&self, handler: Box<dyn FnMut(&AccelerationData)>) {
        RAW_HANDLER.set(handler);
        unsafe {
            sys::accel_raw_data_service_subscribe(
                self.get_samples_per_update(),
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
