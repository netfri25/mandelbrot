use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, Neg, SubAssign};
use std::time::{Duration, Instant};

use macroquad::prelude::*;

use crate::complex::Complex;
use crate::exp2::Exp2;
use crate::from_f32::FromF32;
use crate::producer::Producer;
use crate::types::{Dimensions, Pos, Size};

use super::Renderer;

pub struct MacroquadRenderer<T> {
    zoom: f32,
    offset: Complex<T>,
    resolution: f32,
    frame: Option<Texture2D>,
    last_produce_duration: Duration,
}

impl<T> MacroquadRenderer<T> {
    pub fn new(zoom: f32, offset: Complex<T>, resolution: f32) -> Self {
        Self {
            zoom,
            offset,
            resolution,
            frame: None,
            last_produce_duration: Duration::default(),
        }
    }

    fn update_frame(&mut self, dims: Dimensions, producer: &mut dyn Producer<T>)
    where
        T: FromF32 + Exp2 + Clone,
        T: Add<T, Output = T>,
        T: Mul<T, Output = T>,
    {
        let (ratio_w, ratio_h) = if dims.w > dims.h {
            (1., dims.h as f32 / dims.w as f32)
        } else {
            (dims.w as f32 / dims.h as f32, 1.)
        };

        let zoom_w = T::from_f32(self.zoom).exp2() * T::from_f32(ratio_w);
        let zoom_h = T::from_f32(self.zoom).exp2() * T::from_f32(ratio_h);

        let size = Size {
            w: zoom_w.clone(),
            h: zoom_h.clone(),
        };

        let base_offset_x: T = T::from_f32(-0.5) * zoom_w;
        let base_offset_y: T = T::from_f32(-0.5) * zoom_h;
        let top_left = Pos {
            x: base_offset_x + self.offset.re.clone(),
            y: base_offset_y + self.offset.im.clone(),
        };

        let produce_start = Instant::now();
        let values = producer.produce(top_left, size, dims);
        self.last_produce_duration = produce_start.elapsed();

        let minimum = values.iter().fold(0.9, |a, b| b.min(a));

        let mut image = Image::gen_image_color(dims.w as u16, dims.h as u16, Color::default());

        let colors: Vec<_> = values
            .into_iter()
            .map(|pixel| select_color(pixel, minimum))
            .collect();

        image.update(&colors);
        self.frame = Some(Texture2D::from_image(&image));
    }
}

fn fast_key<T>(target: &mut T, keycode: KeyCode, delta: T, multiplier: f32, dt: f32) -> bool
where
    T: FromF32,
    T: Mul<T, Output = T>,
    T: AddAssign<T>,
{
    match () {
        _ if is_key_down(keycode) && is_key_down(KeyCode::LeftControl) => {
            *target += T::from_f32(dt * multiplier) * delta;
        }

        _ if is_key_down(keycode) && is_key_down(KeyCode::LeftShift) => {
            *target += T::from_f32(dt) * delta;
        }

        _ if is_key_pressed(keycode) => {
            *target += delta;
        }

        _ => return false,
    }

    true
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

        let offset_multiplier = 1.5;
        let zoom_multiplier = 5.;

        let mut update = false;

        // I hate this, but it works. will be very hard to extend
        let mut refs = [&mut self.offset.im, &mut self.offset.re];
        let keycodes = [KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D];

        for (i, keycode) in keycodes.into_iter().enumerate() {
            let r = &mut refs[i / 2];

            let delta = if i % 2 == 0 {
                -offset_delta.clone()
            } else {
                offset_delta.clone()
            };

            update |= fast_key(*r, keycode, delta, offset_multiplier, dt);
        }

        let keycodes = [KeyCode::Equal, KeyCode::Minus];

        for (i, keycode) in keycodes.into_iter().enumerate() {
            let delta = if i % 2 == 0 { -zoom_delta } else { zoom_delta };
            update |= fast_key(&mut self.zoom, keycode, delta, zoom_multiplier, dt);
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

        if update || self.frame.is_none() {
            self.update_frame(dims, producer)
        }

        let frame = self
            .frame
            .as_ref()
            .expect("frame should be initialized here");

        draw_texture_ex(
            frame,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    dims.w as f32 / self.resolution,
                    dims.h as f32 / self.resolution,
                )),
                ..Default::default()
            },
        );

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

        draw_text(
            format!("zoom:     {:?}", self.zoom),
            0.,
            3. * line_delta * size,
            size,
            WHITE,
        );

        draw_text(
            format!("zoom exp: {:?}", T::from_f32(self.zoom).exp2()),
            0.,
            4. * line_delta * size,
            size,
            WHITE,
        );

        draw_text(
            format!("{:>9.02?}", self.last_produce_duration),
            0.,
            5. * line_delta * size,
            size,
            WHITE,
        );

        draw_text(
            format!(
                "{:>7.02}UPS",
                self.last_produce_duration.as_secs_f32().recip()
            ),
            0.,
            6. * line_delta * size,
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
