use std::ops::{Add, Div, Mul, Sub};

use std::simd::{Mask, MaskElement, Select, Simd, SimdCast, SimdElement, cmp::SimdPartialOrd};

use super::Producer;

use crate::from_f32::FromF32;
use crate::types::{Dimensions, Pos, Size};

#[derive(Clone)]
pub struct SimdProducer<const LANES: usize> {
    max_iterations: u32,
}

impl<const LANES: usize> SimdProducer<LANES> {
    pub fn new(max_iterations: u32) -> Self {
        Self { max_iterations }
    }
}

impl<const LANES: usize, T> Producer<T> for SimdProducer<LANES>
where
    T: FromF32 + Clone + PartialOrd + SimdElement + SimdCast,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Div<T, Output = T>,
    <T as SimdElement>::Mask: MaskElement,
    Mask<<T as SimdElement>::Mask, LANES>: Select<Simd<T, LANES>>,
    Simd<T, LANES>: SimdPartialOrd<Mask = Mask<<T as SimdElement>::Mask, LANES>>,
    Simd<T, LANES>: Add<Simd<T, LANES>, Output = Simd<T, LANES>>,
    Simd<T, LANES>: Sub<Simd<T, LANES>, Output = Simd<T, LANES>>,
    Simd<T, LANES>: Mul<Simd<T, LANES>, Output = Simd<T, LANES>>,
{
    fn produce(&mut self, start: Pos<T>, size: Size<T>, dims: Dimensions) -> Vec<f32> {
        let max_iterations = self.max_iterations;
        let step_x = size.w / T::from_f32(dims.w as f32);
        let step_y = size.h / T::from_f32(dims.h as f32);

        let mut output = Vec::with_capacity(dims.w * dims.h);

        let lane_offsets = Simd::from_array(std::array::from_fn(|i| T::from_f32(i as f32)));
        let starts_xs = Simd::splat(start.x);
        let steps_xs = Simd::splat(step_x);

        for y in range(start.y, step_y).take(dims.h) {
            let ys = Simd::splat(y);

            // chunks
            let chunks_count = dims.w / LANES;
            for chunk_index in 0..chunks_count {
                let chunk_offsets = Simd::splat(T::from_f32((chunk_index * LANES) as f32));
                let xs = starts_xs + steps_xs * (lane_offsets + chunk_offsets);

                let mask = Mask::from_bitmask(u64::MAX);
                let result = divergence_iteration_simd(xs, ys, max_iterations, mask)
                    .to_array()
                    .map(|value| value as f32 / max_iterations as f32);

                output.extend(result);
            }

            // remainder
            let remainder_size = dims.w % LANES;
            if remainder_size != 0 {
                let start_x = start.x + step_x * T::from_f32((chunks_count * LANES) as f32);
                let remainder: Vec<_> = range(start_x, step_x).take(remainder_size).collect();

                let xs = Simd::load_or(&remainder, Simd::splat(T::from_f32(0.)));
                let bitmask = (1 << remainder_size) - 1;
                let mask = Mask::from_bitmask(bitmask);
                let result = divergence_iteration_simd(xs, ys, max_iterations, mask)
                    .to_array()
                    .into_iter()
                    .take(remainder_size)
                    .map(move |value| value as f32 / max_iterations as f32);

                output.extend(result);
            }
        }

        output
    }
}

fn range<T>(start: T, step: T) -> std::iter::Successors<T, impl FnMut(&T) -> Option<T>>
where
    T: Add<T, Output = T> + Clone,
{
    std::iter::successors(Some(start), move |x| Some(x.clone() + step.clone()))
}

#[inline(always)]
fn divergence_iteration_simd<T, const LANES: usize>(
    re: Simd<T, LANES>,
    im: Simd<T, LANES>,
    max_iterations: u32,
    start_mask: Mask<<T as SimdElement>::Mask, LANES>,
) -> Simd<u32, LANES>
where
    T: FromF32 + SimdElement,
    <T as SimdElement>::Mask: MaskElement,
    Mask<<T as SimdElement>::Mask, LANES>: Select<Simd<T, LANES>>,
    Simd<T, LANES>: SimdPartialOrd<Mask = Mask<<T as SimdElement>::Mask, LANES>>,
    Simd<T, LANES>: Add<Simd<T, LANES>, Output = Simd<T, LANES>>,
    Simd<T, LANES>: Sub<Simd<T, LANES>, Output = Simd<T, LANES>>,
    Simd<T, LANES>: Mul<Simd<T, LANES>, Output = Simd<T, LANES>>,
{
    let x0 = re;
    let y0 = im;

    let mut x = x0;
    let mut y = y0;

    let bound = Simd::splat(T::from_f32(4.0));
    let mut iteration = 1;

    let mut active = start_mask;
    let mut iterations = Simd::splat(iteration);

    while iteration < max_iterations && active.any() {
        let x2 = x * x;
        let y2 = y * y;

        y = Simd::splat(T::from_f32(2.)) * x * y + y0;
        x = x2 - y2 + x0;

        active &= (x2 + y2).simd_le(bound);

        iteration += 1;
        iterations += active.cast::<i32>().select(Simd::splat(1), Simd::splat(0));
    }

    iterations
}
