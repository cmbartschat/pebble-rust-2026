use alloc::boxed::Box;

use crate::{
    Angle,
    service::global_callback::GlobalCallback,
    sys::{self},
};

pub struct Compass;

static HANDLER: GlobalCallback<CompassHeading, ()> = GlobalCallback::new();

impl Compass {
    pub const fn new() -> Self {
        Self
    }

    pub fn set_filter(&self, filter: Angle) {
        unsafe {
            sys::compass_service_set_heading_filter(filter.value);
        }
    }

    pub fn subscribe(&self, handler: Box<dyn FnMut(CompassHeading)>) {
        HANDLER.set(handler);
        unsafe {
            sys::compass_service_subscribe(Some(global_compass_handler));
        }
    }

    pub fn unsubscribe(&self) {
        unsafe { sys::compass_service_unsubscribe() }
        HANDLER.clear()
    }

    pub fn peek(&self) -> CompassHeading {
        let mut ptr = Box::<sys::CompassHeadingData>::new(sys::CompassHeadingData {
            magnetic_heading: 0,
            true_heading: 0,
            compass_status: sys::CompassStatus_CompassStatusUnavailable,
            is_declination_valid: false,
        });
        unsafe { sys::compass_service_peek(ptr.as_mut()) };
        CompassHeading::from(ptr.as_ref())
    }
}

extern "C" fn global_compass_handler(event: sys::CompassHeadingData) {
    let event = CompassHeading::from(&event);
    HANDLER.dispatch(event);
}

pub enum CompassHeading {
    Unavailable,
    Invalid,
    Calibrating(Angle),
    Calibrated(Angle),
}

impl From<&sys::CompassHeadingData> for CompassHeading {
    fn from(value: &sys::CompassHeadingData) -> Self {
        match value.compass_status {
            sys::CompassStatus_CompassStatusCalibrated => Self::Calibrated(Angle {
                value: value.magnetic_heading,
            }),
            sys::CompassStatus_CompassStatusCalibrating => Self::Calibrating(Angle {
                value: value.magnetic_heading,
            }),
            sys::CompassStatus_CompassStatusUnavailable => Self::Unavailable,
            _ => Self::Invalid,
        }
    }
}
