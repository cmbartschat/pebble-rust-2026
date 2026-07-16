use core::ffi::{CStr, c_int};
use core::ptr::NonNull;

use crate::bitmap::Bitmap;
use crate::{Angle, TextAlignment, TextAttributes, sys};
use crate::{GColor, GPoint, GRect};

pub struct GContext {
    raw: NonNull<sys::GContext>,
}

impl GContext {
    pub fn from_raw(inner: *mut sys::GContext) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(inner)?,
        })
    }

    fn as_ptr_mut(&mut self) -> *mut sys::GContext {
        self.raw.as_ptr()
    }

    pub fn draw_pixel(&mut self, point: GPoint) {
        unsafe { sys::graphics_draw_pixel(self.as_ptr_mut(), point) };
    }

    pub fn draw_line(&mut self, point1: GPoint, point2: GPoint) {
        unsafe { sys::graphics_draw_line(self.as_ptr_mut(), point1, point2) };
    }

    pub fn draw_rect(&mut self, rect: GRect) {
        unsafe { sys::graphics_draw_rect(self.as_ptr_mut(), rect) };
    }

    pub fn draw_round_rect(&mut self, rect: GRect, radius: u16) {
        unsafe { sys::graphics_draw_round_rect(self.as_ptr_mut(), rect, radius) };
    }

    pub fn fill_rect(&mut self, rect: GRect) {
        unsafe { sys::graphics_fill_rect(self.as_ptr_mut(), rect, 0, 0) };
    }

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

    pub fn fill_selective_round_rect(&mut self, rect: GRect, radius: u16, mask: sys::GCornerMask) {
        unsafe { sys::graphics_fill_rect(self.as_ptr_mut(), rect, radius, mask) };
    }

    pub fn draw_circle(&mut self, point: GPoint, radius: u16) {
        unsafe { sys::graphics_draw_circle(self.as_ptr_mut(), point, radius) };
    }

    pub fn fill_circle(&mut self, point: GPoint, radius: u16) {
        unsafe { sys::graphics_fill_circle(self.as_ptr_mut(), point, radius) };
    }

    pub fn draw_bitmap(&mut self, bitmap: &Bitmap, bounds: GRect) {
        unsafe {
            sys::graphics_draw_bitmap_in_rect(
                self.as_ptr_mut(),
                bitmap.handle.borrow().raw.as_ptr(),
                bounds,
            )
        };
    }

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
                attributes.overflow.into(),
                alignment.into(),
                attributes.get_raw(),
            );
        };
    }

    pub fn set_stroke_color(&mut self, color: GColor) {
        unsafe { sys::graphics_context_set_stroke_color(self.as_ptr_mut(), color) };
    }

    pub fn set_fill_color(&mut self, color: GColor) {
        unsafe { sys::graphics_context_set_fill_color(self.as_ptr_mut(), color) };
    }

    pub fn set_text_color(&mut self, color: GColor) {
        unsafe { sys::graphics_context_set_text_color(self.as_ptr_mut(), color) };
    }

    pub fn set_compositing_mode(&mut self, mode: sys::GCompOp) {
        unsafe { sys::graphics_context_set_compositing_mode(self.as_ptr_mut(), mode) };
    }

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

    pub fn set_antialiased(&mut self, enabled: bool) {
        unsafe { sys::graphics_context_set_antialiased(self.as_ptr_mut(), enabled) };
    }

    pub fn set_stroke_width(&mut self, width: u8) {
        unsafe { sys::graphics_context_set_stroke_width(self.as_ptr_mut(), width) };
    }

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

    pub fn fill_radial(&mut self, bounds: GRect, thickness: u16, start: Angle, end: Angle) {
        unsafe {
            sys::graphics_fill_radial(
                self.as_ptr_mut(),
                bounds,
                sys::GOvalScaleMode_GOvalScaleModeFitCircle,
                thickness,
                start.value,
                end.value,
            )
        };
    }

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
