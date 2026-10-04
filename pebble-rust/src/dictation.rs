use core::{
    cell::RefCell,
    ffi::{CStr, c_char, c_void},
    pin::Pin,
    ptr::NonNull,
};

use alloc::{boxed::Box, rc::Rc};

use crate::{handle::Handle, log_c_str, sys};

type DictationResult<T> = Result<T, DictationError>;

struct DictationContext {
    #[allow(clippy::type_complexity)]
    callback: Box<dyn FnMut(DictationResult<&str>)>,
}

struct DictationSessionInner {
    raw: NonNull<sys::DictationSession>,
    _context: Pin<Box<DictationContext>>,
}

impl Drop for DictationSessionInner {
    fn drop(&mut self) {
        unsafe { sys::dictation_session_destroy(self.raw.as_ptr()) };
    }
}

extern "C" fn global_handle_dictation_end(
    _session: *mut sys::DictationSession,
    status: u8,
    data: *mut u8,
    context: *mut c_void,
) {
    let context = unsafe { (context as *mut DictationContext).as_mut() };
    let Some(context) = context else {
        log_c_str(c"Unexpected: missing dictation context");
        return;
    };

    let get_data = move || {
        from_session_status(status)?;
        let str = unsafe { CStr::from_ptr(data as *mut c_char) };
        str.to_str().map_err(|_| DictationError::Utf8Error)
    };

    (context.callback)(get_data());
}

impl DictationSessionInner {
    pub fn new(
        max_bytes: u32,
        handler: impl FnMut(Result<&str, DictationError>) + 'static,
    ) -> Option<Self> {
        let mut context = Box::new(DictationContext {
            callback: Box::new(handler),
        });
        let raw = NonNull::new(unsafe {
            sys::dictation_session_create(
                max_bytes,
                Some(global_handle_dictation_end),
                context.as_mut() as *mut DictationContext as *mut c_void,
            )
        })?;
        Some(Self {
            raw,
            _context: Box::into_pin(context),
        })
    }

    pub fn start(&mut self) -> DictationResult<()> {
        from_session_status(unsafe { sys::dictation_session_start(self.raw.as_ptr()) })
    }

    pub fn stop(&mut self) -> DictationResult<()> {
        from_session_status(unsafe { sys::dictation_session_stop(self.raw.as_ptr()) })
    }

    pub fn set_confirmation_enabled(&mut self, is_enabled: bool) {
        unsafe { sys::dictation_session_enable_confirmation(self.raw.as_ptr(), is_enabled) };
    }

    pub fn set_error_dialogs_enabled(&mut self, is_enabled: bool) {
        unsafe { sys::dictation_session_enable_error_dialogs(self.raw.as_ptr(), is_enabled) };
    }
}

/// Error states for dictation
#[derive(Debug, Copy, Clone)]
pub enum DictationError {
    /// User rejected transcription and exited UI.
    TranscriptionRejected,
    /// User exited UI after transcription error.
    TranscriptionRejectedWithError,
    /// Too many errors occurred during transcription and the UI exited.
    SystemAborted,
    /// No speech was detected and UI exited.
    NoSpeechDetected,
    /// No BT or internet connection.
    ConnectivityError,
    /// Voice transcription disabled for this user.
    Disabled,
    /// Voice transcription failed due to internal error.
    InternalError,
    /// Cloud recognizer failed to transcribe speech (only possible if error dialogs disabled).
    RecognizerError,
    /// An unexpected error code was encountered.
    Unspecified,
    /// The transcription was not convertible to UTF-8.
    Utf8Error,
}

const fn from_session_status(value: sys::DictationSessionStatus) -> DictationResult<()> {
    Err(match value {
        sys::DictationSessionStatus_DictationSessionStatusSuccess => return Ok(()),
        sys::DictationSessionStatus_DictationSessionStatusFailureTranscriptionRejected => {
            DictationError::TranscriptionRejected
        }
        sys::DictationSessionStatus_DictationSessionStatusFailureTranscriptionRejectedWithError => {
            DictationError::TranscriptionRejectedWithError
        }
        sys::DictationSessionStatus_DictationSessionStatusFailureSystemAborted => {
            DictationError::SystemAborted
        }
        sys::DictationSessionStatus_DictationSessionStatusFailureNoSpeechDetected => {
            DictationError::NoSpeechDetected
        }
        sys::DictationSessionStatus_DictationSessionStatusFailureConnectivityError => {
            DictationError::ConnectivityError
        }
        sys::DictationSessionStatus_DictationSessionStatusFailureDisabled => {
            DictationError::Disabled
        }
        sys::DictationSessionStatus_DictationSessionStatusFailureInternalError => {
            DictationError::InternalError
        }
        sys::DictationSessionStatus_DictationSessionStatusFailureRecognizerError => {
            DictationError::RecognizerError
        }
        _ => DictationError::Unspecified,
    })
}

/// Represents the ability to request voice dictation from the user
#[derive(Clone)]
pub struct DictationSession {
    handle: Handle<DictationSessionInner>,
}

impl DictationSession {
    /// Create a new instance of a dictation session
    pub fn new(handler: impl FnMut(Result<&str, DictationError>) + 'static) -> Option<Self> {
        Some(Self {
            handle: Rc::new(RefCell::new(DictationSessionInner::new(0, handler)?)),
        })
    }

    /// Create a new instance of a dictation session with a specified length limit
    pub fn new_truncated(
        max_bytes: u32,
        handler: impl FnMut(Result<&str, DictationError>) + 'static,
    ) -> Option<Self> {
        if max_bytes == 0 {
            log_c_str(c"Unexpected: max_bytes cannot be 0");
            return None;
        }
        Some(Self {
            handle: Rc::new(RefCell::new(DictationSessionInner::new(
                max_bytes, handler,
            )?)),
        })
    }

    /// Start the session
    pub fn start(&mut self) -> DictationResult<()> {
        self.handle.borrow_mut().start()
    }

    /// Stop the session
    pub fn stop(&mut self) -> DictationResult<()> {
        self.handle.borrow_mut().stop()
    }

    /// Configure whether the user will review and confirm the text.
    pub fn set_confirmation_enabled(&mut self, is_enabled: bool) {
        self.handle
            .borrow_mut()
            .set_confirmation_enabled(is_enabled);
    }

    /// Configure whether the user will see error dialogs.
    pub fn set_error_dialogs_enabled(&mut self, is_enabled: bool) {
        self.handle
            .borrow_mut()
            .set_error_dialogs_enabled(is_enabled);
    }
}
