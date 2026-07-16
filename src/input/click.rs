use core::{ffi::c_void, marker::PhantomData, ops::RangeInclusive, time::Duration};

use alloc::boxed::Box;

use crate::{Button, log_c_str, sys};

pub type ClickCallback = Box<dyn FnMut(&ClickRecognizer) + 'static>;

#[derive(Default)]
pub struct ButtonClickConfig {
    pub(crate) single: Option<ClickCallback>,
    pub(crate) long_start: Option<ClickCallback>,
    pub(crate) long_release: Option<ClickCallback>,
    pub(crate) multi: Option<ClickCallback>,
}

use super::handlers::*;

#[derive(Default)]
pub struct ClickConfig {
    pub(crate) up: ButtonClickConfig,
    pub(crate) select: ButtonClickConfig,
    pub(crate) down: ButtonClickConfig,
    pub(crate) back: ButtonClickConfig,
}

fn duration_to_millis(duration: Duration) -> u16 {
    duration.as_millis().min(u16::MAX as u128) as u16
}

pub struct ClickConfigBuilder<'a> {
    handlers: &'a mut ClickConfig,
}

impl<'a> ClickConfigBuilder<'a> {
    pub(crate) unsafe fn new(handlers: &'a mut ClickConfig) -> Self {
        Self { handlers }
    }

    pub fn single(
        &mut self,
        button: Button,
        handler: impl FnMut(&ClickRecognizer) + 'static,
        repeat_after: Option<Duration>,
    ) {
        let (global_handler, click_config): (
            extern "C" fn(*mut c_void, *mut c_void),
            &mut ButtonClickConfig,
        ) = match button {
            Button::Back => (global_handle_click_single_back, &mut self.handlers.back),
            Button::Up => (global_handle_click_single_up, &mut self.handlers.up),
            Button::Select => (global_handle_click_single_select, &mut self.handlers.select),
            Button::Down => (global_handle_click_single_down, &mut self.handlers.down),
        };

        click_config.single = Some(Box::new(handler));

        if let Some(repeat) = repeat_after {
            unsafe {
                sys::window_single_repeating_click_subscribe(
                    button.into(),
                    duration_to_millis(repeat),
                    Some(global_handler),
                );
            }
        } else {
            unsafe {
                sys::window_single_click_subscribe(button.into(), Some(global_handler));
            }
        }
    }

    pub fn long(
        &mut self,
        button: Button,
        delay: Duration,
        start: impl FnMut(&ClickRecognizer) + 'static,
        release: impl FnMut(&ClickRecognizer) + 'static,
    ) {
        let (global_start_handler, global_release_handler, click_config): (
            extern "C" fn(*mut c_void, *mut c_void),
            extern "C" fn(*mut c_void, *mut c_void),
            &mut ButtonClickConfig,
        ) = match button {
            Button::Back => {
                log_c_str(c"Long press cannot be registered for Back button");
                return;
            }
            Button::Up => (
                global_handle_long_start_up,
                global_handle_long_release_up,
                &mut self.handlers.up,
            ),
            Button::Select => (
                global_handle_long_start_select,
                global_handle_long_release_select,
                &mut self.handlers.select,
            ),
            Button::Down => (
                global_handle_long_start_down,
                global_handle_long_release_down,
                &mut self.handlers.down,
            ),
        };

        click_config.long_start = Some(Box::new(start));
        click_config.long_release = Some(Box::new(release));

        unsafe {
            sys::window_long_click_subscribe(
                button.into(),
                duration_to_millis(delay),
                Some(global_start_handler),
                Some(global_release_handler),
            );
        }
    }

    pub fn multi(
        &mut self,
        button: Button,
        range: RangeInclusive<u8>,
        delay: Duration,
        handler: impl FnMut(&ClickRecognizer) + 'static,
    ) {
        let (global_handler, click_config): (
            extern "C" fn(*mut c_void, *mut c_void),
            &mut ButtonClickConfig,
        ) = match button {
            Button::Back => (global_handle_click_multi_back, &mut self.handlers.back),
            Button::Up => (global_handle_click_multi_up, &mut self.handlers.up),
            Button::Select => (global_handle_click_multi_select, &mut self.handlers.select),
            Button::Down => (global_handle_click_multi_down, &mut self.handlers.down),
        };

        click_config.multi = Some(Box::new(handler));

        unsafe {
            sys::window_multi_click_subscribe(
                button.into(),
                *range.start(),
                *range.end(),
                duration_to_millis(delay),
                true,
                Some(global_handler),
            );
        }
    }
}

pub struct ClickRecognizer<'a> {
    pub(crate) raw: sys::ClickRecognizerRef,
    pub(crate) phantom: PhantomData<&'a c_void>,
}

impl<'a> ClickRecognizer<'a> {
    pub fn click_count(&self) -> u8 {
        unsafe { sys::click_number_of_clicks_counted(self.raw) }
    }

    pub fn button(&self) -> Button {
        unsafe { sys::click_recognizer_get_button_id(self.raw) }.into()
    }
    pub fn repeating(&self) -> bool {
        unsafe { sys::click_recognizer_is_repeating(self.raw) }
    }
}
