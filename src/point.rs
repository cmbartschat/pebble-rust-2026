use core::ops::{Add, Sub};

use crate::{Angle, GPoint, GRect};

use crate::sys;

impl GPoint {
    pub const ORIGIN: Self = Self::new(0, 0);

    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn new_on_circle(bounds: GRect, angle: Angle) -> Self {
        unsafe {
            sys::gpoint_from_polar(
                bounds,
                sys::GOvalScaleMode_GOvalScaleModeFitCircle,
                angle.value,
            )
        }
    }

    pub fn new_on_oval(bounds: GRect, angle: Angle) -> Self {
        unsafe {
            sys::gpoint_from_polar(
                bounds,
                sys::GOvalScaleMode_GOvalScaleModeFillCircle,
                angle.value,
            )
        }
    }

    /// Offsets (adds to) both coordinates of the point by the given offset.
    pub const fn offset_both(mut self, offset: i16) -> Self {
        self.x += offset;
        self.y += offset;
        self
    }

    /// Const version of `self - rhs`.
    pub const fn subtract(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }

    /// Const version of `self + rhs`.
    pub const fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl PartialEq for GPoint {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::gpoint_equal(self, other) }
    }
}

impl Add for GPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl Sub for GPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract(rhs)
    }
}
