use crate::{Angle, GAlign, GEdgeInsets, GPoint, GRect, GSize};

use crate::sys;

impl GRect {
    pub fn new(x: i16, y: i16, w: i16, h: i16) -> Self {
        Self {
            origin: GPoint { x, y },
            size: GSize { w, h },
        }
    }

    pub fn new_on_circle(bounds: GRect, angle: Angle, size: GSize) -> Self {
        unsafe {
            sys::grect_centered_from_polar(
                bounds,
                sys::GOvalScaleMode_GOvalScaleModeFitCircle,
                angle.value,
                size,
            )
        }
    }

    pub fn new_on_oval(bounds: GRect, angle: Angle, size: GSize) -> Self {
        unsafe {
            sys::grect_centered_from_polar(
                bounds,
                sys::GOvalScaleMode_GOvalScaleModeFillCircle,
                angle.value,
                size,
            )
        }
    }

    #[must_use]
    pub fn inset(self, insets: GEdgeInsets) -> Self {
        unsafe { sys::grect_inset(self, insets) }
    }

    #[must_use]
    pub fn shrink(self, amount: i32) -> Self {
        unsafe { sys::grect_crop(self, amount) }
    }

    #[must_use]
    pub fn expand(self, amount: i32) -> Self {
        self.shrink(-amount)
    }

    #[must_use]
    pub fn center_point(&self) -> GPoint {
        unsafe { sys::grect_center_point(self) }
    }

    #[must_use]
    pub fn clip(mut self, clipper: &GRect) -> Self {
        unsafe { sys::grect_clip(&mut self, clipper) };
        self
    }

    #[must_use]
    pub fn align(mut self, container: &GRect, align: GAlign) -> Self {
        unsafe { sys::grect_align(&mut self, container, align as sys::GAlign, false) };
        self
    }

    #[must_use]
    pub fn clip_align(mut self, container: &GRect, align: GAlign) -> Self {
        unsafe { sys::grect_align(&mut self, container, align as sys::GAlign, true) };
        self
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        unsafe { sys::grect_is_empty(self) }
    }

    #[must_use]
    pub fn standardize(mut self) -> Self {
        unsafe { sys::grect_standardize(&mut self) }
        self
    }

    #[must_use]
    pub fn contains_point(&self, p: GPoint) -> bool {
        unsafe { sys::grect_contains_point(self, &p) }
    }
}

impl PartialEq for GRect {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::grect_equal(self, other) }
    }
}
