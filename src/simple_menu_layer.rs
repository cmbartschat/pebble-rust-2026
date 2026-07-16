use core::{
    ffi::c_void,
    pin::Pin,
    ptr::{NonNull, null_mut},
    str::FromStr,
};

use alloc::{boxed::Box, ffi::CString, vec::Vec};

use crate::{
    Bitmap, GRect, Layer, Window,
    handle::{Handle, new_handle},
    input::context::InputReceiver,
    layer::{ChildLayer, LayerInner},
    log_c_str, sys,
    window::WeakWindow,
};

struct SimpleMenuLayerRaw {
    raw: NonNull<sys::SimpleMenuLayer>,
}

impl SimpleMenuLayerRaw {
    pub unsafe fn new(
        frame: GRect,
        window: &mut Window,
        options: &[sys::SimpleMenuSection],
        context: *mut c_void,
    ) -> Option<Self> {
        let raw = {
            window
                .handle
                .borrow_mut()
                .create_simple_menu_layer(frame, options, context)
        };
        let raw = NonNull::new(raw)?;
        Some(Self { raw })
    }

    pub unsafe fn get_base_layer(&self) -> Option<Layer> {
        unsafe {
            Some(Layer {
                handle: new_handle(LayerInner::from_ptr(
                    sys::simple_menu_layer_get_layer(self.raw.as_ptr()),
                    false,
                )?),
            })
        }
    }
}

impl Drop for SimpleMenuLayerRaw {
    fn drop(&mut self) {
        unsafe { sys::simple_menu_layer_destroy(self.raw.as_ptr()) };
    }
}

struct SimpleMenuLayerInner {
    base_layer: Layer,
    #[allow(dead_code)]
    raw: SimpleMenuLayerRaw,
    #[allow(dead_code)]
    sys_options: Pin<Box<[sys::SimpleMenuSection]>>,
    #[allow(dead_code)]
    context: Pin<Box<SimpleMenuContext>>,
    window: WeakWindow,
}

#[derive(Clone)]
pub struct SimpleMenuLayer {
    handle: Handle<SimpleMenuLayerInner>,
}

impl ChildLayer for SimpleMenuLayer {
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

pub struct SimpleMenuItem {
    title: CString,
    subtitle: Option<CString>,
    icon: Option<Bitmap>,
    callback: Box<dyn FnMut() + 'static>,
}

extern "C" fn global_simple_menu_select_handler(index: i32, context: *mut c_void) {
    let context = unsafe { (context as *mut SimpleMenuContext).as_mut() }.unwrap();
    let mut count: i32 = 0;
    for section in context.sections.iter_mut() {
        for item in section.items.iter_mut() {
            if count == index {
                item.callback.as_mut()();
                return;
            }
            count += 1;
        }
    }
    log_c_str(c"no matched callback for index");
}

impl SimpleMenuItem {
    pub fn new(
        title: &str,
        subtitle: Option<&str>,
        icon: Option<Bitmap>,
        callback: impl FnMut() + 'static,
    ) -> Self {
        Self {
            title: CString::from_str(title).expect("Invalid title passed to SimpleMenuItem"),
            subtitle: subtitle
                .map(|e| CString::from_str(e).expect("Invalid subtitle passed to SimpleMenuItem")),
            icon,
            callback: Box::new(callback),
        }
    }

    pub(crate) fn as_sys(&self) -> sys::SimpleMenuItem {
        sys::SimpleMenuItem {
            title: self.title.as_c_str().as_ptr(),
            subtitle: self
                .subtitle
                .as_ref()
                .map_or(null_mut(), |f| f.as_c_str().as_ptr()),
            icon: self
                .icon
                .as_ref()
                .map_or(null_mut(), |f| f.handle.borrow().raw.as_ptr()),
            callback: Some(global_simple_menu_select_handler),
        }
    }
}

pub struct SimpleMenuSection {
    title: Option<CString>,
    items: Vec<SimpleMenuItem>,
}

impl SimpleMenuSection {
    pub fn new(title: &str) -> Self {
        Self {
            title: Some(
                CString::from_str(title).expect("Invalid title passed to SimpleMenuSection"),
            ),
            items: Vec::new(),
        }
    }

    pub fn new_untitled() -> Self {
        Self {
            title: None,
            items: Vec::new(),
        }
    }

    pub fn push(&mut self, item: SimpleMenuItem) {
        self.items.push(item);
    }

    pub fn as_sys(
        &self,
        context: &mut Vec<Pin<Box<[sys::SimpleMenuItem]>>>,
    ) -> sys::SimpleMenuSection {
        let items = self.items.iter().map(|f| f.as_sys()).collect::<Vec<_>>();
        let pinned = Pin::new(items.into_boxed_slice());
        let res = sys::SimpleMenuSection {
            title: self
                .title
                .as_ref()
                .map_or(null_mut(), |f| f.as_c_str().as_ptr()),
            items: pinned.as_ptr(),
            num_items: pinned.len() as u32,
        };
        context.push(pinned);

        res
    }
}

struct SimpleMenuContext {
    sections: Pin<Box<[SimpleMenuSection]>>,
    #[allow(dead_code)]
    item_lists: Vec<Pin<Box<[sys::SimpleMenuItem]>>>,
}

impl SimpleMenuLayer {
    pub fn new(
        frame: GRect,
        window: &mut Window,
        options: Box<[SimpleMenuSection]>,
    ) -> Option<Self> {
        unsafe {
            let options = Pin::new(options);
            let mut item_lists: Vec<Pin<Box<[sys::SimpleMenuItem]>>> = Vec::new();
            let sys_options: Vec<_> = options.iter().map(|f| f.as_sys(&mut item_lists)).collect();
            let ctx = SimpleMenuContext {
                sections: options,
                item_lists,
            };

            let sys_options: Pin<Box<[sys::SimpleMenuSection]>> =
                Pin::new(sys_options.into_boxed_slice());
            let mut context = Box::pin(ctx);
            let context_ptr: &mut SimpleMenuContext = &mut context;
            let raw = SimpleMenuLayerRaw::new(
                frame,
                window,
                &sys_options,
                context_ptr as *mut SimpleMenuContext as *mut c_void,
            )?;
            let base_layer = raw.get_base_layer()?;

            let res = Self {
                handle: new_handle(SimpleMenuLayerInner {
                    raw,
                    base_layer,
                    sys_options,
                    context,
                    window: window.downgrade(),
                }),
            };

            window.retain_input_receiver(res.clone());

            Some(res)
        }
    }

    pub fn detach(&mut self) {
        let inner = self.handle.borrow_mut();
        inner.base_layer.remove_from_parent();
        if let Some(mut window) = inner.window.upgrade() {
            window.remove_input_receiver(self)
        }
    }
}

impl InputReceiver for SimpleMenuLayer {
    fn get_id(&self) -> usize {
        self.handle.borrow().raw.raw.as_ptr() as usize
    }
}
