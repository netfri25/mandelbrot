use std::num::NonZeroU32;
use std::ops::{Deref, DerefMut};

use crate::types::{Dimensions, View};

pub mod naive;
#[cfg(not(feature = "no_simd"))]
pub mod simd;
pub mod threaded;

pub trait Producer {
    /// produce values to be rendered as the mandelbrot set
    ///
    /// for each "pixel", returns a value that represents the number of iterations for divergence,
    /// or None if it didn't diverge.
    ///
    /// **Parameters**
    ///
    /// * `view`: section in the mandelbrot set to explore
    /// * `dims`: the amount of "pixels" to generate
    ///
    fn produce(&mut self, view: &View, dims: Dimensions) -> Vec<Option<NonZeroU32>>;
}

impl<P> Producer for P
where
    P: DerefMut,
    <P as Deref>::Target: Producer,
{
    fn produce(&mut self, view: &View, dims: Dimensions) -> Vec<Option<NonZeroU32>> {
        self.deref_mut().produce(view, dims)
    }
}
