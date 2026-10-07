//! The Pebble Rust 2026 SDK supplies two ready-to-use global allocators, both of which have different tradeoffs.
//! Each allocator is enabled with a feature flag, but neither is added as a global allocator by default.
//! See the allocator documentation on how to set them as a global allocator.
//!
//! - Pebble C `malloc` allocator: "malloc-allocator" feature, [`MallocAllocator`].
//!   Simpler to use, good for C interop, probably less efficient.
//! - [`embedded_alloc`] allocator: "embedded-allocator" feature, [`EmbeddedAllocator`].
//!   More efficient, requires explicit initialization, reserves a large heap chunk upfront.

#[cfg(feature = "embedded-allocator")]
mod embedded;
#[cfg(feature = "malloc-allocator")]
mod malloc;

#[cfg(feature = "malloc-allocator")]
pub use malloc::Allocator as MallocAllocator;

#[cfg(feature = "embedded-allocator")]
pub use embedded::Allocator as EmbeddedAllocator;

mod c_malloc {
    #[allow(unused)]
    unsafe extern "C" {
        pub unsafe fn malloc(size: usize) -> *mut u8;
        pub unsafe fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
        pub unsafe fn free(ptr: *mut u8);
        pub unsafe fn calloc(count: usize, size: usize) -> *mut u8;
    }
}
