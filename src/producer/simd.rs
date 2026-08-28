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

        let lanes = T::from_f32(LANES as f32);
        let chunk_step_x = step_x * lanes;

        let lane_offsets = Simd::from_array(std::array::from_fn(|i| T::from_f32(i as f32)));

        let step_xs = Simd::splat(step_x);
        let chunk_step_xs = Simd::splat(chunk_step_x);

        let chunks_count = dims.w / LANES;
        let remainder = dims.w % LANES;

        let mut output = Vec::with_capacity(dims.w * dims.h);

        let mut y = start.y;

        for _ in 0..dims.h {
            let ys = Simd::splat(y);

            // chunks
            let mut xs = Simd::splat(start.x) + step_xs * lane_offsets;
            for _ in 0..chunks_count {
                let iterations = divergence_iteration_simd(xs, ys, max_iterations);

                let normalized = iterations
                    .to_array()
                    .map(|value| value as f32 / max_iterations as f32);

                output.extend(normalized);

                xs += chunk_step_xs;
            }

            // remainder
            if remainder != 0 {
                let start_x = start.x + chunk_step_x * T::from_f32(chunks_count as f32);

                let xs = Simd::splat(start_x) + step_xs * lane_offsets;

                let iterations = divergence_iteration_simd(xs, ys, max_iterations);

                let normalized = iterations
                    .to_array()
                    .into_iter()
                    .map(|value| value as f32 / max_iterations as f32)
                    .take(remainder);

                output.extend(normalized);
            }

            y = y + step_y;
        }

        output
    }
}

// https://www.intel.com/content/www/us/en/developer/articles/technical/accelerating-compute-intensive-workloads-with-intel-avx-512-using-microsoft-visual-studio.html
#[inline(always)]
fn divergence_iteration_simd<T, const LANES: usize>(
    re: Simd<T, LANES>,
    im: Simd<T, LANES>,
    max_iterations: u32,
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
    let bound = Simd::splat(T::from_f32(4.0));
    let two = Simd::splat(T::from_f32(2.));
    let one = Simd::splat(1u32);
    let zero = Simd::splat(0u32);

    let x0 = re;
    let y0 = im;

    let mut x = x0;
    let mut y = y0;

    let mut iterations = Simd::splat(0);

    for _ in 0..max_iterations {
        let x2 = x * x;
        let y2 = y * y;

        let new_y = x * y;
        let new_x = x2 - y2;

        let active = (x2 + y2).simd_le(bound);

        y = two * new_y + y0;
        x = new_x + x0;

        if !active.any() {
            break;
        }

        iterations += active.cast::<i32>().select(one, zero);
    }

    iterations
}
