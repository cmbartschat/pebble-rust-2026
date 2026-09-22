use alloc::boxed::Box;

use crate::{Angle, log_c_str, service::global_callback::GlobalCallback, sys};

/// Accessor for compass data.
pub struct Compass(());

static HANDLER: GlobalCallback<CompassHeading, ()> = GlobalCallback::new();

impl Compass {
    pub(crate) const fn new() -> Self {
        Self(())
    }

    /// Sets the minimum angular change required to generate new compass heading events.
    /// The angular distance is measured relative to the last delivered heading event.
    /// Use 0 to be notified of all movements.
    /// Values over 180° are not valid.
    /// The default is 1°.
    pub fn set_minimum_angle_change(&self, mut minimum_angle_change: Angle) {
        minimum_angle_change.normalize();
        if minimum_angle_change > Angle::from_degrees(180) {
            log_c_str(c"Unexpected: minimum angle change should not be over 180 degrees");
            minimum_angle_change = Angle::from_degrees(180);
        }
        unsafe {
            sys::compass_service_set_heading_filter(minimum_angle_change.value);
        }
    }

    /// Subscribe to compass events.
    /// The function receives the current compass heading, see [`CompassHeading`] for details.
    /// This overrides any previous handler that is subscribed to these events.
    pub fn subscribe(&self, handler: Box<dyn FnMut(CompassHeading)>) {
        HANDLER.set(handler);
        unsafe {
            sys::compass_service_subscribe(Some(global_compass_handler));
        }
    }

    /// Unsubscribe from compass events.
    pub fn unsubscribe(&self) {
        unsafe { sys::compass_service_unsubscribe() }
        HANDLER.clear()
    }

    /// Retrieve the current compass heading.
    pub fn peek(&self) -> CompassHeading {
        let mut ptr = sys::CompassHeadingData {
            magnetic_heading: 0,
            true_heading: 0,
            compass_status: sys::CompassStatus_CompassStatusUnavailable,
            is_declination_valid: false,
        };
        unsafe { sys::compass_service_peek(&raw mut ptr) };
        ptr.into()
    }
}

extern "C" fn global_compass_handler(event: sys::CompassHeadingData) {
    let event = CompassHeading::from(event);
    HANDLER.dispatch(event);
}

/// Possible compass data.
// TODO: Add Into<Option<Angle>>
pub enum CompassHeading {
    /// Compass heading is unavailable.
    Unavailable,
    /// Compass heading is invalid.
    Invalid,
    /// Compass heading is being calibrated, but there is a valid angle.
    Calibrating(Angle),
    /// Compass heading is fully calibrated.
    Calibrated(Angle),
}

impl From<sys::CompassHeadingData> for CompassHeading {
    fn from(value: sys::CompassHeadingData) -> Self {
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
