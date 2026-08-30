use std::ops::{AddAssign, Mul};
use std::time::{Duration, Instant};

use macroquad::prelude::*;

use crate::exp2::Exp2;
use crate::from_f64::FromF64;
use crate::high_precision::HighPrecision;
use crate::producer::Producer;
use crate::types::{Dimensions, Pos, Size};

use super::Renderer;

pub struct MacroquadRenderer {
    zoom: f64,
    offset: Pos,
    resolution: f32,
    frame: Option<Texture2D>,
    last_produce_duration: Duration,
    should_show_info: bool,
}

impl MacroquadRenderer {
    pub fn new(zoom: f64, offset: Pos, resolution: f32) -> Self {
        Self {
            zoom,
            offset,
            resolution,
            frame: None,
            last_produce_duration: Duration::default(),
            should_show_info: false,
        }
    }

    fn update_frame(&mut self, dims: Dimensions, producer: &mut (impl Producer + ?Sized)) {
        let (ratio_w, ratio_h) = if dims.w > dims.h {
            (1., dims.h as f64 / dims.w as f64)
        } else {
            (dims.w as f64 / dims.h as f64, 1.)
        };

        let zoom_exp = self.zoom.exp2();
        let zoom_w = zoom_exp * ratio_w;
        let zoom_h = zoom_exp * ratio_h;

        let size = Size {
            w: HighPrecision::from_f64(zoom_w),
            h: HighPrecision::from_f64(zoom_h),
        };

        let base_offset_x = HighPrecision::from_f64(-zoom_w / 2.);
        let base_offset_y = HighPrecision::from_f64(-zoom_h / 2.);
        let top_left = Pos {
            x: base_offset_x + self.offset.x,
            y: base_offset_y + self.offset.y,
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
    T: FromF64,
    T: Mul<T, Output = T>,
    T: AddAssign<T>,
{
    match () {
        _ if is_key_down(keycode) && is_key_down(KeyCode::LeftControl) => {
            *target += T::from_f64((dt * multiplier) as f64) * delta;
        }

        _ if is_key_down(keycode) && is_key_down(KeyCode::LeftShift) => {
            *target += T::from_f64(dt as f64) * delta;
        }

        _ if is_key_pressed(keycode) => {
            *target += delta;
        }

        _ => return false,
    }

    true
}

impl MacroquadRenderer {
    // returns `true` if the frame should be updated
    fn handle_input(&mut self) -> bool {
        let dt = get_frame_time() * 20.;
        let zoom_delta = 0.05;
        let offset_delta =
            HighPrecision::from_f64(zoom_delta) * HighPrecision::from_f64(self.zoom).exp2();

        let offset_multiplier = 1.5;
        let zoom_multiplier = 5.;

        let mut update = false;

        // I hate this, but it works. will be very hard to extend
        let mut refs = [&mut self.offset.y, &mut self.offset.x];
        let keycodes = [KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D];

        for (i, keycode) in keycodes.into_iter().enumerate() {
            let r = &mut refs[i / 2];

            let delta = if i % 2 == 0 {
                -offset_delta
            } else {
                offset_delta
            };

            update |= fast_key(*r, keycode, delta, offset_multiplier, dt);
        }

        let keycodes = [KeyCode::Equal, KeyCode::Minus];

        for (i, keycode) in keycodes.into_iter().enumerate() {
            let delta = if i % 2 == 0 { -zoom_delta } else { zoom_delta };
            update |= fast_key(&mut self.zoom, keycode, delta, zoom_multiplier, dt);
        }

        if is_key_pressed(KeyCode::I) {
            self.should_show_info = !self.should_show_info;
        }

        update
    }

    fn show_info(&self) {
        let size = screen_height() as f32 / 25.;
        let line_delta = 0.7;
        let text_color = BROWN;

        draw_text(
            format!("x: {:?}", self.offset.x),
            0.,
            1. * line_delta * size,
            size,
            text_color,
        );
        draw_text(
            format!("y: {:?}", self.offset.y),
            0.,
            2. * line_delta * size,
            size,
            text_color,
        );

        draw_text(
            format!("zoom:     {:?}", self.zoom),
            0.,
            3. * line_delta * size,
            size,
            text_color,
        );

        draw_text(
            format!("zoom exp: {:?}", self.zoom.exp2()),
            0.,
            4. * line_delta * size,
            size,
            text_color,
        );

        draw_text(
            format!("{:>9.02?}", self.last_produce_duration),
            0.,
            5. * line_delta * size,
            size,
            text_color,
        );

        draw_text(
            format!(
                "{:>7.02}UPS",
                self.last_produce_duration.as_secs_f32().recip()
            ),
            0.,
            6. * line_delta * size,
            size,
            text_color,
        );
    }
}

impl<P> Renderer<P> for MacroquadRenderer
where
    P: Producer + ?Sized,
{
    fn render(&mut self, producer: &mut P) {
        let update = self.handle_input();

        let dims = Dimensions {
            w: (self.resolution * screen_width()) as u64,
            h: (self.resolution * screen_height()) as u64,
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

        if self.should_show_info {
            self.show_info()
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
