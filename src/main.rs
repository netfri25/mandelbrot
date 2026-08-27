use macroquad::prelude::*;

pub use crate::complex::Complex;
pub use crate::fast_float::{FastF32, FastF64};
use crate::from_f32::FromF32;
use crate::producer::Producer;
use crate::renderer::Renderer;

mod complex;
mod exp2;
mod fast_float;
mod from_f32;
mod producer;
mod renderer;
mod types;

pub type Posit = fast_posit::Posit<64, 2, i64>;

// change this to use a different type
type NumberType = f64;

const WIDTH: i32 = 500;
const HEIGHT: i32 = 500;
const ZOOM: f32 = 1.;
const RESOLUTION: f32 = 0.15;
const ITERATIONS: u32 = 400;
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
    rayon::ThreadPoolBuilder::new().num_threads(THREADS).build_global().unwrap();

    let producer = producer::naive::NaiveProducer::new(ITERATIONS);
    let mut make_producer = move || Box::new(producer.clone()) as Box<dyn Producer<_> + Send>;
    let mut producer = producer::threaded::ThreadedProducer::new(THREADS, &mut make_producer);
    let mut producer = producer::timed::TimedProducer(&mut producer);

    #[allow(clippy::useless_conversion)]
    let mut renderer = renderer::macroquad::MacroquadRenderer::new(
        ZOOM,
        Complex::new(FromF32::from_f32(0.2), FromF32::from_f32(0.0)),
        RESOLUTION,
    );

    loop {
        clear_background(BLACK);
        Renderer::<NumberType>::render(&mut renderer, &mut producer);
        next_frame().await
    }
}
