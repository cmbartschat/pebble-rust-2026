//! Get information about the application heap.

use crate::sys;

/// Return the number of bytes used on the heap.
pub fn bytes_used() -> usize {
    unsafe { sys::heap_bytes_used() }
}

/// Return the number of bytes free on the heap.
pub fn bytes_free() -> usize {
    unsafe { sys::heap_bytes_free() }
}
