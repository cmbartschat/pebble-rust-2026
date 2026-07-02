use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::sys;

#[derive(Copy, Clone, PartialEq)]
pub struct Angle {
    value: u16,
}

impl Angle {
    pub fn from_degrees(deg: i32) -> Self {
        Self {
            value: ((deg * sys::TRIG_MAX_ANGLE as i32) / 360).rem_euclid(u16::MAX as i32) as u16,
        }
    }

    pub fn to_degrees(self) -> i32 {
        self.value as i32 * 360 / sys::TRIG_MAX_ANGLE as i32
    }

    pub fn sin(self) -> Ratio {
        Ratio {
            value: unsafe { sys::sin_lookup(self.value as i32) },
        }
    }

    pub fn cos(self) -> Ratio {
        Ratio {
            value: unsafe { sys::cos_lookup(self.value as i32) },
        }
    }

    pub fn towards(self, target: Self, by: Self) -> Self {
        let up_offset = target.value.wrapping_sub(self.value);
        let down_offset = 0u16.wrapping_sub(up_offset);
        if up_offset <= by.value || down_offset <= by.value {
            return target;
        }

        if up_offset > down_offset {
            self - by
        } else {
            self + by
        }
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.value = self.value.wrapping_add(rhs.value)
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.value = self.value.wrapping_sub(rhs.value)
    }
}

#[derive(Copy, Clone)]
pub struct Ratio {
    value: i32,
}

impl Ratio {
    pub fn scale(self, factor: i32) -> i32 {
        (self.value * factor) / (sys::TRIG_MAX_RATIO as i32)
    }
}

mod sys_math {
    unsafe extern "C" {
        pub fn rand() -> u32;
        pub fn srand(_: u32);
    }
}

pub struct Random {
    value: u32,
}

impl Random {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let value = unsafe { sys_math::rand() };
        Self { value }
    }

    pub fn seed(seed: u32) {
        unsafe { sys_math::srand(seed) }
    }

    pub fn uniform(&self, range: u32) -> u32 {
        self.value % range
    }
}

impl From<Random> for u32 {
    fn from(value: Random) -> Self {
        value.value
    }
}
