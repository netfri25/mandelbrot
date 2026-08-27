use std::ops::{Add, Div, Mul, Neg, Sub};

use super::Producer;

use crate::complex::Complex;
use crate::from_f32::FromF32;
use crate::types::{Dimensions, Pos, Size};

#[derive(Clone)]
pub struct NaiveProducer {
    max_iterations: u32,
}

impl NaiveProducer {
    pub fn new(max_iterations: u32) -> Self {
        Self { max_iterations }
    }
}

impl<T> Producer<T> for NaiveProducer
where
    T: FromF32 + Clone + PartialOrd,
    T: Neg<Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Div<T, Output = T>,
{
    fn produce(&mut self, start: Pos<T>, size: Size<T>, dims: Dimensions) -> Vec<f32> {
        let max_iterations = self.max_iterations;
        let step_x = size.w / T::from_f32(dims.w as f32);
        let step_y = size.h / T::from_f32(dims.h as f32);

        range(start.y, step_y)
            .take(dims.h)
            .flat_map(move |y| {
                range(start.x.clone(), step_x.clone())
                    .take(dims.w)
                    .map(move |x| {
                        let value = Complex::new(x, y.clone());
                        (divergence_iteration(value, max_iterations) as f32)
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

fn divergence_iteration<T>(c: Complex<T>, max_iterations: u32) -> u32
where
    T: FromF32 + Clone + PartialOrd,
    T: Neg<Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    let x0 = c.re;
    let y0 = c.im;

    let mut x = T::from_f32(0.);
    let mut y = T::from_f32(0.);
    let mut x2 = T::from_f32(0.);
    let mut y2 = T::from_f32(0.);

    let bound = T::from_f32(4.0);
    let mut iteration = 1;

    while x2.clone() + y2.clone() <= bound && iteration < max_iterations {
        let prev_x = x.clone();
        let prev_y = y.clone();

        x2 = x.clone() * x.clone();
        y2 = y.clone() * y.clone();

        y = T::from_f32(2.) * x.clone() * y + y0.clone();
        x = x2.clone() - y2.clone() + x0.clone();

        // if it converges earlier, then it's inside
        if prev_x == x && prev_y == y {
            return max_iterations;
        }

        iteration += 1;
    }

    iteration
}
