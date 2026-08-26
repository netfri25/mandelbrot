use crate::producer::Producer;

pub mod macroquad;

pub trait Renderer<T> {
    fn render(&mut self, producer: &mut dyn Producer<T>);
}
