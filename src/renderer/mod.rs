use crate::producer::Producer;

pub mod macroquad;

pub trait Renderer<P, T>
where
    P: Producer<T> + ?Sized,
{
    fn render(&mut self, producer: &mut P);
}

impl<R, P, T> Renderer<P, T> for Box<R>
where
    R: Renderer<P, T> + ?Sized,
    P: Producer<T> + ?Sized,
{
    fn render(&mut self, producer: &mut P) {
        self.as_mut().render(producer)
    }
}
