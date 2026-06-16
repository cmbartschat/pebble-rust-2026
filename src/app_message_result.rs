use crate::{dictionary::DictionaryWriteError, sys};

#[derive(Copy, Clone, Debug)]
pub enum AppMessageError {
    SendTimeout,
    SendRejected,
    NotConnected,
    AppNotRunning,
    InvalidArgs,
    Busy,
    BufferOverflow,
    AlreadyReleased,
    CallbackAlreadyRegistered,
    CallbackNotRegistered,
    OutOfMemory,
    Closed,
    InternalError,
    InvalidState,
    Unknown,
}

impl From<DictionaryWriteError> for AppMessageError {
    fn from(value: DictionaryWriteError) -> Self {
        match value {
            DictionaryWriteError::NotEnoughStorage => Self::OutOfMemory,
            DictionaryWriteError::InvalidArgs => Self::InvalidArgs,
            DictionaryWriteError::Unknown => Self::Unknown,
        }
    }
}

pub type AppMessageResult<T> = Result<T, AppMessageError>;

pub(crate) fn app_message_result_from_raw(v: sys::AppMessageResult) -> AppMessageResult<()> {
    use super::AppMessageError as Error;
    Err(match v {
        sys::AppMessageResult_APP_MSG_OK => return Ok(()),
        sys::AppMessageResult_APP_MSG_SEND_TIMEOUT => Error::SendTimeout,
        sys::AppMessageResult_APP_MSG_SEND_REJECTED => Error::SendRejected,
        sys::AppMessageResult_APP_MSG_NOT_CONNECTED => Error::NotConnected,
        sys::AppMessageResult_APP_MSG_APP_NOT_RUNNING => Error::AppNotRunning,
        sys::AppMessageResult_APP_MSG_INVALID_ARGS => Error::InvalidArgs,
        sys::AppMessageResult_APP_MSG_BUSY => Error::Busy,
        sys::AppMessageResult_APP_MSG_BUFFER_OVERFLOW => Error::BufferOverflow,
        sys::AppMessageResult_APP_MSG_ALREADY_RELEASED => Error::AlreadyReleased,
        sys::AppMessageResult_APP_MSG_CALLBACK_ALREADY_REGISTERED => {
            Error::CallbackAlreadyRegistered
        }
        sys::AppMessageResult_APP_MSG_CALLBACK_NOT_REGISTERED => Error::CallbackNotRegistered,
        sys::AppMessageResult_APP_MSG_OUT_OF_MEMORY => Error::OutOfMemory,
        sys::AppMessageResult_APP_MSG_CLOSED => Error::Closed,
        sys::AppMessageResult_APP_MSG_INTERNAL_ERROR => Error::InternalError,
        sys::AppMessageResult_APP_MSG_INVALID_STATE => Error::InvalidState,
        _ => Error::Unknown,
    })
}
