use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num_traits::{Float, FromPrimitive, Num, NumCast, One, ToPrimitive, Zero};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct FastF32(pub f32);

impl Zero for FastF32 {
    fn zero() -> Self {
        Self(f32::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for FastF32 {
    fn one() -> Self {
        Self(f32::one())
    }
}

impl Num for FastF32 {
    type FromStrRadixErr = <f32 as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        <f32 as Num>::from_str_radix(str, radix).map(Self)
    }
}

impl FromPrimitive for FastF32 {
    fn from_i64(n: i64) -> Option<Self> {
        f32::from_i64(n).map(Self)
    }

    fn from_u64(n: u64) -> Option<Self> {
        f32::from_u64(n).map(Self)
    }

    fn from_isize(n: isize) -> Option<Self> {
        f32::from_isize(n).map(Self)
    }

    fn from_i8(n: i8) -> Option<Self> {
        f32::from_i8(n).map(Self)
    }

    fn from_i16(n: i16) -> Option<Self> {
        f32::from_i16(n).map(Self)
    }

    fn from_i32(n: i32) -> Option<Self> {
        f32::from_i32(n).map(Self)
    }

    fn from_i128(n: i128) -> Option<Self> {
        f32::from_i128(n).map(Self)
    }

    fn from_usize(n: usize) -> Option<Self> {
        f32::from_usize(n).map(Self)
    }

    fn from_u8(n: u8) -> Option<Self> {
        f32::from_u8(n).map(Self)
    }

    fn from_u16(n: u16) -> Option<Self> {
        f32::from_u16(n).map(Self)
    }

    fn from_u32(n: u32) -> Option<Self> {
        f32::from_u32(n).map(Self)
    }

    fn from_u128(n: u128) -> Option<Self> {
        f32::from_u128(n).map(Self)
    }

    fn from_f32(n: f32) -> Option<Self> {
        f32::from_f32(n).map(Self)
    }

    fn from_f64(n: f64) -> Option<Self> {
        f32::from_f64(n).map(Self)
    }
}

impl ToPrimitive for FastF32 {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    fn to_isize(&self) -> Option<isize> {
        self.0.to_isize()
    }

    fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }

    fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }

    fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }

    fn to_i128(&self) -> Option<i128> {
        self.0.to_i128()
    }

    fn to_usize(&self) -> Option<usize> {
        self.0.to_usize()
    }

    fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }

    fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }

    fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }

    fn to_u128(&self) -> Option<u128> {
        self.0.to_u128()
    }

    fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}

impl NumCast for FastF32 {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        <f32 as NumCast>::from(n).map(Self)
    }
}

impl Float for FastF32 {
    fn nan() -> Self {
        Self(f32::nan())
    }

    fn infinity() -> Self {
        Self(f32::infinity())
    }

    fn neg_infinity() -> Self {
        Self(f32::neg_infinity())
    }

    fn neg_zero() -> Self {
        Self(f32::neg_zero())
    }

    fn min_value() -> Self {
        Self(f32::min_value())
    }

    fn min_positive_value() -> Self {
        Self(f32::min_positive_value())
    }

    fn max_value() -> Self {
        Self(f32::max_value())
    }

    fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    fn is_normal(self) -> bool {
        self.0.is_normal()
    }

    fn classify(self) -> std::num::FpCategory {
        self.0.classify()
    }

    fn floor(self) -> Self {
        Self(self.0.floor())
    }

    fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    fn round(self) -> Self {
        Self(self.0.round())
    }

    fn trunc(self) -> Self {
        Self(self.0.trunc())
    }

    fn fract(self) -> Self {
        Self(self.0.fract())
    }

    fn abs(self) -> Self {
        Self(self.0.abs())
    }

    fn signum(self) -> Self {
        Self(self.0.signum())
    }

    fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Self(self.0.mul_add(a.0, b.0))
    }

    fn recip(self) -> Self {
        Self(self.0.recip())
    }

    fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }

    fn powf(self, n: Self) -> Self {
        Self(self.0.powf(n.0))
    }

    fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }

    fn exp(self) -> Self {
        Self(self.0.exp())
    }

    fn exp2(self) -> Self {
        Self(self.0.exp2())
    }

    fn ln(self) -> Self {
        Self(self.0.ln())
    }

    fn log(self, base: Self) -> Self {
        Self(self.0.log(base.0))
    }

    fn log2(self) -> Self {
        Self(self.0.log2())
    }

    fn log10(self) -> Self {
        Self(self.0.log10())
    }

    fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    fn abs_sub(self, other: Self) -> Self {
        #[allow(deprecated)]
        Self(self.0.abs_sub(other.0))
    }

    fn cbrt(self) -> Self {
        Self(self.0.cbrt())
    }

    fn hypot(self, other: Self) -> Self {
        Self(self.0.hypot(other.0))
    }

    fn sin(self) -> Self {
        Self(self.0.sin())
    }

    fn cos(self) -> Self {
        Self(self.0.cos())
    }

    fn tan(self) -> Self {
        Self(self.0.tan())
    }

    fn asin(self) -> Self {
        Self(self.0.asin())
    }

    fn acos(self) -> Self {
        Self(self.0.acos())
    }

    fn atan(self) -> Self {
        Self(self.0.atan())
    }

    fn atan2(self, other: Self) -> Self {
        Self(self.0.atan2(other.0))
    }

    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.0.sin_cos();
        (Self(sin), Self(cos))
    }

    fn exp_m1(self) -> Self {
        Self(self.0.exp_m1())
    }

    fn ln_1p(self) -> Self {
        Self(self.0.ln_1p())
    }

    fn sinh(self) -> Self {
        Self(self.0.sinh())
    }

    fn cosh(self) -> Self {
        Self(self.0.cosh())
    }

    fn tanh(self) -> Self {
        Self(self.0.tanh())
    }

    fn asinh(self) -> Self {
        Self(self.0.asinh())
    }

    fn acosh(self) -> Self {
        Self(self.0.acosh())
    }

    fn atanh(self) -> Self {
        Self(self.0.atanh())
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        self.0.integer_decode()
    }
}

impl std::fmt::Display for FastF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Neg for FastF32 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for FastF32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.algebraic_add(rhs.0))
    }
}

impl Sub for FastF32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.algebraic_sub(rhs.0))
    }
}

impl Mul for FastF32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.algebraic_mul(rhs.0))
    }
}

impl Div for FastF32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.algebraic_div(rhs.0))
    }
}

impl Rem for FastF32 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0.algebraic_rem(rhs.0))
    }
}
