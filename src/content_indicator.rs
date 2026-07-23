use core::ptr::{NonNull, null};

use crate::{GAlign, GColor, Layer, color, sys};

pub struct ContentIndicator {
    raw: NonNull<sys::ContentIndicator>,
    owned: bool,
    up_layer: Option<Layer>,
    down_layer: Option<Layer>,
}

impl Drop for ContentIndicator {
    fn drop(&mut self) {
        if self.owned {
            unsafe { sys::content_indicator_destroy(self.raw.as_ptr()) };
        }
    }
}

impl ContentIndicator {
    pub(crate) fn from_ptr(ptr: *mut sys::ContentIndicator, owned: bool) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(ptr)?,
            owned,
            up_layer: None,
            down_layer: None,
        })
    }

    pub fn new() -> Option<Self> {
        let ptr = unsafe { sys::content_indicator_create() };
        Self::from_ptr(ptr, true)
    }

    pub fn get_content_available(&self, direction: ContentIndicatorDirection) -> bool {
        unsafe { sys::content_indicator_get_content_available(self.raw.as_ptr(), direction as u8) }
    }

    pub fn set_content_available(&mut self, direction: ContentIndicatorDirection, available: bool) {
        unsafe {
            sys::content_indicator_set_content_available(
                self.raw.as_ptr(),
                direction as u8,
                available,
            )
        }
    }

    fn save_layer(&mut self, direction: ContentIndicatorDirection, layer: Option<Layer>) {
        let saved_layer = match direction {
            ContentIndicatorDirection::Up => &mut self.up_layer,
            ContentIndicatorDirection::Down => &mut self.down_layer,
        };

        *saved_layer = layer;
    }

    pub fn configure_direction(
        &mut self,
        direction: ContentIndicatorDirection,
        config: ContentIndicatorConfig,
    ) -> Result<(), ConfigConflict> {
        let sys_config = sys::ContentIndicatorConfig {
            layer: config.layer.handle.borrow().raw.as_ptr(),
            times_out: config.times_out,
            alignment: config.alignment as u8,
            colors: sys::ContentIndicatorConfig__bindgen_ty_1 {
                foreground: config.foreground,
                background: config.background,
            },
        };
        unsafe {
            let succeeded = sys::content_indicator_configure_direction(
                self.raw.as_ptr(),
                direction as u8,
                &sys_config,
            );
            if !succeeded {
                return Err(ConfigConflict);
            }
        }

        self.save_layer(direction, Some(config.layer));

        Ok(())
    }

    pub fn reset_direction(&mut self, direction: ContentIndicatorDirection) {
        unsafe {
            sys::content_indicator_configure_direction(self.raw.as_ptr(), direction as u8, null());
        }
        self.save_layer(direction, None);
    }
}

#[derive(Debug)]
pub struct ConfigConflict;

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
#[repr(u8)]
pub enum ContentIndicatorDirection {
    Up = sys::ContentIndicatorDirection_ContentIndicatorDirectionUp,
    Down = sys::ContentIndicatorDirection_ContentIndicatorDirectionDown,
}

pub struct ContentIndicatorConfig {
    pub layer: Layer,
    pub times_out: bool,
    pub alignment: GAlign,
    pub foreground: GColor,
    pub background: GColor,
}

impl ContentIndicatorConfig {
    pub fn basic(layer: Layer) -> Self {
        Self {
            layer,
            times_out: false,
            alignment: GAlign::Center,
            foreground: color::GCOLOR_BLACK,
            background: color::GCOLOR_WHITE,
        }
    }
}
