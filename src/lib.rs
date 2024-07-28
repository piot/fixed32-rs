/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-rs
 * Licensed under the MIT License. See LICENSE file for details.
 */
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

/// The scaling factor used for fixed-point arithmetic.
pub const SCALE: i32 = 0x10000;
pub const SCALE_I64: i64 = SCALE as i64;
pub const FSCALE: f32 = SCALE as f32;

/// A fixed-point number with 16.16 format.
#[derive(Clone, Copy, Default, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub struct Fp(pub i32);

impl Fp {
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
        Self(SCALE)
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
        Self(-SCALE)
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

    pub const MIN: Fp = Fp(i32::MIN);
    pub const MAX: Fp = Fp(i32::MAX);

    #[inline]
    fn from_float(value: f32) -> Self {
        Fp((value * FSCALE) as i32)
    }

    #[inline]
    fn to_float(self) -> f32 {
        self.0 as f32 / FSCALE
    }

    #[inline]
    fn from_int(value: i16) -> Self {
        Self((value as i32) * SCALE)
    }

    #[inline]
    fn to_int(self) -> i16 {
        (self.0 / SCALE) as i16
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
        write!(f, "fp:{:.3} ({})", (self.0 as f32) / FSCALE, self.0)
    }
}

impl fmt::Display for Fp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", (self.0 as f32) / FSCALE)
    }
}

impl Mul<Fp> for Fp {
    type Output = Fp;

    #[inline]
    fn mul(self, rhs: Fp) -> Self {
        Fp((((self.0 as i64) * (rhs.0 as i64)) / (SCALE as i64)) as i32)
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
        let quotient = dividend_i64 * SCALE_I64 / divisor_i64;

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
        eprintln!("hello {:?} / {:?}", (self as i32) * SCALE, rhs.0);
        Fp((self as i32) * SCALE / rhs.0 * SCALE)
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
    fn mul(self, rhs: i16) -> Self::Output {
        Fp(self.0 * (rhs as i32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let result = Fp::from(2) + Fp::from(2);
        assert_eq!(result.0, 4 * SCALE);
    }

    #[test]
    fn mul() {
        let result = Fp::from(3) * Fp::from(2);
        assert_eq!(result.0, 6 * SCALE);
    }

    #[test]
    fn div() {
        let result = Fp::from(99) / Fp::from(3);
        assert_eq!(result.0, 33 * SCALE);
    }

    #[test]
    fn div_bigger_number() {
        let result = Fp::from(30000) / Fp::from(12);
        assert_eq!(result.0, 2500 * SCALE);
    }

    #[test]
    fn sub() {
        let result = Fp::from(-42) + Fp::from(43);
        assert_eq!(result.0, 1 * SCALE);
    }

    #[test]
    fn div_int() {
        let result = 400 / Fp::from(10);
        let i: i32 = result.into();
        assert_eq!(i, 40);
    }

    #[test]
    fn mul_int() {
        let result = 99 * Fp::from(10);
        let i: i16 = result.into();
        assert_eq!(i, 990);
    }

    #[test]
    fn from_float() {
        let result = 99 * Fp::from(10.0);
        let i: i16 = result.into();
        assert_eq!(i, 990);
    }

    #[test]
    fn from_int() {
        let result = 99 * Fp::from(10);
        let i: i16 = result.into();
        assert_eq!(i, 990);
    }
}
