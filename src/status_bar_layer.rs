use core::{ffi::c_uint, ptr::NonNull};

use crate::{
    Layer,
    handle::{Handle, new_handle},
    layer::{ChildLayer, LayerInner},
    sys::{self, GColor},
};

struct StatusBarLayerInner {
    base_layer: Layer,
    raw: NonNull<sys::StatusBarLayer>,
}

#[derive(Clone)]
pub struct StatusBarLayer {
    handle: Handle<StatusBarLayerInner>,
}

impl ChildLayer for StatusBarLayer {
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

impl StatusBarLayer {
    pub fn new() -> Option<Self> {
        unsafe {
            let raw = NonNull::new(sys::status_bar_layer_create())?;

            let base = LayerInner::from_ptr(sys::status_bar_layer_get_layer(raw.as_ptr()), false);
            let Some(base_layer) = base else {
                sys::status_bar_layer_destroy(raw.as_ptr());
                return None;
            };

            Some(Self {
                handle: new_handle(StatusBarLayerInner {
                    raw,
                    base_layer: Layer {
                        handle: new_handle(base_layer),
                    },
                }),
            })
        }
    }

    pub fn get_background_color(&self) -> GColor {
        unsafe { sys::status_bar_layer_get_background_color(self.handle.borrow().raw.as_ptr()) }
    }

    pub fn get_foreground_color(&self) -> GColor {
        unsafe { sys::status_bar_layer_get_background_color(self.handle.borrow().raw.as_ptr()) }
    }

    pub fn set_background_color(&self) -> GColor {
        unsafe { sys::status_bar_layer_get_background_color(self.handle.borrow().raw.as_ptr()) }
    }

    pub fn set_colors(&mut self, foreground: GColor, background: GColor) {
        unsafe {
            sys::status_bar_layer_set_colors(
                self.handle.borrow_mut().raw.as_ptr(),
                foreground,
                background,
            )
        }
    }

    pub fn set_separator_mode(&mut self, mode: StatusBarSeparatorMode) {
        unsafe {
            sys::status_bar_layer_set_separator_mode(
                self.handle.borrow_mut().raw.as_ptr(),
                mode.into(),
            )
        }
    }
}

pub enum StatusBarSeparatorMode {
    None,
    Dotted,
}

impl From<StatusBarSeparatorMode> for c_uint {
    fn from(value: StatusBarSeparatorMode) -> c_uint {
        match value {
            StatusBarSeparatorMode::None => {
                sys::StatusBarLayerSeparatorMode_StatusBarLayerSeparatorModeNone
            }
            StatusBarSeparatorMode::Dotted => {
                sys::StatusBarLayerSeparatorMode_StatusBarLayerSeparatorModeDotted
            }
        }
    }
}

impl Drop for StatusBarLayerInner {
    fn drop(&mut self) {
        unsafe { sys::status_bar_layer_destroy(self.raw.as_ptr()) };
    }
}
