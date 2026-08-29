use crate::producer::Producer;

pub mod macroquad;

pub trait Renderer<P, T>
where
    P: Producer<T> + ?Sized,
{
    fn render(&mut self, producer: &mut P);
}
