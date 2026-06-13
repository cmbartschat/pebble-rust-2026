use crate::sys;

pub struct Font {
    pub(crate) inner: *mut sys::FontInfo,
}

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
        if font.is_null() {
            return None;
        }

        Some(Font { inner: font })
    }
}

// impl Font {
//     pub fn system() -> Option<Self> {
//         let font = unsafe { sys::fonts_get_system_font(sys::FONT_KEY_GOTHIC_24.as_ptr()) };
//         if font.is_null() {
//             return None;
//         }

//         Some(Self { inner: font })
//     }
// }
