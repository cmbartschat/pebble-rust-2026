//! Color constants.
//! The constant names match the ones from the [color picker tool](https://developer.repebble.com/guides/tools-and-resources/color-picker/).
//!
//! To create a [`GColor`], you can use the [`crate::hex_color`] macro.
#![allow(missing_docs)]

use ufmt::uDebug;

use crate::GColor;
use crate::sys;

pub const GCOLOR_BLACK: GColor = GColor { argb: 0b11000000 };
pub const GCOLOR_OXFORD_BLUE: GColor = GColor { argb: 0b11000001 };
pub const GCOLOR_DUKE_BLUE: GColor = GColor { argb: 0b11000010 };
pub const GCOLOR_BLUE: GColor = GColor { argb: 0b11000011 };
pub const GCOLOR_DARK_GREEN: GColor = GColor { argb: 0b11000100 };
pub const GCOLOR_MIDNIGHT_GREEN: GColor = GColor { argb: 0b11000101 };
pub const GCOLOR_COBALT_BLUE: GColor = GColor { argb: 0b11000110 };
pub const GCOLOR_BLUE_MOON: GColor = GColor { argb: 0b11000111 };
pub const GCOLOR_ISLAMIC_GREEN: GColor = GColor { argb: 0b11001000 };
pub const GCOLOR_JAEGER_GREEN: GColor = GColor { argb: 0b11001001 };
pub const GCOLOR_TIFFANY_BLUE: GColor = GColor { argb: 0b11001010 };
pub const GCOLOR_VIVID_CERULEAN: GColor = GColor { argb: 0b11001011 };
pub const GCOLOR_GREEN: GColor = GColor { argb: 0b11001100 };
pub const GCOLOR_MALACHITE: GColor = GColor { argb: 0b11001101 };
pub const GCOLOR_MEDIUM_SPRING_GREEN: GColor = GColor { argb: 0b11001110 };
pub const GCOLOR_CYAN: GColor = GColor { argb: 0b11001111 };
pub const GCOLOR_BULGARIAN_ROSE: GColor = GColor { argb: 0b11010000 };
pub const GCOLOR_IMPERIAL_PURPLE: GColor = GColor { argb: 0b11010001 };
pub const GCOLOR_INDIGO: GColor = GColor { argb: 0b11010010 };
pub const GCOLOR_ELECTRIC_ULTRAMARINE: GColor = GColor { argb: 0b11010011 };
pub const GCOLOR_ARMY_GREEN: GColor = GColor { argb: 0b11010100 };
pub const GCOLOR_DARK_GRAY: GColor = GColor { argb: 0b11010101 };
pub const GCOLOR_LIBERTY: GColor = GColor { argb: 0b11010110 };
pub const GCOLOR_VERY_LIGHT_BLUE: GColor = GColor { argb: 0b11010111 };
pub const GCOLOR_KELLY_GREEN: GColor = GColor { argb: 0b11011000 };
pub const GCOLOR_MAY_GREEN: GColor = GColor { argb: 0b11011001 };
pub const GCOLOR_CADET_BLUE: GColor = GColor { argb: 0b11011010 };
pub const GCOLOR_PICTON_BLUE: GColor = GColor { argb: 0b11011011 };
pub const GCOLOR_BRIGHT_GREEN: GColor = GColor { argb: 0b11011100 };
pub const GCOLOR_SCREAMIN_GREEN: GColor = GColor { argb: 0b11011101 };
pub const GCOLOR_MEDIUM_AQUAMARINE: GColor = GColor { argb: 0b11011110 };
pub const GCOLOR_ELECTRIC_BLUE: GColor = GColor { argb: 0b11011111 };
pub const GCOLOR_DARK_CANDY_APPLE_RED: GColor = GColor { argb: 0b11100000 };
pub const GCOLOR_JAZZBERRY_JAM: GColor = GColor { argb: 0b11100001 };
pub const GCOLOR_PURPLE: GColor = GColor { argb: 0b11100010 };
pub const GCOLOR_VIVID_VIOLET: GColor = GColor { argb: 0b11100011 };
pub const GCOLOR_WINDSOR_TAN: GColor = GColor { argb: 0b11100100 };
pub const GCOLOR_ROSE_VALE: GColor = GColor { argb: 0b11100101 };
pub const GCOLOR_PURPUREUS: GColor = GColor { argb: 0b11100110 };
pub const GCOLOR_LAVENDER_INDIGO: GColor = GColor { argb: 0b11100111 };
pub const GCOLOR_LIMERICK: GColor = GColor { argb: 0b11101000 };
pub const GCOLOR_BRASS: GColor = GColor { argb: 0b11101001 };
pub const GCOLOR_LIGHT_GRAY: GColor = GColor { argb: 0b11101010 };
pub const GCOLOR_BABY_BLUE_EYES: GColor = GColor { argb: 0b11101011 };
pub const GCOLOR_SPRING_BUD: GColor = GColor { argb: 0b11101100 };
pub const GCOLOR_INCHWORM: GColor = GColor { argb: 0b11101101 };
pub const GCOLOR_MINT_GREEN: GColor = GColor { argb: 0b11101110 };
pub const GCOLOR_CELESTE: GColor = GColor { argb: 0b11101111 };
pub const GCOLOR_RED: GColor = GColor { argb: 0b11110000 };
pub const GCOLOR_FOLLY: GColor = GColor { argb: 0b11110001 };
pub const GCOLOR_FASHION_MAGENTA: GColor = GColor { argb: 0b11110010 };
pub const GCOLOR_MAGENTA: GColor = GColor { argb: 0b11110011 };
pub const GCOLOR_ORANGE: GColor = GColor { argb: 0b11110100 };
pub const GCOLOR_SUNSET_ORANGE: GColor = GColor { argb: 0b11110101 };
pub const GCOLOR_BRILLIANT_ROSE: GColor = GColor { argb: 0b11110110 };
pub const GCOLOR_SHOCKING_PINK: GColor = GColor { argb: 0b11110111 };
pub const GCOLOR_CHROME_YELLOW: GColor = GColor { argb: 0b11111000 };
pub const GCOLOR_RAJAH: GColor = GColor { argb: 0b11111001 };
pub const GCOLOR_MELON: GColor = GColor { argb: 0b11111010 };
pub const GCOLOR_RICH_BRILLIANT_LAVENDER: GColor = GColor { argb: 0b11111011 };
pub const GCOLOR_YELLOW: GColor = GColor { argb: 0b11111100 };
pub const GCOLOR_ICTERINE: GColor = GColor { argb: 0b11111101 };
pub const GCOLOR_PASTEL_YELLOW: GColor = GColor { argb: 0b11111110 };
pub const GCOLOR_WHITE: GColor = GColor { argb: 0b11111111 };
pub const GCOLOR_CLEAR: GColor = GColor { argb: 0b00000000 };

impl PartialEq for GColor {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::gcolor_equal(*self, *other) }
    }
}

impl GColor {
    /// Returns a color that is maximally legible over the given background color.
    pub fn legible_over(other: Self) -> GColor {
        unsafe { sys::gcolor_legible_over(other) }
    }

    /// Converts this color to standard 32-bit sRGB color representation,
    /// where each byte represents one of the four channels R, G, B, A.
    /// This color is little-endian, so R occupies the MSB while A occupies the LSB.
    pub const fn rgba_32bit(&self) -> u32 {
        let red = self.red() as u32;
        let green = self.green() as u32;
        let blue = self.blue() as u32;
        let alpha = self.alpha() as u32;
        red << 24 | green << 16 | blue << 8 | alpha
    }

    const ALPHA_MASK: u8 = 0b11000000;
    const RED_MASK: u8 = 0b00110000;
    const GREEN_MASK: u8 = 0b00001100;
    const BLUE_MASK: u8 = 0b00000011;
    const ALPHA_SHIFT: u8 = 6;
    const RED_SHIFT: u8 = 3;
    const GREEN_SHIFT: u8 = 2;
    const BLUE_SHIFT: u8 = 0;

    /// Inverse of the LUT in `proc::get_2bit_value`
    const TWO_BIT_LUT: [u8; 4] = [0x00, 0x55, 0xaa, 0xff];

    /// Returns the inner 8-bit color.
    /// The color format is ARGB (from MSBit to LSBit), where each channel occupies two bits.
    pub const fn argb(&self) -> u8 {
        // SAFETY: The union only contains one field, accessing it is always sound.
        unsafe { self.argb }
    }

    /// Returns the 2-bit alpha component of this color.
    pub const fn alpha_2bit(&self) -> u8 {
        (self.argb() & Self::ALPHA_MASK) >> Self::ALPHA_SHIFT
    }
    /// Returns the 2-bit red component of this color.
    pub const fn red_2bit(&self) -> u8 {
        (self.argb() & Self::RED_MASK) >> Self::RED_SHIFT
    }
    /// Returns the 2-bit green component of this color.
    pub const fn green_2bit(&self) -> u8 {
        (self.argb() & Self::GREEN_MASK) >> Self::GREEN_SHIFT
    }
    /// Returns the 2-bit blue component of this color.
    pub const fn blue_2bit(&self) -> u8 {
        (self.argb() & Self::BLUE_MASK) >> Self::BLUE_SHIFT
    }

    /// Returns the alpha component of the color, as an 8-bit value.
    /// 0 = fully transparent, 255 = fully opaque, as usual.
    /// Note that the mapping from 4-bit to 8-bit-color is nonlinear and matches the [`crate::hex_color`] macro’s inputs:
    ///
    /// - 0b00 = 0x00
    /// - 0b01 = 0x55
    /// - 0b10 = 0xaa
    /// - 0b11 = 0xff
    pub const fn alpha(&self) -> u8 {
        Self::TWO_BIT_LUT[self.alpha_2bit() as usize]
    }
    /// Returns the red component of the color, as an 8-bit value.
    /// 0 = no red, 255 = fully red, as usual.
    /// Note that the mapping from 4-bit to 8-bit-color is nonlinear and matches the [`crate::hex_color`] macro’s inputs:
    ///
    /// - 0b00 = 0x00
    /// - 0b01 = 0x55
    /// - 0b10 = 0xaa
    /// - 0b11 = 0xff
    pub const fn red(&self) -> u8 {
        Self::TWO_BIT_LUT[self.red_2bit() as usize]
    }
    /// Returns the green component of the color, as an 8-bit value.
    /// 0 = no green, 255 = fully green, as usual.
    /// Note that the mapping from 4-bit to 8-bit-color is nonlinear and matches the [`crate::hex_color`] macro’s inputs:
    ///
    /// - 0b00 = 0x00
    /// - 0b01 = 0x55
    /// - 0b10 = 0xaa
    /// - 0b11 = 0xff
    pub const fn green(&self) -> u8 {
        Self::TWO_BIT_LUT[self.green_2bit() as usize]
    }
    /// Returns the blue component of the color, as an 8-bit value.
    /// 0 = no blue, 255 = fully blue, as usual.
    /// Note that the mapping from 4-bit to 8-bit-color is nonlinear and matches the [`crate::hex_color`] macro’s inputs:
    ///
    /// - 0b00 = 0x00
    /// - 0b01 = 0x55
    /// - 0b10 = 0xaa
    /// - 0b11 = 0xff
    pub const fn blue(&self) -> u8 {
        Self::TWO_BIT_LUT[self.blue_2bit() as usize]
    }
}

impl uDebug for GColor {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.debug_struct("GColor")?
            .field("alpha", &self.alpha_2bit())?
            .field("red", &self.red_2bit())?
            .field("green", &self.green_2bit())?
            .field("blue", &self.blue_2bit())?
            .finish()
    }
}
