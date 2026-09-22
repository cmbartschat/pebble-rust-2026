use crate::sys::*;

/// Pebble watch information.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct WatchInfo {
    /// The watch platform.
    pub platform: Platform,
    /// The watch model.
    pub model: Option<WatchModel>,
    /// The watch color.
    pub color: Option<WatchColor>,
    /// The firmware version.
    pub firmware_version: WatchInfoVersion,
}

/// Pebble SDK platform.
///
/// All of the functionality of this type is available at compile time, since the platform is fixed at that point.
/// Use [`Platform::current`] to retrieve the platform in const context.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Platform {
    /// Classic, Steel
    Aplite = PlatformType_PlatformTypeAplite,
    /// Time, Time Steel
    Basalt = PlatformType_PlatformTypeBasalt,
    /// Time Round
    Chalk = PlatformType_PlatformTypeChalk,
    /// Pebble 2
    Diorite = PlatformType_PlatformTypeDiorite,
    /// 2 Duo
    Emery = PlatformType_PlatformTypeEmery,
    /// Time 2
    Flint = PlatformType_PlatformTypeFlint,
    /// Round 2
    Gabbro = PlatformType_PlatformTypeGabbro,
}

impl Platform {
    /// Returns the current platform.
    /// This is a compile-time constant.
    #[inline]
    #[allow(clippy::needless_return)] // syntax error otherwise due to cfgs
    pub const fn current() -> Self {
        #[cfg(platform = "aplite")]
        return Self::Aplite;
        #[cfg(platform = "basalt")]
        return Self::Basalt;
        #[cfg(platform = "chalk")]
        return Self::Chalk;
        #[cfg(platform = "diorite")]
        return Self::Diorite;
        #[cfg(platform = "emery")]
        return Self::Emery;
        #[cfg(platform = "flint")]
        return Self::Flint;
        #[cfg(platform = "gabbro")]
        return Self::Gabbro;
    }

    /// Returns whether this is a platform for a Core Devices watch.
    #[inline]
    pub const fn is_core_devices(&self) -> bool {
        matches!(self, Self::Flint | Self::Gabbro | Self::Emery)
    }

    /// Returns whether this is a platform for a round watch.
    #[inline]
    pub const fn is_round(&self) -> bool {
        matches!(self, Self::Chalk | Self::Gabbro)
    }

    /// Returns whether this platform’s watch supports colors.
    #[inline]
    pub const fn has_color(&self) -> bool {
        !matches!(self, Self::Aplite | Self::Diorite | Self::Flint)
    }
}

/// Pebble watch color.
/// This is the detailed color, with different values for each watch.
/// For a simplified color enum that unifies all common colors, see [`WatchColor::simple_color`].
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum WatchColor {
    /// Original Pebble, black.
    OriginalBlack = WatchInfoColor_WATCH_INFO_COLOR_BLACK,
    /// Original Pebble, white.
    OriginalWhite = WatchInfoColor_WATCH_INFO_COLOR_WHITE,
    /// Original Pebble, red.
    OriginalRed = WatchInfoColor_WATCH_INFO_COLOR_RED,
    /// Original Pebble, orange.
    OriginalOrange = WatchInfoColor_WATCH_INFO_COLOR_ORANGE,
    /// Original Pebble, gray.
    OriginalGray = WatchInfoColor_WATCH_INFO_COLOR_GRAY,
    /// Original Pebble, stainless steel.
    OriginalStainlessSteel = WatchInfoColor_WATCH_INFO_COLOR_STAINLESS_STEEL,
    /// Original Pebble, matte black.
    OriginalMatteBlack = WatchInfoColor_WATCH_INFO_COLOR_MATTE_BLACK,
    /// Original Pebble, blue.
    OriginalBlue = WatchInfoColor_WATCH_INFO_COLOR_BLUE,
    /// Original Pebble, green.
    OriginalGreen = WatchInfoColor_WATCH_INFO_COLOR_GREEN,
    /// Original Pebble, pink.
    OriginalPink = WatchInfoColor_WATCH_INFO_COLOR_PINK,
    /// Time, white.
    TimeWhite = WatchInfoColor_WATCH_INFO_COLOR_TIME_WHITE,
    /// Time, black.
    TimeBlack = WatchInfoColor_WATCH_INFO_COLOR_TIME_BLACK,
    /// Time, red.
    TimeRed = WatchInfoColor_WATCH_INFO_COLOR_TIME_RED,
    /// Time Steel, silver.
    SteelSilver = WatchInfoColor_WATCH_INFO_COLOR_TIME_STEEL_SILVER,
    /// Time Steel, black.
    SteelBlack = WatchInfoColor_WATCH_INFO_COLOR_TIME_STEEL_BLACK,
    /// Time Steel, gold.
    SteelGold = WatchInfoColor_WATCH_INFO_COLOR_TIME_STEEL_GOLD,
    /// Time Round, 14mm lug size, silver.
    TimeRound14Silver = WatchInfoColor_WATCH_INFO_COLOR_TIME_ROUND_SILVER_14,
    /// Time Round, 14mm lug size, black.
    TimeRound14Black = WatchInfoColor_WATCH_INFO_COLOR_TIME_ROUND_BLACK_14,
    /// Time Round, 20mm lug size, silver.
    TimeRound20Silver = WatchInfoColor_WATCH_INFO_COLOR_TIME_ROUND_SILVER_20,
    /// Time Round, 20mm lug size, silver.
    TimeRound20Black = WatchInfoColor_WATCH_INFO_COLOR_TIME_ROUND_BLACK_20,
    /// Time Round, 14mm lug size, rose gold.
    TimeRound14RoseGold = WatchInfoColor_WATCH_INFO_COLOR_TIME_ROUND_ROSE_GOLD_14,
    /// Pebble 2 HR, black.
    Hr2Black = WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_HR_BLACK,
    /// Pebble 2 HR, lime (green).
    Hr2Lime = WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_HR_LIME,
    /// Pebble 2 HR, flame (red).
    Hr2Flame = WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_HR_FLAME,
    /// Pebble 2 HR, white.
    Hr2White = WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_HR_WHITE,
    /// Pebble 2 HR, aqua (blue).
    Hr2Aqua = WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_HR_AQUA,
    /// Pebble 2 SE, black.
    Se2Black = WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_SE_BLACK,
    /// Pebble 2 SE, white.
    Se2White = WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_SE_WHITE,
    /// Pebble Time 2, black.
    Time2Black = WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_TIME_2_BLACK,
    /// Pebble Time 2, silver.
    Time2Silver = WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_TIME_2_SILVER,
    /// Pebble Time 2, gold.
    Time2Gold = WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_TIME_2_GOLD,
    /// Pebble 2 Duo, black.
    Duo2Black = WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_P2D_BLACK,
    /// Pebble 2 Duo, white.
    Duo2White = WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_P2D_WHITE,
    /// Pebble Time 2 (Core Devices), gray.
    Time2CoreDevicesGray = WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PT2_BLACK_GREY,
    /// Pebble Time 2 (Core Devices), red.
    Time2CoreDevicesRed = WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PT2_BLACK_RED,
    /// Pebble Time 2 (Core Devices), silver and blue.
    Time2CoreDevicesSilverBlue = WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PT2_SILVER_BLUE,
    /// Pebble Time 2 (Core Devices), silver and gray.
    Time2CoreDevicesSilverGray = WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PT2_SILVER_GREY,
    /// Pebble Round 2, 20mm lug size, black.
    Round2_20Black = WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PR2_BLACK_20,
    /// Pebble Round 2, 20mm lug size, silver.
    Round2_20Silver = WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PR2_SILVER_20,
    /// Pebble Round 2, 14mm lug size, gold.
    Round2_14Gold = WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PR2_GOLD_14,
    /// Pebble Round 2, 14mm lug size, silver.
    Round2_14Silver = WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PR2_SILVER_14,
}

impl TryFrom<u8> for WatchColor {
    type Error = ();

    #[allow(non_upper_case_globals, non_snake_case)]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            WatchInfoColor_WATCH_INFO_COLOR_BLACK => Self::OriginalBlack,
            WatchInfoColor_WATCH_INFO_COLOR_WHITE => Self::OriginalWhite,
            WatchInfoColor_WATCH_INFO_COLOR_RED => Self::OriginalRed,
            WatchInfoColor_WATCH_INFO_COLOR_ORANGE => Self::OriginalOrange,
            WatchInfoColor_WATCH_INFO_COLOR_GRAY => Self::OriginalGray,
            WatchInfoColor_WATCH_INFO_COLOR_STAINLESS_STEEL => Self::OriginalStainlessSteel,
            WatchInfoColor_WATCH_INFO_COLOR_MATTE_BLACK => Self::OriginalMatteBlack,
            WatchInfoColor_WATCH_INFO_COLOR_BLUE => Self::OriginalBlue,
            WatchInfoColor_WATCH_INFO_COLOR_GREEN => Self::OriginalGreen,
            WatchInfoColor_WATCH_INFO_COLOR_PINK => Self::OriginalPink,
            WatchInfoColor_WATCH_INFO_COLOR_TIME_WHITE => Self::TimeWhite,
            WatchInfoColor_WATCH_INFO_COLOR_TIME_BLACK => Self::TimeBlack,
            WatchInfoColor_WATCH_INFO_COLOR_TIME_RED => Self::TimeRed,
            WatchInfoColor_WATCH_INFO_COLOR_TIME_STEEL_SILVER => Self::SteelSilver,
            WatchInfoColor_WATCH_INFO_COLOR_TIME_STEEL_BLACK => Self::SteelBlack,
            WatchInfoColor_WATCH_INFO_COLOR_TIME_STEEL_GOLD => Self::SteelGold,
            WatchInfoColor_WATCH_INFO_COLOR_TIME_ROUND_SILVER_14 => Self::TimeRound14Silver,
            WatchInfoColor_WATCH_INFO_COLOR_TIME_ROUND_BLACK_14 => Self::TimeRound14Black,
            WatchInfoColor_WATCH_INFO_COLOR_TIME_ROUND_SILVER_20 => Self::TimeRound20Silver,
            WatchInfoColor_WATCH_INFO_COLOR_TIME_ROUND_BLACK_20 => Self::TimeRound20Black,
            WatchInfoColor_WATCH_INFO_COLOR_TIME_ROUND_ROSE_GOLD_14 => Self::TimeRound14RoseGold,
            WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_HR_BLACK => Self::Hr2Black,
            WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_HR_LIME => Self::Hr2Lime,
            WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_HR_FLAME => Self::Hr2Flame,
            WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_HR_WHITE => Self::Hr2White,
            WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_HR_AQUA => Self::Hr2Aqua,
            WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_SE_BLACK => Self::Se2Black,
            WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_2_SE_WHITE => Self::Se2White,
            WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_TIME_2_BLACK => Self::Time2Black,
            WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_TIME_2_SILVER => Self::Time2Silver,
            WatchInfoColor_WATCH_INFO_COLOR_PEBBLE_TIME_2_GOLD => Self::Time2Gold,
            WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_P2D_BLACK => Self::Duo2Black,
            WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_P2D_WHITE => Self::Duo2White,
            WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PT2_BLACK_GREY => {
                Self::Time2CoreDevicesGray
            }
            WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PT2_BLACK_RED => Self::Time2CoreDevicesRed,
            WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PT2_SILVER_BLUE => {
                Self::Time2CoreDevicesSilverBlue
            }
            WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PT2_SILVER_GREY => {
                Self::Time2CoreDevicesSilverGray
            }
            WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PR2_BLACK_20 => Self::Round2_20Black,
            WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PR2_SILVER_20 => Self::Round2_20Silver,
            WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PR2_GOLD_14 => Self::Round2_14Gold,
            WatchInfoColor_WATCH_INFO_COLOR_COREDEVICES_PR2_SILVER_14 => Self::Round2_14Silver,
            _ => return Err(()),
        })
    }
}

impl WatchColor {
    /// Returns the simplified color of this watch.
    /// The result is opinionated, but should be more helpful if you’re trying to be forwards-compatible with matching your menu/watch face to the watch color.
    pub const fn simple_color(self) -> SimpleWatchColor {
        match self {
            Self::OriginalBlack
            | Self::SteelBlack
            | Self::OriginalMatteBlack
            | Self::TimeBlack
            | Self::TimeRound20Black
            | Self::Hr2Black
            | Self::Time2Black
            | Self::Duo2Black
            | Self::Round2_20Black
            | Self::Se2Black
            | Self::TimeRound14Black => SimpleWatchColor::Black,
            Self::Se2White
            | Self::Duo2White
            | Self::Hr2White
            | Self::OriginalWhite
            | Self::TimeWhite => SimpleWatchColor::White,
            Self::Hr2Flame | Self::Time2CoreDevicesRed | Self::OriginalRed | Self::TimeRed => {
                SimpleWatchColor::Redish
            }
            Self::OriginalOrange => SimpleWatchColor::Orange,
            Self::Time2CoreDevicesGray | Self::Time2CoreDevicesSilverGray | Self::OriginalGray => {
                SimpleWatchColor::Gray
            }
            Self::OriginalStainlessSteel
            | Self::Round2_14Silver
            | Self::TimeRound14Silver
            | Self::TimeRound20Silver
            | Self::SteelSilver
            | Self::Time2Silver
            | Self::Round2_20Silver => SimpleWatchColor::Silver,
            Self::Hr2Aqua | Self::Time2CoreDevicesSilverBlue | Self::OriginalBlue => {
                SimpleWatchColor::Blueish
            }
            Self::OriginalGreen | Self::Hr2Lime => SimpleWatchColor::Greenish,
            Self::OriginalPink => SimpleWatchColor::Pink,
            Self::TimeRound14RoseGold => SimpleWatchColor::RoseGold,
            Self::Time2Gold | Self::SteelGold | Self::Round2_14Gold => SimpleWatchColor::Gold,
        }
    }
}

impl From<WatchColor> for SimpleWatchColor {
    fn from(value: WatchColor) -> Self {
        value.simple_color()
    }
}

/// Simplified and slightly opinionated version of [`WatchColor`].
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum SimpleWatchColor {
    /// Black (including matte black).
    Black,
    /// Silver or (stainless) steel.
    Silver,
    /// Gold.
    Gold,
    /// Rose gold.
    RoseGold,
    /// White or almost white.
    White,
    /// Gray.
    Gray,
    /// Red or any red-like color.
    Redish,
    /// Orange.
    Orange,
    /// Blue or any blue-like color.
    Blueish,
    /// Green or any green-like color.
    Greenish,
    /// Pink.
    Pink,
}

/// Pebble watch model.
/// For generic properties like whether this is a round watch, or whether it has colors, see [`Platform`], where that data is represented more accurately.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum WatchModel {
    /// Original Pebble.
    Original = WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_ORIGINAL,
    /// Pebble Steel.
    Steel = WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_STEEL,
    /// Pebble Time.
    Time = WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_TIME,
    /// Pebble Time Steel.
    TimeSteel = WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_TIME_STEEL,
    /// Pebble Time Round. Note that the C API separates the different lug sizes here, while it doesn’t do the same for the Round 2.
    /// For consistency, we summarize the two models here as well.
    /// If you need the lug size difference, see [`WatchColor`], which differentiates lug sizes for both Round versions.
    TimeRound = WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_TIME_ROUND_14,
    /// Pebble Time 2 (original).
    Time2 = WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_TIME_2,
    /// Pebble 2 HR.
    Hr2 = WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_2_HR,
    /// Pebble 2 SE.
    Se2 = WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_2_SE,
    /// Pebble 2 Duo (CoreDevices)
    Duo2 = WatchInfoModel_WATCH_INFO_MODEL_COREDEVICES_P2D,
    /// Pebble Time 2 (CoreDevices / re-manufactured version)
    Time2CoreDevices = WatchInfoModel_WATCH_INFO_MODEL_COREDEVICES_PT2,
    /// Pebble Round 2
    Round2 = WatchInfoModel_WATCH_INFO_MODEL_COREDEVICES_PR2,
}

impl TryFrom<u8> for WatchModel {
    type Error = ();

    #[allow(non_upper_case_globals, non_snake_case)]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_ORIGINAL => Self::Original,
            WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_STEEL => Self::Steel,
            WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_TIME => Self::Time,
            WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_TIME_STEEL => Self::TimeSteel,
            WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_TIME_ROUND_14
            | WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_TIME_ROUND_20 => Self::TimeRound,
            WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_TIME_2 => Self::Time2,
            WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_2_HR => Self::Hr2,
            WatchInfoModel_WATCH_INFO_MODEL_PEBBLE_2_SE => Self::Se2,
            WatchInfoModel_WATCH_INFO_MODEL_COREDEVICES_P2D => Self::Duo2,
            WatchInfoModel_WATCH_INFO_MODEL_COREDEVICES_PT2 => Self::Time2CoreDevices,
            WatchInfoModel_WATCH_INFO_MODEL_COREDEVICES_PR2 => Self::Round2,
            _ => return Err(()),
        })
    }
}

impl WatchInfo {
    /// Returns the current watch information.
    pub fn current() -> Self {
        let color = unsafe { watch_info_get_color() };
        let model = unsafe { watch_info_get_model() };
        let firmware_version = unsafe { watch_info_get_firmware_version() };
        Self {
            firmware_version,
            platform: Platform::current(),
            color: WatchColor::try_from(color).ok(),
            model: WatchModel::try_from(model).ok(),
        }
    }
}
