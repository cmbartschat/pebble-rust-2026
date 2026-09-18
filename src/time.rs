use core::ffi::CStr;

use alloc::{ffi::CString, vec};

use crate::{log::log_c_str, sys};

#[derive(Debug, Copy, Clone)]
pub struct Time {
    value: sys::time_t,
}

impl Time {
    pub fn now() -> Self {
        let mut res = Self { value: 0 };
        unsafe {
            sys::time(core::ptr::addr_of_mut!(res.value));
        }
        res
    }

    pub const fn from_epoch_seconds(seconds: sys::time_t) -> Self {
        Self { value: seconds }
    }

    pub const fn epoch_seconds(&self) -> sys::time_t {
        self.value
    }

    pub fn to_local(&self) -> LocalTime {
        LocalTime {
            value: unsafe { sys::localtime(core::ptr::addr_of!(self.value)).read() },
        }
    }

    pub fn to_utc(&self) -> LocalTime {
        LocalTime {
            value: unsafe { sys::gmtime(core::ptr::addr_of!(self.value)).read() },
        }
    }
}

#[derive(Debug, Clone)]
pub struct LocalTime {
    value: sys::tm,
}

impl LocalTime {
    pub fn now() -> Self {
        Time::now().into()
    }

    pub const fn second(&self) -> i32 {
        self.value.tm_sec
    }

    pub const fn minute(&self) -> i32 {
        self.value.tm_min
    }

    pub const fn hour(&self) -> i32 {
        self.value.tm_hour
    }

    pub const fn day(&self) -> i32 {
        self.value.tm_mday
    }

    pub const fn month(&self) -> i32 {
        self.value.tm_mon
    }

    pub const fn year(&self) -> i32 {
        self.value.tm_year
    }

    /// Formats this time. See the [strftime] documentation on the format string syntax.
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TimeUnits: u8 {
        const Second = sys::TimeUnits_SECOND_UNIT;
        const Minute = sys::TimeUnits_MINUTE_UNIT;
        const Hour = sys::TimeUnits_HOUR_UNIT;
        const Day = sys::TimeUnits_DAY_UNIT;
        const Month = sys::TimeUnits_MONTH_UNIT;
        const Year = sys::TimeUnits_YEAR_UNIT;
    }
}
