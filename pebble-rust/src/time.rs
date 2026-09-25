use core::ffi::CStr;

use alloc::{ffi::CString, vec};

use crate::{log::log_c_str, sys};

/// A global time.
///
/// Note: This is a fairly inaccurate type, owing to the C API restrictions.
///       For more complex time calculations including timezone handling,
///       we recommend using a Rust crate which more accurately accounts for real world problems, like [jiff](https://crates.io/crates/jiff).
///       For interacting with such types, it’s usually enough to given them the current Unix timestamp, i.e. [`Self::epoch_seconds`].
#[derive(Debug, Copy, Clone)]
pub struct Time {
    value: sys::time_t,
}

impl Time {
    /// Returns the current time.
    pub fn now() -> Self {
        let mut res = Self { value: 0 };
        unsafe {
            sys::time(core::ptr::addr_of_mut!(res.value));
        }
        res
    }

    /// Creates a new time from a given seconds offset from the Unix epoch (1970-01-01T00:00:00Z).
    pub const fn from_epoch_seconds(seconds: sys::time_t) -> Self {
        Self { value: seconds }
    }

    /// Returns the offset in seconds from the Unix epoch (1970-01-01T00:00:00Z).
    pub const fn epoch_seconds(&self) -> sys::time_t {
        self.value
    }

    /// Returns the time in the timezone of the user.
    pub fn to_local(&self) -> LocalTime {
        LocalTime {
            value: unsafe { sys::localtime(core::ptr::addr_of!(self.value)).read() },
        }
    }

    /// Returns the time in UTC.
    pub fn to_utc(&self) -> LocalTime {
        LocalTime {
            value: unsafe { sys::gmtime(core::ptr::addr_of!(self.value)).read() },
        }
    }
}

/// A local, timezone-dependent time.
/// This carries Gregorian calendar data and uses the proleptic Gregorian calendar (reference point Unix epoch, calculated backwards beyond the 16th century).
///
/// Note: This may either be UTC or local time.
///       See [`Time`] for further caveats.
#[derive(Debug, Clone)]
pub struct LocalTime {
    value: sys::tm,
}

impl LocalTime {
    /// Returns the current local time.
    pub fn now() -> Self {
        Time::now().into()
    }

    /// Returns the seconds within the minute.
    pub const fn second(&self) -> i32 {
        self.value.tm_sec
    }

    /// Returns the minutes within the hour.
    pub const fn minute(&self) -> i32 {
        self.value.tm_min
    }

    /// Returns the hours within the day.
    pub const fn hour(&self) -> i32 {
        self.value.tm_hour
    }

    /// Returns the one-based day of the month.
    pub const fn day(&self) -> i32 {
        self.value.tm_mday
    }

    /// Returns the zero-based month index.
    pub const fn month(&self) -> i32 {
        self.value.tm_mon
    }

    /// Returns the year.
    pub const fn year(&self) -> i32 {
        self.value.tm_year
    }

    /// Formats this time. See the [`sys::strftime`] documentation on the format string syntax.
    pub fn format(&self, format: &CStr) -> CString {
        // Triple size is a good estimate, since even a year specifier (%Y) only expands to double size.
        let mut buffer = vec![0u8; format.count_bytes() * 3];
        let written = unsafe {
            sys::strftime(
                buffer.as_mut_ptr(),
                buffer.len(),
                format.as_ptr(),
                &self.value,
            )
        };
        if written == 0 {
            log_c_str(c"LocalTime::to_string failed to write");
            panic!("Time overflowed buffer");
        }
        CString::new(&buffer[0..written]).unwrap()
    }

    /// Formats the hour and minute from this time, as any digital clock would.
    /// This respects the user’s 24-clock configuration, but does not print AM/PM.
    pub fn format_hh_mm(&self) -> CString {
        self.format(if unsafe { sys::clock_is_24h_style() } {
            c"%H:%M"
        } else {
            c"%I:%M"
        })
    }
}

impl From<Time> for LocalTime {
    fn from(value: Time) -> Self {
        value.to_local()
    }
}

impl TryFrom<&mut LocalTime> for Time {
    type Error = ();

    fn try_from(value: &mut LocalTime) -> Result<Self, Self::Error> {
        let res = unsafe { sys::mktime(&mut value.value) };
        if res == -1 {
            return Err(());
        }
        Ok(Self::from_epoch_seconds(res))
    }
}

impl TryFrom<&LocalTime> for Time {
    type Error = ();

    fn try_from(value: &LocalTime) -> Result<Self, Self::Error> {
        (&mut value.clone()).try_into()
    }
}

impl TryFrom<LocalTime> for Time {
    type Error = ();

    fn try_from(mut value: LocalTime) -> Result<Self, Self::Error> {
        (&mut value).try_into()
    }
}

bitflags::bitflags! {
    /// The possible time units, in particular for callback granularity.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TimeUnits: u8 {
        /// Every second.
        const Second = sys::TimeUnits_SECOND_UNIT;
        /// Every minute.
        const Minute = sys::TimeUnits_MINUTE_UNIT;
        /// Every hour.
        const Hour = sys::TimeUnits_HOUR_UNIT;
        /// Every day.
        const Day = sys::TimeUnits_DAY_UNIT;
        /// Every month.
        const Month = sys::TimeUnits_MONTH_UNIT;
        /// Every year.
        const Year = sys::TimeUnits_YEAR_UNIT;
    }
}
