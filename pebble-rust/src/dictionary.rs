use core::{ffi::CStr, marker::PhantomData, ptr::NonNull};

use alloc::slice;

use crate::{key::MessageKey, sys};

/// Errors that can happen when writing to a [`DictionaryView`].
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum DictionaryWriteError {
    /// Other error.
    Unknown = 1,
    /// Not enough storage in the dictionary.
    NotEnoughStorage = sys::DictionaryResult_DICT_NOT_ENOUGH_STORAGE,
    /// Invalid arguments supplied.
    InvalidArgs = sys::DictionaryResult_DICT_INVALID_ARGS,
}

/// Result type for writing to a [`DictionaryView`]
pub type DictionaryWriteResult = Result<(), DictionaryWriteError>;

const fn to_write_result(v: sys::DictionaryResult) -> Result<(), DictionaryWriteError> {
    Err(match v {
        sys::DictionaryResult_DICT_OK => return Ok(()),
        sys::DictionaryResult_DICT_NOT_ENOUGH_STORAGE => DictionaryWriteError::NotEnoughStorage,
        sys::DictionaryResult_DICT_INVALID_ARGS => DictionaryWriteError::InvalidArgs,
        _ => DictionaryWriteError::Unknown,
    })
}

/// Values stored in a dictionary.
pub enum Value<'a> {
    /// Byte array.
    Bytes(&'a [u8]),
    /// String.
    CStr(&'a CStr),
    /// Unsigned integer.
    Uint(u32),
    /// Integer.
    Int(i32),
}

impl From<&'_ Value<'_>> for Option<u32> {
    fn from(val: &'_ Value<'_>) -> Self {
        val.as_u32()
    }
}

impl Value<'_> {
    /// Returns the unsigned integer in this value if possible.
    pub const fn as_u32(&self) -> Option<u32> {
        match self {
            Self::Bytes(_) => None,
            Self::CStr(_) => None,
            Self::Uint(v) => Some(*v),
            Self::Int(v) if *v >= 0i32 => Some(*v as u32),
            Self::Int(_v) => None,
        }
    }
}

impl<'a> From<u32> for Value<'a> {
    fn from(value: u32) -> Self {
        Self::Uint(value)
    }
}
impl<'a> From<u16> for Value<'a> {
    fn from(value: u16) -> Self {
        Self::Uint(value as _)
    }
}
impl<'a> From<u8> for Value<'a> {
    fn from(value: u8) -> Self {
        Self::Uint(value as _)
    }
}
impl<'a> From<i32> for Value<'a> {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}
impl<'a> From<i16> for Value<'a> {
    fn from(value: i16) -> Self {
        Self::Int(value as _)
    }
}
impl<'a> From<i8> for Value<'a> {
    fn from(value: i8) -> Self {
        Self::Int(value as _)
    }
}
impl<'a> From<&'a [u8]> for Value<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::Bytes(value)
    }
}
impl<'a> From<&'a CStr> for Value<'a> {
    fn from(value: &'a CStr) -> Self {
        Self::CStr(value)
    }
}

/// A key-value dictionary tuple.
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

    /// Returns the key of this tuple.
    pub const fn key(&self) -> u32 {
        unsafe { self.raw.as_ref() }.key
    }

    const unsafe fn extract_bytes(&self) -> &'a [u8] {
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

    /// Returns the value for this tuple.
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

impl<'a> From<Tuple<'a>> for (u32, Value<'a>) {
    fn from(val: Tuple<'a>) -> Self {
        (val.key(), val.value())
    }
}

/// A collection of [`Tuple`]s.
pub struct Tuples<'a> {
    pub(crate) raw: NonNull<sys::DictionaryIterator>,
    pub(crate) p: PhantomData<&'a mut DictionaryView>,
    first: bool,
}

impl<'a> Iterator for Tuples<'a> {
    type Item = (u32, Value<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.first {
            self.first = false;
            unsafe { sys::dict_read_first(self.raw.as_ptr()) }
        } else {
            unsafe { sys::dict_read_next(self.raw.as_ptr()) }
        };
        Tuple::from_raw(next).map(Into::into)
    }
}

/// A dictionary view for a key-value store like persistent watch storage.
/// This implements [`IntoIterator`], so you can simply iterate over a mutable reference of it.
pub struct DictionaryView {
    raw: NonNull<sys::DictionaryIterator>,
}

impl DictionaryView {
    /// Create a new view from the raw C object.
    pub fn from_raw(raw: *mut sys::DictionaryIterator) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(raw)?,
        })
    }

    /// Return a value from a key.
    pub fn get(&self, key: MessageKey) -> Option<Value<'_>> {
        let next = unsafe { sys::dict_find(self.raw.as_ptr(), *key) };
        Tuple::from_raw(next).map(|e| e.value())
    }

    const fn iter(&mut self) -> Tuples<'_> {
        Tuples {
            raw: self.raw,
            p: PhantomData,
            first: true,
        }
    }
}

impl<'a> IntoIterator for &'a mut DictionaryView {
    type Item = (u32, Value<'a>);
    type IntoIter = Tuples<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// A builder for a dictionary, effectively the inverse of [`DictionaryView`].
pub struct DictionaryBuilder {
    raw: NonNull<sys::DictionaryIterator>,
}

impl DictionaryBuilder {
    pub(crate) fn from_ptr(inner: *mut sys::DictionaryIterator) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(inner)?,
        })
    }

    /// Set a certain message key to a certain value.
    /// You can pass anything that is allowed as a value to this function, e.g. byte slices, integers, and C strings.
    // Value lifetime doesn’t have to outlive this builder’s lifetime, since the C API copies it in every case.
    pub fn set<'s, 'v>(
        &'s mut self,
        key: MessageKey,
        value: impl Into<Value<'v>>,
    ) -> DictionaryWriteResult {
        let value = value.into();
        match value {
            Value::Bytes(items) => self.write_bytes(key, items),
            Value::CStr(cstr) => self.write_cstr(key, cstr),
            Value::Uint(uint) if uint <= u8::MAX as _ => self.write_u8(key, uint as u8),
            Value::Uint(uint) if uint <= u16::MAX as _ => self.write_u16(key, uint as u16),
            Value::Int(int) if i8::MIN as i32 <= int && int <= i8::MAX as _ => {
                self.write_i8(key, int as i8)
            }
            Value::Int(int) if i16::MIN as i32 <= int && int <= i16::MAX as _ => {
                self.write_i16(key, int as i16)
            }
            Value::Uint(uint) => self.write_u32(key, uint),
            Value::Int(int) => self.write_i32(key, int),
        }
    }
    fn write_bytes(&mut self, key: MessageKey, value: &[u8]) -> DictionaryWriteResult {
        let Ok(size) = u16::try_from(value.len()) else {
            return Err(DictionaryWriteError::InvalidArgs);
        };
        to_write_result(unsafe {
            sys::dict_write_data(self.raw.as_ptr(), *key, value.as_ptr(), size)
        })
    }
    fn write_cstr(&mut self, key: MessageKey, value: &CStr) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_cstring(self.raw.as_ptr(), *key, value.as_ptr()) })
    }
    fn write_u8(&mut self, key: MessageKey, value: u8) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_uint8(self.raw.as_ptr(), *key, value) })
    }
    fn write_u16(&mut self, key: MessageKey, value: u16) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_uint16(self.raw.as_ptr(), *key, value) })
    }
    fn write_u32(&mut self, key: MessageKey, value: u32) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_uint32(self.raw.as_ptr(), *key, value) })
    }
    fn write_i8(&mut self, key: MessageKey, value: i8) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_int8(self.raw.as_ptr(), *key, value) })
    }
    fn write_i16(&mut self, key: MessageKey, value: i16) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_int16(self.raw.as_ptr(), *key, value) })
    }
    fn write_i32(&mut self, key: MessageKey, value: i32) -> DictionaryWriteResult {
        to_write_result(unsafe { sys::dict_write_int32(self.raw.as_ptr(), *key, value) })
    }
}
