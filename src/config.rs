use std::pin::Pin;

use clap::{Parser, ValueEnum};

use crate::fast_float::{FastF32, FastF64};
use crate::producer::Producer;
use crate::producer::threaded::ThreadedProducer;

#[derive(Parser)]
pub struct Config {
    /// type of number to use for calculations
    #[arg(value_enum, default_value_t = NumberType::F64)]
    pub number_type: NumberType,

    /// amount of threads to use for rendering
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..), default_value_t = 16)]
    pub threads: u32,

    /// amount of SIMD lanes to use (only for `f32` or `f64`).
    /// use 0 for a non-SIMD implementation
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=64), default_value_t = 64)]
    pub simd: u8,

    /// window size, in the format `WxH`.
    #[arg(long, short, value_parser = WindowSize::parse, default_value = "800x600")]
    pub window_size: WindowSize,

    /// allow window to be resized
    #[arg(long, short = 'r')]
    pub window_resizable: bool,

    /// rendering resolution
    #[arg(long, value_parser = Resolution::parse, default_value = "0.5")]
    pub resolution: Resolution,

    /// amount of iterations to use for evaluation
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..), default_value_t = 400)]
    pub iterations: u32,
}

impl Config {
    pub fn create_macroquad_program(self) -> Pin<Box<dyn Future<Output = ()>>> {
        rayon::ThreadPoolBuilder::new()
            .num_threads(self.threads as usize)
            .build_global()
            .unwrap();

        let resolution = self.resolution.0;
        let mut producer = self.create_producer();
        let mut renderer = crate::renderer::macroquad::MacroquadRenderer::new(
            1.,
            Default::default(),
            resolution,
        );

        let program = async move {
            loop {
                macroquad::prelude::clear_background(::macroquad::prelude::BLACK);
                crate::renderer::Renderer::render(&mut renderer, producer.as_mut());
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

    fn create_producer(&self) -> Box<dyn Producer + Send> {
        match self.number_type {
            NumberType::F64 => self.create_simd_or_naive_producer::<f64>(self.simd),
            NumberType::F32 => self.create_simd_or_naive_producer::<f32>(self.simd),
            NumberType::FastF64 => self.create_naive_producer::<FastF64>(),
            NumberType::FastF32 => self.create_naive_producer::<FastF32>(),
            NumberType::Posit => self.create_naive_producer::<fast_posit::p64>(),
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

    fn create_simd_producer<T, const LANES: usize>(&self) -> Box<dyn Producer + Send>
    where
        T: Send + 'static,
        crate::producer::simd::SimdProducer<T, LANES>: crate::producer::Producer,
    {
        let iterations = self.iterations;
        self.make_threaded(move || crate::producer::simd::SimdProducer::<T, LANES>::new(iterations))
    }

    fn create_naive_producer<T>(&self) -> Box<dyn Producer + Send>
    where
        T: Send + 'static,
        crate::producer::naive::NaiveProducer<T>: crate::producer::Producer,
    {
        let iterations = self.iterations;
        self.make_threaded(move || crate::producer::naive::NaiveProducer::<T>::new(iterations))
    }

    // NOTE: this can probably be done with code generation, but a using `g<c-a>` in vim is enough
    fn create_simd_or_naive_producer<T>(&self, lanes: u8) -> Box<dyn Producer + Send>
    where
        T: Send + 'static,
        crate::producer::naive::NaiveProducer<T>: Producer + Send + 'static,
        crate::producer::simd::SimdProducer<T, 1>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 2>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 3>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 4>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 5>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 6>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 7>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 8>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 9>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 10>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 11>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 12>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 13>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 14>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 15>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 16>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 17>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 18>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 19>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 20>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 21>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 22>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 23>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 24>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 25>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 26>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 27>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 28>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 29>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 30>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 31>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 32>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 33>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 34>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 35>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 36>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 37>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 38>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 39>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 40>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 41>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 42>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 43>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 44>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 45>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 46>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 47>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 48>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 49>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 50>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 51>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 52>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 53>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 54>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 55>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 56>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 57>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 58>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 59>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 60>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 61>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 62>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 63>: crate::producer::Producer,
        crate::producer::simd::SimdProducer<T, 64>: crate::producer::Producer,
    {
        match lanes {
            1 => self.create_simd_producer::<T, 1>(),
            2 => self.create_simd_producer::<T, 2>(),
            3 => self.create_simd_producer::<T, 3>(),
            4 => self.create_simd_producer::<T, 4>(),
            5 => self.create_simd_producer::<T, 5>(),
            6 => self.create_simd_producer::<T, 6>(),
            7 => self.create_simd_producer::<T, 7>(),
            8 => self.create_simd_producer::<T, 8>(),
            9 => self.create_simd_producer::<T, 9>(),
            10 => self.create_simd_producer::<T, 10>(),
            11 => self.create_simd_producer::<T, 11>(),
            12 => self.create_simd_producer::<T, 12>(),
            13 => self.create_simd_producer::<T, 13>(),
            14 => self.create_simd_producer::<T, 14>(),
            15 => self.create_simd_producer::<T, 15>(),
            16 => self.create_simd_producer::<T, 16>(),
            17 => self.create_simd_producer::<T, 17>(),
            18 => self.create_simd_producer::<T, 18>(),
            19 => self.create_simd_producer::<T, 19>(),
            20 => self.create_simd_producer::<T, 20>(),
            21 => self.create_simd_producer::<T, 21>(),
            22 => self.create_simd_producer::<T, 22>(),
            23 => self.create_simd_producer::<T, 23>(),
            24 => self.create_simd_producer::<T, 24>(),
            25 => self.create_simd_producer::<T, 25>(),
            26 => self.create_simd_producer::<T, 26>(),
            27 => self.create_simd_producer::<T, 27>(),
            28 => self.create_simd_producer::<T, 28>(),
            29 => self.create_simd_producer::<T, 29>(),
            30 => self.create_simd_producer::<T, 30>(),
            31 => self.create_simd_producer::<T, 31>(),
            32 => self.create_simd_producer::<T, 32>(),
            33 => self.create_simd_producer::<T, 33>(),
            34 => self.create_simd_producer::<T, 34>(),
            35 => self.create_simd_producer::<T, 35>(),
            36 => self.create_simd_producer::<T, 36>(),
            37 => self.create_simd_producer::<T, 37>(),
            38 => self.create_simd_producer::<T, 38>(),
            39 => self.create_simd_producer::<T, 39>(),
            40 => self.create_simd_producer::<T, 40>(),
            41 => self.create_simd_producer::<T, 41>(),
            42 => self.create_simd_producer::<T, 42>(),
            43 => self.create_simd_producer::<T, 43>(),
            44 => self.create_simd_producer::<T, 44>(),
            45 => self.create_simd_producer::<T, 45>(),
            46 => self.create_simd_producer::<T, 46>(),
            47 => self.create_simd_producer::<T, 47>(),
            48 => self.create_simd_producer::<T, 48>(),
            49 => self.create_simd_producer::<T, 49>(),
            50 => self.create_simd_producer::<T, 50>(),
            51 => self.create_simd_producer::<T, 51>(),
            52 => self.create_simd_producer::<T, 52>(),
            53 => self.create_simd_producer::<T, 53>(),
            54 => self.create_simd_producer::<T, 54>(),
            55 => self.create_simd_producer::<T, 55>(),
            56 => self.create_simd_producer::<T, 56>(),
            57 => self.create_simd_producer::<T, 57>(),
            58 => self.create_simd_producer::<T, 58>(),
            59 => self.create_simd_producer::<T, 59>(),
            60 => self.create_simd_producer::<T, 60>(),
            61 => self.create_simd_producer::<T, 61>(),
            62 => self.create_simd_producer::<T, 62>(),
            63 => self.create_simd_producer::<T, 63>(),
            64 => self.create_simd_producer::<T, 64>(),
            _ => self.create_naive_producer::<T>(),
        }
    }
}

#[derive(Default, Clone, Copy, ValueEnum)]
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
