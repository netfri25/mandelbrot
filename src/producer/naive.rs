use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::Producer;

use crate::from_f64::FromF64;
use crate::high_precision::HighPrecision;
use crate::types::{Dimensions, Pos, Size};

#[derive(Clone)]
pub struct NaiveProducer<T> {
    max_iterations: u32,
    _number_type: PhantomData<T>,
}

impl<T> NaiveProducer<T> {
    pub fn new(max_iterations: u32) -> Self {
        Self {
            max_iterations,
            _number_type: PhantomData,
        }
    }
}

impl<T> Producer for NaiveProducer<T>
where
    T: FromF64 + Clone + PartialOrd,
    T: Neg<Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Div<T, Output = T>,
{
    fn produce(&mut self, start: Pos, size: Size, dims: Dimensions) -> Vec<f32> {
        let max_iterations = self.max_iterations;
        let step_x = size.w / HighPrecision::from_f64(dims.w as f64);
        let step_y = size.h / HighPrecision::from_f64(dims.h as f64);

        range(start.y, step_y)
            .take(dims.h as usize)
            .flat_map(move |y| {
                range(start.x, step_x).take(dims.w as usize).map(move |x| {
                    let x = T::from_f64(x.to_f64());
                    let y = T::from_f64(y.to_f64());
                    (divergence_iteration(x, y, max_iterations) as f32)
                        .algebraic_div(max_iterations as f32)
                })
            })
            .collect()
    }
}

fn range<T>(start: T, step: T) -> std::iter::Successors<T, impl FnMut(&T) -> Option<T>>
where
    T: Add<T, Output = T> + Clone,
{
    std::iter::successors(Some(start), move |x| Some(x.clone() + step.clone()))
}

fn divergence_iteration<T>(re: T, im: T, max_iterations: u32) -> u32
where
    T: FromF64 + Clone + PartialOrd,
    T: Neg<Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    let x0 = re;
    let y0 = im;

    let mut x = T::from_f64(0.);
    let mut y = T::from_f64(0.);
    let mut x2 = T::from_f64(0.);
    let mut y2 = T::from_f64(0.);

    let bound = T::from_f64(4.0);
    let mut iteration = 1;

    while x2.clone() + y2.clone() <= bound && iteration < max_iterations {
        let prev_x = x.clone();
        let prev_y = y.clone();

        x2 = x.clone() * x.clone();
        y2 = y.clone() * y.clone();

        y = T::from_f64(2.) * x.clone() * y + y0.clone();
        x = x2.clone() - y2.clone() + x0.clone();

        // if it converges earlier, then it's inside
        if prev_x == x && prev_y == y {
            return max_iterations;
        }

        iteration += 1;
    }

    iteration
}
