use core::ptr::NonNull;

use crate::{
    handle::{Handle, new_handle},
    key::ResourceId,
    sys,
};

pub struct FontInner {
    pub(crate) raw: NonNull<sys::FontInfo>,
    custom: bool,
}

impl FontInner {
    pub fn load_custom(resource_id: u32) -> Option<Self> {
        let font = unsafe {
            let handle = sys::resource_get_handle(resource_id);
            sys::fonts_load_custom_font(handle)
        };

        Some(FontInner {
            raw: NonNull::new(font)?,
            custom: true,
        })
    }
}

impl Drop for FontInner {
    fn drop(&mut self) {
        if self.custom {
            unsafe { sys::fonts_unload_custom_font(self.raw.as_ptr()) };
        }
    }
}

/// A font for writing text.
#[derive(Clone)]
pub struct Font {
    pub(crate) handle: Handle<FontInner>,
}

impl Font {
    /// Load a custom font from the given resource ID.
    pub fn load_custom(resource: ResourceId) -> Option<Self> {
        Some(Self {
            handle: new_handle(FontInner::load_custom(*resource)?),
        })
    }
}

/// The selection of system fonts.
/// Note that some fonts are not available on all platforms.
/// See the [system fonts list](https://developer.repebble.com/guides/app-resources/system-fonts/) for details on each font.
#[derive(Copy, Clone)]
pub enum SystemFont {
    /// Bitham, 30px, Black.
    Bitham30Black,
    /// Bitham, 34px, Medium, numbers and symbols only.
    Bitham34MediumNumbers,
    /// Bitham, 42px, Bold.
    Bitham42Bold,
    /// Bitham, 42px, Light.
    Bitham42Light,
    /// Bitham, 42px, Medium, numbers and symbols only.
    Bitham42MediumNumbers,
    /// Droid Serif, 28px, Bold.
    DroidSerif28Bold,
    /// Gothic, 14px.
    Gothic14,
    /// Gothic, 14px, Bold.
    Gothic14Bold,
    /// Gothic, 18px.
    Gothic18,
    /// Gothic, 18px, Bold.
    Gothic18Bold,
    /// Gothic, 24px.
    Gothic24,
    /// Gothic, 24px, Bold.
    Gothic24Bold,
    /// Gothic, 28px.
    Gothic28,
    /// Gothic, 28px, Bold.
    Gothic28Bold,
    /// LECO, 20px, Bold, numbers and symbols only.
    Leco20BoldNumbers,
    /// LECO, 26px, Bold, numbers and symbols and AM/PM only.
    Leco26BoldNumbersAmPm,
    /// LECO, 28px, Light, numbers and symbols only.
    Leco28LightNumbers,
    /// LECO, 32px, Bold, numbers and symbols only.
    Leco32BoldNumbers,
    /// LECO, 36px, Bold, numbers and symbols only.
    Leco36BoldNumbers,
    /// LECO, 38px, Bold, numbers and symbols only.
    Leco38BoldNumbers,
    /// LECO, 42px, numbers and symbols only.
    Leco42Numbers,
    #[cfg(not(any(
        platform = "aplite",
        platform = "basalt",
        platform = "chalk",
        platform = "diorite"
    )))]
    /// LECO, 60px, Bold, numbers and symbols and AM/PM only.
    Leco60BoldNumbersAmPm,
    #[cfg(not(any(
        platform = "aplite",
        platform = "basalt",
        platform = "chalk",
        platform = "diorite"
    )))]
    /// LECO, 60px, numbers and symbols and AM/PM only.
    Leco60NumbersAmPm,
    /// Roboto, 49px, Bold, "subset" (unspecified).
    RobotoBoldSubset49,
    /// Roboto Condensed, 21px.
    RobotoCondensed21,
}

impl SystemFont {
    /// Load the system font into memory.
    pub fn load(self) -> Option<Font> {
        let ptr: *const u8 = match self {
            Self::Bitham30Black => sys::FONT_KEY_BITHAM_30_BLACK.as_ptr(),
            Self::Bitham34MediumNumbers => sys::FONT_KEY_BITHAM_34_MEDIUM_NUMBERS.as_ptr(),
            Self::Bitham42Bold => sys::FONT_KEY_BITHAM_42_BOLD.as_ptr(),
            Self::Bitham42Light => sys::FONT_KEY_BITHAM_42_LIGHT.as_ptr(),
            Self::Bitham42MediumNumbers => sys::FONT_KEY_BITHAM_42_MEDIUM_NUMBERS.as_ptr(),
            Self::DroidSerif28Bold => sys::FONT_KEY_DROID_SERIF_28_BOLD.as_ptr(),
            Self::Gothic14 => sys::FONT_KEY_GOTHIC_14.as_ptr(),
            Self::Gothic14Bold => sys::FONT_KEY_GOTHIC_14_BOLD.as_ptr(),
            Self::Gothic18 => sys::FONT_KEY_GOTHIC_18.as_ptr(),
            Self::Gothic18Bold => sys::FONT_KEY_GOTHIC_18_BOLD.as_ptr(),
            Self::Gothic24 => sys::FONT_KEY_GOTHIC_24.as_ptr(),
            Self::Gothic24Bold => sys::FONT_KEY_GOTHIC_24_BOLD.as_ptr(),
            Self::Gothic28 => sys::FONT_KEY_GOTHIC_28.as_ptr(),
            Self::Gothic28Bold => sys::FONT_KEY_GOTHIC_28_BOLD.as_ptr(),
            Self::Leco20BoldNumbers => sys::FONT_KEY_LECO_20_BOLD_NUMBERS.as_ptr(),
            Self::Leco26BoldNumbersAmPm => sys::FONT_KEY_LECO_26_BOLD_NUMBERS_AM_PM.as_ptr(),
            Self::Leco28LightNumbers => sys::FONT_KEY_LECO_28_LIGHT_NUMBERS.as_ptr(),
            Self::Leco32BoldNumbers => sys::FONT_KEY_LECO_32_BOLD_NUMBERS.as_ptr(),
            Self::Leco36BoldNumbers => sys::FONT_KEY_LECO_36_BOLD_NUMBERS.as_ptr(),
            Self::Leco38BoldNumbers => sys::FONT_KEY_LECO_38_BOLD_NUMBERS.as_ptr(),
            Self::Leco42Numbers => sys::FONT_KEY_LECO_42_NUMBERS.as_ptr(),
            #[cfg(not(any(
                platform = "aplite",
                platform = "basalt",
                platform = "chalk",
                platform = "diorite"
            )))]
            Self::Leco60BoldNumbersAmPm => sys::FONT_KEY_LECO_60_BOLD_NUMBERS_AM_PM.as_ptr(),
            #[cfg(not(any(
                platform = "aplite",
                platform = "basalt",
                platform = "chalk",
                platform = "diorite"
            )))]
            Self::Leco60NumbersAmPm => sys::FONT_KEY_LECO_60_NUMBERS_AM_PM.as_ptr(),
            Self::RobotoBoldSubset49 => sys::FONT_KEY_ROBOTO_BOLD_SUBSET_49.as_ptr(),
            Self::RobotoCondensed21 => sys::FONT_KEY_ROBOTO_CONDENSED_21.as_ptr(),
        };

        let font = unsafe { sys::fonts_get_system_font(ptr) };
        Some(Font {
            handle: new_handle(FontInner {
                raw: NonNull::new(font)?,
                custom: false,
            }),
        })
    }
}
