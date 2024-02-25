use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use std::fmt;

pub const SCALE: i32 = 1000;
pub const FSCALE: f32 = SCALE as f32;

#[derive(Debug, Clone, Copy, Default, Ord, Eq, PartialEq, PartialOrd)]
pub struct Fp(pub i32);

impl Fp {
    pub fn from_float(value: f32) -> Self {
        Fp((value * FSCALE) as i32)
    }

    pub fn to_float(&self) -> f32 {
        self.0 as f32 / FSCALE
    }

    pub fn from_int(value: i16) -> Self {
        Self((value as i32) * SCALE)
    }

    pub fn to_int(&self) -> i16 {
        (self.0 / SCALE) as i16
    }

    pub fn one() -> Self {
        Self(SCALE)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub const MIN:Fp = Fp(i32::MIN);
    pub const MAX:Fp = Fp(i32::MAX);

}

impl fmt::Display for Fp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fp:{:.3} ({})", (self.0 as f32) / FSCALE, self.0)
    }
}

impl Mul<Fp> for Fp {
    type Output = Fp;

    fn mul(self, rhs: Fp) -> Self::Output {
        Fp(  (((self.0 as i64) * (rhs.0 as i64)) / (SCALE as i64)) as i32)
    }
}

impl Div<Fp> for Fp {
    type Output = Fp;

    fn div(self, rhs: Fp) -> Self::Output {
        Fp((self.0 * SCALE) / rhs.0) // Scale to avoid overflow
    }
}

impl Sub<Fp> for Fp {
    type Output = Fp;

    fn sub(self, rhs: Fp) -> Self::Output {
        Fp(self.0 - rhs.0)
    }
}

impl Add<Fp> for Fp {
    type Output = Fp;

    fn add(self, rhs: Fp) -> Self::Output {
        Fp(self.0 + rhs.0)
    }
}

impl AddAssign for Fp {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Neg for Fp {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Fp(-self.0)
    }
}

impl Div<Fp> for i16 {
    type Output = Fp;

    fn div(self, rhs: Fp) -> Self::Output {
        eprintln!("hello {:?} / {:?}", (self as i32) * SCALE, rhs.0);
        Fp ( (self as i32) * SCALE / rhs.0 * SCALE)
    }
}

impl Mul<Fp> for i16 {
    type Output = Fp;

    fn mul(self, rhs: Fp) -> Self::Output {
        Fp((self as i32) * rhs.0)
    }
}

impl Mul<i16> for Fp {
    type Output = Fp;

    fn mul(self, rhs: i16) -> Self::Output {
        Fp(self.0 * (rhs as i32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let result = Fp::from_int(2) + Fp::from_int(2);
        assert_eq!(result.0, 4 * SCALE);
    }

    #[test]
    fn mul() {
        let result = Fp::from_int(3) * Fp::from_int(2);
        assert_eq!(result.0, 6 * SCALE);
    }

    #[test]
    fn div() {
        let result = Fp::from_int(99) / Fp::from_int(3);
        assert_eq!(result.0, 33 * SCALE);
    }

    #[test]
    fn sub() {
        let result = Fp::from_int(-42) + Fp::from_int(43);
        assert_eq!(result.0, 1 * SCALE);
    }

    #[test]
    fn div_int() {
        let result = 400 / Fp::from_int(10);
        assert_eq!(result.to_int(), 40);
    }

    #[test]
    fn mul_int() {
        let result = 99 * Fp::from_int(10);
        assert_eq!(result.to_int(), 990);
    }

}
