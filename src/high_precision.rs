// TODO:
//  to support future numbers with higher precision, switch this type to use a higher precision type
//  ideally, it should also support `Copy`, but just `Clone` is also fine, but it will require a bit
//  of refactoring.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use fixed::prelude::*;

use crate::exp2::Exp2;
use crate::from_f64::FromF64;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct HighPrecision(fixed::types::I10F118);

impl HighPrecision {
    pub fn to_f64(self) -> f64 {
        FromFixed::from_fixed(self.0)
    }
}

impl From<f32> for HighPrecision {
    fn from(value: f32) -> Self {
        Self(value.to_fixed())
    }
}

impl From<f64> for HighPrecision {
    fn from(value: f64) -> Self {
        Self(value.to_fixed())
    }
}

impl FromF64 for HighPrecision {
    fn from_f64(value: f64) -> Self {
        Self::from(value)
    }
}

impl Exp2 for HighPrecision {
    fn exp2(self) -> Self {
        Self(fixed_analytics::pow2(self.0))
    }
}

impl Neg for HighPrecision {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Add<Self> for HighPrecision {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub<Self> for HighPrecision {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Mul<Self> for HighPrecision {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Div<Self> for HighPrecision {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}

impl AddAssign<Self> for HighPrecision {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign<Self> for HighPrecision {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<Self> for HighPrecision {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign<Self> for HighPrecision {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl From<HighPrecision> for f32 {
    fn from(value: HighPrecision) -> Self {
        value.to_f64() as Self
    }
}

impl From<HighPrecision> for f64 {
    fn from(value: HighPrecision) -> Self {
        value.to_f64() as Self
    }
}

impl From<HighPrecision> for fast_posit::p64 {
    fn from(value: HighPrecision) -> Self {
        Self::from_f64(value.to_f64())
    }
}
