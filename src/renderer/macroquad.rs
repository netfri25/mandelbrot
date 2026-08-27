use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use macroquad::prelude::*;

use crate::complex::Complex;
use crate::producer::Producer;
use crate::types::{Dimensions, Pos, Size};

use super::Renderer;

#[derive(Default)]
pub struct MacroquadRenderer<T> {
    zoom: f64,
    offset: Complex<T>,
    resolution: f32,
}

impl<T> MacroquadRenderer<T> {
    pub fn new(zoom: f64, offset: Complex<T>, resolution: f32) -> Self {
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

impl<T> MacroquadRenderer<T>
where
    T: From<f64> + Clone,
    T: AddAssign<T>,
    T: SubAssign<T>,
{
    pub fn handle_input(&mut self) {
        let dt = get_frame_time();
        let zoom_delta = 2.0 * dt as f64;
        let offset_delta = T::from(zoom_delta / 2.0 * self.zoom.exp());

        if fast_key(KeyCode::W) {
            self.offset.im -= offset_delta.clone();
        }

        if fast_key(KeyCode::S) {
            self.offset.im += offset_delta.clone();
        }

        if fast_key(KeyCode::A) {
            self.offset.re -= offset_delta.clone();
        }

        if fast_key(KeyCode::D) {
            self.offset.re += offset_delta.clone();
        }

        if fast_key(KeyCode::Minus) {
            self.zoom += zoom_delta;
            eprintln!("new zoom: {}", self.zoom);
            eprintln!("zoom exp: {}", self.zoom.exp());
        }

        if fast_key(KeyCode::Equal) {
            self.zoom -= zoom_delta;
            eprintln!("new zoom: {}", self.zoom);
            eprintln!("zoom exp: {}", self.zoom.exp());
        }
    }
}

// FIX: fix precision error on zoom
impl<T> Renderer<T> for MacroquadRenderer<T>
where
    T: From<f64> + Clone,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
    T: AddAssign<T>,
    T: SubAssign<T>,
{
    // TODO: make this work in non-square aspect ratios
    fn render(&mut self, producer: &mut dyn Producer<T>) {
        self.handle_input();

        let zoom: T = self.zoom.exp().into();

        let dims = Dimensions {
            w: (self.resolution * screen_width()) as usize,
            h: (self.resolution * screen_height()) as usize,
        };

        let size = Size {
            w: zoom.clone(),
            h: zoom.clone(),
        };

        let base_offset: T = T::from(-0.5) * zoom;
        let top_left = Pos {
            x: base_offset.clone() + self.offset.re.clone(),
            y: base_offset.clone() + self.offset.im.clone(),
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
