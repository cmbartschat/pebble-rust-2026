use crate::bitmap::Bitmap;
use crate::sys;
use crate::sys::{GColor, GPoint, GRect};

pub struct GContext {
    inner: *mut sys::GContext,
}

pub struct NullContextError;

impl GContext {
    pub fn from_raw(inner: *mut sys::GContext) -> Result<Self, NullContextError> {
        if inner.is_null() {
            return Err(NullContextError);
        }
        Ok(Self { inner })
    }

    pub fn draw_pixel(&mut self, point: GPoint) {
        unsafe { sys::graphics_draw_pixel(self.inner, point) };
    }

    pub fn draw_line(&mut self, point1: GPoint, point2: GPoint) {
        unsafe { sys::graphics_draw_line(self.inner, point1, point2) };
    }

    pub fn draw_rect(&mut self, rect: GRect) {
        unsafe { sys::graphics_draw_rect(self.inner, rect) };
    }

    pub fn draw_round_rect(&mut self, rect: GRect, radius: u16) {
        unsafe { sys::graphics_draw_round_rect(self.inner, rect, radius) };
    }

    pub fn fill_rect(&mut self, rect: GRect) {
        unsafe { sys::graphics_fill_rect(self.inner, rect, 0, 0) };
    }

    pub fn fill_round_rect(&mut self, rect: GRect, radius: u16) {
        unsafe { sys::graphics_fill_rect(self.inner, rect, radius, sys::GCornerMask_GCornersAll) };
    }

    pub fn fill_selective_round_rect(&mut self, rect: GRect, radius: u16, mask: sys::GCornerMask) {
        unsafe { sys::graphics_fill_rect(self.inner, rect, radius, mask) };
    }

    pub fn draw_circle(&mut self, point: GPoint, radius: u16) {
        unsafe { sys::graphics_draw_circle(self.inner, point, radius) };
    }

    pub fn fill_circle(&mut self, point: GPoint, radius: u16) {
        unsafe { sys::graphics_fill_circle(self.inner, point, radius) };
    }

    pub fn draw_bitmap(&mut self, bitmap: &Bitmap, bounds: GRect) {
        unsafe {
            sys::graphics_draw_bitmap_in_rect(
                self.inner,
                bitmap.handle.borrow().raw.as_ptr(),
                bounds,
            )
        };
    }

    pub fn set_stroke_color(&mut self, color: GColor) {
        unsafe { sys::graphics_context_set_stroke_color(self.inner, color) };
    }

    pub fn set_fill_color(&mut self, color: GColor) {
        unsafe { sys::graphics_context_set_fill_color(self.inner, color) };
    }

    pub fn set_text_color(&mut self, color: GColor) {
        unsafe { sys::graphics_context_set_text_color(self.inner, color) };
    }

    pub fn set_compositing_mode(&mut self, mode: sys::GCompOp) {
        unsafe { sys::graphics_context_set_compositing_mode(self.inner, mode) };
    }

    pub fn draw_rotated_bitmap(
        &mut self,
        bitmap: &Bitmap,
        source_center: GPoint,
        rotation: i32, // TODO(christoph): Create angle class
        destination_center: GPoint,
    ) {
        unsafe {
            sys::graphics_draw_rotated_bitmap(
                self.inner,
                bitmap.handle.borrow().raw.as_ptr(),
                source_center,
                rotation,
                destination_center,
            )
        };
    }

    pub fn set_antialiased(&mut self, enabled: bool) {
        unsafe { sys::graphics_context_set_antialiased(self.inner, enabled) };
    }

    pub fn set_stroke_width(&mut self, width: u8) {
        unsafe { sys::graphics_context_set_stroke_width(self.inner, width) };
    }
}
