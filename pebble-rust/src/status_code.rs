use crate::sys;

/// An error returned by some C APIs.
// NOTE: Unfortunately this has to be a public API, since some C APIs don’t specify what errors they return exactly and why.
//       So we have to assume that these APIs could return any StatusError, and it’s more convenient to just expose the type.
//       Whenever a C API limits its (usual) error codes, create a custom error type instead.
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
#[repr(i8)]
pub enum StatusError {
    /// Try operation again.
    Again = sys::StatusCode_E_AGAIN,
    /// System is busy.
    Busy = sys::StatusCode_E_BUSY,
    /// Requested resource does not exist.
    DoesNotExist = sys::StatusCode_E_DOES_NOT_EXIST,
    /// Other error.
    Error = sys::StatusCode_E_ERROR,
    /// Internal error.
    Internal = sys::StatusCode_E_INTERNAL,
    /// Invalid argument.
    InvalidArgument = sys::StatusCode_E_INVALID_ARGUMENT,
    /// Invalid operation.
    InvalidOperation = sys::StatusCode_E_INVALID_OPERATION,
    /// Out of memory.
    OutOfMemory = sys::StatusCode_E_OUT_OF_MEMORY,
    /// Out of resources.
    OutOfResources = sys::StatusCode_E_OUT_OF_RESOURCES,
    /// Out of permanent storage.
    OutOfStorage = sys::StatusCode_E_OUT_OF_STORAGE,
    /// Value out of range.
    Range = sys::StatusCode_E_RANGE,
    /// Unknown.
    Unknown = sys::StatusCode_E_UNKNOWN,
}

impl TryFrom<i32> for StatusError {
    /// Returns the original status code.
    type Error = i32;

    /// Attempts to parse only known errors from the C return code.
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        sys::StatusCode::try_from(value)
            .map_err(|_| value)
            .and_then(|value| Self::try_from(value).map_err(|err| err as _))
    }
}

impl TryFrom<sys::StatusCode> for StatusError {
    /// Returns the original status code.
    type Error = sys::StatusCode;

    /// Attempts to parse only known errors from the C return code.
    fn try_from(value: sys::StatusCode) -> Result<Self, <Self as TryFrom<i8>>::Error> {
        match value {
            sys::StatusCode_E_AGAIN => Ok(Self::Again),
            sys::StatusCode_E_BUSY => Ok(Self::Busy),
            sys::StatusCode_E_DOES_NOT_EXIST => Ok(Self::DoesNotExist),
            sys::StatusCode_E_ERROR => Ok(Self::Error),
            sys::StatusCode_E_INTERNAL => Ok(Self::Internal),
            sys::StatusCode_E_INVALID_ARGUMENT => Ok(Self::InvalidArgument),
            sys::StatusCode_E_INVALID_OPERATION => Ok(Self::InvalidOperation),
            sys::StatusCode_E_OUT_OF_MEMORY => Ok(Self::OutOfMemory),
            sys::StatusCode_E_OUT_OF_RESOURCES => Ok(Self::OutOfResources),
            sys::StatusCode_E_OUT_OF_STORAGE => Ok(Self::OutOfStorage),
            sys::StatusCode_E_RANGE => Ok(Self::Range),
            _ => Err(value),
        }
    }
}

/// A success code returned by some C APIs.
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
#[repr(i8)]
pub(crate) enum StatusSuccess {
    /// Success; sometimes `false`.
    SuccessOrFalse = sys::StatusCode_S_SUCCESS,
    /// `true`
    True = sys::StatusCode_S_TRUE,
    /// No action was taken by the system.
    NoActionRequired = sys::StatusCode_S_NO_ACTION_REQUIRED,
    /// End of list, no more items.
    NoMoreItems = sys::StatusCode_S_NO_MORE_ITEMS,
}

/// Parses the return value from many C functions that return a negative code (represented in [`StatusError`])
/// or a positive success (represented in [`StatusSuccess`]).
/// Despite both of them being `singed char` in C, the functions usually return `int`, so this function does too.
/// It will yield [`StatusError::Unknown`] if the value is out-of-range.
///
/// **Important:** If the C function returns no [`StatusSuccess`] values on success,
/// you need to call this with `result_code.min(0)`, as unknown positive result values will yield an unknown error!
pub(crate) fn parse_status_result(value: i32) -> Result<StatusSuccess, StatusError> {
    let byte_value = i8::try_from(value).map_err(|_| StatusError::Unknown)?;
    match StatusError::try_from(byte_value) {
        Ok(error) => Err(error),
        Err(_) => match byte_value {
            sys::StatusCode_S_NO_ACTION_REQUIRED => Ok(StatusSuccess::NoActionRequired),
            sys::StatusCode_S_NO_MORE_ITEMS => Ok(StatusSuccess::NoMoreItems),
            sys::StatusCode_S_SUCCESS => Ok(StatusSuccess::SuccessOrFalse),
            sys::StatusCode_S_TRUE => Ok(StatusSuccess::True),
            _ => Err(StatusError::Unknown),
        },
    }
}
