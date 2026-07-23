use crate::sys;

#[derive(Copy, Clone, Debug, Hash, PartialEq)]
#[repr(i8)]
pub enum StatusError {
    Again = sys::StatusCode_E_AGAIN,
    Busy = sys::StatusCode_E_BUSY,
    DoesNotExist = sys::StatusCode_E_DOES_NOT_EXIST,
    Error = sys::StatusCode_E_ERROR,
    Internal = sys::StatusCode_E_INTERNAL,
    InvalidArgument = sys::StatusCode_E_INVALID_ARGUMENT,
    InvalidOperation = sys::StatusCode_E_INVALID_OPERATION,
    OutOfMemory = sys::StatusCode_E_OUT_OF_MEMORY,
    OutOfResources = sys::StatusCode_E_OUT_OF_RESOURCES,
    OutOfStorage = sys::StatusCode_E_OUT_OF_STORAGE,
    Range = sys::StatusCode_E_RANGE,
    Unknown = sys::StatusCode_E_UNKNOWN,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq)]
#[repr(i8)]
pub enum StatusSuccess {
    SuccessOrFalse = sys::StatusCode_S_SUCCESS,
    True = sys::StatusCode_S_TRUE,
    NoActionRequired = sys::StatusCode_S_NO_ACTION_REQUIRED,
    NoMoreItems = sys::StatusCode_S_NO_MORE_ITEMS,
}

pub type StatusResult = Result<StatusSuccess, StatusError>;

pub fn parse_status_result(value: sys::StatusCode) -> Result<StatusSuccess, StatusError> {
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
