use num_traits::Float;

use crate::producer::Producer;

pub mod macroquad;

pub trait Renderer<T: Float> {
    fn render(&mut self, producer: &mut dyn Producer<T>);
}
