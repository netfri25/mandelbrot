use std::time::{Duration, Instant};

use macroquad::prelude::*;

use crate::from_f64::FromF64;
use crate::high_precision::HighPrecision;
use crate::producer::Producer;
use crate::types::{Dimensions, Pos, Size};

use super::Renderer;

pub struct MacroquadRenderer {
    zoom: HighPrecision,
    offset: Pos,
    resolution: f32,
    frame: Option<Texture2D>,
    last_produce_duration: Duration,
    should_show_info: bool,
    last_size: Size,
}

impl MacroquadRenderer {
    pub fn new(offset: Pos, resolution: f32) -> Self {
        Self {
            zoom: HighPrecision::from(3.5),
            offset,
            resolution,
            frame: None,
            last_produce_duration: Duration::default(),
            should_show_info: false,
            last_size: Size::default(),
        }
    }

    fn get_size(&mut self, dims: Dimensions) -> Size {
        let dims_w = HighPrecision::from_f64(dims.w as f64);
        let dims_h = HighPrecision::from_f64(dims.h as f64);
        let one = HighPrecision::from_f64(1.);
        let [ratio_w, ratio_h] = if dims_w > dims_h {
            [one, dims_h / dims_w]
        } else {
            [dims_w / dims_h, one]
        };

        let zoom_w = self.zoom * ratio_w;
        let zoom_h = self.zoom * ratio_h;

        self.last_size = Size {
            w: zoom_w,
            h: zoom_h,
        };

        self.last_size.clone()
    }

    fn update_frame(&mut self, dims: Dimensions, producer: &mut (impl Producer + ?Sized)) {
        let size = self.get_size(dims);

        let neg_half = HighPrecision::from_f64(-0.5);
        let base_offset_x = neg_half * size.w;
        let base_offset_y = neg_half * size.h;
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

    // returns `true` if the frame should be updated
    fn handle_input(&mut self) -> bool {
        let mut update = false;
        let dt = HighPrecision::from(get_frame_time());
        update |= self.handle_move(dt);
        update |= self.handle_zoom(dt);

        if is_key_pressed(KeyCode::I) {
            self.should_show_info = !self.should_show_info;
        }

        update
    }

    fn show_info(&self) {
        let size = screen_height() as f32 / 25.;
        let line_delta = 0.7;
        let text_color = BROWN;

        let mut line = 0.;
        let mut add_line = |text| {
            line += 1.;
            draw_text(text, 0., line * line_delta * size, size, text_color);
        };

        add_line(format!("x: {:?}", self.offset.x));
        add_line(format!("y: {:?}", self.offset.y));

        add_line(format!("zoom: {:?}", self.zoom));

        let size = &self.last_size;
        add_line(format!("w: {:?}", size.w));
        add_line(format!("h: {:?}", size.h));

        let ups = self.last_produce_duration.as_secs_f32().recip();
        add_line(format!("{:>9.02?}", self.last_produce_duration));
        add_line(format!("{:>7.02}UPS", ups));
    }

    // returns `true` if the frame should be updated
    fn handle_zoom(&mut self, dt: HighPrecision) -> bool {
        let mut update = false;

        let abs_delta_percent = HighPrecision::from(1.0);
        let multiplier = if is_key_down(KeyCode::LeftShift) {
            HighPrecision::from(2.0)
        } else {
            HighPrecision::from(1.0)
        };

        let delta_percent = if is_key_down(KeyCode::Minus) {
            update = true;
            abs_delta_percent
        } else if is_key_down(KeyCode::Equal) {
            update = true;
            -abs_delta_percent
        } else {
            HighPrecision::from(0.0)
        };

        self.zoom *= HighPrecision::from(1.0) + delta_percent * multiplier * dt;

        update
    }

    // returns `true` if the frame should be updated
    fn handle_move(&mut self, dt: HighPrecision) -> bool {
        let mut update = false;

        let percentage = HighPrecision::from(0.50);

        let multiplier = if is_key_down(KeyCode::LeftShift) {
            HighPrecision::from(2.0)
        } else {
            HighPrecision::from(1.0)
        };

        let dx = self.last_size.w * percentage * dt * multiplier;
        let dy = self.last_size.h * percentage * dt * multiplier;

        if is_key_down(KeyCode::W) {
            self.offset.y -= dy;
            update = true;
        }

        if is_key_down(KeyCode::S) {
            self.offset.y += dy;
            update = true;
        }

        if is_key_down(KeyCode::A) {
            self.offset.x -= dx;
            update = true;
        }

        if is_key_down(KeyCode::D) {
            self.offset.x += dx;
            update = true;
        }

        update
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
