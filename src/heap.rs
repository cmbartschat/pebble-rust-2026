use crate::sys;

pub fn bytes_used() -> usize {
    unsafe { sys::heap_bytes_used() }
}

pub fn bytes_free() -> usize {
    unsafe { sys::heap_bytes_free() }
}
