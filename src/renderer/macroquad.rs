use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, Neg, SubAssign};

use macroquad::prelude::*;

use crate::complex::Complex;
use crate::exp2::Exp2;
use crate::from_f32::FromF32;
use crate::producer::Producer;
use crate::types::{Dimensions, Pos, Size};

use super::Renderer;

#[derive(Default)]
pub struct MacroquadRenderer<T> {
    zoom: f32,
    offset: Complex<T>,
    resolution: f32,
    frame: Vec<f32>,
}

impl<T> MacroquadRenderer<T> {
    pub fn new(zoom: f32, offset: Complex<T>, resolution: f32) -> Self {
        Self {
            zoom,
            offset,
            resolution,
            frame: Default::default(),
        }
    }
}

fn fast_key<T>(target: &mut T, keycode: KeyCode, delta: T, dt: f32) -> bool
where
    T: FromF32,
    T: Mul<T, Output = T>,
    T: AddAssign<T>,
{
    if is_key_down(keycode) && is_key_down(KeyCode::LeftShift) {
        *target += T::from_f32(dt) * delta;
        true
    } else if is_key_pressed(keycode) {
        *target += delta;
        true
    } else {
        false
    }
}

impl<T> MacroquadRenderer<T>
where
    T: FromF32 + Clone + Exp2 + Debug,
    T: Mul<T, Output = T>,
    T: Neg<Output = T>,
    T: AddAssign<T>,
    T: SubAssign<T>,
{
    pub fn handle_input(&mut self) -> bool {
        let dt = get_frame_time() * 20.;
        let zoom_delta = 0.05;
        let offset_delta = T::from_f32(zoom_delta) * T::from_f32(self.zoom).exp2();

        let mut update = false;
        update |= fast_key(&mut self.offset.im, KeyCode::W, -offset_delta.clone(), dt);
        update |= fast_key(&mut self.offset.im, KeyCode::S, offset_delta.clone(), dt);
        update |= fast_key(&mut self.offset.re, KeyCode::A, -offset_delta.clone(), dt);
        update |= fast_key(&mut self.offset.re, KeyCode::D, offset_delta.clone(), dt);

        let prev_zoom = self.zoom;
        update |= fast_key(&mut self.zoom, KeyCode::Equal, -zoom_delta, dt);
        update |= fast_key(&mut self.zoom, KeyCode::Minus, zoom_delta, dt);

        if prev_zoom != self.zoom {
            eprintln!("new zoom: {:?}", self.zoom);
            eprintln!("zoom exp: {:?}", T::from_f32(self.zoom).exp2());
        }

        update
    }
}

impl<T> Renderer<T> for MacroquadRenderer<T>
where
    T: FromF32 + Clone + Exp2 + Debug,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Neg<Output = T>,
    T: AddAssign<T>,
    T: SubAssign<T>,
{
    // TODO: make this work in non-square aspect ratios
    fn render(&mut self, producer: &mut dyn Producer<T>) {
        let update = self.handle_input();

        let dims = Dimensions {
            w: (self.resolution * screen_width()) as usize,
            h: (self.resolution * screen_height()) as usize,
        };

        if self.frame.is_empty() || update {
            let zoom: T = T::from_f32(self.zoom).exp2();

            let size = Size {
                w: zoom.clone(),
                h: zoom.clone(),
            };

            let base_offset: T = T::from_f32(-0.5) * zoom;
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

        let size = screen_height() as f32 / 25.;
        let line_delta = 0.7;
        draw_text(
            format!("x: {:?}", self.offset.re),
            0.,
            1. * line_delta * size,
            size,
            WHITE,
        );
        draw_text(
            format!("y: {:?}", self.offset.im),
            0.,
            2. * line_delta * size,
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
