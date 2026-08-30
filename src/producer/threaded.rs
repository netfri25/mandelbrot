use rayon::prelude::*;

use crate::types::{Dimensions, Pos, Size};

use super::Producer;

pub struct ThreadedProducer<F> {
    threads: usize,
    make_producer: F,
}

impl<F, P> ThreadedProducer<F>
where
    F: FnMut() -> P,
{
    pub fn new(threads: usize, make_producer: F) -> Self {
        Self {
            threads,
            make_producer,
        }
    }
}

impl<F, P> Producer for ThreadedProducer<F>
where
    F: FnMut() -> P,
    P: Producer + Send,
{
    fn produce(&mut self, start: Pos, size: Size, dims: Dimensions) -> Vec<f32> {
        let section_h = dims.h / self.threads as u64;
        let section_h_rem = dims.h % self.threads as u64;

        let producers: Vec<_> = (0..self.threads).map(|_| (self.make_producer)()).collect();

        producers
            .into_par_iter()
            .enumerate()
            .flat_map(move |(section_row, mut producer)| {
                let section_row = section_row as u64;
                let section_offset_x = 0;
                let section_offset_y = section_h * section_row + section_row.min(section_h_rem);

                let section_w = dims.w;
                let section_h = section_h + (section_row < section_h_rem) as u64;

                let section_offset_x = section_offset_x as f64;
                let section_offset_y = section_offset_y as f64;

                let start = Pos {
                    x: section_offset_x / dims.w as f64 * size.w + start.x,
                    y: section_offset_y / dims.h as f64 * size.h + start.y,
                };

                let size = Size {
                    w: section_w as f64 / dims.w as f64 * size.w,
                    h: section_h as f64 / dims.h as f64 * size.h,
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
