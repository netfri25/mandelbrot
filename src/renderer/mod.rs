pub mod macroquad;

pub trait Renderer<P: ?Sized> {
    fn render(&mut self, producer: &mut P);
}
