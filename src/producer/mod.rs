use crate::types::{Dimensions, Pos, Size};

pub mod naive;
#[cfg(not(feature = "no_simd"))]
pub mod simd;
pub mod threaded;

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
