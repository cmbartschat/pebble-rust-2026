use crate::sys;

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
pub enum Button {
    Back = sys::ButtonId_BUTTON_ID_BACK,
    Up = sys::ButtonId_BUTTON_ID_UP,
    Select = sys::ButtonId_BUTTON_ID_SELECT,
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
