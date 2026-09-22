use core::ffi::{CStr, c_int};
use core::ptr::NonNull;

use crate::bitmap::Bitmap;
use crate::{Angle, TextAlignment, TextAttributes, sys};
use crate::{GColor, GPoint, GRect};

/// A graphics drawing context, to use graphics functionality on any layer.
pub struct GContext {
    raw: NonNull<sys::GContext>,
}

impl GContext {
    pub(crate) fn from_raw(inner: *mut sys::GContext) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(inner)?,
        })
    }

    const fn as_ptr_mut(&mut self) -> *mut sys::GContext {
        self.raw.as_ptr()
    }

    /// Draw a single pixel at the given point.
    pub fn draw_pixel(&mut self, point: GPoint) {
        unsafe { sys::graphics_draw_pixel(self.as_ptr_mut(), point) };
    }

    /// Draw a line between the two points.
    pub fn draw_line(&mut self, point1: GPoint, point2: GPoint) {
        unsafe { sys::graphics_draw_line(self.as_ptr_mut(), point1, point2) };
    }

    /// Draw the outline of the rectangle.
    pub fn draw_rect(&mut self, rect: GRect) {
        unsafe { sys::graphics_draw_rect(self.as_ptr_mut(), rect) };
    }

    /// Draw the outline of the rectangle with rounded corners.
    pub fn draw_round_rect(&mut self, rect: GRect, radius: u16) {
        unsafe { sys::graphics_draw_round_rect(self.as_ptr_mut(), rect, radius) };
    }

    /// Fill the rectangle.
    pub fn fill_rect(&mut self, rect: GRect) {
        unsafe { sys::graphics_fill_rect(self.as_ptr_mut(), rect, 0, 0) };
    }

    /// Fill the rectangle with rounded corners.
    pub fn fill_round_rect(&mut self, rect: GRect, radius: u16) {
        unsafe {
            sys::graphics_fill_rect(
                self.as_ptr_mut(),
                rect,
                radius,
                sys::GCornerMask_GCornersAll,
            )
        };
    }

    /// Fill the rectangle with some round and some square corners, as per the given [`CornerMask`].
    pub fn fill_selective_round_rect(&mut self, rect: GRect, radius: u16, mask: CornerMask) {
        unsafe { sys::graphics_fill_rect(self.as_ptr_mut(), rect, radius, mask.bits()) };
    }

    /// Draw the outline of a circle.
    pub fn draw_circle(&mut self, point: GPoint, radius: u16) {
        unsafe { sys::graphics_draw_circle(self.as_ptr_mut(), point, radius) };
    }

    /// Fill a circle.
    pub fn fill_circle(&mut self, point: GPoint, radius: u16) {
        unsafe { sys::graphics_fill_circle(self.as_ptr_mut(), point, radius) };
    }

    /// Draw a given bitmap on the given bounds.
    pub fn draw_bitmap(&mut self, bitmap: &Bitmap, bounds: GRect) {
        unsafe {
            sys::graphics_draw_bitmap_in_rect(
                self.as_ptr_mut(),
                bitmap.handle.borrow().raw.as_ptr(),
                bounds,
            )
        };
    }

    /// Draw text.
    /// The text is drawn within the given bounds, and aligned there to the given [`TextAlignment`].
    /// Further draw options can be specified with the [`TextAttributes`].
    pub fn draw_text(
        &mut self,
        text: &CStr,
        bounds: GRect,
        alignment: TextAlignment,
        attributes: &TextAttributes,
    ) {
        unsafe {
            sys::graphics_draw_text(
                self.as_ptr_mut(),
                text.as_ptr(),
                attributes.font.handle.borrow().raw.as_ptr(),
                bounds,
                attributes.overflow as u8,
                alignment as sys::GTextAlignment,
                attributes.get_raw(),
            );
        };
    }

    /// Set the color for all line, pixel, and outline drawing operations.
    pub fn set_stroke_color(&mut self, color: GColor) {
        unsafe { sys::graphics_context_set_stroke_color(self.as_ptr_mut(), color) };
    }

    /// Set the color for all fill operations.
    pub fn set_fill_color(&mut self, color: GColor) {
        unsafe { sys::graphics_context_set_fill_color(self.as_ptr_mut(), color) };
    }

    /// Set the text color.
    pub fn set_text_color(&mut self, color: GColor) {
        unsafe { sys::graphics_context_set_text_color(self.as_ptr_mut(), color) };
    }

    /// Set the compositing mode for all operations, see [`CompOp`].
    pub fn set_compositing_mode(&mut self, mode: CompOp) {
        unsafe {
            sys::graphics_context_set_compositing_mode(self.as_ptr_mut(), mode as sys::GCompOp)
        };
    }

    /// Draw a rotated bitmap.
    /// The bitmap is rotated around the `source_center` within its local coordinate system, and around the `destination_center` on the canvas.
    pub fn draw_rotated_bitmap(
        &mut self,
        bitmap: &Bitmap,
        source_center: GPoint,
        rotation: Angle,
        destination_center: GPoint,
    ) {
        unsafe {
            sys::graphics_draw_rotated_bitmap(
                self.as_ptr_mut(),
                bitmap.handle.borrow().raw.as_ptr(),
                source_center,
                rotation.value as c_int,
                destination_center,
            )
        };
    }

    /// Enable/disable antialiased drawing.
    pub fn set_antialiased(&mut self, enabled: bool) {
        unsafe { sys::graphics_context_set_antialiased(self.as_ptr_mut(), enabled) };
    }

    /// Set the width of all stroke operations.
    pub fn set_stroke_width(&mut self, width: u8) {
        unsafe { sys::graphics_context_set_stroke_width(self.as_ptr_mut(), width) };
    }

    /// Draw a circular arc within the given boundaries (smaller size is respected), from start to end angle.
    pub fn draw_arc(&mut self, bounds: GRect, start: Angle, end: Angle) {
        unsafe {
            sys::graphics_draw_arc(
                self.as_ptr_mut(),
                bounds,
                sys::GOvalScaleMode_GOvalScaleModeFitCircle,
                start.value,
                end.value,
            )
        };
    }

    /// Draw an oval arc within the given boundaries from start to end angle.
    pub fn draw_stretched_arc(&mut self, bounds: GRect, start: Angle, end: Angle) {
        unsafe {
            sys::graphics_draw_arc(
                self.as_ptr_mut(),
                bounds,
                sys::GOvalScaleMode_GOvalScaleModeFillCircle,
                start.value,
                end.value,
            )
        };
    }

    /// Fills part of a circle clockwise between the start and end angle.
    /// The smaller size of the bounds are respected.
    /// start must be greater than end.
    /// `inset_thickness` offsets the outer boundaries of the radial from the circle outline.
    pub fn fill_radial(&mut self, bounds: GRect, inset_thickness: u16, start: Angle, end: Angle) {
        unsafe {
            sys::graphics_fill_radial(
                self.as_ptr_mut(),
                bounds,
                sys::GOvalScaleMode_GOvalScaleModeFitCircle,
                inset_thickness,
                start.value,
                end.value,
            )
        };
    }

    /// Fills part of an oval clockwise between the start and end angle.
    /// start must be greater than end.
    /// `inset_thickness` offsets the outer boundaries of the radial from the oval outline.
    pub fn fill_stretched_radial(
        &mut self,
        bounds: GRect,
        thickness: u16,
        start: Angle,
        end: Angle,
    ) {
        unsafe {
            sys::graphics_fill_radial(
                self.as_ptr_mut(),
                bounds,
                sys::GOvalScaleMode_GOvalScaleModeFillCircle,
                thickness,
                start.value,
                end.value,
            )
        };
    }
}

/// Compositing modes.
#[repr(u8)]
pub enum CompOp {
    /// Overwrite all destination pixels with the source pixels.
    Assign = sys::GCompOp_GCompOpAssign,
    /// Overwrite destination pixels with the inverse of each source pixel’s color.
    AssignInverted = sys::GCompOp_GCompOpAssignInverted,
    /// Perform logical or between destination and source pixels.
    Or = sys::GCompOp_GCompOpOr,
    /// Perform logical and between destination and source pixels.
    And = sys::GCompOp_GCompOpAnd,
    /// Clears the bits in the destination image, using the source image as mask.
    /// The visual result of this compositing mode is that for the parts where the source image is
    /// white, the destination image will be painted black. Other parts will be left untouched.
    ///
    /// For bitmaps that are not 1-bit bitmaps, this mode is not supported
    /// and the resulting behavior is undefined.
    Clear = sys::GCompOp_GCompOpClear,
    /// Sets the bits in the destination image, using the source image as mask.
    /// This mode is required to apply any transparency of your bitmap.
    ///
    /// For 1-bit bitmaps, the visual result of this compositing
    /// mode is that for the parts where the source image is black, the destination image will be
    /// painted white. Other parts will be left untouched.
    Set = sys::GCompOp_GCompOpSet,
}

bitflags::bitflags! {
    /// Which corners to round in [`GContext::fill_selective_round_rect`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CornerMask: u8 {
        /// No corners rounded.
        const None = sys::GCornerMask_GCornerNone;
        /// Top left rounded.
        const TopLeft = sys::GCornerMask_GCornerTopLeft;
        /// Top right rounded.
        const TopRight = sys::GCornerMask_GCornerTopRight;
        /// Bottom left rounded.
        const BottomLeft = sys::GCornerMask_GCornerBottomLeft;
        /// Bottom right rounded.
        const BottomRight = sys::GCornerMask_GCornerBottomRight;
        /// All corners rounded.
        const All = sys::GCornerMask_GCornersAll;
        /// Top corners rounded.
        const Top = sys::GCornerMask_GCornersTop;
        /// Bottom corners rounded.
        const Bottom = sys::GCornerMask_GCornersBottom;
        /// Left corners rounded.
        const Left = sys::GCornerMask_GCornersLeft;
        /// Right corners rounded.
        const Right = sys::GCornerMask_GCornersRight;
    }
}
