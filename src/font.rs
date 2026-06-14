use core::ptr::NonNull;

use crate::sys;

#[derive(Clone)]
pub struct Font {
    pub(crate) raw: NonNull<sys::FontInfo>,
    custom: bool,
}

impl Font {
    pub fn load_custom(resource_id: u32) -> Option<Self> {
        let font = unsafe {
            let handle = sys::resource_get_handle(resource_id);
            sys::fonts_load_custom_font(handle)
        };

        Some(Font {
            raw: NonNull::new(font)?,
            custom: true,
        })
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        if self.custom {
            unsafe { sys::fonts_unload_custom_font(self.raw.as_ptr()) };
        }
    }
}
pub enum SystemFont {
    Gothic24,
}

impl SystemFont {
    pub fn load(self) -> Option<Font> {
        let ptr = match self {
            Self::Gothic24 => sys::FONT_KEY_GOTHIC_24,
        }
        .as_ptr();

        let font = unsafe { sys::fonts_get_system_font(ptr) };
        Some(Font {
            raw: NonNull::new(font)?,
            custom: false,
        })
    }
}
