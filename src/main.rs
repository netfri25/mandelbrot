use std::time::Instant;

use macroquad::prelude::*;

use crate::fast_f64::FastF64;
use crate::renderer::Renderer;

mod complex;
mod fast_f64;
mod producer;
mod renderer;
mod types;

const WIDTH: i32 = 500;
const HEIGHT: i32 = 500;
const SCALE: f64 = 3.;

fn conf() -> Conf {
    Conf {
        window_title: "almond".into(),
        window_width: WIDTH,
        window_height: HEIGHT,
        high_dpi: false,
        fullscreen: false,
        window_resizable: false,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut producer = producer::naive::NaiveProducer::new(500);
    let mut renderer = renderer::macroquad::MacroquadRenderer::new(SCALE, DVec2::new(-0.2, 0.));

    loop {
        clear_background(BLACK);

        let start = Instant::now();
        Renderer::<FastF64>::render(&mut renderer, &mut producer);
        let elapsed = start.elapsed();
        eprintln!(
            "render took {:.02?} ({:.02} fps)",
            elapsed,
            elapsed.as_secs_f32().recip()
        );

        next_frame().await
    }
}
