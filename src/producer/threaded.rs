use std::ops::{Add, Div, Mul};

use rayon::prelude::*;

use crate::from_f32::FromF32;
use crate::types::{Dimensions, Pos, Size};

use super::Producer;

pub struct ThreadedProducer<'a, T> {
    threads: usize,
    make_producer: &'a mut dyn FnMut() -> Box<dyn Producer<T> + Send>,
}

impl<'a, T> ThreadedProducer<'a, T> {
    pub fn new(
        threads: usize,
        make_producer: &'a mut dyn FnMut() -> Box<dyn Producer<T> + Send>,
    ) -> Self {
        Self {
            threads,
            make_producer,
        }
    }
}

impl<'a, T> Producer<T> for ThreadedProducer<'a, T>
where
    T: FromF32 + Send + Sync + Clone,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Div<T, Output = T>,
{
    fn produce(&mut self, start: Pos<T>, size: Size<T>, dims: Dimensions) -> Vec<f32> {
        let section_h = dims.h / self.threads;
        let section_h_rem = dims.h % self.threads;

        let producers: Vec<_> = (0..self.threads).map(|_| (self.make_producer)()).collect();

        producers
            .into_par_iter()
            .enumerate()
            .flat_map(move |(section_row, mut producer)| {
                let section_offset_x = 0;
                let section_offset_y = section_h * section_row + section_row.min(section_h_rem);

                let section_w = dims.w;
                let section_h = section_h + (section_row < section_h_rem) as usize;

                let start = Pos {
                    x: T::from_f32(section_offset_x as f32) / T::from_f32(dims.w as f32) * size.w.clone()
                        + start.x.clone(),
                    y: T::from_f32(section_offset_y as f32) / T::from_f32(dims.h as f32) * size.h.clone()
                        + start.y.clone(),
                };

                let size = Size {
                    w: T::from_f32(section_w as f32) / T::from_f32(dims.w as f32) * size.w.clone(),
                    h: T::from_f32(section_h as f32) / T::from_f32(dims.h as f32) * size.h.clone(),
                };

                let dims = Dimensions {
                    w: section_w,
                    h: section_h,
                };

                producer.produce(start, size, dims)
            })
            .collect()
    }
}
