use core::ops::Div;
use core::ops::Mul;

use crate::*;

impl GSize {
    pub const fn new(w: i16, h: i16) -> Self {
        Self { w, h }
    }

    /// Creates a square size with identical width and height.
    pub const fn square(size: i16) -> Self {
        Self { w: size, h: size }
    }

    /// Const version of `self * rhs`.
    pub const fn multiply(mut self, rhs: i16) -> Self {
        self.w *= rhs;
        self.h *= rhs;
        self
    }

    /// Const version of `self / rhs`.
    pub const fn divide(mut self, rhs: i16) -> Self {
        self.w /= rhs;
        self.h /= rhs;
        self
    }

    /// Convert a size to a point by mapping width to x and height to y.
    pub const fn as_point(self) -> GPoint {
        GPoint {
            x: self.w,
            y: self.h,
        }
    }

    /// Scale the size by the given ratio.
    pub const fn scaled(mut self, ratio: Ratio) -> Self {
        self.w = ratio.scale(self.w as i32) as i16;
        self.h = ratio.scale(self.h as i32) as i16;
        self
    }
}

impl PartialEq for GSize {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::gsize_equal(self, other) }
    }
}

impl Mul<i16> for GSize {
    type Output = Self;
    fn mul(self, rhs: i16) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Div<i16> for GSize {
    type Output = Self;
    fn div(self, rhs: i16) -> Self::Output {
        self.divide(rhs)
    }
}

impl From<GPoint> for GSize {
    fn from(val: GPoint) -> Self {
        GSize { w: val.x, h: val.y }
    }
}

impl From<GSize> for GPoint {
    fn from(val: GSize) -> Self {
        val.as_point()
    }
}
