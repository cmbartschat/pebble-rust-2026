use crate::sys;

/// Graphics alignment.
/// The values represent all possible 2D alignments, which are 9 possible positions: vertically top, center, bottom, combined with horizontally left, center, right.
#[repr(u8)]
pub enum GAlign {
    /// Vertically and horizontally centered.
    Center = sys::GAlign_GAlignCenter,
    /// Top left.
    TopLeft = sys::GAlign_GAlignTopLeft,
    /// Top right.
    TopRight = sys::GAlign_GAlignTopRight,
    /// Center top.
    Top = sys::GAlign_GAlignTop,
    /// Center left.
    Left = sys::GAlign_GAlignLeft,
    /// Center bottom.
    Bottom = sys::GAlign_GAlignBottom,
    /// Center right.
    Right = sys::GAlign_GAlignRight,
    /// Bottom right.
    BottomRight = sys::GAlign_GAlignBottomRight,
    /// Bottom left.
    BottomLeft = sys::GAlign_GAlignBottomLeft,
}
