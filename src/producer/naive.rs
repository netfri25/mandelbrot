use std::ops::{Add, Neg};

use num_traits::NumOps;

use super::Producer;

use crate::complex::Complex;
use crate::types::{Dimensions, Pos, Size};

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
    T: From<f32> + Clone + PartialOrd + NumOps + Neg<Output = T>
{
    fn produce(&mut self, start: Pos<T>, size: Size<T>, dims: Dimensions) -> Vec<f32> {
        let max_iterations = self.max_iterations;
        let step_x = size.w / T::from(dims.w as f32);
        let step_y = size.h / T::from(dims.h as f32);

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

fn divergence_iteration<T>(c: Complex<T>, max_iterations: u32) -> u32
where
    T: From<f32> + Clone + PartialOrd + NumOps + Neg<Output = T>,
{
    let mut z = c.clone();

    let bound_squared = T::from(5.0);
    let mut iteration = 1;

    loop {
        if iteration >= max_iterations {
            return iteration;
        }

        if z.abs_squared() >= bound_squared {
            return iteration;
        }

        z = z.clone() * z + c.clone();

        // will return `None` on overflow, which is the expected behavior, since `None` represents
        // divergence
        iteration += 1;
    }
}

fn range<T>(start: T, step: T) -> std::iter::Successors<T, impl FnMut(&T) -> Option<T>>
where
    T: Add<T, Output = T> + Clone,
{
    std::iter::successors(Some(start), move |x| Some(x.clone() + step.clone()))
}
