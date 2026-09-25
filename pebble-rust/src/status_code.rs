use crate::sys;

/// An error returned by some C APIs.
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

/// A success code returned by some C APIs.
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
#[repr(i8)]
pub enum StatusSuccess {
    /// Success; sometimes `false`.
    SuccessOrFalse = sys::StatusCode_S_SUCCESS,
    /// `true`
    True = sys::StatusCode_S_TRUE,
    /// No action was taken by the system.
    NoActionRequired = sys::StatusCode_S_NO_ACTION_REQUIRED,
    /// End of list, no more items.
    NoMoreItems = sys::StatusCode_S_NO_MORE_ITEMS,
}

pub(crate) const fn parse_status_result(
    value: sys::StatusCode,
) -> Result<StatusSuccess, StatusError> {
    match value {
        sys::StatusCode_S_NO_ACTION_REQUIRED => Ok(StatusSuccess::NoActionRequired),
        sys::StatusCode_S_NO_MORE_ITEMS => Ok(StatusSuccess::NoMoreItems),
        sys::StatusCode_S_SUCCESS => Ok(StatusSuccess::SuccessOrFalse),
        sys::StatusCode_S_TRUE => Ok(StatusSuccess::True),
        sys::StatusCode_E_AGAIN => Err(StatusError::Again),
        sys::StatusCode_E_BUSY => Err(StatusError::Busy),
        sys::StatusCode_E_DOES_NOT_EXIST => Err(StatusError::DoesNotExist),
        sys::StatusCode_E_ERROR => Err(StatusError::Error),
        sys::StatusCode_E_INTERNAL => Err(StatusError::Internal),
        sys::StatusCode_E_INVALID_ARGUMENT => Err(StatusError::InvalidArgument),
        sys::StatusCode_E_INVALID_OPERATION => Err(StatusError::InvalidOperation),
        sys::StatusCode_E_OUT_OF_MEMORY => Err(StatusError::OutOfMemory),
        sys::StatusCode_E_OUT_OF_RESOURCES => Err(StatusError::OutOfResources),
        sys::StatusCode_E_OUT_OF_STORAGE => Err(StatusError::OutOfStorage),
        sys::StatusCode_E_RANGE => Err(StatusError::Range),
        _ => Err(StatusError::Unknown),
    }
}
