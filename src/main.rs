#![feature(portable_simd)]

use macroquad::prelude::*;

pub use crate::complex::Complex;
pub use crate::fast_float::{FastF32, FastF64};
use crate::renderer::Renderer;

mod complex;
mod exp2;
mod fast_float;
mod from_f32;
pub mod producer;
mod renderer;
mod types;

pub type Posit = fast_posit::Posit<64, 2, i64>;

// change this to use a different type
type NumberType = f64;

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 800;
const ZOOM: f32 = 1.;
const RESOLUTION: f32 = 0.50;
const ITERATIONS: u32 = 400;
const THREADS: usize = 16;
pub const SIMD_LANES: usize = 64;

fn window_conf() -> Conf {
    Conf {
        window_title: "mandelbrot".into(),
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

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(THREADS)
        .build_global()
        .unwrap();

    // let producer = producer::naive::NaiveProducer::new(ITERATIONS);
    let producer = producer::simd::SimdProducer::<SIMD_LANES>::new(ITERATIONS);
    let make_producer = move || producer.clone();
    let mut producer = producer::threaded::ThreadedProducer::new(THREADS, make_producer);

    let mut renderer =
        renderer::macroquad::MacroquadRenderer::new(ZOOM, Default::default(), RESOLUTION);

    let program = async move {
        loop {
            clear_background(BLACK);
            Renderer::<_, NumberType>::render(&mut renderer, &mut producer);
            next_frame().await
        }
    };

    macroquad::Window::from_config(window_conf(), program);
}
