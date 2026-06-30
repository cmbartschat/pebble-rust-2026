use core::{
    ffi::{CStr, c_uint, c_void},
    pin::Pin,
    ptr::{NonNull, addr_of_mut, null_mut},
    str::FromStr,
};

use alloc::{boxed::Box, ffi::CString, vec::Vec};

use crate::{
    color::{GCOLOR_BLACK, GCOLOR_DUKE_BLUE},
    handle::{Handle, new_handle},
    log_c_str,
    sys::{self, GColor},
};

struct ActionData {
    label: Pin<Box<CStr>>,
    callback: Option<Box<dyn FnOnce() + 'static>>,
}

impl ActionData {
    pub(crate) fn new(label: &str, callback: impl FnOnce() + 'static) -> Self {
        let label = CString::from_str(label).ok().unwrap_or_default();
        let label = Pin::new(label.into_boxed_c_str());
        Self {
            label,
            callback: Some(Box::new(callback)),
        }
    }
}

struct ActionMenuInner {
    raw: NonNull<sys::ActionMenu>,
}

struct ActionMenuChildLevel {
    label: Pin<Box<CStr>>,
    level: ActionMenuLevel,
}

enum ActionMenuItem {
    ChildLevel(ActionMenuChildLevel),
    Action(Box<ActionData>),
}
impl From<ActionData> for ActionMenuItem {
    fn from(value: ActionData) -> Self {
        ActionMenuItem::Action(Box::new(value))
    }
}

impl From<ActionMenuChildLevel> for ActionMenuItem {
    fn from(value: ActionMenuChildLevel) -> Self {
        ActionMenuItem::ChildLevel(value)
    }
}

pub struct ActionMenuLevel {
    items: Vec<ActionMenuItem>, // raw: NonNull<sys::ActionMenuLevel>,
}

impl Default for ActionMenuLevel {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionMenuLevel {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_child(&mut self, label: &str, child: Self) {
        let label = CString::from_str(label).ok().unwrap_or_default();
        let label = Pin::new(label.into_boxed_c_str());
        self.items.push(
            ActionMenuChildLevel {
                label,
                level: child,
            }
            .into(),
        );
    }

    pub fn add_action(&mut self, label: &str, callback: impl FnOnce() + 'static) {
        self.items.push(ActionData::new(label, callback).into());
    }

    pub(crate) unsafe fn as_raw(&self) -> *mut sys::ActionMenuLevel {
        let level = unsafe { sys::action_menu_level_create(self.items.len() as u16) };

        for item in self.items.iter() {
            match item {
                ActionMenuItem::ChildLevel(child) => unsafe {
                    sys::action_menu_level_add_child(
                        level,
                        child.level.as_raw(),
                        child.label.as_ptr(),
                    );
                },
                ActionMenuItem::Action(action_data) => unsafe {
                    let context: *const ActionData = Box::as_ref(action_data);
                    sys::action_menu_level_add_action(
                        level,
                        action_data.label.as_ptr(),
                        Some(global_handle_action_perform),
                        context as *mut c_void,
                    );
                },
            }
        }

        level
    }
}

pub struct ActionMenuBuilder {
    foreground_color: GColor,
    background_color: GColor,
    level: ActionMenuLevel,
    align: ActionMenuAlign,
}

impl ActionMenuBuilder {
    pub fn set_foreground_color(mut self, color: GColor) -> Self {
        self.foreground_color = color;
        self
    }

    pub fn set_background_color(mut self, color: GColor) -> Self {
        self.background_color = color;
        self
    }

    pub fn set_align(mut self, align: ActionMenuAlign) -> Self {
        self.align = align;
        self
    }

    pub fn open(self) -> Option<ActionMenu> {
        ActionMenu::open(self)
    }
}

struct ActionMenuContext {
    levels: Pin<Box<ActionMenuLevel>>,
}

pub struct ActionMenu {
    handle: Handle<ActionMenuInner>,
}

impl ActionMenu {
    pub fn begin(level: ActionMenuLevel) -> ActionMenuBuilder {
        ActionMenuBuilder {
            foreground_color: GCOLOR_BLACK,
            background_color: GCOLOR_DUKE_BLUE,
            level,
            align: ActionMenuAlign::Top,
        }
    }

    pub(crate) fn open(builder: ActionMenuBuilder) -> Option<Self> {
        let context = Box::new(ActionMenuContext {
            levels: Box::pin(builder.level),
        });
        let mut config = sys::ActionMenuConfig {
            root_level: unsafe { context.levels.as_raw() },
            context: Box::into_raw(context) as *mut c_void,
            colors: sys::ActionMenuConfig__bindgen_ty_1 {
                background: builder.background_color,
                foreground: builder.foreground_color,
            },
            will_close: None,
            did_close: Some(global_handle_action_menu_did_close),
            align: builder.align.into(),
        };
        let action_menu = unsafe { sys::action_menu_open(addr_of_mut!(config)) };

        Some(Self {
            handle: new_handle(ActionMenuInner {
                raw: NonNull::new(action_menu)?,
            }),
        })
    }

    fn close_inner(&mut self, animated: bool) {
        unsafe {
            sys::action_menu_close(self.handle.borrow_mut().raw.as_ptr(), animated);
        }
    }

    pub fn close(&mut self) {
        self.close_inner(true);
    }

    pub fn close_immediate(&mut self) {
        self.close_inner(false);
    }
}

#[derive(Clone, Copy)]
pub enum ActionMenuAlign {
    Top,
    Center,
}

impl From<ActionMenuAlign> for c_uint {
    fn from(value: ActionMenuAlign) -> Self {
        match value {
            ActionMenuAlign::Top => sys::ActionMenuAlign_ActionMenuAlignTop,
            ActionMenuAlign::Center => sys::ActionMenuAlign_ActionMenuAlignCenter,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ActionMenuLevelDisplayMode {
    Wide,
    Thin,
}

impl From<ActionMenuLevelDisplayMode> for c_uint {
    fn from(value: ActionMenuLevelDisplayMode) -> Self {
        match value {
            ActionMenuLevelDisplayMode::Wide => {
                sys::ActionMenuLevelDisplayMode_ActionMenuLevelDisplayModeWide
            }
            ActionMenuLevelDisplayMode::Thin => {
                sys::ActionMenuLevelDisplayMode_ActionMenuLevelDisplayModeThin
            }
        }
    }
}

extern "C" fn global_handle_action_menu_did_close(
    _menu: *mut sys::ActionMenu,
    _performed_action: *const sys::ActionMenuItem,
    context: *mut c_void,
) {
    let root_level = unsafe { sys::action_menu_get_root_level(_menu) };
    unsafe { sys::action_menu_hierarchy_destroy(root_level, None, null_mut()) };
    drop(unsafe { Box::from_raw(context as *mut ActionMenuContext) });
}

extern "C" fn global_handle_action_perform(
    _menu: *mut sys::ActionMenu,
    action: *const sys::ActionMenuItem,
    _context: *mut c_void,
) {
    let data_ptr = unsafe { sys::action_menu_item_get_action_data(action) };
    if data_ptr.is_null() {
        log_c_str(c"unexpected perform on null action");
        return;
    }
    let mut data = unsafe { Box::from_raw(data_ptr as *mut ActionData) };
    if let Some(callback) = data.callback.take() {
        callback();
    }
    Box::leak(data); // Will be cleaned up in global_handle_action_menu_did_close
}
