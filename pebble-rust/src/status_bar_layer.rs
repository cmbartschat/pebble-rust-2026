use core::ptr::NonNull;

use crate::{
    GColor, GPoint, GRect, Layer,
    handle::{Handle, new_handle},
    layer::{ChildLayer, LayerInner},
    sys,
};

struct StatusBarLayerInner {
    base_layer: Layer,
    raw: NonNull<sys::StatusBarLayer>,
}

/// A status bar at the top that shows at least the current time.
/// See also the [Pebble documentation](https://developer.repebble.com/guides/user-interfaces/layers/#statusbarlayer)
#[derive(Clone)]
pub struct StatusBarLayer {
    handle: Handle<StatusBarLayerInner>,
}

impl ChildLayer for StatusBarLayer {
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

impl StatusBarLayer {
    /// Create a new status bar layer.
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

    /// Returns the background color of the status bar.
    pub fn get_background_color(&self) -> GColor {
        unsafe { sys::status_bar_layer_get_background_color(self.handle.borrow().raw.as_ptr()) }
    }

    /// Returns the foreground color of the status bar.
    pub fn get_foreground_color(&self) -> GColor {
        unsafe { sys::status_bar_layer_get_background_color(self.handle.borrow().raw.as_ptr()) }
    }

    /// Sets the background color of the status bar.
    pub fn set_background_color(&self) -> GColor {
        unsafe { sys::status_bar_layer_get_background_color(self.handle.borrow().raw.as_ptr()) }
    }

    /// Sets both colors of the status bar.
    pub fn set_colors(&mut self, foreground: GColor, background: GColor) {
        unsafe {
            sys::status_bar_layer_set_colors(
                self.handle.borrow_mut().raw.as_ptr(),
                foreground,
                background,
            )
        }
    }

    /// Sets how elements in the status bar are separated, see [`StatusBarSeparatorMode`].
    pub fn set_separator_mode(&mut self, mode: StatusBarSeparatorMode) {
        unsafe {
            sys::status_bar_layer_set_separator_mode(
                self.handle.borrow_mut().raw.as_ptr(),
                mode as u8,
            )
        }
    }

    /// Returns whether the layer is hidden or not.
    pub fn is_hidden(&self) -> bool {
        self.handle.borrow().base_layer.is_hidden()
    }

    /// Hides/shows the layer.
    pub fn set_hidden(&mut self, hidden: bool) {
        self.handle.borrow_mut().base_layer.set_hidden(hidden)
    }

    /// Convert a point in layer coordinates to screen coordinates.
    pub fn convert_point_to_screen(&self, point: GPoint) -> GPoint {
        self.handle
            .borrow_mut()
            .base_layer
            .convert_point_to_screen(point)
    }

    /// Convert a rectangle in layer coordinates to screen coordinates.
    pub fn convert_rect_to_screen(&self, rect: GRect) -> GRect {
        self.handle.borrow().base_layer.convert_rect_to_screen(rect)
    }
}

/// How elements in a status bar are separated.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusBarSeparatorMode {
    /// Not separated.
    None = sys::StatusBarLayerSeparatorMode_StatusBarLayerSeparatorModeNone,
    /// A dotted separator at the bottom of the status bar.s
    Dotted = sys::StatusBarLayerSeparatorMode_StatusBarLayerSeparatorModeDotted,
}

impl Drop for StatusBarLayerInner {
    fn drop(&mut self) {
        unsafe { sys::status_bar_layer_destroy(self.raw.as_ptr()) };
    }
}
