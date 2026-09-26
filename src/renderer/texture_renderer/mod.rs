use std::ops::{Deref, DerefMut};

use macroquad::prelude::Texture2D;

use crate::types::{Dimensions, View};

pub mod cpu;

pub trait TextureRenderer {
    fn render_texture(&mut self, view: &View, dims: Dimensions) -> Texture2D;
}

impl<R> TextureRenderer for R
where
    R: DerefMut,
    <R as Deref>::Target: TextureRenderer,
{
    fn render_texture(&mut self, view: &View, dims: Dimensions) -> Texture2D {
        self.deref_mut().render_texture(view, dims)
    }
}
