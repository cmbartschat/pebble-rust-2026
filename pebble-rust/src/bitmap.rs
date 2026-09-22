use core::{cell::RefCell, ptr::NonNull};

use alloc::rc::Rc;

use crate::{GRect, GSize, key::ResourceId, sys};

pub(crate) struct BitmapInner {
    pub(crate) raw: NonNull<sys::GBitmap>,
    parent: Option<Bitmap>,
}

impl BitmapInner {
    pub(crate) unsafe fn from_ptr(ptr: *mut sys::GBitmap) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(ptr)?,
            parent: None,
        })
    }
}

impl Drop for BitmapInner {
    fn drop(&mut self) {
        unsafe { sys::gbitmap_destroy(self.raw.as_ptr()) };
    }
}

/// An animated or static raster image.
#[derive(Clone)]
pub struct Bitmap {
    pub(crate) handle: Rc<RefCell<BitmapInner>>,
}

impl Bitmap {
    /// Create a bitmap from a given app resource.
    pub fn from_resource(resource_id: ResourceId) -> Option<Self> {
        unsafe { Self::from_ptr(sys::gbitmap_create_with_resource(*resource_id)) }
    }

    /// Create a new empty bitmap with a specified [`BitmapFormat`].
    pub fn new_empty(size: GSize, format: BitmapFormat) -> Option<Self> {
        unsafe {
            Self::from_ptr(sys::gbitmap_create_blank(
                size,
                format as sys::GBitmapFormat,
            ))
        }
    }

    unsafe fn from_ptr(ptr: *mut sys::GBitmap) -> Option<Self> {
        unsafe {
            Some(Self {
                handle: Rc::new(RefCell::new(BitmapInner::from_ptr(ptr)?)),
            })
        }
    }

    fn from_inner(handle: BitmapInner) -> Self {
        Self {
            handle: Rc::new(RefCell::new(handle)),
        }
    }

    /// Creates a new bitmap from part of this bitmap, using the specified boundary rectangle.
    pub fn extract(&self, bounds: GRect) -> Option<Bitmap> {
        let mut inner = unsafe {
            let ptr = sys::gbitmap_create_as_sub_bitmap(self.handle.borrow().raw.as_ptr(), bounds);
            BitmapInner::from_ptr(ptr)?
        };
        inner.parent = Some(self.clone());
        Some(Self::from_inner(inner))
    }

    /// Returns the boundaries of this bitmap.
    pub fn get_bounds(&self) -> GRect {
        unsafe { sys::gbitmap_get_bounds(self.handle.borrow().raw.as_ptr()) }
    }
}

/// The bitmap pixel formats.
#[repr(u8)]
pub enum BitmapFormat {
    /// 1-bit black-and-white bitmap.
    OneBit = sys::GBitmapFormat_GBitmapFormat1Bit,
    /// 8-bit full-color bitmap.
    EightBit = sys::GBitmapFormat_GBitmapFormat8Bit,
    /// 1-bit two-color bitmap with pallette data.
    OneBitPalette = sys::GBitmapFormat_GBitmapFormat1BitPalette,
    /// 2-bit four-color bitmap with pallette data.
    TwoBitPalette = sys::GBitmapFormat_GBitmapFormat2BitPalette,
    /// 4-bit eight-color bitmap with pallette data.
    FourBitPalette = sys::GBitmapFormat_GBitmapFormat4BitPalette,
    /// 8-bit full-color circular bitmap, with a non-rectangular pixel arrangement.
    EightBitCircular = sys::GBitmapFormat_GBitmapFormat8BitCircular,
}
