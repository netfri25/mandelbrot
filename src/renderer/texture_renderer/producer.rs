use macroquad::prelude::{Color, Image, Texture2D};

use crate::producer::Producer;
use crate::renderer::texture_renderer::TextureRenderer;
use crate::types::{Dimensions, View};

pub struct ProducerTextureRenderer<P, F> {
    producer: P,
    colorizer: F,
}

impl<P, F> ProducerTextureRenderer<P, F> {
    pub fn new(producer: P, colorizer: F) -> Self {
        Self {
            producer,
            colorizer,
        }
    }
}

impl<P, F> TextureRenderer for ProducerTextureRenderer<P, F>
where
    P: Producer,
    F: FnMut(f32) -> Color,
{
    fn render_texture(&mut self, view: &View, dims: Dimensions) -> Texture2D {
        let values = self.producer.produce(view, dims);
        let mut image = Image::gen_image_color(dims.w as u16, dims.h as u16, Color::default());

        let pixels = image.get_image_data_mut();
        assert_eq!(values.len(), pixels.len());

        for (value, pixel) in values.into_iter().zip(pixels) {
            let color = (self.colorizer)(value);
            pixel[0] = (color.r * 255.) as u8;
            pixel[1] = (color.g * 255.) as u8;
            pixel[2] = (color.b * 255.) as u8;
            pixel[3] = (color.a * 255.) as u8;
        }

        Texture2D::from_image(&image)
    }
}
