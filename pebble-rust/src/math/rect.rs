use crate::*;

impl GRect {
    /// Creates a new rectangle from the origin point (top left) and the size.
    pub const fn new_from_top_left(origin: GPoint, size: GSize) -> Self {
        Self { origin, size }
    }

    /// Creates a new centered rectangle with the given center point and the size.
    pub const fn new_centered(center: GPoint, size: GSize) -> Self {
        let origin = center.subtract(size.divide(2).as_point());
        Self { origin, size }
    }

    /// Creates a new rectangle with the given x, y, width and height.
    pub const fn new(x: i16, y: i16, w: i16, h: i16) -> Self {
        Self {
            origin: GPoint { x, y },
            size: GSize { w, h },
        }
    }

    /// Create a rectangle that lies on a circle.
    /// The circle is identified by its outer square; the smaller size of the given rectangle is used.
    /// The position on the circle is identified by the angle.
    /// The rectangle is centered on this point, and scaled according to the given size.
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

    /// Create a rectangle that lies on an oval.
    /// The oval is identified by its outer rectangle.
    /// The position on the oval is identified by the angle.
    /// The rectangle is centered on this point, and scaled according to the given size.
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

    /// Insets (reduces the size inward) the rectangle on all four sides by the given insets.
    #[must_use]
    pub fn inset(self, insets: GEdgeInsets) -> Self {
        unsafe { sys::grect_inset(self, insets) }
    }

    /// Shrink all four sides of the rectangle inward by the given amount.
    #[must_use]
    pub fn shrink(self, amount: i32) -> Self {
        unsafe { sys::grect_crop(self, amount) }
    }

    /// Expand all four sides of the rectangle outward by the given amount.
    #[must_use]
    pub fn expand(self, amount: i32) -> Self {
        self.shrink(-amount)
    }

    /// Returns the center of the rectangle.
    #[must_use]
    pub fn center_point(&self) -> GPoint {
        unsafe { sys::grect_center_point(self) }
    }

    /// "Removes" all parts of the rectangle such that it doesn’t exceed the given clipping bounds.
    #[must_use]
    pub fn clip(mut self, clipper: &GRect) -> Self {
        unsafe { sys::grect_clip(&mut self, clipper) };
        self
    }

    /// Aligns the rectangle according to the given alignment in the given container.
    #[must_use]
    pub fn align(mut self, container: &GRect, align: GAlign) -> Self {
        unsafe { sys::grect_align(&mut self, container, align as sys::GAlign, false) };
        self
    }

    /// Aligns the rectangle according to the given alignment in the given container, additionally clipping so that it doesn’t exceed the container boundaries.
    #[must_use]
    pub fn clip_align(mut self, container: &GRect, align: GAlign) -> Self {
        unsafe { sys::grect_align(&mut self, container, align as sys::GAlign, true) };
        self
    }

    /// Returns whether the rectangle is zero-sized in both dimensions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        unsafe { sys::grect_is_empty(self) }
    }

    /// Converts a rectangle's values so that the components of its size (width and/or height) are both positive.
    /// In the width and/or height are negative, the origin will offset, so that the final rectangle overlaps with the original.
    /// For example, a [`GRect`] with size (-10, -5) and origin (20, 20), will be standardized to size (10, 5) and origin (10, 15).
    #[must_use]
    pub fn standardize(mut self) -> Self {
        unsafe { sys::grect_standardize(&mut self) }
        self
    }

    /// Returns whether the given point lies within this rectangle.
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
