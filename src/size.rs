use crate::GSize;

use crate::sys;

impl GSize {
    pub fn new(w: i16, h: i16) -> Self {
        Self { w, h }
    }
}

impl PartialEq for GSize {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::gsize_equal(self, other) }
    }
}
