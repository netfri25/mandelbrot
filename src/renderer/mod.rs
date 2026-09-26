pub mod macroquad;
pub mod texture_renderer;

// the "starting point" of the program.
// renderer should render a single frame to it's output when this method is called.
pub trait Renderer {
    fn render(&mut self);
}
