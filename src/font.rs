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

#[derive(Clone)]
pub struct Font {
    pub(crate) handle: Handle<FontInner>,
}

impl Font {
    pub fn load_custom(resource: ResourceId) -> Option<Self> {
        Some(Self {
            handle: new_handle(FontInner::load_custom(*resource)?),
        })
    }
}

pub enum SystemFont {
    Bitham30Black,
    Bitham34MediumNumbers,
    Bitham42Bold,
    Bitham42Light,
    Bitham42MediumNumbers,
    DroidSerif28Bold,
    Gothic14,
    Gothic14Bold,
    Gothic18,
    Gothic18Bold,
    Gothic24,
    Gothic24Bold,
    Gothic28,
    Gothic28Bold,
    Leco20BoldNumbers,
    Leco26BoldNumbersAmPm,
    Leco28LightNumbers,
    Leco32BoldNumbers,
    Leco36BoldNumbers,
    Leco38BoldNumbers,
    Leco42Numbers,
    Leco60BoldNumbersAmPm,
    Leco60NumbersAmPm,
    RobotoBoldSubset49,
    RobotoCondensed21,
}

impl SystemFont {
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
            Self::Leco60BoldNumbersAmPm => sys::FONT_KEY_LECO_60_BOLD_NUMBERS_AM_PM.as_ptr(),
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
