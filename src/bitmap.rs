use core::{marker::PhantomData, ptr::slice_from_raw_parts_mut};

use crate::{
    GRect, GSize,
    sys::{self, GBitmapFormat, gbitmap_destroy},
};

pub struct GBitmap<'parent> {
    pub(crate) inner: *mut sys::GBitmap,
    _parent: PhantomData<&'parent GBitmap<'parent>>,
}

/*

gbitmap_get_bytes_per_row
gbitmap_get_format
gbitmap_get_data
gbitmap_set_data
gbitmap_get_bounds
gbitmap_set_bounds
gbitmap_get_palette
gbitmap_set_palette
gbitmap_create_with_data
gbitmap_create_as_sub_bitmap
gbitmap_create_from_png_data
gbitmap_create_blank
gbitmap_create_blank_with_palette
gbitmap_create_palettized_from_1bit
gbitmap_destroy
*/

impl GBitmap<'static> {
    pub fn from_resource(resource_id: u32) -> Result<Self, ()> {
        Self::from_ptr(unsafe { sys::gbitmap_create_with_resource(resource_id) })
    }

    pub fn new_empty(size: GSize, format: GBitmapFormat) -> Result<Self, ()> {
        Self::from_ptr(unsafe { sys::gbitmap_create_blank(size, format) })
    }
}

impl<'a> Drop for GBitmap<'a> {
    fn drop(&mut self) {
        unsafe { gbitmap_destroy(self.inner) };
    }
}

impl<'parent> GBitmap<'parent> {
    fn from_ptr(ptr: *mut sys::GBitmap) -> Result<Self, ()> {
        if ptr.is_null() {
            return Err(());
        }
        Ok(Self {
            inner: ptr,
            _parent: PhantomData,
        })
    }

    pub fn extract<'a>(&'a self, bounds: sys::GRect) -> Result<GBitmap<'a>, ()> {
        let ptr = unsafe { sys::gbitmap_create_as_sub_bitmap(self.inner, bounds) };
        Self::from_ptr(ptr)
    }

    pub fn get_data(&mut self) -> Result<&mut [u8], ()> {
        unsafe {
            let bytes_per_row = sys::gbitmap_get_bytes_per_row(self.inner) as usize;
            let rows = sys::gbitmap_get_bounds(self.inner).size.h as usize;
            let len = bytes_per_row * rows;
            let data = sys::gbitmap_get_data(self.inner);
            if data.is_null() {
                return Err(());
            }
            match slice_from_raw_parts_mut(data, len).as_mut() {
                Some(e) => Ok(e),
                None => Err(()),
            }
        }
    }

    pub fn get_bounds(&self) -> GRect {
        unsafe { sys::gbitmap_get_bounds(self.inner) }
    }
}
