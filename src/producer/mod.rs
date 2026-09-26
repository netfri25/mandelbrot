use std::ops::{Deref, DerefMut};

use crate::types::{Dimensions, View};

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
    /// * `view`: section in the mandelbrot set to explore
    /// * `dims`: the amount of "pixels" to generate
    ///
    fn produce(&mut self, view: &View, dims: Dimensions) -> Vec<f32>;
}

impl<P> Producer for P
where
    P: DerefMut,
    <P as Deref>::Target: Producer,
{
    fn produce(&mut self, view: &View, dims: Dimensions) -> Vec<f32> {
        self.deref_mut().produce(view, dims)
    }
}
