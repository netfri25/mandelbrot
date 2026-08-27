use std::time::Instant;

use flexfloat::flexfloat::FlexFloat;
use flexfloat::prelude::DefaultBitArray;
use macroquad::prelude::*;
use num_bigfloat::BigFloat;
use num_traits::FromPrimitive;

use crate::complex::Complex;
pub use crate::fast_float::{FastF32, FastF64};
use crate::renderer::Renderer;

mod complex;
mod fast_float;
mod producer;
mod renderer;
mod types;

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;
const ZOOM: f32 = 1.;
const RESOLUTION: f32 = 0.15;
const ITERATIONS: u32 = 200;

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

    #[allow(clippy::useless_conversion)]
    let mut renderer = renderer::macroquad::MacroquadRenderer::new(
        FromPrimitive::from_f32(ZOOM).unwrap(),
        Complex::new(0.2.into(), 0.0.into()),
        RESOLUTION,
    );

    loop {
        clear_background(BLACK);

        let start = Instant::now();
        Renderer::<FastF64>::render(&mut renderer, &mut producer);
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
