use core::alloc::{GlobalAlloc, Layout};

pub struct Allocator;

unsafe extern "C" {
    pub unsafe fn malloc(size: usize) -> *mut u8;
    pub unsafe fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
    pub unsafe fn free(ptr: *mut u8);
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { malloc(layout.size()) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { free(ptr) };
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { realloc(ptr, new_size) }
    }
}

#[unsafe(no_mangle)]
extern "C" fn __rust_alloc(size: usize) -> *mut u8 {
    unsafe { malloc(size) }
}

#[unsafe(no_mangle)]
extern "C" fn __rust_dealloc(ptr: *mut u8) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
extern "C" fn __rust_realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
    unsafe { realloc(ptr, new_size) }
}

#[unsafe(no_mangle)]
extern "C" fn __rust_alloc_zeroed() -> *mut u8 {
    unsafe { malloc(0) }
}
