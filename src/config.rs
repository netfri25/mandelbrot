use std::num::NonZeroU32;
use std::pin::Pin;

use clap::{Parser, ValueEnum};
use macroquad::color::Color;

use crate::explorer::Explorer;
use crate::fast_float::{FastF32, FastF64};
use crate::high_precision::HighPrecision;
use crate::producer::Producer;
use crate::producer::threaded::ThreadedProducer;
use crate::renderer::texture_renderer::TextureRenderer;
use crate::renderer::texture_renderer::cpu::CpuTextureRenderer;
use crate::renderer::texture_renderer::gpu::GpuTextureRenderer;

#[derive(Parser)]
pub struct Config {
    /// type of number to use for calculations
    #[arg(short, long, default_value = "f64")]
    pub number_type: NumberType,

    /// amount of threads to use for rendering
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..), default_value_t = 16)]
    pub threads: u32,

    /// amount of SIMD lanes to use (only for `f32` or `f64`).
    /// use 0 for a non-SIMD implementation
    #[cfg(not(feature = "no_simd"))]
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=64), default_value_t = 64)]
    pub simd: u8,

    /// window size, in the format `WxH`.
    #[arg(long, short, value_parser = WindowSize::parse, default_value = "800x600")]
    pub window_size: WindowSize,

    /// allow window to be resized
    #[arg(long, short = 'r')]
    pub window_resizable: bool,

    /// run on the GPU (only f32)
    #[arg(long, short)]
    pub gpu: bool,

    /// rendering resolution
    #[arg(long, value_parser = Resolution::parse, default_value = "0.5")]
    pub resolution: Resolution,

    /// amount of iterations to use for evaluation
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..), default_value_t = 400)]
    pub iterations: u32,
}

impl Config {
    pub fn create_macroquad_program(self) -> Pin<Box<dyn Future<Output = ()>>> {
        let program = async move {
            rayon::ThreadPoolBuilder::new()
                .num_threads(self.threads as usize)
                .build_global()
                .unwrap();

            let explorer = self.create_explorer();
            let resolution = self.resolution.0;
            let texture_renderer = self.create_texture_renderer();
            let mut renderer = crate::renderer::macroquad::MacroquadRenderer::new(
                texture_renderer,
                explorer,
                resolution,
            );

            loop {
                macroquad::prelude::clear_background(::macroquad::prelude::BLACK);
                crate::renderer::Renderer::render(&mut renderer);
                macroquad::prelude::next_frame().await
            }
        };

        Box::pin(program)
    }

    pub fn window_config(&self) -> macroquad::window::Conf {
        macroquad::window::Conf {
            window_title: "mandelbrot".into(),
            window_width: self.window_size.width as i32,
            window_height: self.window_size.height as i32,
            window_resizable: self.window_resizable,
            platform: macroquad::miniquad::conf::Platform {
                linux_backend: macroquad::miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn create_texture_renderer(&self) -> Box<dyn TextureRenderer + 'static> {
        if self.gpu {
            Box::new(GpuTextureRenderer::new(self.iterations))
        } else {
            let producer = self.create_cpu_producer();
            let colorizer = self.create_cpu_colorizer();
            Box::new(CpuTextureRenderer::new(producer, colorizer))
        }
    }

    fn create_explorer(&self) -> Explorer {
        // TODO: make these configurable?
        let zoom = 3.5.into();
        let offset = Default::default();
        Explorer::new(zoom, offset)
    }

    fn create_cpu_colorizer(&self) -> impl FnMut(Option<NonZeroU32>) -> Color + 'static {
        // TODO: make this configurable
        bernstein_colorizer
    }

    fn create_cpu_producer(&self) -> Box<dyn Producer + Send> {
        #[cfg(feature = "no_simd")]
        let simd = 0;

        #[cfg(not(feature = "no_simd"))]
        let simd = self.simd;

        match self.number_type {
            NumberType::F64 => self.create_simd_or_naive_cpu_producer::<f64>(simd),
            NumberType::F32 => self.create_simd_or_naive_cpu_producer::<f32>(simd),
            NumberType::FastF64 => self.create_naive_cpu_producer::<FastF64>(),
            NumberType::FastF32 => self.create_naive_cpu_producer::<FastF32>(),
            NumberType::Posit => self.create_naive_cpu_producer::<fast_posit::p64>(),
            NumberType::HighPrecision => self.create_naive_cpu_producer::<HighPrecision>(),
        }
    }

    fn make_threaded<F, P>(&self, mut make_producer: F) -> Box<dyn Producer + Send>
    where
        ThreadedProducer<F>: Producer,
        F: FnMut() -> P + Send + 'static,
        P: Producer + Send + 'static,
    {
        if self.threads == 1 {
            Box::new(make_producer())
        } else {
            Box::new(ThreadedProducer::new(self.threads as usize, make_producer))
        }
    }

    #[cfg(not(feature = "no_simd"))]
    fn create_simd_cpu_producer<T, const LANES: usize>(&self) -> Box<dyn Producer + Send>
    where
        T: Send + 'static,
        crate::producer::simd::SimdProducer<T, LANES>: crate::producer::Producer,
    {
        let iterations = self.iterations;
        self.make_threaded(move || crate::producer::simd::SimdProducer::<T, LANES>::new(iterations))
    }

    fn create_naive_cpu_producer<T>(&self) -> Box<dyn Producer + Send>
    where
        T: Send + 'static,
        crate::producer::naive::NaiveProducer<T>: crate::producer::Producer,
    {
        let iterations = self.iterations;
        self.make_threaded(move || crate::producer::naive::NaiveProducer::<T>::new(iterations))
    }

    #[cfg(feature = "no_simd")]
    fn create_simd_or_naive_cpu_producer<T>(&self, _lanes: u8) -> Box<dyn Producer + Send>
    where
        T: Send + 'static,
        crate::producer::naive::NaiveProducer<T>: Producer + Send + 'static,
    {
        self.create_naive_cpu_producer::<T>()
    }

    // NOTE: this can probably be done with code generation, but a using `g<c-a>` in vim is enough
    #[cfg(not(feature = "no_simd"))]
    fn create_simd_or_naive_cpu_producer<T>(&self, lanes: u8) -> Box<dyn Producer + Send>
    where
        T: Send + 'static,
        crate::producer::naive::NaiveProducer<T>: Producer + Send + 'static,
        crate::producer::simd::SimdProducer<T, 1>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 2>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 4>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 8>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 16>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 32>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 64>: crate::producer::Producer,
    {
        match lanes {
            0 => self.create_naive_cpu_producer::<T>(),
            n if n <= 1 => self.create_simd_cpu_producer::<T, 1>(),
            n if n <= 2 => self.create_simd_cpu_producer::<T, 2>(),
            n if n <= 4 => self.create_simd_cpu_producer::<T, 4>(),
            n if n <= 8 => self.create_simd_cpu_producer::<T, 8>(),
            n if n <= 16 => self.create_simd_cpu_producer::<T, 16>(),
            n if n <= 32 => self.create_simd_cpu_producer::<T, 32>(),
            n if n <= 64 => self.create_simd_cpu_producer::<T, 64>(),
            _ => self.create_naive_cpu_producer::<T>(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
pub enum NumberType {
    #[default]
    /// double precision floating point value. supports SIMD.
    F64,
    /// single precision floating point value. supports SIMD.
    F32,
    /// double precision floating point value, assuming associativity.
    FastF64,
    /// single precision floating point value, assuming associativity.
    FastF32,
    /// posit (Type III Unum)
    Posit,
    /// Program defined high precision number
    HighPrecision,
}

#[derive(Debug, Clone, Copy)]
pub struct WindowSize {
    pub width: u16,
    pub height: u16,
}

impl WindowSize {
    pub fn parse(input: &str) -> Result<Self, String> {
        let (l, r) = input
            .split_once('x')
            .ok_or("no `x` separator found in window size")?;

        let width = l
            .parse()
            .map_err(|e| format!("unable to parse {l} as window width: {e}"))?;

        let height = r
            .parse()
            .map_err(|e| format!("unable to parse {r} as window height: {e}"))?;

        Ok(Self { width, height })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Resolution(pub f32);

impl Resolution {
    pub fn parse(input: &str) -> Result<Self, String> {
        let value = input
            .parse::<f32>()
            .map_err(|e| format!("unable to parse resolution: {e}"))?;

        if value <= 0. || value > 1. {
            return Err(format!(
                "resolution should be in the range (0, 1], but instead got: {value:.02}"
            ));
        }

        Ok(Self(value))
    }
}

fn bernstein_colorizer(iterations: Option<NonZeroU32>) -> Color {
    let Some(iterations) = iterations else {
        return Color::new(0., 0., 0., 1.);
    };

    let t = (iterations.get() as f32).sqrt() * 0.05;
    let t = t.clamp(0.0, 1.0);

    Color::new(
        9.0 * (1.0 - t) * t * t * t,
        15.0 * (1.0 - t) * (1.0 - t) * t * t,
        8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t,
        1.0,
    )
}
