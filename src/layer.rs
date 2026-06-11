use crate::{
    color::{GCOLOR_ARMY_GREEN, GCOLOR_BLACK},
    context::GContext,
    sys::{self, GPoint, GRect, GSize, layer_destroy},
};

pub struct Layer {
    pub(crate) inner: *mut sys::Layer,
    pub(crate) owned: bool,
}

impl Drop for Layer {
    fn drop(&mut self) {
        if (self.owned) {
            unsafe { layer_destroy(self.inner) };
        }
    }
}

extern "C" fn global_layer_update_handler(layer: *mut sys::Layer, ctx: *mut sys::GContext) {
    let Ok(mut ctx) = GContext::from_raw(ctx) else {
        return;
    };

    ctx.set_fill_color(GCOLOR_BLACK);
    ctx.fill_rect(GRect {
        origin: GPoint { x: 3, y: 3 },
        size: GSize { w: 60, h: 30 },
    });
}

impl Layer {
    pub fn new(r: GRect) -> Result<Self, ()> {
        unsafe {
            let layer = sys::layer_create(r);
            if layer.is_null() {
                return Err(());
            }
            Ok(Self {
                inner: layer,
                owned: true,
            })
        }
    }

    pub fn add_child(&mut self, other: &Self) {
        unsafe { sys::layer_add_child(self.inner, other.inner) };
    }

    pub fn mark_dirty(&mut self) {
        unsafe { sys::layer_mark_dirty(self.inner) };
    }

    pub fn set_update(&mut self) {
        unsafe { sys::layer_set_update_proc(self.inner, Some(global_layer_update_handler)) };
    }
}
