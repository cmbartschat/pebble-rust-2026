use core::{
    convert::Infallible,
    hint::cold_path,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use rand::distr::{
    Distribution, StandardUniform, Uniform,
    uniform::{SampleUniform, UniformInt, UniformSampler},
};
use ufmt::derive::uDebug;

use crate::{GSize, sys};

/// A fixed-point angle.
///
/// To generate random angles, use the uniform sampling support from [`rand`].
#[derive(Copy, Clone, PartialEq, PartialOrd, uDebug)]
#[repr(transparent)] // ensure optimal ABI
pub struct Angle {
    pub(crate) value: i32,
}

impl Angle {
    /// Creates an angle from a whole number of degrees.
    pub const fn from_degrees(deg: i32) -> Self {
        Self {
            value: (deg * sys::TRIG_MAX_ANGLE as i32) / 360,
        }
    }

    /// Returns the truncated degrees in this angle.
    pub const fn to_degrees(self) -> i32 {
        self.value * 360 / sys::TRIG_MAX_ANGLE as i32
    }

    /// Normalizes the angle, so that it is in the range of 0 - 360 degrees.
    pub const fn normalize(&mut self) {
        // make it positive; avoids weird modulus behavior
        while self.value < 0 {
            cold_path();
            self.value += sys::TRIG_MAX_ANGLE as i32;
        }
        self.value %= sys::TRIG_MAX_ANGLE as i32;
    }

    /// Returns this angle in its normalized form.
    pub const fn normalized(mut self) -> Self {
        self.normalize();
        self
    }

    /// Creates an angle from a raw angle value between 0 and [`sys::TRIG_MAX_ANGLE`].
    ///
    /// # Safety
    ///
    /// This function is unsafe, because passing an invalid angle value in here
    /// will cause undefined behavior in the trigonometry functions like [`Self::sin`].
    pub const unsafe fn from_raw(value: i32) -> Self {
        Self { value }
    }

    /// Returns the sine of the angle.
    pub fn sin(self) -> Ratio {
        Ratio {
            value: unsafe { sys::sin_lookup(self.value) },
        }
    }

    /// Returns the cosine of the angle.
    pub fn cos(self) -> Ratio {
        Ratio {
            value: unsafe { sys::cos_lookup(self.value) },
        }
    }

    /// Returns the circle-aware arctangent ("atan2", since it takes 2 arguments instead of one) of the given coordinates.
    /// In other words, this returns the angle of the given coordinates in relation to the positive X direction (which is 0 degrees).
    pub fn atan2(size: GSize) -> Angle {
        Self {
            value: unsafe { sys::atan2_lookup(size.h, size.w) },
        }
    }

    /// Move self towards target by a specified angle.
    pub const fn towards(self, target: Self, by: Self) -> Self {
        if by.value < 0 {
            return self;
        }
        let by = by.value;
        let offset = target.value - self.value;
        if offset.abs() <= by {
            return target;
        }
        let by = Self { value: by };
        if offset < 0 {
            self.subtract(by)
        } else {
            self.add(by)
        }
    }

    /// Move self towards target by a specified angle, wrapping around 360 degrees.
    /// Otherwise identical to [`Self::towards`].
    pub const fn towards_wrap(self, target: Self, by: Self) -> Self {
        Self::from_absolute(self.to_absolute().towards(target.to_absolute(), by))
    }

    const fn to_absolute(self) -> AbsoluteAngle {
        AbsoluteAngle {
            value: self.value.rem_euclid(u16::MAX as i32) as u16,
        }
    }

    const fn from_absolute(absolute: AbsoluteAngle) -> Self {
        Self {
            value: absolute.value as i32,
        }
    }

    /// Const version of regular subtraction, necessary until const traits are available.
    pub const fn subtract(mut self, rhs: Self) -> Self {
        self.value = self.value + rhs.value;
        self
    }

    /// Const version of regular addition, necessary until const traits are available.
    pub const fn add(mut self, rhs: Self) -> Self {
        self.value = self.value + rhs.value;
        self
    }

    /// Const version of regular multiplication, necessary until const traits are available.
    pub const fn multiply(mut self, rhs: i32) -> Self {
        self.value = self.value * rhs;
        self
    }
    /// Const version of regular division, necessary until const traits are available.
    pub const fn divide(mut self, rhs: i32) -> Self {
        self.value = self.value / rhs;
        self
    }
}

#[doc(hidden)]
pub struct UniformAngle(UniformInt<i32>);

impl UniformSampler for UniformAngle {
    type X = Angle;

    fn new<B1, B2>(low: B1, high: B2) -> Result<Self, rand::distr::uniform::Error>
    where
        B1: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
    {
        UniformInt::<i32>::new(low.borrow().value, high.borrow().value).map(UniformAngle)
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Self, rand::distr::uniform::Error>
    where
        B1: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
    {
        UniformInt::<i32>::new_inclusive(low.borrow().value, high.borrow().value).map(UniformAngle)
    }

    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Angle {
            value: self.0.sample(rng),
        }
    }
}

impl SampleUniform for Angle {
    type Sampler = UniformAngle;
}

impl Distribution<Angle> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Angle {
        use rand::RngExt;
        Angle {
            value: rng.sample(Uniform::new(0, sys::TRIG_MAX_ANGLE as i32).unwrap()),
        }
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract(rhs)
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl Mul<i32> for Angle {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Div<i32> for Angle {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        self.divide(rhs)
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.subtract(rhs);
    }
}

impl MulAssign<i32> for Angle {
    fn mul_assign(&mut self, rhs: i32) {
        *self = self.multiply(rhs);
    }
}

impl DivAssign<i32> for Angle {
    fn div_assign(&mut self, rhs: i32) {
        *self = self.divide(rhs);
    }
}

/// A fixed-point ratio.
/// This is most importantly the result type of trigonometric functions on [`Angle`].
#[derive(Copy, Clone, Debug, uDebug)]
#[repr(transparent)] // ensure optimal ABI
pub struct Ratio {
    value: i32,
}

impl Ratio {
    /// Creates a new ratio based on a thousands ("milli" SI prefix) fixed-point value.
    pub const fn from_milli(milli: i32) -> Self {
        Self {
            value: (milli * sys::TRIG_MAX_RATIO as i32) / 1000,
        }
    }

    /// Returns the ratio in thousands precision.
    pub const fn milli(self) -> i32 {
        self.fixed_scale::<1000>()
    }

    /// Version of [`Self::scale`] with the scale factor fixed as a const parameter.
    /// This should allow the resulting code to be optimally performant.
    pub const fn fixed_scale<const FACTOR: i32>(self) -> i32 {
        (self.value * FACTOR) / (sys::TRIG_MAX_RATIO as i32)
    }

    /// Scales the given value by this ratio.
    pub const fn scale(self, factor: i32) -> i32 {
        (self.value * factor) / (sys::TRIG_MAX_RATIO as i32)
    }
}

mod sys_math {
    unsafe extern "C" {
        // Maximum value here is __RAND_MAX, which is 2^31-1.
        pub fn rand() -> u32;
        pub fn srand(_: u32);
    }
}

/// A random number generator.
/// This implements [`rand::Rng`], so it can be used as an RNG source for anything in the de facto standard `rand` ecosystem.
/// It does not implement [`rand::SeedableRng`] because there is only one global seed, which is not how this trait expects everything to work.
/// Use [`Rng::seed`] if you want to seed the global RNG.
///
/// Some usage examples:
///
/// ```rust,ignore
/// use pebble_sdk::Rng;
/// use rand::{RngExt, seq::IndexedRandom};
///
/// Rng.seed(42);
///
/// let value_in_range = Rng.random_range(70..=200);
/// let chosen = [1, 2, 3, 4].choose(&mut Rng);
/// ```
///
/// For a simpler API, you can use [`RandomValue`], but note that its functionality is usually less correct than anything available through `rand`.
pub struct Rng;

impl rand::TryRng for Rng {
    type Error = Infallible;

    // Note for all these implementations:
    // Because RAND_MAX is not u32::MAX, we need to grab at least two values even for u32.
    // To guard against further reductions of RAND_MAX, we assume that RAND_MAX >= 2^24.

    #[inline]
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok((Self::next() << 16) ^ Self::next())
    }

    #[inline]
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok((Self::next() as u64) << 48 ^ (Self::next() as u64) << 24 ^ Self::next() as u64)
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)] // expected
    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        rand::rand_core::utils::fill_bytes_via_next_word(dst, || Ok(Self::next()))
    }
}

impl Rng {
    /// Set the seed for the random number generator.
    pub fn seed(seed: u32) {
        unsafe { sys_math::srand(seed) }
    }

    #[inline]
    fn next() -> u32 {
        unsafe { sys_math::rand() }
    }
}

/// This type offers more basic randomness functionality, which is less correct than [`Rng`],
/// but also frequently uses less code[^code-size].
/// It is generally recommended to use [`Rng`] instead, unless you know what you are doing.
///
/// [^code-size]: In one test, [`RandomValue::range`] used 170 fewer code bytes than `Rng.random_range`.
#[repr(transparent)]
pub struct RandomValue {
    value: u32,
}

impl RandomValue {
    /// Generate a new random value.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let value = unsafe { sys_math::rand() };
        Self { value }
    }
    /// Generate a random value from 0 to the upper bound.
    ///
    /// Note that depending on the exact bound, this value is usually not exactly uniformly distributed.
    /// This may be fine for simple applications.
    /// Use [`Rng`] with [`rand::RngExt::random_range`] instead if you want uniformly distributed values.
    ///
    /// Furthermore, values above 2^31 - 1 cannot be generated.
    #[allow(clippy::missing_const_for_fn)] // doesn’t make sense for a runtime value
    pub fn range(&self, upper_bound: u32) -> u32 {
        self.value % upper_bound
    }
}

const _: () = assert!(u16::MAX as u32 == sys::TRIG_MAX_ANGLE - 1);

/// An absolute angle that always wraps at 360 degrees.
#[derive(Copy, Clone, PartialEq, Debug, uDebug)]
#[repr(transparent)] // ensure optimal ABI
pub struct AbsoluteAngle {
    value: u16,
}

impl AbsoluteAngle {
    /// Creates an angle from a whole number of degrees, modulo 360, so from_degrees(361) == from_degrees(1)
    pub const fn from_degrees(deg: i32) -> Self {
        Angle::from_degrees(deg).to_absolute()
    }

    /// Move self towards target by a specified angle, wrapping around 360 degrees.
    pub const fn towards(self, target: Self, by: Angle) -> Self {
        if by.value < 0 {
            return self;
        }
        if by.value > sys::TRIG_MAX_ANGLE as i32 {
            return target;
        }
        let by = by.value as u16;
        let up_offset = target.value.wrapping_sub(self.value);
        let down_offset = 0u16.wrapping_sub(up_offset);
        if up_offset <= by || down_offset <= by {
            return target;
        }

        let by = Self { value: by };

        if up_offset > down_offset {
            self.subtract(by)
        } else {
            self.add(by)
        }
    }

    /// Const version of regular subtraction, necessary until const traits are available.
    pub const fn subtract(mut self, rhs: Self) -> Self {
        self.value = self.value.wrapping_sub(rhs.value);
        self
    }

    /// Const version of regular addition, necessary until const traits are available.
    pub const fn add(mut self, rhs: Self) -> Self {
        self.value = self.value.wrapping_add(rhs.value);
        self
    }
}

impl Sub for AbsoluteAngle {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Add for AbsoluteAngle {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for AbsoluteAngle {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl SubAssign for AbsoluteAngle {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }
}

impl From<Angle> for AbsoluteAngle {
    fn from(value: Angle) -> Self {
        Self {
            value: value.value.rem_euclid(sys::TRIG_MAX_ANGLE as i32) as u16,
        }
    }
}

impl From<AbsoluteAngle> for Angle {
    fn from(value: AbsoluteAngle) -> Self {
        Self {
            value: value.value as i32,
        }
    }
}
