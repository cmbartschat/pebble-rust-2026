//! Logging functionality.
//! See the [Pebble developer docs](https://developer.repebble.com/guides/debugging/debugging-with-app-logs)
//! and the battery usage warning below for important information on logging.
//!
//! All logging in `pebble_rust_2026` utilizes [`ufmt`] for formatting.
//! See the ufmt documentation for details.
//!
//! Logs of certain lower levels can be disabled by setting the `PEBBLE_LOG` environment variable during compilation.
//! All logs at a level below the specified level will be disabled.
//! The following options are available for that flag, all case-insensitive:
//!
//! - `error`: Only logs [`Level::Error`]
//! - `warn`: Logs [`Level::Warning`] and higher severities
//! - `info`: Logs [`Level::Info`] and higher severities
//! - `debug`: Logs [`Level::Debug`] and higher severities
//! - `trace`: Logs [`Level::Trace`] and higher severities (aka. everything)
//! - `off`: All logs disabled.
//!
//! As explained below, the default for builds with `debug_assertions` is `debug`,
//! and for release builds it’s `error`.
//! If you want to reduce battery usage maximally, use `off`.
//!
//! # Battery Usage Warning
//!
//! Logging a message requires the message to be sent over Bluetooth to the attached app.
//! This happens even when you are not looking at the logs, which is especially the case for your users in production!
//! Therefore, logs should be reduced to a minimum in release builds or removed entirely.
//! For these reasons, the default log level for debug builds is [`Level::Debug`],
//! but [`Level::Error`] for release builds.
// Note: The macros in this module require a bunch of internal APIs to be accessible to other crates,
//       which necessitates liberal use of #[doc(hidden)].
//       As is usual in Rust, these are not considered part of the public API and allowed to break semver rules.

use core::{convert::Infallible, ffi::c_int};

use crate::sys::{
    self, AppLogLevel_APP_LOG_LEVEL_DEBUG, AppLogLevel_APP_LOG_LEVEL_DEBUG_VERBOSE,
    AppLogLevel_APP_LOG_LEVEL_ERROR, AppLogLevel_APP_LOG_LEVEL_INFO,
    AppLogLevel_APP_LOG_LEVEL_WARNING,
};

use ufmt::derive::uDebug;

// Do not require users to directly depend on these if they don’t need to.
// This re-export allows us to access the required crates through pebble_rust_2026 in the log macros.
#[doc(hidden)]
pub use ufmt as internal_ufmt;

// Re-export the macros here for convenience; it doesn’t really matter from where people call them.
pub use crate::{debug, error, info, log, trace, warn};

/// Possible logging levels.
/// These are very similar to logging levels used in other popular Rust logging crates, such as defmt, log, and trace.
/// You only need to use these directly when invoking [`log`] yourself.
///
/// You can turn off logs at or below a specific level using the environment variable `PEBBLE_LOG` (like `DEFMT_LOG`),
/// see the crate documentation for details.
///
/// When comparing log levels, note that more critical levels are considered "below" less critical level.
/// As such, when setting the maximum log level to `X`, all logs with `Level <= X` are printed.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, uDebug)]
#[repr(u8)]
pub enum Level {
    /// Errors.
    Error = AppLogLevel_APP_LOG_LEVEL_ERROR,
    /// Warnings.
    Warning = AppLogLevel_APP_LOG_LEVEL_WARNING,
    /// Informational messages.
    Info = AppLogLevel_APP_LOG_LEVEL_INFO,
    /// Debugging messages.
    Debug = AppLogLevel_APP_LOG_LEVEL_DEBUG,
    /// Very verbose debugging messages.
    Trace = AppLogLevel_APP_LOG_LEVEL_DEBUG_VERBOSE,
}

impl Level {
    /// Const version of [`PartialOrd::le`] (`<=`) that doesn’t require const traits.
    const fn le(&self, other: &Self) -> bool {
        (*self as u8) <= (*other as u8)
    }
}

/// Write a message to the Pebble app log.
///
/// This uses a syntax similar to most logging macros:
///
/// ```rust,no_run
/// # #![no_std]
/// # #![no_main]
/// # extern crate alloc;
/// # use pebble_rust_2026::{*, log::*};
/// # #[global_allocator]
/// # static ALLOCATOR: MallocAllocator = MallocAllocator;
/// # fn dummy() {
/// # let level = Level::Info;
/// log!(level, "format string" /*, arguments... */);
/// # }
/// ```
///
/// `level` is a [`Level`], the format string uses [`ufmt`] syntax, and `arguments` are the formatting arguments.
///
/// The level does not only determine the logging level received by the Pebble API,
/// but also allows disabling logs below a certain level.
/// See the [module documentation](`crate::log`) for details on log levels.
#[macro_export]
macro_rules! log {
    // Avoid calling ufmt or allocating if the user passes just a static string
    ($level:expr, $format:literal) => {{
        use $crate::log;
        // Since this is const function and #inline, the compiler should see
        // that it’s always statically `true` or `false` and eliminate the code below accordingly.
        if log::is_level_enabled($level) {
            // Indicate to the compiler to definitely compile-time-calculate this value.
            const LINE: u32 = ::core::panic::Location::caller().line();
            log::do_log($level, ::core::module_path!(), LINE, ($format));
        }
    }};
    ($level:expr, $format:literal, $($arg:tt)+) => {{
        use $crate::log;
        // HACK: ufmt inserts plenty of references to the `ufmt` path in its macro, which breaks if the user doesn’t depend on ufmt directly.
        // We therefore re-export our copy of ufmt to make this macro work.
        #[allow(unused)]
        use $crate::log::internal_ufmt as ufmt;
        if log::is_level_enabled($level) {
            const LINE: u32 = ::core::panic::Location::caller().line();
            let mut output = log::CStringWriter::new();
            let _ = ufmt::uwrite!(&mut output, $format, $($arg)*);
            log::do_log($level, ::core::module_path!(), LINE, output.as_ref());
        }
    }};
}

/// Log at the error level ([`Level::Error`]).
/// This has the same syntax and behavior as the [`log`] macro, but without the level argument.
/// Also see the [module documentation](`crate::log`) for more information.
#[macro_export]
macro_rules! error {
    ($format:literal) => {{
        $crate::log!($crate::log::Level::Error, $format);
    }};
    ($format:literal, $($arg:tt)+) => {{
        $crate::log!($crate::log::Level::Error, $format, $($arg)*);
    }};
}
/// Log at the warning level ([`Level::Warning`]).
/// This has the same syntax and behavior as the [`log`] macro, but without the level argument.
/// Also see the [module documentation](`crate::log`) for more information.
#[macro_export]
macro_rules! warn {
    ($format:literal) => {{
        $crate::log!($crate::log::Level::Warning, $format);
    }};
    ($format:literal, $($arg:tt)+) => {{
        $crate::log!($crate::log::Level::Warning, $format, $($arg)*);
    }};
}
/// Log at the info level ([`Level::Info`]).
/// This has the same syntax and behavior as the [`log`] macro, but without the level argument.
/// Also see the [module documentation](`crate::log`) for more information.
#[macro_export]
macro_rules! info {
    ($format:literal) => {{
        $crate::log!($crate::log::Level::Info, $format);
    }};
    ($format:literal, $($arg:tt)+) => {{
        $crate::log!($crate::log::Level::Info, $format, $($arg)*);
    }};
}
/// Log at the debug level ([`Level::Debug`]).
/// This has the same syntax and behavior as the [`log`] macro, but without the level argument.
/// Also see the [module documentation](`crate::log`) for more information.
#[macro_export]
macro_rules! debug {
    ($format:literal) => {{
        $crate::log!($crate::log::Level::Debug, $format);
    }};
    ($format:literal, $($arg:tt)+) => {{
        $crate::log!($crate::log::Level::Debug, $format, $($arg)*);
    }};
}
/// Log at the trace level ([`Level::Trace`]).
/// This has the same syntax and behavior as the [`log`] macro, but without the level argument.
/// Also see the [module documentation](`crate::log`) for more information.
#[macro_export]
macro_rules! trace {
    ($format:literal) => {{
        $crate::log!($crate::log::Level::Trace, $format);
    }};
    ($format:literal, $($arg:tt)+) => {{
        $crate::log!($crate::log::Level::Trace, $format, $($arg)*);
    }};
}

/// Extracted for use by `level_enabled` so the expensive level retrieving logic is only run once.
const CONFIGURED_LOG_LEVEL: Level = {
    let level_flag = option_env!("PEBBLE_LOG");

    let default_level = {
        #[cfg(debug_assertions)]
        {
            Level::Debug
        }
        #[cfg(not(debug_assertions))]
        {
            Level::Error
        }
    };
    if let Some(level_flag) = level_flag {
        let level_flag_bytes = level_flag.as_bytes();

        // const implementation of a basic string match, necessary while PartialEq is not usable in const contexts
        if level_flag_bytes.len() == 5
            && level_flag_bytes[0].eq_ignore_ascii_case(&b't')
            && level_flag_bytes[1].eq_ignore_ascii_case(&b'r')
            && level_flag_bytes[2].eq_ignore_ascii_case(&b'a')
            && level_flag_bytes[3].eq_ignore_ascii_case(&b'c')
            && level_flag_bytes[4].eq_ignore_ascii_case(&b'e')
        {
            Level::Trace
        } else if level_flag_bytes.len() == 5
            && level_flag_bytes[0].eq_ignore_ascii_case(&b'd')
            && level_flag_bytes[1].eq_ignore_ascii_case(&b'e')
            && level_flag_bytes[2].eq_ignore_ascii_case(&b'b')
            && level_flag_bytes[3].eq_ignore_ascii_case(&b'u')
            && level_flag_bytes[4].eq_ignore_ascii_case(&b'g')
        {
            Level::Debug
        } else if level_flag_bytes.len() == 4
            && level_flag_bytes[0].eq_ignore_ascii_case(&b'i')
            && level_flag_bytes[1].eq_ignore_ascii_case(&b'n')
            && level_flag_bytes[2].eq_ignore_ascii_case(&b'f')
            && level_flag_bytes[3].eq_ignore_ascii_case(&b'o')
        {
            Level::Info
        } else if level_flag_bytes.len() == 4
            && level_flag_bytes[0].eq_ignore_ascii_case(&b'w')
            && level_flag_bytes[1].eq_ignore_ascii_case(&b'a')
            && level_flag_bytes[2].eq_ignore_ascii_case(&b'r')
            && level_flag_bytes[3].eq_ignore_ascii_case(&b'n')
        {
            Level::Warning
        } else if level_flag_bytes.len() == 5
            && level_flag_bytes[0].eq_ignore_ascii_case(&b'e')
            && level_flag_bytes[1].eq_ignore_ascii_case(&b'r')
            && level_flag_bytes[2].eq_ignore_ascii_case(&b'r')
            && level_flag_bytes[3].eq_ignore_ascii_case(&b'o')
            && level_flag_bytes[4].eq_ignore_ascii_case(&b'r')
        {
            Level::Error
        } else {
            panic!(
                "Invalid value for the `PEBBLE_LOG` environment variable, use error, warn, info, debug, trace, or off."
            );
        }
    } else {
        default_level
    }
};

const LOG_IS_OFF: bool = {
    let level_flag = option_env!("PEBBLE_LOG");
    if let Some(level_flag) = level_flag {
        let level_flag_bytes = level_flag.as_bytes();
        // "off"
        level_flag_bytes.len() == 3
            && level_flag_bytes[0].eq_ignore_ascii_case(&b'o')
            && level_flag_bytes[1].eq_ignore_ascii_case(&b'f')
            && level_flag_bytes[2].eq_ignore_ascii_case(&b'f')
    } else {
        false
    }
};

#[doc(hidden)]
#[inline(always)]
pub const fn is_level_enabled(level: Level) -> bool {
    !LOG_IS_OFF && level.le(&CONFIGURED_LOG_LEVEL)
}

/// Code-size-optimized output for ufmt.
#[doc(hidden)]
pub struct CStringWriter {
    /// The Pebble C API only outputs ~100 characters, so we limit ourselves too.
    buffer: [u8; 128],
    /// Invariant: used_len <= buffer.len()
    used_len: usize,
}

// Warning: All of the functions in here are extremely optimization-sensitive.
// The ultimate goal of these weird shenanigans is to avoid the compiler inserting calls to functions like memcpy and memclr,
// which are extremely large without build_std.
// We can probably replace all of them with simple memcpy calls (e.g. `copy_from_slice`) once build_std is stable.
// Here are some things that influence whether this gets "optimized" badly:
// - Using core::hint::black_box in strategic places.
// - Avoiding #[inline] as much as necessary.
// - Using unsafe code (mainly switching between get and get_unchecked for indexing).
// Please check any changes against nm and objcopy and look at the generated assembly.
// See `just no-memcpy-in-logging` for a useful script.
impl CStringWriter {
    #[doc(hidden)]
    pub const fn new() -> Self {
        Self {
            buffer: [0; _],
            used_len: 0,
        }
    }

    const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    fn copy(&mut self, string: &str) {
        let buf = string.as_bytes();
        // Maximum number of data we can copy from the buffer.
        let limit = buf.len().min(self.capacity() - self.used_len);
        // Hand-rolled memcpy, since calls to Rust’s iterator functionality will result in the large memcpy implementations in compiler_builtins
        let mut idx = 0;
        while idx < limit {
            debug_assert!(self.used_len < self.capacity());
            // SAFETY: the limit calculation ensured that used_len is never too big and idx neither.
            unsafe {
                *self.buffer.get_unchecked_mut(self.used_len) =
                    core::hint::black_box(*buf.get_unchecked(idx))
            };
            self.used_len += 1;
            idx += 1;
        }
    }

    fn copy_char(&mut self, chr: char) {
        let mut dst = [0u8; 4];
        let string = chr.encode_utf8(&mut dst);
        self.copy(string);
    }
}

impl ufmt::uWrite for CStringWriter {
    type Error = Infallible;

    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.copy(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.copy_char(c);
        Ok(())
    }
}

impl AsRef<str> for CStringWriter {
    fn as_ref(&self) -> &str {
        // SAFETY: We were the only ones writing to the buffer, and we only ever wrote valid UTF-8 strings into it.
        // Besides, this doesn’t really matter, since we pass this str into a C API soon enough.
        unsafe { str::from_utf8_unchecked(&self.buffer) }
    }
}

#[doc(hidden)]
#[inline(always)]
pub fn do_log(level: Level, module: &str, line: u32, message: &str) {
    // Manually make a null-terminated string from the module.
    // Since the Pebble API truncates these at 15/16 bytes anyways, we can use a stack buffer and copy as much as necessary.
    // We use a 32-byte buffer (with one extra null byte) just in case they decide to increase the limit in the future.
    let mut module_buffer = [0u8; 33];
    let len = module.len().min(32);
    // Print the end of the module name, so truncation works correctly.
    let truncated_module = &module.as_bytes()[module.len() - len..];
    // Manual memcpy should reduce code size.
    for i in 0..len {
        // SAFETY: `len` never exceeds either buffer’s size.
        unsafe {
            *module_buffer.get_unchecked_mut(i) = *truncated_module.get_unchecked(i);
        }
    }

    // SAFETY: Ultimately we have very little information on how this C API can cause UB in detail, but the basics are covered:
    // - src_filename is a valid C string pointer, see above.
    // - If `line` wrapped during the cast, it yields bogus line numbers but no UB.
    // - The format string is a static C string pointer and obviously valid.
    // - The format "%.*s" demands two arguments: the maximum number of characters (=minimum field width or precision) to be copied from a C string,
    //   and the C string pointer itself, see https://en.cppreference.com/c/io/fprintf.
    // - If message length wrapped during the cast, the resulting precision can never be larger than the actual size of the string.
    //   Per cppreference: "If the value of the argument is negative, then [...] the absolute value used for minimum field width."
    //   So the string is truncated but no UB happens. In any case, the system will OOM long before this happens with a 2GiB string :)
    // - The message pointer points to a valid byte sequence of at least the length specified before.
    //   If the message contains null bytes, we have experimentally determined that it will be truncated, but no UB happens.
    unsafe {
        sys::app_log(
            level as sys::AppLogLevel,
            module_buffer.as_ptr(),
            line as c_int,
            c"%.*s".as_ptr(),
            message.len() as c_int,
            message.as_ptr(),
        );
    }
}
