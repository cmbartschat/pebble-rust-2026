use core::ffi::c_void;

use crate::{
    key::MessageKey,
    status_code::{StatusError, parse_status_result},
    sys,
};

/// Persistent (non-volatile) app storage.
pub struct Persist;

impl Persist {
    /// Write a boolean value to persistent storage.
    pub fn write_bool(&self, key: MessageKey, value: bool) {
        unsafe {
            sys::persist_write_bool(*key, value);
        }
    }
    /// Read a boolean value from persistent storage.
    pub fn read_bool(&self, key: MessageKey) -> Option<bool> {
        unsafe {
            if !sys::persist_exists(*key) {
                return None;
            }
            Some(sys::persist_read_bool(*key))
        }
    }

    /// Write an integer value to storage.
    pub fn write_int(&self, key: MessageKey, value: i32) {
        unsafe {
            sys::persist_write_int(*key, value);
        }
    }
    /// Read an integer value from persistent storage.
    pub fn read_int(&self, key: MessageKey) -> Option<i32> {
        unsafe {
            if !sys::persist_exists(*key) {
                return None;
            }
            Some(sys::persist_read_int(*key))
        }
    }

    /// Delete the value associated with a key from persistent storage.
    pub fn delete(&self, key: MessageKey) {
        unsafe { sys::persist_delete(*key) };
    }

    /// Write bytes to persistent storage.
    pub fn write_bytes(&self, key: MessageKey, value: &[u8]) -> Result<(), StatusError> {
        unsafe {
            let result =
                sys::persist_write_data(*key, value.as_ptr() as *const c_void, value.len());
            parse_status_result(result.min(0))?;
            Ok(())
        }
    }
    /// Read bytes from persistent storage.
    pub fn read_bytes<'a>(
        &self,
        key: MessageKey,
        target: &'a mut [u8],
    ) -> Result<Option<&'a mut [u8]>, StatusError> {
        unsafe {
            if !sys::persist_exists(*key) {
                return Ok(None);
            }
            let result = sys::persist_read_data(*key, target.as_ptr() as *mut c_void, target.len());
            parse_status_result(result.min(0))?;

            Ok(Some(&mut target[0..result as usize]))
        }
    }
}
