use core::ptr::{NonNull, null};

use crate::{GAlign, GColor, Layer, color, sys};

/// Indicator for the content of a [`crate::ScrollLayer`].
/// This type is usually obtained in the callback passed to [`crate::ScrollLayer::with_content_indicator`].
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

    /// Creates a new indicator.
    /// This is not usually necessary, for a scroll layer you should use [`crate::ScrollLayer::with_content_indicator`]
    /// which gives you a prefabricated content indicator.
    pub fn new() -> Option<Self> {
        let ptr = unsafe { sys::content_indicator_create() };
        Self::from_ptr(ptr, true)
    }

    /// Returns whether there is content available in the specified direction.
    pub fn get_content_available(&self, direction: ContentIndicatorDirection) -> bool {
        unsafe { sys::content_indicator_get_content_available(self.raw.as_ptr(), direction as u8) }
    }

    /// Set whether the content indicator should indicate if there is content available in the specified direction.
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

    /// Configures this indicator for the specified direction.
    /// All options are available in [`ContentIndicatorConfig`].
    /// If the config conflicts with another previous config, this returns [`ConfigConflict`].
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
                return Err(ConfigConflict(()));
            }
        }

        self.save_layer(direction, Some(config.layer));

        Ok(())
    }

    /// Configure this content indicator for the specified direction, with all settings reset.
    pub fn reset_direction(&mut self, direction: ContentIndicatorDirection) {
        unsafe {
            sys::content_indicator_configure_direction(self.raw.as_ptr(), direction as u8, null());
        }
        self.save_layer(direction, None);
    }
}

/// Error for [`ContentIndicator::configure_direction`].
#[derive(Debug)]
pub struct ConfigConflict(());

/// Which direction a content indicator applies to.
#[derive(Copy, Clone, PartialEq, Hash, Eq)]
#[repr(u8)]
pub enum ContentIndicatorDirection {
    /// The up direction.
    Up = sys::ContentIndicatorDirection_ContentIndicatorDirectionUp,
    /// The down direction.
    Down = sys::ContentIndicatorDirection_ContentIndicatorDirectionDown,
}

/// Configuration for [`ContentIndicator::configure_direction`].
pub struct ContentIndicatorConfig {
    /// Which layer the content indicator should appear on.
    pub layer: Layer,
    /// Whether displaying the indicator should time out.
    pub times_out: bool,
    /// Alignment of the indicator within the layer.
    pub alignment: GAlign,
    /// Foreground color of the indicator.
    pub foreground: GColor,
    /// Background color of the indicator.
    pub background: GColor,
}

impl ContentIndicatorConfig {
    /// Returns a basic content indicator for the given layer.
    pub const fn basic(layer: Layer) -> Self {
        Self {
            layer,
            times_out: false,
            alignment: GAlign::Center,
            foreground: color::GCOLOR_BLACK,
            background: color::GCOLOR_WHITE,
        }
    }
}
