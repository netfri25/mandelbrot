use std::cmp::Ordering;

use macroquad::prelude::*;
use num_traits::{Float, FromPrimitive};

use crate::complex::Complex;
use crate::producer::Producer;
use crate::types::{Dimensions, Pos, Size};

use super::Renderer;

#[derive(Default)]
pub struct MacroquadRenderer<T: Float> {
    zoom: T,
    offset: Complex<T>,
    resolution: f32,
}

impl<T: Float> MacroquadRenderer<T> {
    pub fn new(zoom: T, offset: Complex<T>, resolution: f32) -> Self {
        Self {
            zoom,
            offset,
            resolution,
        }
    }
}

fn fast_key(keycode: KeyCode) -> bool {
    is_key_pressed(keycode) || is_key_down(keycode) && is_key_down(KeyCode::LeftShift)
}

impl<T: Float + FromPrimitive> MacroquadRenderer<T> {
    pub fn handle_input(&mut self) {
        let dt = get_frame_time();
        let two = T::from_f32(2.).unwrap();
        let zoom_delta = two * T::from_f32(dt).unwrap();
        let offset_delta = zoom_delta / two * self.zoom.exp();

        if fast_key(KeyCode::W) {
            self.offset.im = self.offset.im - offset_delta;
        }

        if fast_key(KeyCode::S) {
            self.offset.im = self.offset.im + offset_delta;
        }

        if fast_key(KeyCode::A) {
            self.offset.re = self.offset.re - offset_delta;
        }

        if fast_key(KeyCode::D) {
            self.offset.re = self.offset.re + offset_delta;
        }

        if fast_key(KeyCode::Minus) {
            self.zoom = self.zoom + zoom_delta;
        }

        if fast_key(KeyCode::Equal) {
            self.zoom = self.zoom - zoom_delta;
        }
    }
}

impl<T: Float + FromPrimitive> Renderer<T> for MacroquadRenderer<T> {
    // TODO: make this work in non-square aspect ratios
    fn render(&mut self, producer: &mut dyn Producer<T>) {
        self.handle_input();

        let zoom = self.zoom.exp();

        let dims = Dimensions {
            w: (self.resolution * screen_width()) as usize,
            h: (self.resolution * screen_height()) as usize,
        };

        let size = Size { w: zoom, h: zoom };

        let base_offset = T::from_f32(-0.5).unwrap() * zoom;
        let top_left = Pos {
            x: base_offset + self.offset.re,
            y: base_offset + self.offset.im,
        };

        let pixels = producer.produce(top_left, size, dims);
        let pixel_size = self.resolution.recip();

        let minimum = pixels
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or_default()
            .min(0.9);

        for (y, pixel_row) in pixels.chunks_exact(dims.w).enumerate() {
            for (x, pixel) in pixel_row.iter().enumerate() {
                let x = pixel_size * x as f32;
                let y = pixel_size * y as f32;
                let color = select_color(*pixel, minimum);
                draw_rectangle(x, y, pixel_size, pixel_size, color);
            }
        }
    }
}

fn select_color(value: f32, minimum: f32) -> Color {
    let l = BLACK;
    let h = WHITE;
    if value >= 1.0 {
        return h;
    }

    let t = 1.0 - (-(value - minimum) / 0.1).exp();
    Color::new(
        l.r * (1.0 - t) + h.r * t,
        l.g * (1.0 - t) + h.g * t,
        l.b * (1.0 - t) + h.b * t,
        1.,
    )
}
