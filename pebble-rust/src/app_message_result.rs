use crate::{dictionary::DictionaryWriteError, sys};

/// Errors that can occur when app messages are attempted to be retrieved.
#[derive(Copy, Clone, Debug)]
#[repr(u16)] // match C API
pub enum AppMessageError {
    /// An unknown error.
    /// This is the default value for C API errors that we haven’t accounted for.
    // Using an explicit value to force a compiler error once the C API values below change unexpectedly
    Unknown = 1,
    /// Message sending was not acknowledged by the smartphone app within an OS-defined timeout.
    SendTimeout = sys::AppMessageResult_APP_MSG_SEND_TIMEOUT,
    /// The smartphone app rejected the data.
    SendRejected = sys::AppMessageResult_APP_MSG_SEND_REJECTED,
    /// The watch is not connected to a smartphone app.
    NotConnected = sys::AppMessageResult_APP_MSG_NOT_CONNECTED,
    /// The smartphone app is not running.
    AppNotRunning = sys::AppMessageResult_APP_MSG_APP_NOT_RUNNING,
    /// Invalid arguments were provided when calling the C API.
    InvalidArgs = sys::AppMessageResult_APP_MSG_INVALID_ARGS,
    /// There are pending (in or outbound) messages that need to be processed first before new ones can be received or sent.
    Busy = sys::AppMessageResult_APP_MSG_BUSY,
    /// Our buffer was too small to contain an incoming message.
    BufferOverflow = sys::AppMessageResult_APP_MSG_BUFFER_OVERFLOW,
    /// Some resource (the C API does not clarify which) has already been released.
    AlreadyReleased = sys::AppMessageResult_APP_MSG_ALREADY_RELEASED,
    /// A callback has already been registered.
    CallbackAlreadyRegistered = sys::AppMessageResult_APP_MSG_CALLBACK_ALREADY_REGISTERED,
    /// The callback could not be deregistered, because it had not been registered before.
    CallbackNotRegistered = sys::AppMessageResult_APP_MSG_CALLBACK_NOT_REGISTERED,
    /// An OOM was encountered while allocating app message data.
    OutOfMemory = sys::AppMessageResult_APP_MSG_OUT_OF_MEMORY,
    /// An app message was closed (?)
    Closed = sys::AppMessageResult_APP_MSG_CLOSED,
    /// Internal OS error.
    InternalError = sys::AppMessageResult_APP_MSG_INTERNAL_ERROR,
    /// A function was called with the app message in an invalid state.
    InvalidState = sys::AppMessageResult_APP_MSG_INVALID_STATE,
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

/// Result type for functions that deal with app messages.
pub type AppMessageResult<T> = Result<T, AppMessageError>;

pub(crate) const fn app_message_result_from_raw(v: sys::AppMessageResult) -> AppMessageResult<()> {
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
