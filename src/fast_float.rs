use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::from_f64::FromF64;

macro_rules! impl_binop {
    (
        $target:ident,
        $name:ident,
        $func:ident,
        $name_assign:ident,
        $func_assign:ident,
        | $lhs:ident , $rhs:ident | $body:expr
    ) => {
        impl $name<Self> for $target {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self {
                let $lhs = self;
                let $rhs = rhs;
                $body
            }
        }

        impl<'a> $name<&'a $target> for $target {
            type Output = $target;

            fn $func(self, rhs: &'a $target) -> Self::Output {
                $target::$func(self, *rhs)
            }
        }

        impl<'a> $name<&'a $target> for &'a $target {
            type Output = $target;

            fn $func(self, rhs: &'a $target) -> Self::Output {
                $target::$func(*self, *rhs)
            }
        }

        impl<U> $name_assign<U> for $target
        where
            Self: $name<U, Output = Self>,
        {
            fn $func_assign(&mut self, rhs: U) {
                *self = $name::$func(*self, rhs)
            }
        }
    };
}

macro_rules! impl_all {
    ($target_name:ident, $target_type:ident) => {
        #[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $target_name(pub $target_type);

        impl FromF64 for $target_name {
            fn from_f64(value: f64) -> Self {
                Self(value as $target_type)
            }
        }

        impl<T> From<T> for $target_name
        where
            $target_type: From<T>,
        {
            fn from(value: T) -> Self {
                Self($target_type::from(value))
            }
        }

        impl std::fmt::LowerExp for $target_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::LowerExp::fmt(&self.0, f)
            }
        }

        impl std::fmt::Display for $target_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl Neg for $target_name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl_binop! {
            $target_name,
            Add,
            add,
            AddAssign,
            add_assign,
            |lhs, rhs| Self(lhs.0.algebraic_add(rhs.0))
        }

        impl_binop! {
            $target_name,
            Sub,
            sub,
            SubAssign,
            sub_assign,
            |lhs, rhs| Self(lhs.0.algebraic_sub(rhs.0))
        }

        impl_binop! {
            $target_name,
            Mul,
            mul,
            MulAssign,
            mul_assign,
            |lhs, rhs| Self(lhs.0.algebraic_mul(rhs.0))
        }

        impl_binop! {
            $target_name,
            Div,
            div,
            DivAssign,
            div_assign,
            |lhs, rhs| Self(lhs.0.algebraic_div(rhs.0))
        }

        impl_binop! {
            $target_name,
            Rem,
            rem,
            RemAssign,
            rem_assign,
            |lhs, rhs| Self(lhs.0.algebraic_rem(rhs.0))
        }
    };
}

impl_all!(FastF32, f32);
impl_all!(FastF64, f64);
