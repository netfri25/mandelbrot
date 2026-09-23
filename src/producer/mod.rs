use crate::high_precision::HighPrecision;
use crate::types::{Dimensions, Pos, Size};

pub mod naive;
pub mod offset;
pub mod scale;
#[cfg(not(feature = "no_simd"))]
pub mod simd;
pub mod threaded;
pub mod zoom;

pub use offset::Offset;
pub use scale::Scale;
pub use zoom::Zoom;

pub trait Producer {
    /// produce values to be rendered as the mandelbrot set
    ///
    /// for each "pixel", returns a value from 0 to 1 that represents if it's outside of the
    /// mandelbrot set (0) or inside (1).
    ///
    /// **Parameters**
    ///
    /// * `start`: the starting position to draw from. the renderer decides which point is it, but
    ///   usually it will be the top left.
    /// * `size`: the width and height of the area in the mandelbrot set to draw
    /// * `dims`: the amount of "pixels" to generate
    ///
    fn produce(&mut self, start: Pos, size: Size, dims: Dimensions) -> Vec<f32>;
}

pub trait ProducerExt: Sized {
    fn zoom(self, zoom: HighPrecision) -> Zoom<Self> {
        Zoom::new(self, zoom)
    }

    fn scale(self, scale: f32) -> Scale<Self> {
        Scale::new(self, scale)
    }

    fn offset(self, offset: Pos) -> Offset<Self> {
        Offset::new(self, offset)
    }
}

impl<P> ProducerExt for P where P: Producer {}

impl<P> Producer for &mut P
where
    P: Producer + ?Sized,
{
    fn produce(&mut self, start: Pos, size: Size, dims: Dimensions) -> Vec<f32> {
        (**self).produce(start, size, dims)
    }
}

impl<P> Producer for Box<P>
where
    P: Producer + ?Sized
{
    fn produce(&mut self, start: Pos, size: Size, dims: Dimensions) -> Vec<f32> {
        (**self).produce(start, size, dims)
    }
}
