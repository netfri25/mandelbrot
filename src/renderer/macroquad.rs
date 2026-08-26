use macroquad::prelude::*;
use num_traits::{Float, FromPrimitive};

use crate::producer::Producer;
use crate::types::{Pos, Size};

use super::Renderer;

#[derive(Default)]
pub struct MacroquadRenderer {
    scale: f64,
}

impl MacroquadRenderer {
    pub fn new(scale: f64) -> Self {
        Self { scale }
    }
}

impl<T: Float + FromPrimitive> Renderer<T> for MacroquadRenderer {
    // TODO: make this work in non-square aspect ratios
    fn render(&mut self, producer: &mut dyn Producer<T>) {
        let w = screen_width() as f64;
        let h = screen_height() as f64;

        let size = Size {
            w: T::from_f64(self.scale).unwrap(),
            h: T::from_f64(self.scale).unwrap(),
        };

        let top_left = Pos {
            x: T::from_f64(-0.5 * self.scale).unwrap(),
            y: T::from_f64(-0.5 * self.scale).unwrap(),
        };

        let step_x = T::from_f64(self.scale / w).unwrap();
        let step_y = T::from_f64(self.scale / h).unwrap();

        let pixels = producer.produce(top_left, size, step_x, step_y);

        for (y, pixel_row) in pixels.chunks_exact(w as usize).enumerate() {
            for (x, pixel) in pixel_row.iter().enumerate() {
                let x = x as f32;
                let y = y as f32;
                let color = select_color(*pixel);
                draw_rectangle(x, y, 1., 1., color);
            }
        }
    }
}

fn select_color(value: f32) -> Color {
    let l = BLACK;
    let h = WHITE;
    if value >= 1.0 {
        return h;
    }

    let t = 1.0 - (-value / 0.1).exp();
    Color::new(
        l.r * (1.0 - t) + h.r * t,
        l.g * (1.0 - t) + h.g * t,
        l.b * (1.0 - t) + h.b * t,
        1.,
    )
}
