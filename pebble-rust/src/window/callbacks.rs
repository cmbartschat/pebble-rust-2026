use crate::{APP, sys, window::user_data::WindowUserData};

pub extern "C" fn global_handle_load(window: *mut sys::Window) {
    unsafe {
        let void_ptr = sys::window_get_user_data(window);
        let user_data_ptr = void_ptr as *mut WindowUserData;
        let Some(data) = user_data_ptr.as_mut() else {
            panic!("Window does not have a user data");
        };
        data
    }
    .dispatch_load();
}

pub extern "C" fn global_handle_appear(window: *mut sys::Window) {
    unsafe {
        let void_ptr = sys::window_get_user_data(window);
        let user_data_ptr = void_ptr as *mut WindowUserData;
        let Some(data) = user_data_ptr.as_mut() else {
            panic!("Window does not have a user data");
        };
        data
    }
    .dispatch_appear();
}

pub extern "C" fn global_handle_disappear(window: *mut sys::Window) {
    unsafe {
        let void_ptr = sys::window_get_user_data(window);
        let user_data_ptr = void_ptr as *mut WindowUserData;
        let Some(data) = user_data_ptr.as_mut() else {
            panic!("Window does not have a user data");
        };
        data
    }
    .dispatch_disappear();
}

pub extern "C" fn global_handle_unload(window: *mut sys::Window) {
    unsafe {
        let void_ptr = sys::window_get_user_data(window);
        let user_data_ptr = void_ptr as *mut WindowUserData;
        let Some(data) = user_data_ptr.as_mut() else {
            panic!("Window does not have a user data");
        };
        data
    }
    .dispatch_unload();

    APP.notify_unload(window);
}
