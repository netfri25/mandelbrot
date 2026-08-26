use num_traits::{Float, FromPrimitive};

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

impl<T: Float + FromPrimitive> Producer<T> for NaiveProducer {
    fn produce(&mut self, start: Pos<T>, size: Size<T>, dims: Dimensions) -> Vec<f32> {
        let max_iterations = self.max_iterations;
        let step_x = size.w / T::from_usize(dims.w).unwrap();
        let step_y = size.h / T::from_usize(dims.h).unwrap();

        range(start.y, step_y)
            .take(dims.h)
            .flat_map(move |y| {
                range(start.x, step_x).take(dims.w).map(move |x| {
                    let value = Complex::new(x, y);
                    (divergence_iteration(value, max_iterations) as f32)
                        .algebraic_div(max_iterations as f32)
                })
            })
            .collect()
    }
}

fn divergence_iteration<T: Float + FromPrimitive>(c: Complex<T>, max_iterations: u32) -> u32 {
    let mut z = c;

    let bound_squared = T::from_f32(5.0).unwrap();
    let mut iteration = 1;

    loop {
        if iteration >= max_iterations {
            return iteration;
        }

        if z.abs_squared() >= bound_squared {
            return iteration;
        }

        z = z * z + c;

        // will return `None` on overflow, which is the expected behavior, since `None` represents
        // divergence
        iteration += 1;
    }
}

fn range<T: Float>(start: T, step: T) -> impl Iterator<Item = T> {
    std::iter::successors(Some(start), move |x| Some(*x + step))
}
