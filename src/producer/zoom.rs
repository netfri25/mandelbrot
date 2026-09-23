use crate::from_f64::FromF64;
use crate::high_precision::HighPrecision;
use crate::producer::Producer;
use crate::types::{Dimensions, Pos, Size};

pub struct Zoom<P> {
    producer: P,
    zoom: HighPrecision,
}

impl<P> Zoom<P> {
    pub fn new(producer: P, zoom: HighPrecision) -> Self {
        Self { producer, zoom }
    }

    pub fn zoom(&self) -> HighPrecision {
        self.zoom
    }

    pub fn get_zoom_size(&self, size: &Size) -> Size {
        Size {
            w: self.zoom() * size.w,
            h: self.zoom() * size.h,
        }
    }

    pub fn zoom_mut(&mut self) -> &mut HighPrecision {
        &mut self.zoom
    }
}

impl<P> Producer for Zoom<P>
where
    P: Producer,
{
    fn produce(&mut self, start: Pos, size: Size, dims: Dimensions) -> Vec<f32> {
        let zoom = self.get_zoom_size(&size);

        let start = Pos {
            x: start.x - (zoom.w - size.w) / 2.0.into(),
            y: start.y - (zoom.h - size.h) / 2.0.into(),
        };

        let size = zoom;
        self.producer.produce(start, size, dims)
    }
}

impl<P> std::ops::DerefMut for Zoom<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.producer
    }
}

impl<P> std::ops::Deref for Zoom<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.producer
    }
}
