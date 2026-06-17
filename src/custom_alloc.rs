use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

use crate::log::log_c_str;

pub struct Allocator;

#[global_allocator]
static ALLOC: Allocator = Allocator;

const NATIVE_ALIGN: usize = usize::BITS as usize / 8;

unsafe extern "C" {
    pub unsafe fn malloc(size: usize) -> *mut u8;
    pub unsafe fn aligned_alloc(alignment: usize, size: usize) -> *mut u8;
    pub unsafe fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
    pub unsafe fn free(ptr: *mut u8);
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= NATIVE_ALIGN {
            return unsafe { malloc(layout.size()) };
        }

        log_c_str(c"  ---   ---   ---   alloc custom align");

        unsafe { aligned_alloc(layout.align(), layout.size()) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { free(ptr) };
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if layout.align() <= NATIVE_ALIGN {
            return unsafe { realloc(ptr, new_size) };
        }

        log_c_str(c"  ---   ---   ---   realloc custom align");

        let new_layout = match Layout::from_size_align(new_size, layout.align()) {
            Ok(l) => l,
            Err(_) => return ptr::null_mut(),
        };

        let new_ptr = unsafe { self.alloc(new_layout) };
        if new_ptr.is_null() {
            return ptr::null_mut();
        }

        let copy_size = core::cmp::min(layout.size(), new_size);
        unsafe {
            ptr::copy_nonoverlapping(ptr, new_ptr, copy_size);
            free(ptr);
        }

        new_ptr
    }
}
