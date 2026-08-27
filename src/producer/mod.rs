use crate::types::{Dimensions, Pos, Size};

pub mod naive;
pub mod threaded;
pub mod timed;

pub trait Producer<T> {
    // TODO: add doc comment that explains this method
    fn produce(&mut self, start: Pos<T>, size: Size<T>, dims: Dimensions) -> Vec<f32>;
}
