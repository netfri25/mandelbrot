use crate::producer::Producer;

pub mod macroquad;

pub trait Renderer<P>
where
    P: Producer + ?Sized,
{
    fn render(&mut self, producer: &mut P);
}

impl<R, P> Renderer<P> for Box<R>
where
    R: Renderer<P> + ?Sized,
    P: Producer + ?Sized,
{
    fn render(&mut self, producer: &mut P) {
        self.as_mut().render(producer)
    }
}
