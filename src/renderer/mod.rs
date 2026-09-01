use crate::producer::Producer;

pub mod macroquad;

pub trait Renderer<P>
where
    P: Producer + ?Sized,
{
    fn render(&mut self, producer: &mut P);
}
