use crate::sys::{GEdgeInsets, GPoint, GRect, GSize};

use crate::sys;

impl GRect {
    pub fn new(x: i16, y: i16, w: i16, h: i16) -> Self {
        Self {
            origin: GPoint { x, y },
            size: GSize { w, h },
        }
    }

    pub fn inset(self, insets: GEdgeInsets) -> Self {
        unsafe { sys::grect_inset(self, insets) }
    }

    pub fn shrink(self, amount: i32) -> Self {
        unsafe { sys::grect_crop(self, amount) }
    }

    pub fn expand(self, amount: i32) -> Self {
        self.shrink(-amount)
    }
}
