use core::{ffi::CStr, marker::PhantomData, ptr::NonNull};

use alloc::slice;

use crate::{key::MessageKey, sys};

#[derive(Debug)]
pub enum DictionaryWriteError {
    NotEnoughStorage,
    InvalidArgs,
    Unknown,
}

pub type DictionaryWriteResult = Result<(), DictionaryWriteError>;

fn to_write_result(v: sys::DictionaryResult) -> Result<(), DictionaryWriteError> {
    Err(match v {
        sys::DictionaryResult_DICT_OK => return Ok(()),
        sys::DictionaryResult_DICT_NOT_ENOUGH_STORAGE => DictionaryWriteError::NotEnoughStorage,
        sys::DictionaryResult_DICT_INVALID_ARGS => DictionaryWriteError::InvalidArgs,
        _ => DictionaryWriteError::Unknown,
    })
}

pub enum Value<'a> {
    Bytes(&'a [u8]),
    CStr(&'a CStr),
    Uint(u32),
    Int(i32),
}

impl Value<'_> {
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Self::Bytes(_) => None,
            Self::CStr(_) => None,
            Self::Uint(v) => Some(*v),
            Self::Int(v) if *v >= 0i32 => Some(*v as u32),
            Self::Int(_v) => None,
        }
    }
}

pub struct Tuple<'a> {
    raw: NonNull<sys::Tuple>,
    tuple: PhantomData<&'a sys::Tuple>,
}

impl<'a> Tuple<'a> {
    pub(crate) fn from_raw(raw: *mut sys::Tuple) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(raw)?,
            tuple: PhantomData,
        })
    }

    pub fn key(&self) -> u32 {
        unsafe { self.raw.as_ref() }.key
    }

    unsafe fn extract_bytes(&self) -> &'a [u8] {
        unsafe {
            let raw_tuple: &sys::Tuple = self.raw.as_ref();
            let base_addr = core::ptr::addr_of!(raw_tuple.value) as *const u8;
            slice::from_raw_parts(base_addr, raw_tuple.length as usize)
        }
    }

    unsafe fn extract_c_str(&self) -> &'a CStr {
        CStr::from_bytes_with_nul(unsafe { self.extract_bytes() }).expect("Invalid string in tuple")
    }

    unsafe fn extract_uint(&self) -> u32 {
        unsafe {
            let (length, val) = {
                let raw_tuple: &sys::Tuple = self.raw.as_ref();

                let base_addr = core::ptr::addr_of!(raw_tuple.value);
                let sized_value = base_addr as *const sys::Tuple__bindgen_ty_1;
                (raw_tuple.length, core::ptr::read_unaligned(sized_value))
            };
            match length {
                1 => u8::from_le(*val.uint8.as_ref()) as u32,
                2 => u16::from_le(*val.uint16.as_ref()) as u32,
                4 => u32::from_le(*val.uint32.as_ref()),
                _ => panic!("Bad length for uint"),
            }
        }
    }

    unsafe fn extract_int(&self) -> i32 {
        unsafe {
            let (length, val) = {
                let raw_tuple: &sys::Tuple = self.raw.as_ref();

                let base_addr = core::ptr::addr_of!(raw_tuple.value);
                let sized_value = base_addr as *const sys::Tuple__bindgen_ty_1;
                (raw_tuple.length, core::ptr::read_unaligned(sized_value))
            };
            match length {
                1 => i8::from_le(*val.int8.as_ref()) as i32,
                2 => i16::from_le(*val.int16.as_ref()) as i32,
                4 => i32::from_le(*val.int32.as_ref()),
                _ => panic!("Bad length for int"),
            }
        }
    }

    pub fn value(&self) -> Value<'a> {
        match unsafe { self.raw.as_ref().type_() } {
            sys::TupleType_TUPLE_BYTE_ARRAY => Value::Bytes(unsafe { self.extract_bytes() }),
            sys::TupleType_TUPLE_CSTRING => Value::CStr(unsafe { self.extract_c_str() }),
            sys::TupleType_TUPLE_UINT => Value::Uint(unsafe { self.extract_uint() }),
            sys::TupleType_TUPLE_INT => Value::Int(unsafe { self.extract_int() }),
            _ => panic!("No valid type"),
        }
    }
}

pub struct Tuples<'a> {
    pub(crate) raw: NonNull<sys::DictionaryIterator>,
    pub(crate) p: PhantomData<&'a mut DictionaryView>,
    first: bool,
}

impl<'a> Iterator for Tuples<'a> {
    type Item = Tuple<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.first {
            self.first = false;
            unsafe { sys::dict_read_first(self.raw.as_ptr()) }
        } else {
            unsafe { sys::dict_read_next(self.raw.as_ptr()) }
        };
        Tuple::from_raw(next)
    }
}

pub struct DictionaryView {
    raw: NonNull<sys::DictionaryIterator>,
}

impl DictionaryView {
    pub fn from_raw(raw: *mut sys::DictionaryIterator) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(raw)?,
        })
    }

    pub fn get(&self, key: MessageKey) -> Option<Value<'_>> {
        let next = unsafe { sys::dict_find(self.raw.as_ptr(), *key) };
        Tuple::from_raw(next).map(|e| e.value())
    }

    pub fn iter(&mut self) -> Tuples<'_> {
        Tuples {
            raw: self.raw,
            p: PhantomData,
            first: true,
        }
    }
}

pub struct DictionaryBuilder {
    raw: NonNull<sys::DictionaryIterator>,
}

impl DictionaryBuilder {
    pub(crate) fn from_ptr(inner: *mut sys::DictionaryIterator) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(inner)?,
        })
    }

    pub fn write_bytes(&mut self, key: MessageKey, value: &[u8]) -> DictionaryWriteResult {
        let Ok(size) = u16::try_from(value.len()) else {
            return Err(DictionaryWriteError::InvalidArgs);
        };
        to_write_result(unsafe {
            sys::dict_write_data(self.raw.as_ptr(), *key, value.as_ptr(), size)
        })
    }
    pub fn write_cstr(&mut self, key: MessageKey, value: &CStr) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_cstring(self.raw.as_ptr(), *key, value.as_ptr()) })
    }
    pub fn write_u8(&mut self, key: MessageKey, value: u8) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_uint8(self.raw.as_ptr(), *key, value) })
    }
    pub fn write_u16(&mut self, key: MessageKey, value: u16) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_uint16(self.raw.as_ptr(), *key, value) })
    }
    pub fn write_u32(&mut self, key: MessageKey, value: u32) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_uint32(self.raw.as_ptr(), *key, value) })
    }
    pub fn write_i8(&mut self, key: MessageKey, value: i8) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_int8(self.raw.as_ptr(), *key, value) })
    }
    pub fn write_i16(&mut self, key: MessageKey, value: i16) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_int16(self.raw.as_ptr(), *key, value) })
    }
    pub fn write_i32(&mut self, key: MessageKey, value: i32) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_int32(self.raw.as_ptr(), *key, value) })
    }
}
