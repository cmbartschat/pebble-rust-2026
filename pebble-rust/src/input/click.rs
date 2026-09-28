use core::{ffi::c_void, marker::PhantomData, ops::RangeInclusive, time::Duration};

use alloc::boxed::Box;

use crate::{Button, log_c_str, sys};

pub(crate) type ClickCallback = Box<dyn FnMut(&ClickRecognizer) + 'static>;

#[derive(Default)]
pub struct ButtonClickConfig {
    pub(crate) single: Option<ClickCallback>,
    pub(crate) long_start: Option<ClickCallback>,
    pub(crate) long_release: Option<ClickCallback>,
    pub(crate) multi: Option<ClickCallback>,
}

use super::handlers::*;

/// A click configuration, created by [`ClickConfigBuilder`].
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

/// A builder for button (click) handlers on layers and windows.
///
/// Layers and windows usually have a `set_click_provider` function that you give a callback to.
/// That callback receives a mutable reference to this builder.
/// Using various methods on this builder, you can setup many handlers for various button interactions.
pub struct ClickConfigBuilder<'a> {
    handlers: &'a mut ClickConfig,
}

impl<'a> ClickConfigBuilder<'a> {
    pub(crate) const unsafe fn new(handlers: &'a mut ClickConfig) -> Self {
        Self { handlers }
    }

    /// Set the handler for clicking the given button exactly once.
    /// If there is no long click handler, and `repeat_after` has been set, this handler is repeatedly called after `repeat_after` has elapsed.
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
                    button as u8,
                    duration_to_millis(repeat),
                    Some(global_handler),
                );
            }
        } else {
            unsafe {
                sys::window_single_click_subscribe(button as u8, Some(global_handler));
            }
        }
    }

    /// Set the handler for long-clicking the given button.
    /// The `start` handler is called after the button has been held down for `delay`; set delay to zero to use the system default.
    /// The `release` handler is called when the button is released.
    /// This disables dispatching long—clicks to the [`Self::single`] handler for this button.
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
                button as u8,
                duration_to_millis(delay),
                Some(global_start_handler),
                Some(global_release_handler),
            );
        }
    }

    /// Set the handler for clicking the given button multiple times.
    /// The `range` determines how many times the button has to be clicked at minimum and maximum for the handler to be called.
    /// The `delay` determines when the multi-click sequence is reset, after the last button click has been registered,
    /// use None for the system default.
    pub fn multi(
        &mut self,
        button: Button,
        range: RangeInclusive<u8>,
        delay: Option<Duration>,
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

        let mut min_clicks = *range.start();
        let mut max_clicks = *range.end();
        if min_clicks < 2 {
            log_c_str(c"At least 2 clicks are required for the multi-click handler");
            min_clicks = 2;
        }
        if max_clicks < min_clicks {
            log_c_str(c"Maximum clicks must not be smaller than minimum clicks");
            // As per C API: "A value of 0 means use "min" also as "max"."
            max_clicks = 0;
        }

        click_config.multi = Some(Box::new(handler));

        unsafe {
            sys::window_multi_click_subscribe(
                button as u8,
                min_clicks,
                max_clicks,
                duration_to_millis(delay.unwrap_or(Duration::ZERO)),
                true,
                Some(global_handler),
            );
        }
    }
}

/// A recognizer for certain click patterns.
/// This is passed to click callbacks.
pub struct ClickRecognizer<'a> {
    pub(crate) raw: sys::ClickRecognizerRef,
    pub(crate) phantom: PhantomData<&'a c_void>,
}

impl<'a> ClickRecognizer<'a> {
    /// The number of clicks recognized.
    pub fn click_count(&self) -> u8 {
        unsafe { sys::click_number_of_clicks_counted(self.raw) }
    }
    /// The button that was clicked.
    pub fn button(&self) -> Button {
        unsafe { sys::click_recognizer_get_button_id(self.raw) }.into()
    }
    /// Whether the button was repeated.
    pub fn repeating(&self) -> bool {
        unsafe { sys::click_recognizer_is_repeating(self.raw) }
    }
}
