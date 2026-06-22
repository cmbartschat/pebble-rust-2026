use crate::sys;

#[derive(PartialEq, Clone, Copy)]
pub enum Button {
    Back,
    Up,
    Select,
    Down,
}

impl From<Button> for sys::ButtonId {
    fn from(val: Button) -> Self {
        match val {
            Button::Back => sys::ButtonId_BUTTON_ID_BACK,
            Button::Up => sys::ButtonId_BUTTON_ID_UP,
            Button::Select => sys::ButtonId_BUTTON_ID_SELECT,
            Button::Down => sys::ButtonId_BUTTON_ID_DOWN,
        }
    }
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
