use core::{cell::RefCell, slice};

use alloc::boxed::Box;
use critical_section::Mutex;

use crate::{log_c_str, service::global_callback::GlobalCallback, sys};

/// Accessor for the accelerometer data.
/// See the member functions for details.
pub struct Accelerometer {
    samples_per_update: Mutex<RefCell<u32>>,
}

/// Specifies one of the three possible accelerometer axes, both in positive and negative directions.
/// As specified in the [Pebble documentation](https://developer.repebble.com/docs/c/Foundation/Event_Service/AccelerometerService/#AccelerometerRawData):
/// - X is towards the right of the watch.
/// - Y is towards the top of the watch.
/// - Z is vertically out of the watch screen.
#[derive(Copy, Clone, PartialEq, Hash)]
pub enum AccelerometerAxis {
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
pub type AccelerometerData = sys::AccelData;
/// Simple accelerometer data, only including values for the three accelerometer axes.
pub type AccelerometerRawData = sys::AccelRawData;

/// The possible accelerometer data sampling rates.
/// All sampling rates are in Hz, or samples per second.
#[derive(Clone, Copy, PartialEq, Hash)]
#[repr(u8)]
#[non_exhaustive]
pub enum AccelerometerSamplingRate {
    /// 10 Hz.
    Hz10 = sys::AccelSamplingRate_ACCEL_SAMPLING_10HZ,
    /// 25 Hz.
    Hz25 = sys::AccelSamplingRate_ACCEL_SAMPLING_25HZ,
    /// 50 Hz.
    Hz50 = sys::AccelSamplingRate_ACCEL_SAMPLING_50HZ,
    /// 100 Hz.
    Hz100 = sys::AccelSamplingRate_ACCEL_SAMPLING_100HZ,
}

static TAP_HANDLER: GlobalCallback<AccelerometerAxis, ()> = GlobalCallback::new();
static DATA_HANDLER: GlobalCallback<&[AccelerometerData], ()> = GlobalCallback::new();
static RAW_HANDLER: GlobalCallback<&AccelerometerRawData, ()> = GlobalCallback::new();

/// The handler for accelerometer events.
/// This is a function that takes in a slice of [`AccelerometerData`].
pub type AccelerometerHandler = Box<dyn FnMut(&[AccelerometerData])>;

impl Accelerometer {
    pub(crate) const fn new() -> Self {
        Self {
            samples_per_update: Mutex::new(RefCell::new(1)),
        }
    }

    /// Retrieve the current accelerometer data.
    /// Returns None if the current data cannot be retrieved,
    /// either when the accelerometer is not running, or when you are subscribed to accelerometer events.
    /// Since you cannot unsubscribe from raw events, this function becomes unavailable once
    pub fn peek(&self) -> Option<AccelerometerData> {
        let mut data = AccelerometerData {
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
    pub fn set_sampling_rate(&self, rate: AccelerometerSamplingRate) {
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

    /// Subscribe to accelerometer events.
    /// To configure the sample rate, or the number of samples per update, use [`Self::set_sampling_rate`] and [`Self::set_samples_per_update`], which is also possible after the handler has been already set.
    /// This overrides any previous handler that is subscribed to these events.
    pub fn subscribe(&self, handler: AccelerometerHandler) {
        DATA_HANDLER.set(handler);
        unsafe {
            sys::accel_data_service_subscribe(
                self.get_samples_per_update(),
                Some(global_accel_data_handler),
            );
        }
    }

    /// Unsubscribe from accelerometer events.
    pub fn unsubscribe(&self) {
        unsafe { sys::accel_data_service_unsubscribe() };
        DATA_HANDLER.clear()
    }

    /// Subscribe to tap events.
    /// These are emitted whenever the watch is tapped or shaken along an axis.
    /// The handler is a function that receives the accelerometer axis as its only argument.
    /// This overrides any previous handler that is subscribed to these events.
    pub fn subscribe_to_tap(&self, handler: Box<dyn FnMut(AccelerometerAxis)>) {
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

    /// Subscribe to raw accelerometer events.
    /// These events omit the timestamp and the vibration information.
    /// To configure the sample rate, or the number of samples per update, use [`Self::set_sampling_rate`] and [`Self::set_samples_per_update`], which is also possible after the handler has been already set.
    /// This overrides any previous handler that is subscribed to these events.
    ///
    /// Note: You cannot unsubscribe from raw events, since this functionality is unfortunately not available from the C API.
    pub fn subscribe_to_raw(&self, handler: Box<dyn FnMut(&AccelerometerRawData)>) {
        RAW_HANDLER.set(handler);
        unsafe {
            sys::accel_raw_data_service_subscribe(
                self.get_samples_per_update(),
                Some(global_accel_raw_data_handler),
            );
        }
    }
}

extern "C" fn global_accel_data_handler(data: *mut AccelerometerData, num_samples: u32) {
    let slice = unsafe { slice::from_raw_parts(data, num_samples as usize) };
    DATA_HANDLER.dispatch(slice);
}

extern "C" fn global_accel_tap_handler(axis: sys::AccelAxisType, direction: i32) {
    let axis = match (axis, direction) {
        (sys::AccelAxisType_ACCEL_AXIS_X, -1) => AccelerometerAxis::NegX,
        (sys::AccelAxisType_ACCEL_AXIS_X, 1) => AccelerometerAxis::PosX,
        (sys::AccelAxisType_ACCEL_AXIS_Y, -1) => AccelerometerAxis::NegY,
        (sys::AccelAxisType_ACCEL_AXIS_Y, 1) => AccelerometerAxis::PosY,
        (sys::AccelAxisType_ACCEL_AXIS_Z, -1) => AccelerometerAxis::NegZ,
        (sys::AccelAxisType_ACCEL_AXIS_Z, 1) => AccelerometerAxis::PosZ,
        _ => {
            return;
        }
    };

    TAP_HANDLER.dispatch(axis);
}

extern "C" fn global_accel_raw_data_handler(
    data: *mut AccelerometerRawData,
    _something: u32,
    _timestamp_ms: u64,
) {
    unsafe {
        RAW_HANDLER.dispatch(data.as_ref().unwrap());
    }
}
