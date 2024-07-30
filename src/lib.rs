/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-rs
 * Licensed under the MIT License. See LICENSE file for details.
 */
use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub, SubAssign};

use crate::lookup_slices::{ACOS_TABLE, ASIN_TABLE, COS_TABLE, SIN_TABLE};

mod lookup_slices;
mod test;

/// A fixed-point number with 16.16 format.
#[derive(Clone, Copy, Default, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub struct Fp(pub i32);

impl Fp {
    /// The scaling factor used for fixed-point arithmetic.
    pub const SHIFT: i32 = 16;
    pub const SCALE: i32 = 1 << Self::SHIFT;

    const HALF_SCALE: i32 = Self::SCALE / 2;
    pub const SCALE_I64: i64 = Self::SCALE as i64;
    pub const FSCALE: f32 = Self::SCALE as f32;
    pub const FRAC_PI_2: Fp = Fp(Self::SCALE * 1570 / 1000); // π/2 ≈ 1.570
    pub const PI: Fp = Fp(Self::SCALE * 3141 / 1000); // π ≈ 3.141
    pub const TAU: Fp = Fp(Self::SCALE * 6283 / 1000); // 2π ≈ 6.283

    /// Returns the constant `Fp` value for one.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// assert_eq!(1, Fp::one().into());
    /// ```
    #[inline]
    pub fn one() -> Self {
        Self(Self::SCALE)
    }

    /// Checks if the `Fp` value is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed32::Fp;
    /// assert!(Fp::zero().is_zero());
    /// ```
    #[inline]
    pub fn is_zero(self) -> bool {
        self.0 == 0
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
    pub fn neg_one() -> Self {
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
    pub fn zero() -> Self {
        Self(0)
    }

    #[inline]
    // Clamp value to the range [-1, 1]
    pub fn normalize(self) -> Self {
        if self.0 < -Self::SCALE {
            Fp(-Self::SCALE)
        } else if self.0 > Self::SCALE {
            Fp(Self::SCALE)
        } else {
            self
        }
    }

    // Method to perform floor operation
    fn floor(self) -> Self {
        Self(self.0 & 0xFFFF0000u32 as i32)
    }

    #[inline]
    pub fn ceil(self) -> Self {
        let remainder = self.0 & (Self::SCALE - 1);
        if remainder == 0 {
            self
        } else {
            Self(self.0 + Self::SCALE - remainder)
        }
    }

    #[inline]
    pub fn round(self) -> Self {
        let rounded_value = (self.0 + Self::HALF_SCALE) & !(Self::SCALE - 1);
        Self(rounded_value)
    }

    #[inline]
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
    pub fn sin(self) -> Self {
        lookup_radian(&SIN_TABLE, self)
    }
    #[inline]
    pub fn asin(self) -> Self {
        lookup_unit_interval(&ASIN_TABLE, self)
    }

    #[inline]
    pub fn cos(self) -> Self {
        lookup_radian(&COS_TABLE, self)
    }

    #[inline]
    pub fn acos(self) -> Self {
        lookup_unit_interval(&ACOS_TABLE, self)
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
    #[inline]
    pub fn sqrt(self) -> Self {
        if self.0 < 0 {
            panic!("negative numbers are undefined for sqrt() {self}");
        }

        let mut guess = self;
        const TWO: Fp = Fp(Fp::SCALE * 2);
        const TOLERANCE: i32 = 1;

        while (guess * guess - self).abs().0 > TOLERANCE {
            guess = (guess + self / guess) / TWO;
        }

        guess
    }

    pub const MIN: Fp = Fp(i32::MIN);
    pub const MAX: Fp = Fp(i32::MAX);

    #[inline]
    fn from_float(value: f32) -> Self {
        Fp((value * Self::FSCALE) as i32)
    }

    #[inline]
    fn to_float(self) -> f32 {
        self.0 as f32 / Self::FSCALE
    }

    #[inline]
    fn from_int(value: i16) -> Self {
        Self((value as i32) * Self::SCALE)
    }

    #[inline]
    fn to_int(self) -> i16 {
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
        fp.to_int() as i32
    }
}

impl From<Fp> for usize {
    #[inline]
    fn from(fp: Fp) -> Self {
        if fp.0 < 0 {
            panic!("Cannot convert a negative Fp value to usize: {fp}");
        }
        fp.to_int() as usize
    }
}

impl From<Fp> for u16 {
    #[inline]
    fn from(fp: Fp) -> Self {
        if fp.0 < 0 {
            panic!("Cannot convert a negative Fp value to u16: {fp}");
        }
        fp.to_int() as u16
    }
}

impl From<Fp> for u32 {
    #[inline]
    fn from(fp: Fp) -> Self {
        if fp.0 < 0 {
            panic!("Cannot convert a negative Fp value to u32: {fp}");
        }
        fp.to_int() as u32
    }
}

impl From<f32> for Fp {
    #[inline]
    fn from(v: f32) -> Self {
        Fp::from_float(v)
    }
}

impl From<i16> for Fp {
    #[inline]
    fn from(v: i16) -> Self {
        Fp::from_int(v)
    }
}

impl fmt::Debug for Fp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fp:{:.3} ({})", (self.0 as f32) / Self::FSCALE, self.0)
    }
}

impl fmt::Display for Fp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", (self.0 as f32) / Self::FSCALE)
    }
}

impl Mul<Fp> for Fp {
    type Output = Fp;

    #[inline]
    fn mul(self, rhs: Fp) -> Self {
        Fp((((self.0 as i64) * (rhs.0 as i64)) / (Self::SCALE as i64)) as i32)
    }
}

impl Div<Fp> for Fp {
    type Output = Fp;

    #[inline]
    fn div(self, rhs: Fp) -> Self {
        if rhs.0 == 0 {
            panic!("division by zero");
        }

        let dividend_i64 = self.0 as i64;
        let divisor_i64 = rhs.0 as i64;
        let quotient = dividend_i64 * Self::SCALE_I64 / divisor_i64;

        if quotient > i32::MAX as i64 || quotient < i32::MIN as i64 {
            panic!("overflow occurred in Fp::div");
        }

        Self(quotient as i32)
    }
}

impl Sub<Fp> for Fp {
    type Output = Fp;

    #[inline]
    fn sub(self, rhs: Fp) -> Self {
        Fp(self.0 - rhs.0)
    }
}

impl Add<Fp> for Fp {
    type Output = Fp;

    #[inline]
    fn add(self, rhs: Fp) -> Self {
        Fp(self.0 + rhs.0)
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
        Fp(-self.0)
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
    type Output = Fp;

    #[inline]
    fn mul(self, rhs: i16) -> Self {
        Fp(self.0 * (rhs as i32))
    }
}

impl Rem for Fp {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }
}

#[inline]
fn lookup_normalized(slice: &[Fp], fraction: Fp) -> Fp {
    let frac_index = fraction * (slice.len() as i16);
    let index: usize = frac_index.floor().into();
    let index = index.min(slice.len() - 1);
    slice[index]
}

#[inline]
fn lookup_unit_interval(slice: &[Fp], unit_interval: Fp) -> Fp {
    if unit_interval < Fp::neg_one() || unit_interval > Fp::one() {
        panic!("illegal range for unit interval lookup {unit_interval} must be between -1 to 1");
    }
    let num_entries = slice.len() as i16;
    let frac_index = (unit_interval + Fp::one()) * num_entries / Fp::from(2);
    let index: usize = frac_index.floor().into();
    let index = index.min(slice.len() - 1);
    slice[index]
}

fn lookup_radian(slice: &[Fp], radians: Fp) -> Fp {
    let radians_modulo = radians % Fp::TAU;
    let normalized_slice_index = radians_modulo / Fp::TAU;
    lookup_normalized(slice, normalized_slice_index)
}
