use macroquad::prelude::*;

use crate::complex::Complex;
pub use crate::fast_float::{FastF32, FastF64};
use crate::producer::Producer;
use crate::renderer::Renderer;

mod complex;
mod fast_float;
mod producer;
mod renderer;
mod types;
mod exp;

type ZoomType = FastF64;
type NumberType = FastF64;

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;
const ZOOM: f64 = 1.;
const RESOLUTION: f32 = 0.15;
const ITERATIONS: u32 = 200;
const THREADS: usize = 16;

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
    let producer = producer::naive::NaiveProducer::new(ITERATIONS);
    let mut make_producer = move || Box::new(producer.clone()) as Box<dyn Producer<_> + Send>;
    let mut producer = producer::threaded::ThreadedProducer::new(THREADS, &mut make_producer);

    #[allow(clippy::useless_conversion)]
    let mut renderer = renderer::macroquad::MacroquadRenderer::new(
        ZoomType::from(ZOOM),
        Complex::new(0.2.into(), 0.0.into()),
        RESOLUTION,
    );

    loop {
        clear_background(BLACK);
        Renderer::<NumberType>::render(&mut renderer, &mut producer);
        next_frame().await
    }
}


