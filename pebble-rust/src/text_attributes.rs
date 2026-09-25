use core::ptr::{NonNull, null_mut};

use crate::{Font, GPoint, GRect, sys};

/// Attributes for painting text.
/// See also [`crate::GContext::draw_text`].
pub struct TextAttributes {
    raw: Option<NonNull<sys::GTextAttributes>>,
    pub(crate) overflow: TextOverflowMode,
    pub(crate) font: Font,
}

impl Drop for TextAttributes {
    fn drop(&mut self) {
        if let Some(raw) = self.raw.map(|f| f.as_ptr()) {
            unsafe { sys::graphics_text_attributes_destroy(raw) };
        }
    }
}

impl TextAttributes {
    /// Create new text attributes using the given font as a font.
    pub const fn new(font: Font) -> Self {
        Self {
            raw: None,
            overflow: TextOverflowMode::WordWrap,
            font,
        }
    }

    pub(crate) const unsafe fn get_raw(&self) -> *mut sys::GTextAttributes {
        match self.raw {
            Some(e) => e.as_ptr(),
            None => null_mut(),
        }
    }

    fn get_raw_mut(&mut self) -> *mut sys::GTextAttributes {
        match self.raw {
            Some(e) => e.as_ptr(),
            None => {
                let raw = NonNull::new(unsafe { sys::graphics_text_attributes_create() }).unwrap();
                self.raw = Some(raw);
                raw.as_ptr()
            }
        }
    }

    /// Set how the text should behave on overflow, see [`TextOverflowMode`].
    pub const fn set_overflow(mut self, mode: TextOverflowMode) -> Self {
        self.overflow = mode;
        self
    }

    /// Enables paging and locks the text flow calculation to a fixed point on the screen.
    /// `content_origin` is the absolute coordinate on the screen where the text content starts before an animation or scrolling takes place.
    /// Usually the frame's origin of a layer in screen coordinates.
    /// `paging_on_screen` is the rectangle in absolute coordinates on the screen that describes where the text appears.
    /// Usually the container's absolute frame in screen coordinates.
    pub fn enable_paging(mut self, content_origin: GPoint, paging_on_screen: GRect) -> Self {
        unsafe {
            sys::graphics_text_attributes_enable_paging(
                self.get_raw_mut(),
                content_origin,
                paging_on_screen,
            )
        };
        self
    }

    /// Disables paging.
    pub fn disable_paging(mut self) -> Self {
        unsafe { sys::graphics_text_attributes_restore_default_paging(self.get_raw_mut()) };
        self
    }

    /// Enables the text to flow along the boundaries of the screen.
    pub fn enable_screen_text_flow(mut self, inset: u8) -> Self {
        unsafe { sys::graphics_text_attributes_enable_screen_text_flow(self.get_raw_mut(), inset) };
        self
    }

    /// Disables the text flow along the screen boundaries.
    pub fn disable_screen_text_flow(mut self) -> Self {
        unsafe { sys::graphics_text_attributes_restore_default_text_flow(self.get_raw_mut()) };
        self
    }
}

/// Behavior for how the text should overflow if it’s too large for the boundaries.
/// In all cases, text is first wrapped at word boundaries.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum TextOverflowMode {
    /// Abbreviate text beyond the last line with a trailing ellipsis (…)
    TrailingEllipsis = sys::GTextOverflowMode_GTextOverflowModeTrailingEllipsis,
    /// Trim spaces and treat newlines as spaces, thereby filling text most efficiently (while still wrapping of course).
    Fill = sys::GTextOverflowMode_GTextOverflowModeFill,
    /// Wrap at word boundaries to next line, and clip the last line..
    WordWrap = sys::GTextOverflowMode_GTextOverflowModeWordWrap,
}

/// How to align text in its bounding box.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum TextAlignment {
    /// Left-aligned.
    Left = sys::GTextAlignment_GTextAlignmentLeft,
    /// Centered.
    Center = sys::GTextAlignment_GTextAlignmentCenter,
    /// Right-aligned.
    Right = sys::GTextAlignment_GTextAlignmentRight,
}
