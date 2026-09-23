use std::ops::{Deref, DerefMut};

use crate::producer::Producer;
use crate::types::{Dimensions, Pos, Size};

pub struct Offset<P> {
    producer: P,
    offset: Pos,
}

impl<P> Offset<P> {
    pub fn new(producer: P, offset: Pos) -> Self {
        Self { producer, offset }
    }

    pub fn offset(&self) -> &Pos {
        &self.offset
    }

    pub fn offset_mut(&mut self) -> &mut Pos {
        &mut self.offset
    }
}

impl<P> Producer for Offset<P>
where
    P: Producer
{
    fn produce(&mut self, start: Pos, size: Size, dims: Dimensions) -> Vec<f32> {
        let start = Pos {
            x: start.x + self.offset.x,
            y: start.y + self.offset.y,
        };

        self.producer.produce(start, size, dims)
    }
}

impl<P> DerefMut for Offset<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.producer
    }
}

impl<P> Deref for Offset<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.producer
    }
}
