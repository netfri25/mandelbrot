use num_traits::Float;

use crate::types::{Pos, Size};

pub mod naive;

pub trait Producer<T: Float> {
    // TODO: add doc comment that explains this method
    fn produce(&mut self, pos: Pos<T>, size: Size<T>, step_x: T, step_y: T) -> Vec<f32>;
}
