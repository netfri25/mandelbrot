use std::time::{Duration, Instant};

use macroquad::prelude::*;

use crate::explorer::Explorer;
use crate::high_precision::HighPrecision;
use crate::producer::Producer;
use crate::types::{Dimensions, Size};

use super::Renderer;

pub struct MacroquadRenderer<P> {
    producer: P,
    frame: Option<Texture2D>,
    resolution: f32,
    last_produce_duration: Duration,
    should_show_info: bool,
    last_size: Size,
    explorer: Explorer,
}

impl<P> MacroquadRenderer<P> {
    pub fn new(producer: P, explorer: Explorer, resolution: f32) -> Self {
        Self {
            producer,
            frame: None,
            resolution,
            last_produce_duration: Duration::default(),
            should_show_info: false,
            last_size: Size::default(),
            explorer,
        }
    }

    fn update_frame(&mut self, dims: Dimensions)
    where
        P: Producer,
    {
        let dims = Dimensions {
            w: (dims.w as f32 * self.resolution) as u64,
            h: (dims.h as f32 * self.resolution) as u64,
        };

        let view = self.explorer.view(&dims);
        let produce_start = Instant::now();
        let values = self.producer.produce(&view, dims);
        self.last_produce_duration = produce_start.elapsed();

        let minimum = values.iter().fold(0.9, |a, b| b.min(a));

        let mut image = Image::gen_image_color(dims.w as u16, dims.h as u16, Color::default());
        let ratio_size = self.explorer.ratio_size(&dims);
        self.last_size = self.explorer.zoom_size(&ratio_size);

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

        add_line(format!("x: {:?}", self.explorer.offset.x));
        add_line(format!("y: {:?}", self.explorer.offset.y));

        add_line(format!("zoom: {:?}", self.explorer.zoom));

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

        self.explorer.zoom *= HighPrecision::from(1.0) + delta_percent * multiplier * dt;

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
            self.explorer.offset.y -= dy;
            update = true;
        }

        if is_key_down(KeyCode::S) {
            self.explorer.offset.y += dy;
            update = true;
        }

        if is_key_down(KeyCode::A) {
            self.explorer.offset.x -= dx;
            update = true;
        }

        if is_key_down(KeyCode::D) {
            self.explorer.offset.x += dx;
            update = true;
        }

        update
    }
}

impl<P> Renderer for MacroquadRenderer<P>
where
    P: Producer,
{
    fn render(&mut self) {
        let update = self.handle_input();

        let dims = Dimensions {
            w: screen_width() as u64,
            h: screen_height() as u64,
        };

        if update || self.frame.is_none() {
            self.update_frame(dims)
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
                    dims.w as f32,
                    dims.h as f32,
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
