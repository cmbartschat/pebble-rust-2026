use crate::{Angle, GPoint, GRect};

use crate::sys::{
    self, GOvalScaleMode_GOvalScaleModeFillCircle, GOvalScaleMode_GOvalScaleModeFitCircle,
};

impl GPoint {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn new_on_circle(bounds: GRect, angle: Angle) -> Self {
        unsafe {
            sys::gpoint_from_polar(
                bounds,
                GOvalScaleMode_GOvalScaleModeFitCircle,
                angle.value as i32,
            )
        }
    }

    pub fn new_on_oval(bounds: GRect, angle: Angle) -> Self {
        unsafe {
            sys::gpoint_from_polar(
                bounds,
                GOvalScaleMode_GOvalScaleModeFillCircle,
                angle.value as i32,
            )
        }
    }
}

impl PartialEq for GPoint {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::gpoint_equal(self, other) }
    }
}
