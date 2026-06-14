use core::ptr::NonNull;

use crate::sys;

pub type Font = NonNull<sys::FontInfo>;

pub enum SystemFont {
    Gothic24,
}

impl SystemFont {
    pub fn load(&self) -> Option<Font> {
        let ptr = match self {
            Self::Gothic24 => sys::FONT_KEY_GOTHIC_24,
        }
        .as_ptr();

        let font = unsafe { sys::fonts_get_system_font(ptr) };
        NonNull::new(font)
    }
}
