use std::time::Instant;

use macroquad::prelude::*;
use num_traits::FromPrimitive;

use crate::complex::Complex;
pub use crate::fast_f32::FastF32;
pub use crate::fast_f64::FastF64;
use crate::renderer::Renderer;

mod complex;
mod fast_f32;
mod fast_f64;
mod producer;
mod renderer;
mod types;

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;
const ZOOM: f32 = 1.;
const RESOLUTION: f32 = 0.1;
const ITERATIONS: u32 = 1000;

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
    let mut producer = producer::naive::NaiveProducer::new(ITERATIONS);
    let mut renderer = renderer::macroquad::MacroquadRenderer::new(
        FromPrimitive::from_f32(ZOOM).unwrap(),
        Complex::new(
            FromPrimitive::from_f32(-0.2).unwrap(),
            FromPrimitive::from_f32(0.).unwrap(),
        ),
        RESOLUTION,
    );

    loop {
        clear_background(BLACK);

        let start = Instant::now();
        Renderer::<f64>::render(&mut renderer, &mut producer);
        let elapsed = start.elapsed();
        eprintln!(
            "render took {:.02?} ({:.02} max fps)",
            elapsed,
            elapsed.as_secs_f32().recip()
        );

        draw_fps();
        next_frame().await
    }
}
