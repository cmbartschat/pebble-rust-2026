use core::{ffi::c_void, pin::Pin, ptr::NonNull};

use alloc::{boxed::Box, rc::Rc};

use crate::{
    ClickConfigBuilder, ClickRecognizer, ContentIndicator, GPoint, GRect, GSize, Layer, Window,
    handle::{Handle, WeakHandle, new_handle},
    input::{
        context::{InputContext, InputReceiver},
        handlers::global_click_config_handler,
    },
    layer::{ChildLayer, LayerInner},
    sys::{self, ScrollLayerCallbacks},
    window::WeakWindow,
};

struct ScrollLayerRaw {
    raw: NonNull<sys::ScrollLayer>,
}

impl From<NonNull<sys::ScrollLayer>> for ScrollLayerRaw {
    fn from(value: NonNull<sys::ScrollLayer>) -> Self {
        Self { raw: value }
    }
}

impl ScrollLayerRaw {
    fn as_ptr(&self) -> *const sys::ScrollLayer {
        self.raw.as_ptr()
    }

    fn as_ptr_mut(&mut self) -> *mut sys::ScrollLayer {
        self.raw.as_ptr()
    }
}

impl Drop for ScrollLayerRaw {
    fn drop(&mut self) {
        unsafe { sys::scroll_layer_destroy(self.raw.as_ptr()) }
    }
}

struct ScrollLayerInner {
    content_indicator: Option<ContentIndicator>,
    shadow_hidden: bool,
    base_layer: Layer,
    raw: ScrollLayerRaw,
    pub(crate) input_context: Pin<Box<InputContext>>,
    attached_window: Option<WeakWindow>,
}

impl ScrollLayerInner {
    fn as_ptr(&self) -> *const sys::ScrollLayer {
        self.raw.as_ptr()
    }

    fn as_ptr_mut(&mut self) -> *mut sys::ScrollLayer {
        self.raw.as_ptr_mut()
    }

    fn get_or_create_content_indicator(&mut self) -> Option<&mut ContentIndicator> {
        if self.content_indicator.is_none() {
            self.content_indicator = ContentIndicator::from_ptr(
                unsafe { sys::scroll_layer_get_content_indicator(self.as_ptr_mut()) },
                false,
            );
        }
        self.content_indicator.as_mut()
    }
}

#[derive(Clone)]
pub struct ScrollLayer {
    handle: Handle<ScrollLayerInner>,
}

impl ChildLayer for ScrollLayer {
    fn remove_from_parent(&self) {
        self.handle.borrow_mut().base_layer.remove_from_parent();
        let mut inner = self.handle.borrow_mut();
        if let Some(mut window) = inner.attached_window.take().and_then(|e| e.upgrade()) {
            window.remove_input_receiver(self);
        }
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

impl ScrollLayer {
    pub fn new(r: GRect) -> Option<Self> {
        unsafe {
            let raw = NonNull::new(sys::scroll_layer_create(r))?;

            let base = LayerInner::from_ptr(sys::scroll_layer_get_layer(raw.as_ptr()), false);
            let Some(base_layer) = base else {
                sys::scroll_layer_destroy(raw.as_ptr());
                return None;
            };

            let mut input_context = Box::new(InputContext::default());
            sys::scroll_layer_set_context(
                raw.as_ptr(),
                input_context.as_mut() as *mut InputContext as *mut c_void,
            );

            sys::scroll_layer_set_callbacks(
                raw.as_ptr(),
                ScrollLayerCallbacks {
                    click_config_provider: Some(global_click_config_handler),
                    content_offset_changed_handler: None,
                },
            );

            Some(Self {
                handle: new_handle(ScrollLayerInner {
                    content_indicator: None,
                    shadow_hidden: false,
                    raw: raw.into(),
                    base_layer: Layer {
                        handle: new_handle(base_layer),
                    },
                    input_context: Box::into_pin(input_context),
                    attached_window: None,
                }),
            })
        }
    }

    fn inner_mut<T>(&mut self, f: impl FnOnce(&mut ScrollLayerInner) -> T) -> T {
        let mut inner = self.handle.borrow_mut();
        f(&mut inner)
    }

    pub fn get_frame(&self) -> GRect {
        self.handle.borrow().base_layer.get_frame()
    }

    pub fn set_frame(&mut self, frame: GRect) {
        unsafe { sys::scroll_layer_set_frame(self.as_ptr_mut(), frame) };
    }

    fn as_ptr(&self) -> *const sys::ScrollLayer {
        self.handle.borrow().as_ptr()
    }

    fn as_ptr_mut(&self) -> *mut sys::ScrollLayer {
        self.handle.borrow_mut().as_ptr_mut()
    }

    pub fn add_child<T>(&mut self, child: &mut T)
    where
        T: Clone + ChildLayer + 'static,
    {
        self.inner_mut(|inner| {
            child.record_new_parent(&inner.base_layer);
            inner.base_layer.handle.borrow_mut().retain_child(child);
            unsafe {
                sys::scroll_layer_add_child(inner.as_ptr_mut(), child.ptr_to_child_with());
            }
        });
    }

    pub fn get_content_size(&self) -> GSize {
        unsafe { sys::scroll_layer_get_content_size(self.as_ptr()) }
    }

    pub fn set_content_size(&mut self, size: GSize) {
        unsafe { sys::scroll_layer_set_content_size(self.as_ptr_mut(), size) };
    }

    pub fn set_click_config_onto_window(&mut self, window: &mut Window) {
        let extra = self.clone();
        self.inner_mut(|f| {
            f.attached_window = Some(window.downgrade());
            let mut window_inner = window.handle.borrow_mut();
            window_inner.set_scroll_layer_click_config(f.as_ptr_mut());
            window_inner.retain_input_receiver(extra);
        })
    }

    pub fn get_shadow_hidden(&mut self) -> bool {
        unsafe { sys::scroll_layer_get_shadow_hidden(self.handle.borrow().raw.as_ptr()) }
    }

    pub fn set_shadow_hidden(&mut self, hidden: bool) {
        self.inner_mut(|inner| {
            unsafe { sys::scroll_layer_set_shadow_hidden(inner.as_ptr_mut(), hidden) };
            inner.shadow_hidden = hidden;
        })
    }

    pub fn get_paging(&self) -> bool {
        unsafe { sys::scroll_layer_get_paging(self.as_ptr_mut()) }
    }

    pub fn set_paging(&mut self, enabled: bool) {
        self.inner_mut(|inner| {
            unsafe { sys::scroll_layer_set_paging(inner.as_ptr_mut(), enabled) };
            if !enabled {
                unsafe {
                    sys::scroll_layer_set_shadow_hidden(inner.as_ptr_mut(), inner.shadow_hidden)
                };
            }
        })
    }

    pub fn get_content_offset(&self) -> GPoint {
        unsafe { sys::scroll_layer_get_content_offset(self.as_ptr_mut()) }
    }

    pub fn _set_content_offset(&mut self, point: GPoint, animated: bool) {
        unsafe { sys::scroll_layer_set_content_offset(self.as_ptr_mut(), point, animated) };
    }

    pub fn set_content_offset(&mut self, point: GPoint) {
        self._set_content_offset(point, true);
    }

    pub fn set_content_offset_immediate(&mut self, point: GPoint) {
        self._set_content_offset(point, false);
    }

    pub fn set_click_provider(&mut self, builder: impl Fn(&mut ClickConfigBuilder) + 'static) {
        self.inner_mut(|inner| {
            inner.input_context.configure_click = Some(Box::new(builder));
        });
    }

    pub fn up_click_handler(&mut self, click: &ClickRecognizer) {
        unsafe {
            sys::scroll_layer_scroll_up_click_handler(click.raw, self.as_ptr_mut() as *mut c_void)
        };
    }

    pub fn down_click_handler(&mut self, click: &ClickRecognizer) {
        unsafe {
            sys::scroll_layer_scroll_down_click_handler(
                click.raw,
                self.as_ptr_mut() as *mut c_void,
            );
        }
    }

    pub fn remove(&mut self) {
        ChildLayer::remove_from_parent(self);
    }

    pub fn with_content_indicator(&mut self, f: impl FnOnce(&mut ContentIndicator)) {
        self.inner_mut(|inner| {
            if let Some(indicator) = inner.get_or_create_content_indicator() {
                f(indicator);
            }
        })
    }

    pub fn downgrade(&self) -> WeakScrollLayer {
        WeakScrollLayer::from(self)
    }
}

impl InputReceiver for ScrollLayer {
    fn get_id(&self) -> usize {
        self.handle.as_ptr() as usize
    }
}

#[derive(Clone)]
pub struct WeakScrollLayer {
    handle: WeakHandle<ScrollLayerInner>,
}

impl WeakScrollLayer {
    pub fn from(layer: &ScrollLayer) -> Self {
        Self {
            handle: Rc::downgrade(&layer.handle),
        }
    }
    pub fn upgrade(&self) -> Option<ScrollLayer> {
        Some(ScrollLayer {
            handle: self.handle.upgrade()?,
        })
    }
}
