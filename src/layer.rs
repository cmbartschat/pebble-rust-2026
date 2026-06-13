use crate::{
    color::GCOLOR_BLACK,
    context::GContext,
    log::log_c_str,
    sys::{self, GRect, layer_destroy},
};

pub struct Layer {
    pub(crate) inner: *mut sys::Layer,
    pub(crate) owned: bool,
}

impl Drop for Layer {
    fn drop(&mut self) {
        if self.owned {
            unsafe { layer_destroy(self.inner) };
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn global_layer_update_handler(_layer: *mut sys::Layer, ctx: *mut sys::GContext) {
    log_c_str(c"global_layer_update_handler");
    let Ok(mut ctx) = GContext::from_raw(ctx) else {
        return;
    };

    ctx.set_fill_color(GCOLOR_BLACK);
    ctx.fill_rect(GRect::new(50, 50, 250, 250));
}

pub struct LayerCreateFailed;

impl Layer {
    pub fn new(r: GRect) -> Result<Self, LayerCreateFailed> {
        unsafe {
            let layer = sys::layer_create(r);
            if layer.is_null() {
                return Err(LayerCreateFailed);
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

    pub fn set_bounds(&mut self, bounds: GRect) {
        unsafe { sys::layer_set_bounds(self.inner, bounds) };
    }

    pub fn from_raw(inner: *mut sys::Layer) -> Self {
        Self {
            inner,
            owned: false,
        }
    }

    pub fn set_update_proc(
        &mut self,
        proc: unsafe extern "C" fn(layer: *mut sys::Layer, ctx: *mut sys::GContext),
    ) {
        unsafe { sys::layer_set_update_proc(self.inner, Some(proc)) };
    }
}

// type UpdateProc = Fn( *mut sys::Layer, *mut sys::GContext)
