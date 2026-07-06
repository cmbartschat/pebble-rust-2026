use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
    slice,
};

use crate::log::log_c_str;

pub struct Allocator;

#[global_allocator]
static ALLOC: Allocator = Allocator;

// NOTE(christoph): Tested on real target hardware
const NATIVE_ALIGN: usize = size_of::<*mut u8>();

unsafe extern "C" {
    pub unsafe fn malloc(size: usize) -> *mut u8;
    pub unsafe fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
    pub unsafe fn free(ptr: *mut u8);
    pub unsafe fn calloc(count: usize, size: usize) -> *mut u8;
}

impl Allocator {
    fn alloc_inner(layout: Layout, alloc: fn(usize) -> *mut u8) -> *mut u8 {
        if layout.align() > NATIVE_ALIGN {
            let Some(required_size) = layout
                .size()
                .checked_add(layout.align())
                .and_then(|f| f.checked_add(size_of::<*mut u8>()))
            else {
                log_c_str(c"Unexpected: Requested memory overflows including padding and metadata");
                return null_mut();
            };
            let base = alloc(required_size);
            if base.is_null() {
                return null_mut();
            }
            debug_assert!((base as usize).is_multiple_of(NATIVE_ALIGN));
            let align_offset = match base.align_offset(layout.align()) {
                usize::MAX => {
                    log_c_str(c"Unexpected: Unable to locate valid alignment");
                    unsafe { free(base) };
                    return null_mut();
                }
                e if e < size_of::<*mut u8>() => e + layout.align(),
                e => e,
            };
            let ptr = base.wrapping_add(align_offset);
            unsafe { *(ptr as *mut *mut u8).sub(1) = base };
            return ptr;
        }

        alloc(layout.size())
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        log_c_str(c"alloc");
        Allocator::alloc_inner(layout, |c| unsafe { malloc(c) })
    }

    // unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
    //     log_c_str(c"alloc_zeroed");
    //     Allocator::alloc_inner(layout, |c| unsafe { calloc(1, c) })
    // }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        log_c_str(c"dealloc");
        if layout.align() > NATIVE_ALIGN {
            unsafe {
                let base = *(ptr as *mut *mut u8).sub(1);
                free(base);
            }
            return;
        }
        unsafe { free(ptr) };
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        log_c_str(c"realloc");
        if layout.align() > NATIVE_ALIGN {
            unsafe {
                let new_ptr =
                    self.alloc(Layout::from_size_align_unchecked(new_size, layout.align()));
                if new_ptr.is_null() {
                    return null_mut();
                }
                let copy_bytes = layout.size().min(new_size);
                let source = slice::from_raw_parts(ptr, copy_bytes);
                let dest = slice::from_raw_parts_mut(new_ptr, copy_bytes);
                dest.copy_from_slice(source);
                self.dealloc(ptr, layout);
                return new_ptr;
            }
        }

        unsafe { realloc(ptr, new_size) }
    }
}
