use core::{ffi::c_void, marker::PhantomData};

use crate::{
    ClickConfig, ClickConfigBuilder,
    input::{
        click::{ClickCallback, ClickRecognizer},
        context::InputContext,
    },
    sys,
};

fn dispatch_click(recognizer: *mut c_void, handler: &mut Option<ClickCallback>) {
    if let Some(callback) = handler {
        let recognizer = recognizer as sys::ClickRecognizerRef;
        let recognizer = ClickRecognizer {
            raw: recognizer,
            phantom: PhantomData,
        };
        callback(&recognizer);
    }
}

pub extern "C" fn global_click_config_handler(context: *mut c_void) {
    let context = unsafe { (context as *mut InputContext).as_mut() }.unwrap();
    if let Some(configure) = &mut context.configure_click {
        let mut handle = unsafe { ClickConfigBuilder::new(&mut context.config) };
        configure(&mut handle);
    }
}

pub extern "C" fn global_handle_click_single_up(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.up.single);
}

pub extern "C" fn global_handle_click_single_select(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.select.single);
}

pub extern "C" fn global_handle_click_single_down(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.down.single);
}

pub extern "C" fn global_handle_click_single_back(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.back.single);
}

pub extern "C" fn global_handle_click_multi_up(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.up.multi);
}

pub extern "C" fn global_handle_click_multi_select(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.select.multi);
}

pub extern "C" fn global_handle_click_multi_down(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.down.multi);
}

pub extern "C" fn global_handle_click_multi_back(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.back.multi);
}

pub extern "C" fn global_handle_long_start_up(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.up.long_start);
}

pub extern "C" fn global_handle_long_start_select(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.select.long_start);
}

pub extern "C" fn global_handle_long_start_down(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.down.long_start);
}

pub extern "C" fn global_handle_long_release_up(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.up.long_release);
}

pub extern "C" fn global_handle_long_release_select(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.select.long_release);
}

pub extern "C" fn global_handle_long_release_down(recognizer: *mut c_void, context: *mut c_void) {
    let context = unsafe { (context as *mut ClickConfig).as_mut() }.unwrap();
    dispatch_click(recognizer, &mut context.down.long_release);
}
