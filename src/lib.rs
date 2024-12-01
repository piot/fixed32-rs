/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use crate::lookup_slices::{ACOS_TABLE, ASIN_TABLE, COS_TABLE, SIN_TABLE};
use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub, SubAssign};

mod lookup_slices;

/// A fixed-point number with 16.16 format.
#[derive(Clone, Copy, Default, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub struct Fp(i32);

impl Fp {
    /// The scaling factor used for fixed-point arithmetic.
    pub const SHIFT: i32 = 16;
    pub const SCALE: i32 = 1 << Self::SHIFT;

    const HALF_SCALE: i32 = Self::SCALE / 2;
    pub const SCALE_I64: i64 = Self::SCALE as i64;
    pub const FSCALE: f32 = Self::SCALE as f32;
    pub const FRAC_PI_2: Self = Self(Self::SCALE * 1570 / 1000); // π/2 ≈ 1.570
    pub const PI: Self = Self(Self::SCALE * 3141 / 1000); // π ≈ 3.141
    pub const TAU: Self = Self(Self::SCALE * 6283 / 1000); // 2π ≈ 6.283

    /// Creates a new `Fp` instance from a raw integer value.
    ///
    /// <div class="warning">
    ///
    ///  **Warning:** This constructor should be used with caution. It directly creates
    /// an `Fp` instance from a raw integer without any validation or
    /// scaling logic. It is almost always preferable to use higher-level
    /// constructors or conversion traits like `From<T>` to ensure that the
    /// fixed-point values are correctly initialized.
    /// </div>
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed32::Fp;
    /// let fp = Fp::from_raw(100);
    /// ```
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        Self(raw)
    }

    /// Returns the constant `Fp` value for one.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// assert_eq!(1, Fp::one().into());
    /// ```
    #[inline]
    #[must_use]
    pub const fn one() -> Self {
        Self(Self::SCALE)
    }

    /// Returns the constant `Fp` value for negative one.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// assert_eq!(<Fp as Into<i16>>::into(Fp::neg_one()), -1);
    /// ```
    #[inline]
    #[must_use]
    pub const fn neg_one() -> Self {
        Self(-Self::SCALE)
    }

    /// Returns the constant `Fp` value for zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// assert_eq!(<Fp as Into<i16>>::into(Fp::zero()), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Checks if the `Fp` value is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// assert!(Fp::zero().is_zero());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    // Clamp value to the range [-1, 1]
    pub const fn normalize(self) -> Self {
        if self.0 < -Self::SCALE {
            Self(-Self::SCALE)
        } else if self.0 > Self::SCALE {
            Self(Self::SCALE)
        } else {
            self
        }
    }

    // Method to perform floor operation
    #[inline]
    #[must_use]
    pub const fn floor(self) -> Self {
        Self(self.0 & 0xFFFF0000u32 as i32)
    }

    #[inline]
    #[must_use]
    pub const fn ceil(self) -> Self {
        let remainder = self.0 & (Self::SCALE - 1);
        if remainder == 0 {
            self
        } else {
            Self(self.0 + Self::SCALE - remainder)
        }
    }

    #[inline]
    #[must_use]
    pub const fn round(self) -> Self {
        let rounded_value = (self.0 + Self::HALF_SCALE) & !(Self::SCALE - 1);
        Self(rounded_value)
    }

    #[inline]
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    #[inline]
    #[must_use]
    pub fn sin(self) -> Self {
        lookup_radian(&SIN_TABLE, self)
    }

    #[inline]
    #[must_use]
    pub fn asin(self) -> Self {
        lookup_unit_interval(&ASIN_TABLE, self)
    }

    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        lookup_radian(&COS_TABLE, self)
    }

    #[inline]
    #[must_use]
    pub fn acos(self) -> Self {
        lookup_unit_interval(&ACOS_TABLE, self)
    }

    #[inline]
    #[must_use]
    pub const fn abs(self) -> Self {
        Self(self.0.abs())
    }

    #[inline]
    #[must_use]
    pub const fn signum(self) -> Self {
        Self(self.0.signum() * Self::SCALE)
    }

    #[inline]
    #[must_use]
    pub fn sqrt(self) -> Self {
        assert!(
            self.0 >= 0,
            "negative numbers are undefined for sqrt() {self}"
        );

        if self.0 == 0 {
            return self;
        }

        const MAX_ITERATIONS: usize = 10;
        const TOLERANCE: i32 = 1;
        const TWO: Fp = Fp(Fp::SCALE * 2);

        let mut guess = self / TWO;

        for _ in 0..MAX_ITERATIONS {
            let next_guess = (guess + self / guess) / TWO;

            // Check if the change is within the tolerance level
            if (next_guess - guess).abs().0 <= TOLERANCE {
                return next_guess;
            }

            guess = next_guess;
        }

        guess // Return the last guess if convergence wasn't fully reached
    }

    /// Returns the raw integer value from the `Fp`.
    ///
    /// This method retrieves the underlying raw scaled value stored in the
    /// `Fp` instance. The returned value is the raw integer that represents
    /// the scaled fixed-point number.
    ///
    /// <div class="warning">
    ///
    ///  **Warning:** Directly using the raw value returned by this method should be avoided
    /// unless absolutely necessary. It is generally preferable to use higher-level
    /// methods or conversion traits like `From<T>` and `into()` for conversions,
    /// which handle scaling and ensure correctness. Using `inner()` may expose
    /// the raw value in a way that bypasses intended abstractions and checks,
    /// potentially leading to incorrect usage.
    ///
    /// </div>
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed32::Fp;
    /// let fp = Fp::from_raw(100);
    /// let raw_value = fp.inner();
    ///
    /// // Preferred conversion using From<T> trait
    /// let value_from_fp: i32 = fp.into();
    /// ```
    #[inline]
    #[must_use]
    pub const fn inner(self) -> i32 {
        self.0
    }

    pub const MIN: Self = Self(i32::MIN);
    pub const MAX: Self = Self(i32::MAX);

    #[inline]
    #[must_use]
    fn from_float(value: f32) -> Self {
        Self((value * Self::FSCALE) as i32)
    }

    #[inline]
    #[must_use]
    fn to_float(self) -> f32 {
        self.0 as f32 / Self::FSCALE
    }

    #[inline]
    #[must_use]
    const fn from_int(value: i16) -> Self {
        Self((value as i32) * Self::SCALE)
    }

    #[inline]
    #[must_use]
    const fn to_int(self) -> i16 {
        (self.0 / Self::SCALE) as i16
    }
}

impl From<Fp> for f32 {
    #[inline]
    fn from(fp: Fp) -> Self {
        fp.to_float()
    }
}

impl From<Fp> for i16 {
    #[inline]
    fn from(fp: Fp) -> Self {
        fp.to_int()
    }
}

impl From<Fp> for i32 {
    #[inline]
    fn from(fp: Fp) -> Self {
        fp.to_int() as Self
    }
}

impl TryFrom<Fp> for u16 {
    type Error = String;

    #[inline]
    fn try_from(fp: Fp) -> Result<Self, Self::Error> {
        if fp.0 < 0 {
            Err(format!("Cannot convert a negative Fp value to u16: {fp}"))
        } else {
            Ok(fp.to_int() as Self)
        }
    }
}

impl TryFrom<Fp> for u32 {
    type Error = String;

    #[inline]
    fn try_from(fp: Fp) -> Result<Self, Self::Error> {
        if fp.0 < 0 {
            Err(format!("Cannot convert a negative Fp value to u32: {fp}"))
        } else {
            Ok(fp.to_int() as Self)
        }
    }
}

impl TryFrom<Fp> for usize {
    type Error = String;

    #[inline]
    fn try_from(fp: Fp) -> Result<Self, Self::Error> {
        if fp.0 < 0 {
            Err(format!("Cannot convert a negative Fp value to usize: {fp}"))
        } else {
            Ok(fp.to_int() as Self)
        }
    }
}

impl From<f32> for Fp {
    #[inline]
    fn from(v: f32) -> Self {
        Self::from_float(v)
    }
}

impl From<i16> for Fp {
    #[inline]
    fn from(v: i16) -> Self {
        Self::from_int(v)
    }
}

impl TryFrom<i32> for Fp {
    type Error = std::num::TryFromIntError;

    #[inline]
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        let v_i16 = i16::try_from(v)?;
        Ok(Self::from_int(v_i16))
    }
}

impl fmt::Debug for Fp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fp:{:.4} ({})", (self.0 as f32) / Self::FSCALE, self.0)
    }
}

impl fmt::Display for Fp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}", (self.0 as f32) / Self::FSCALE)
    }
}

impl Mul<Self> for Fp {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self((((self.0 as i64) * (rhs.0 as i64)) / (Self::SCALE as i64)) as i32)
    }
}

impl Div<Self> for Fp {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        assert!(rhs.0 != 0, "division by zero");

        let dividend_i64 = self.0 as i64;
        let divisor_i64 = rhs.0 as i64;
        let quotient = dividend_i64 * Self::SCALE_I64 / divisor_i64;

        assert!(
            !(quotient > i32::MAX as i64 || quotient < i32::MIN as i64),
            "overflow occurred in Fp::div"
        );

        Self(quotient as i32)
    }
}

impl Sub<Self> for Fp {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Add<Self> for Fp {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Fp {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl SubAssign for Fp {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl Neg for Fp {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Div<Fp> for i16 {
    type Output = Fp;

    #[inline]
    fn div(self, rhs: Fp) -> Self::Output {
        Fp((self as i32) * Fp::SCALE / rhs.0 * Fp::SCALE)
    }
}

impl Mul<Fp> for i16 {
    type Output = Fp;

    #[inline]
    fn mul(self, rhs: Fp) -> Self::Output {
        Fp((self as i32) * rhs.0)
    }
}

impl Mul<i16> for Fp {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i16) -> Self {
        Self(self.0 * (rhs as i32))
    }
}

impl Rem for Fp {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }
}

impl PartialOrd<i16> for Fp {
    fn partial_cmp(&self, other: &i16) -> Option<Ordering> {
        Some(self.0.cmp(&(*other as i32 * Self::SCALE)))
    }
}

impl PartialEq<i16> for Fp {
    fn eq(&self, other: &i16) -> bool {
        self.0 == (*other as i32 * Self::SCALE)
    }
}

#[inline]
fn lookup_normalized(slice: &[Fp], fraction: Fp) -> Fp {
    let frac_index = fraction * (slice.len() as i16);
    let index: usize = frac_index.try_into().expect("should work");
    let index = index.min(slice.len() - 1);
    slice[index]
}

#[inline]
fn lookup_unit_interval(slice: &[Fp], unit_interval: Fp) -> Fp {
    assert!(
        !(unit_interval < Fp::neg_one() || unit_interval > Fp::one()),
        "illegal range for unit interval lookup {unit_interval} must be between -1 to 1"
    );
    let num_entries = slice.len() as i16;
    let frac_index = (unit_interval + Fp::one()) * num_entries / Fp::from(2);
    let index: usize = frac_index.try_into().expect("should work");
    let index = index.min(slice.len() - 1);
    slice[index]
}

#[inline]
fn lookup_radian(slice: &[Fp], radians: Fp) -> Fp {
    let radians_modulo = radians % Fp::TAU;
    let normalized_slice_index = radians_modulo / Fp::TAU;
    lookup_normalized(slice, normalized_slice_index)
}
