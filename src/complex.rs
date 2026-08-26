use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::Float;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Complex<T: Float> {
    pub re: T,
    pub im: T,
}

impl<T: Float> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }

    pub fn conj(self) -> Self {
        Self {
            im: -self.im,
            ..self
        }
    }

    pub fn abs_squared(self) -> T {
        self.re * self.re + self.im * self.im
    }
}

impl<T: Float> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> std::fmt::Display for Complex<T>
where
    T: Float + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {}i)", self.re, self.im)
    }
}

macro_rules! impl_binop {
    (
        $name:ident,
        $func:ident,
        $name_assign:ident,
        $func_assign:ident,
        | $lhs:ident , $rhs:ident | $body:expr
    ) => {
        impl<T: Float> $name for Complex<T> {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self {
                let $lhs = self;
                let $rhs = rhs;
                $body
            }
        }

        impl<T: Float, U> $name_assign<U> for Complex<T>
        where
            Self: $name<U, Output = Self>,
        {
            fn $func_assign(&mut self, rhs: U) {
                *self = $name::$func(*self, rhs)
            }
        }
    };
}

macro_rules! impl_binop_scalar {
    ($name:ident, $func:ident, | $lhs:ident, $rhs:ident | $body:expr) => {
        impl<T: Float> $name<T> for Complex<T> {
            type Output = Self;

            fn $func(self, rhs: T) -> Self {
                let $lhs = self;
                let $rhs = rhs;
                $body
            }
        }
    };
}

impl<T: Float> Neg for Complex<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl_binop_scalar! {
    Add,
    add,
    |lhs, rhs| Self {
        re: lhs.re + rhs,
        im: lhs.im,
    }
}

impl_binop_scalar! {
    Sub,
    sub,
    |lhs, rhs| Self {
        re: lhs.re - rhs,
        im: lhs.im,
    }
}

impl_binop_scalar! {
    Mul,
    mul,
    |lhs, rhs| Self {
        re: lhs.re * rhs,
        im: lhs.im * rhs,
    }
}

impl_binop_scalar! {
    Div,
    div,
    |lhs, rhs| Self {
        re: lhs.re / rhs,
        im: lhs.im / rhs,
    }
}

impl_binop! {
    Add,
    add,
    AddAssign,
    add_assign,
    |lhs, rhs| Self {
        re: lhs.re + rhs.re,
        im: lhs.im + rhs.im
    }
}

impl_binop! {
    Sub,
    sub,
    SubAssign,
    sub_assign,
    |lhs, rhs| Self {
        re: lhs.re - rhs.re,
        im: lhs.im - rhs.im
    }
}

// (a + bi)*(c + di) = (a*c - b*d) + (b*c + a*d)i
impl_binop! {
    Mul,
    mul,
    MulAssign,
    mul_assign,
    |lhs, rhs| Self {
        re: (lhs.re * rhs.re) - (lhs.im * rhs.im),
        im: (lhs.re * rhs.im) + (lhs.im * rhs.re),
    }
}

// (a + bi) / (c + di)
// ((a + bi) * (c - di)) / ((c + di) * (c - di))
// ((a*c + b*d) + (b*c - a*d)i) / (c^2 + d^2)
// (lhs * Conj(rhs)) / (Re(lhs)^2 + Im(lhs)^2)
impl_binop! {
    Div,
    div,
    DivAssign,
    div_assign,
    |lhs, rhs| {
        let num = lhs * rhs.conj();
        let den = rhs.re * rhs.re + rhs.im * rhs.im;
        num / den
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_op {
        ($op:expr, $tests:expr) => {
            for test in $tests {
                let [lhs, rhs, expected] = test;
                let lhs: Complex<f32> = lhs.into();
                let rhs: Complex<f32> = rhs.into();
                let expected: Complex<f32> = expected.into();
                let result = $op(lhs, rhs);
                assert_eq!(
                    result,
                    expected,
                    "{} {} {} expected {} but got {}",
                    stringify!($op),
                    lhs,
                    rhs,
                    expected,
                    result
                );
            }
        };
    }

    #[test]
    fn add() {
        // [a, b, c]
        // a + b = c
        let tests = [
            [(1., 2.), (3., -4.), (4., -2.)],
            [(-1.2, 3.4), (-5.6, -7.8), (-6.8, -4.4)],
        ];

        test_op!(Add::add, tests);
    }

    #[test]
    fn sub() {
        // [a, b, c]
        // a - b = c
        let tests = [
            [(1., 2.), (3., -4.), (-2., 6.)],
            [(-1.2, 3.4), (-5.6, -7.8), (4.3999996, 11.200001)],
        ];

        test_op!(Sub::sub, tests);
    }

    #[test]
    fn mul() {
        // [a, b, c]
        // a * b = c
        let tests = [
            [(1., 2.), (3., -4.), (11., 2.)],
            [(-1.2, 3.4), (-5.6, -7.8), (33.24, -9.68)],
        ];

        test_op!(Mul::mul, tests);
    }

    #[test]
    fn div() {
        // [a, b, c]
        // a / b = c
        let tests = [
            [(1., 2.), (3., -4.), (-0.2, 0.4)],
            [(-1.2, 3.4), (-5.6, -7.8), (-0.21475053, -0.30802605)],
            [(1., 1.), (1., 1.), (1., 0.)],
            [(1., 0.), (5., 5.), (0.1, -0.1)],
            [(0., 0.), (3., 2.), (0., 0.)],
        ];

        test_op!(Div::div, tests);
    }
}
