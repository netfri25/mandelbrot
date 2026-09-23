use crate::high_precision::HighPrecision;
use crate::producer::Producer;
use crate::types::{Dimensions, Pos, Size};

pub struct Scale<P> {
    producer: P,
    scale: f32,
}

impl<P> Scale<P> {
    pub fn new(producer: P, scale: f32) -> Self {
        Self { producer, scale }
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn scale_mut(&mut self) -> &mut f32 {
        &mut self.scale
    }
}

impl<P> Producer for Scale<P>
where
    P: Producer,
{
    fn produce(&mut self, start: Pos, size: Size, dims: Dimensions) -> Vec<f32> {
        let scaled_w = (dims.w as f32 / self.scale).round();
        let scaled_h = (dims.h as f32 / self.scale).round();

        // FIXME:
        //  this doesn't behave as expected, and I should figure out by how much it should offset
        //  the start position.

        let scaled_dims = Dimensions {
            w: scaled_w as u64,
            h: scaled_h as u64,
        };

        let half_scale = HighPrecision::from(self.scale / 2.0);
        let start = Pos {
            x: start.x + size.w / half_scale,
            y: start.y + size.h / half_scale,
        };

        let dims = scaled_dims;
        self.producer.produce(start, size, dims)
    }
}

impl<P> std::ops::DerefMut for Scale<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.producer
    }
}

impl<P> std::ops::Deref for Scale<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.producer
    }
}
