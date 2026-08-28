use std::time::Instant;

use crate::types::{Dimensions, Pos, Size};

use super::Producer;

pub struct TimedProducer<'a, T>(pub &'a mut dyn Producer<T>);

impl<'a, T> Producer<T> for TimedProducer<'a, T> {
    fn produce(&mut self, start: Pos<T>, size: Size<T>, dims: Dimensions) -> Vec<f32> {
        let time_start = Instant::now();
        let result = self.0.produce(start, size, dims);
        let elapsed = time_start.elapsed();

        eprintln!(
            "produce took {:>8.02?} ({:>7.02} max UPS)",
            elapsed,
            elapsed.as_secs_f64().recip()
        );

        result
    }
}
