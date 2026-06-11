use crate::{
    color::{GCOLOR_ARMY_GREEN, GCOLOR_BLACK},
    context::GContext,
    log::{log_num, log_str},
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
    log_num(1030);

    let Ok(mut ctx) = GContext::from_raw(ctx) else {
        log_num(1031);
        return;
    };
    log_num(1040);

    ctx.set_fill_color(GCOLOR_BLACK);
    ctx.fill_rect(GRect::new(50, 50, 250, 250));
}

impl Layer {
    pub fn new(r: GRect) -> Result<Self, ()> {
        unsafe {
            let layer = sys::layer_create(r);
            if layer.is_null() {
                log_num(10001);
                return Err(());
            }
            log_num(10010);
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
        log_num(5000);
        unsafe { sys::layer_set_update_proc(self.inner, Some(global_layer_update_handler)) };
    }
}
