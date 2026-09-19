use core::{cell::RefCell, ptr::NonNull};

use alloc::rc::Rc;

use crate::{
    CompOp, GAlign, GColor, GPoint, GRect, Layer,
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

    fn id(&self) -> usize {
        self.handle.borrow().base_layer.id()
    }

    fn ptr_to_child_with(&mut self) -> *mut sys::Layer {
        self.handle.borrow_mut().base_layer.ptr_to_child_with()
    }

    fn record_new_parent(&self, parent: &Layer) {
        self.handle
            .borrow_mut()
            .base_layer
            .record_new_parent(parent);
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

    pub fn remove(&mut self) {
        ChildLayer::remove_from_parent(self);
    }

    pub fn get_hidden(&self) -> bool {
        self.handle.borrow().base_layer.get_hidden()
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        self.handle.borrow_mut().base_layer.set_hidden(hidden)
    }

    pub fn get_unobstructed_bounds(&self) -> GRect {
        self.handle.borrow().base_layer.get_unobstructed_bounds()
    }

    pub fn convert_point_to_screen(&self, point: GPoint) -> GPoint {
        self.handle
            .borrow_mut()
            .base_layer
            .convert_point_to_screen(point)
    }

    pub fn convert_rect_to_screen(&self, rect: GRect) -> GRect {
        self.handle.borrow().base_layer.convert_rect_to_screen(rect)
    }

    pub fn set_compositing_mode(&mut self, mode: CompOp) {
        unsafe {
            sys::bitmap_layer_set_compositing_mode(
                self.handle.borrow_mut().raw.as_ptr(),
                mode as u8,
            )
        };
    }

    pub fn set_alignment(&mut self, align: GAlign) {
        unsafe {
            sys::bitmap_layer_set_alignment(self.handle.borrow_mut().raw.as_ptr(), align as u8)
        };
    }

    pub fn set_background_color(&mut self, color: GColor) {
        unsafe {
            sys::bitmap_layer_set_background_color(self.handle.borrow_mut().raw.as_ptr(), color)
        };
    }
}
