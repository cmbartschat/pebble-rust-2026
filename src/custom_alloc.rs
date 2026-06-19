use core::alloc::{GlobalAlloc, Layout};

use crate::log::log_c_str;

pub struct Allocator;

#[global_allocator]
static ALLOC: Allocator = Allocator;

const NATIVE_ALIGN: usize = usize::BITS as usize / 8;

unsafe extern "C" {
    pub unsafe fn malloc(size: usize) -> *mut u8;
    pub unsafe fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
    pub unsafe fn free(ptr: *mut u8);
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() > NATIVE_ALIGN {
            log_c_str(
                c"  ---   ---   ---   alloc called with align > NATIVE_ALIGN (may cause issues)",
            );
        }

        unsafe { malloc(layout.size()) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { free(ptr) };
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if layout.align() > NATIVE_ALIGN {
            log_c_str(
                c"  ---   ---   ---   realloc called with align > NATIVE_ALIGN (may cause issues)",
            );
        }

        unsafe { realloc(ptr, new_size) }
    }
}
