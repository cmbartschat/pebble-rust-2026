use core::{cell::RefCell, ptr::NonNull};

use alloc::rc::Rc;

use crate::{
    GRect, Layer,
    bitmap::Bitmap,
    layer::{ChildLayer, LayerInner},
    sys,
};

struct BitmapLayerInner {
    raw: NonNull<sys::BitmapLayer>,
    base_layer: Layer,
    bitmap: Option<Bitmap>,
}

impl Drop for BitmapLayerInner {
    fn drop(&mut self) {
        unsafe { sys::bitmap_layer_destroy(self.raw.as_ptr()) };
    }
}

#[derive(Clone)]
pub struct BitmapLayer {
    handle: Rc<RefCell<BitmapLayerInner>>,
}

impl ChildLayer for BitmapLayer {
    fn remove_from_parent(&self) {
        self.handle.borrow_mut().base_layer.remove_from_parent();
    }

    fn is_same(&self, other: &Layer) -> bool {
        self.handle.borrow().base_layer.is_same(other)
    }

    fn set_parent(&mut self, other: &mut Layer) {
        self.handle.borrow_mut().base_layer.set_parent(other);
    }
}

impl BitmapLayer {
    pub fn new(r: GRect) -> Option<Self> {
        unsafe {
            let raw = NonNull::new(sys::bitmap_layer_create(r))?;
            let base = LayerInner::from_ptr(sys::bitmap_layer_get_layer(raw.as_ptr()), false);
            let Some(base_layer) = base else {
                sys::bitmap_layer_destroy(raw.as_ptr());
                return None;
            };

            Some(Self {
                handle: Rc::new(RefCell::new(BitmapLayerInner {
                    raw,
                    bitmap: None,
                    base_layer: Layer {
                        handle: Rc::new(RefCell::new(base_layer)),
                    },
                })),
            })
        }
    }

    fn inner_mut(&mut self, f: impl FnOnce(&mut BitmapLayerInner)) {
        let mut inner = self.handle.borrow_mut();
        f(&mut inner);
    }

    pub fn set_bitmap(&mut self, bitmap: &Bitmap) {
        self.inner_mut(|inner| {
            unsafe {
                sys::bitmap_layer_set_bitmap(
                    inner.raw.as_ptr(),
                    bitmap.handle.borrow().raw.as_ptr(),
                )
            };
            inner.bitmap = Some(bitmap.clone());
        });
    }
}
