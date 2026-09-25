use crate::sys;

/// The watch buttons.
#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
pub enum Button {
    /// The back button, which exits the current menu or app.
    Back = sys::ButtonId_BUTTON_ID_BACK,
    /// The up button.
    Up = sys::ButtonId_BUTTON_ID_UP,
    /// The select (center) button.
    Select = sys::ButtonId_BUTTON_ID_SELECT,
    /// The down button.
    Down = sys::ButtonId_BUTTON_ID_DOWN,
}

impl From<sys::ButtonId> for Button {
    fn from(value: sys::ButtonId) -> Self {
        match value {
            sys::ButtonId_BUTTON_ID_BACK => Self::Back,
            sys::ButtonId_BUTTON_ID_UP => Self::Up,
            sys::ButtonId_BUTTON_ID_SELECT => Self::Select,
            sys::ButtonId_BUTTON_ID_DOWN => Self::Down,
            _ => panic!("Invalid button"),
        }
    }
}
