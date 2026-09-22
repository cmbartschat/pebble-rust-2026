use core::alloc::{GlobalAlloc, Layout};
use embedded_alloc::LlffHeap as Heap;

use super::c_malloc::*;
#[allow(unused)]
use crate::{heap, log_c_str, log_fmt};

/// An allocator using an [`embedded_alloc::LlffHeap`].
/// Upon startup ([`Self::initialize`]), this allocator reserves up to `MAX_HEAP` bytes from the C heap.
///
/// It is entirely safe to set `MAX_HEAP` above the theoretical or practical memory capacity of a platform,
/// since the allocator will reserve less memory in that case.
/// The default is 120 KiB, which is pretty much the practical maximum on all current platforms.
/// Additionally, the allocator will always keep a small buffer reserved for C allocations to avoid crashing the application.
///
/// # Usage
///
/// To use this allocator in your program, first declare it as the global allocator.
/// Then, you need to call [`Self::initialize`] as soon as your application starts:
/// ```rust,no_run
/// # #![no_std]
/// # #![no_main]
/// use pebble_rust_2026::EmbeddedAllocator;
///
/// #[global_allocator]
/// static ALLOCATOR: EmbeddedAllocator = EmbeddedAllocator::new();
///
/// # fn app_main() {
/// // Should be the first thing in your main function.
/// ALLOCATOR.initialize();
/// // Now you can use the heap!
/// # }
/// ```
///
/// You may also modify the maximum size by declaring the static’s type accordingly:
/// ```rust,no_run
/// # #![no_std]
/// # #![no_main]
/// # use pebble_rust_2026::EmbeddedAllocator;
/// // 64 KiB heap maximum.
/// #[global_allocator]
/// static ALLOCATOR: EmbeddedAllocator<{ 1024 * 64 }> = EmbeddedAllocator::new();
/// ```
// TODO: Implement the upcoming stable Allocator trait, so this can be used for custom allocation as well.
pub struct Allocator<const MAX_HEAP: usize = { 1024 * 120 }> {
    heap: Heap,
}

/// How much memory to leave empty for other stuff.
/// If there’s too little remaining space, the program crashes.
const HEAP_BUFFER: usize = 2048;

impl<const MAX_HEAP: usize> Allocator<MAX_HEAP> {
    /// Create a new heap.
    pub const fn new() -> Self {
        Self {
            heap: Heap::empty(),
        }
    }

    /// Initialize the heap.
    ///
    /// # Panics
    ///
    /// This function panics when called more than once.
    /// As such, it is safe to call (even multiple times).
    pub fn initialize(&self) {
        // subtract buffer, then align to buffer size as well (hopefully improving malloc behavior)
        let free_heap = heap::bytes_free();
        let allocatable = free_heap - HEAP_BUFFER;
        let to_allocate = (allocatable - (allocatable % HEAP_BUFFER)).min(MAX_HEAP);

        assert!(to_allocate > 0);
        // SAFETY: `malloc` is safe to call with a nonzero argument.
        let heap_mem = unsafe { malloc(to_allocate) };
        if heap_mem.is_null() {
            log_c_str(c"EmbeddedAllocator: malloc returned null!");
            return;
        }

        unsafe {
            // SAFETY: We uphold the safety invariants of LlffHeap::init:
            // - "start_addr points to valid memory": heap_mem is not null, and therefore malloc guarantees it is valid.
            // - "size is correct": We requested to_allocate from malloc, and malloc guarantees that the allocation is at least of this size.
            self.heap.init(heap_mem as usize, to_allocate);

            #[cfg(debug_assertions)]
            {
                // SAFETY: We pass three arguments formatted as `long int`, which are i32s on ARM T32.
                log_fmt!(
                    c"EmbeddedAllocator: allocated heap at %lx size %ld from free %ld",
                    heap_mem as isize as i32,
                    to_allocate as i32,
                    free_heap as i32
                );
            }
        }
    }
}

// SAFETY: We defer to embedded-alloc’s allocator, whose implementation is sound.
unsafe impl<const MAX_HEAP: usize> GlobalAlloc for Allocator<MAX_HEAP> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { self.heap.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { self.heap.alloc_zeroed(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { self.heap.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { self.heap.realloc(ptr, layout, new_size) }
    }
}
