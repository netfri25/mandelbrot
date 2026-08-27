use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, SubAssign};

use macroquad::prelude::*;

use crate::complex::Complex;
use crate::exp::Exp;
use crate::producer::Producer;
use crate::types::{Dimensions, Pos, Size};

use super::Renderer;

#[derive(Default)]
pub struct MacroquadRenderer<T, Z> {
    zoom: Z,
    offset: Complex<T>,
    resolution: f32,
    frame: Vec<f32>,
}

impl<T, Z> MacroquadRenderer<T, Z> {
    pub fn new(zoom: Z, offset: Complex<T>, resolution: f32) -> Self {
        Self {
            zoom,
            offset,
            resolution,
            frame: Default::default(),
        }
    }
}

fn fast_key(keycode: KeyCode) -> bool {
    is_key_pressed(keycode) || is_key_down(keycode) && is_key_down(KeyCode::LeftShift)
}

impl<T, Z> MacroquadRenderer<T, Z>
where
    T: From<Z> + Clone,
    T: AddAssign<T>,
    T: SubAssign<T>,
    Z: From<f32> + Exp + Display + Clone,
    Z: Mul<Z, Output = Z>,
    Z: AddAssign<Z>,
    Z: SubAssign<Z>,
{
    pub fn handle_input(&mut self) -> bool {
        let dt = get_frame_time();
        let zoom_delta = <Z as From<_>>::from(dt.min(0.3));
        let offset_delta = T::from(zoom_delta.clone() * self.zoom.clone().exp());

        let mut update = false;

        if fast_key(KeyCode::W) {
            self.offset.im -= offset_delta.clone();
            update = true;
        }

        if fast_key(KeyCode::S) {
            self.offset.im += offset_delta.clone();
            update = true;
        }

        if fast_key(KeyCode::A) {
            self.offset.re -= offset_delta.clone();
            update = true;
        }

        if fast_key(KeyCode::D) {
            self.offset.re += offset_delta.clone();
            update = true;
        }

        if fast_key(KeyCode::Minus) {
            self.zoom += zoom_delta.clone();
            eprintln!("new zoom: {}", self.zoom);
            eprintln!("zoom exp: {}", self.zoom.clone().exp());
            update = true;
        }

        if fast_key(KeyCode::Equal) {
            self.zoom -= zoom_delta;
            eprintln!("new zoom: {}", self.zoom);
            eprintln!("zoom exp: {}", self.zoom.clone().exp());
            update = true;
        }

        update
    }
}

impl<T, Z> Renderer<T> for MacroquadRenderer<T, Z>
where
    T: From<f32> + From<Z> + Clone + Display,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
    T: AddAssign<T>,
    T: SubAssign<T>,
    Z: From<f32> + Exp + Display + Clone,
    Z: Mul<Z, Output = Z>,
    Z: AddAssign<Z>,
    Z: SubAssign<Z>,
{
    // TODO: make this work in non-square aspect ratios
    fn render(&mut self, producer: &mut dyn Producer<T>) {
        let update = self.handle_input();

        let dims = Dimensions {
            w: (self.resolution * screen_width()) as usize,
            h: (self.resolution * screen_height()) as usize,
        };

        if self.frame.is_empty() || update {
            let zoom: T = self.zoom.clone().exp().into();

            let size = Size {
                w: zoom.clone(),
                h: zoom.clone(),
            };

            let base_offset: T = T::from(-0.5) * zoom;
            let top_left = Pos {
                x: base_offset.clone() + self.offset.re.clone(),
                y: base_offset.clone() + self.offset.im.clone(),
            };

            self.frame = producer.produce(top_left, size, dims);
        }

        let pixel_size = self.resolution.recip();

        let minimum = self
            .frame
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or_default()
            .min(0.9);

        for (y, pixel_row) in self.frame.chunks_exact(dims.w).enumerate() {
            for (x, pixel) in pixel_row.iter().enumerate() {
                let x = pixel_size * x as f32;
                let y = pixel_size * y as f32;
                let color = select_color(*pixel, minimum);
                draw_rectangle(x, y, pixel_size, pixel_size, color);
            }
        }

        let size = screen_height() as f32 / 15.;
        draw_text(
            format!("x: {}", self.offset.re),
            0.,
            0.5 * size,
            size,
            WHITE,
        );
        draw_text(
            format!("y: {}", self.offset.im),
            0.,
            1.5 * size,
            size,
            WHITE,
        );
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
