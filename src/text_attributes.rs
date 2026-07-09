use core::{
    ffi::c_uint,
    ptr::{NonNull, null_mut},
};

use crate::{Font, GPoint, GRect, sys};

pub struct TextAttributes {
    raw: Option<NonNull<sys::GTextAttributes>>,
    pub(crate) overflow: TextOverflowMode,
    pub(crate) font: Font,
}

impl TextAttributes {
    pub fn new(font: Font) -> Self {
        Self {
            raw: None,
            overflow: TextOverflowMode::WordWrap,
            font,
        }
    }

    pub(crate) unsafe fn get_raw(&self) -> *mut sys::GTextAttributes {
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

    pub fn set_overflow(mut self, mode: TextOverflowMode) -> Self {
        self.overflow = mode;
        self
    }

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

    pub fn disable_paging(mut self) -> Self {
        unsafe { sys::graphics_text_attributes_restore_default_paging(self.get_raw_mut()) };
        self
    }

    pub fn enable_screen_text_flow(mut self, inset: u8) -> Self {
        unsafe { sys::graphics_text_attributes_enable_screen_text_flow(self.get_raw_mut(), inset) };
        self
    }

    pub fn disable_screen_text_flow(mut self) -> Self {
        unsafe { sys::graphics_text_attributes_restore_default_text_flow(self.get_raw_mut()) };
        self
    }
}

#[derive(Clone, Copy)]
pub enum TextOverflowMode {
    TrailingEllipsis,
    Fill,
    WordWrap,
}

impl From<TextOverflowMode> for c_uint {
    fn from(value: TextOverflowMode) -> Self {
        match value {
            TextOverflowMode::TrailingEllipsis => {
                sys::GTextOverflowMode_GTextOverflowModeTrailingEllipsis
            }
            TextOverflowMode::Fill => sys::GTextOverflowMode_GTextOverflowModeFill,
            TextOverflowMode::WordWrap => sys::GTextOverflowMode_GTextOverflowModeWordWrap,
        }
    }
}

#[derive(Clone, Copy)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

impl From<TextAlignment> for c_uint {
    fn from(value: TextAlignment) -> Self {
        match value {
            TextAlignment::Left => sys::GTextAlignment_GTextAlignmentLeft,
            TextAlignment::Center => sys::GTextAlignment_GTextAlignmentCenter,
            TextAlignment::Right => sys::GTextAlignment_GTextAlignmentRight,
        }
    }
}
