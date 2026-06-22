use alloc::boxed::Box;

use crate::{ClickConfig, ClickConfigBuilder};

pub type ClickConfigureCallback = dyn Fn(&mut ClickConfigBuilder) + 'static;

#[derive(Default)]
pub struct InputContext {
    pub(crate) configure_click: Option<Box<ClickConfigureCallback>>,
    pub(crate) config: ClickConfig,
}
