use num_traits::Float;

use crate::types::{Dimensions, Pos, Size};

pub mod naive;

pub trait Producer<T: Float> {
    // TODO: add doc comment that explains this method
    fn produce(&mut self, start: Pos<T>, size: Size<T>, dims: Dimensions) -> Vec<f32>;
}
