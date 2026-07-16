use crate::sys;

#[repr(u32)]
pub enum GAlign {
    Center = sys::GAlign_GAlignCenter,
    TopLeft = sys::GAlign_GAlignTopLeft,
    TopRight = sys::GAlign_GAlignTopRight,
    Top = sys::GAlign_GAlignTop,
    Left = sys::GAlign_GAlignLeft,
    Bottom = sys::GAlign_GAlignBottom,
    Right = sys::GAlign_GAlignRight,
    BottomRight = sys::GAlign_GAlignBottomRight,
    BottomLeft = sys::GAlign_GAlignBottomLeft,
}
