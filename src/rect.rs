use crate::sys::{GPoint, GRect, GSize};

impl GRect {
    pub fn new(x: i16, y: i16, w: i16, h: i16) -> Self {
        Self {
            origin: GPoint { x, y },
            size: GSize { w, h },
        }
    }
}
