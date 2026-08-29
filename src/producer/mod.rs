use crate::types::{Dimensions, Pos, Size};

pub mod naive;
pub mod simd;
pub mod threaded;

pub trait Producer<T> {
    // TODO: add doc comment that explains this method
    fn produce(&mut self, start: Pos<T>, size: Size<T>, dims: Dimensions) -> Vec<f32>;
}

impl<P, T> Producer<T> for Box<P>
where
    P: Producer<T> + ?Sized,
{
    fn produce(&mut self, start: Pos<T>, size: Size<T>, dims: Dimensions) -> Vec<f32> {
        self.as_mut().produce(start, size, dims)
    }
}
