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

macro_rules! create_macroquad_program_body {
    ($producer:expr, $resolution:expr) => {{
        let mut producer = $producer;
        let mut renderer = $crate::renderer::macroquad::MacroquadRenderer::new(
            1.,
            Default::default(),
            $resolution,
        );

        let program = async move {
            loop {
                ::macroquad::prelude::clear_background(::macroquad::prelude::BLACK);
                $crate::renderer::Renderer::render(&mut renderer, &mut producer);
                ::macroquad::prelude::next_frame().await
            }
        };

        Box::pin(program)
    }};
}

fn make_threaded<F, P>(threads: u32, mut make_producer: F) -> Box<dyn Producer + Send>
where
    ThreadedProducer<F>: Producer,
    F: FnMut() -> P + Send + 'static,
    P: Producer + Send + 'static,
{
    if threads == 1 {
        Box::new(make_producer())
    } else {
        Box::new(ThreadedProducer::new(threads as usize, make_producer))
    }
}

impl Config {
    pub fn create_macroquad_program(self) -> Pin<Box<dyn Future<Output = ()>>> {
        rayon::ThreadPoolBuilder::new()
            .num_threads(self.threads as usize)
            .build_global()
            .unwrap();

        let resolution = self.resolution.0;

        match self.number_type {
            NumberType::F64 => {
                let producer = make_threaded(self.threads, move || match self.simd {
                    0 => self.create_naive_producer::<f64>(),
                    lanes => self.create_simd_producer::<f64>(lanes),
                });
                create_macroquad_program_body!(producer, resolution)
            }

            NumberType::F32 => {
                let producer = make_threaded(self.threads, move || match self.simd {
                    0 => self.create_naive_producer::<f32>(),
                    lanes => self.create_simd_producer::<f32>(lanes),
                });
                create_macroquad_program_body!(producer, resolution)
            }

            NumberType::FastF64 => {
                let producer = make_threaded(self.threads, move || {
                    self.create_naive_producer::<FastF64>()
                });
                create_macroquad_program_body!(producer, resolution)
            }

            NumberType::FastF32 => {
                let producer = make_threaded(self.threads, move || {
                    self.create_naive_producer::<FastF32>()
                });
                create_macroquad_program_body!(producer, resolution)
            }

            NumberType::Posit => {
                let producer = make_threaded(self.threads, move || {
                    self.create_naive_producer::<fast_posit::p64>()
                });

                create_macroquad_program_body!(producer, resolution)
            }
        }
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

    fn create_naive_producer<T>(&self) -> Box<dyn Producer + Send>
    where
        crate::producer::naive::NaiveProducer<T>: Producer + Send + 'static,
    {
        Box::new(crate::producer::naive::NaiveProducer::new(self.iterations))
    }

    // NOTE: this can probably be done with code generation, but a using `g<c-a>` in vim is enough
    fn create_simd_producer<T>(&self, lanes: u8) -> Box<dyn Producer + Send>
    where
        T: Send + 'static,
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
            1 => Box::new(crate::producer::simd::SimdProducer::<T, 1>::new(
                self.iterations,
            )),
            2 => Box::new(crate::producer::simd::SimdProducer::<T, 2>::new(
                self.iterations,
            )),
            3 => Box::new(crate::producer::simd::SimdProducer::<T, 3>::new(
                self.iterations,
            )),
            4 => Box::new(crate::producer::simd::SimdProducer::<T, 4>::new(
                self.iterations,
            )),
            5 => Box::new(crate::producer::simd::SimdProducer::<T, 5>::new(
                self.iterations,
            )),
            6 => Box::new(crate::producer::simd::SimdProducer::<T, 6>::new(
                self.iterations,
            )),
            7 => Box::new(crate::producer::simd::SimdProducer::<T, 7>::new(
                self.iterations,
            )),
            8 => Box::new(crate::producer::simd::SimdProducer::<T, 8>::new(
                self.iterations,
            )),
            9 => Box::new(crate::producer::simd::SimdProducer::<T, 9>::new(
                self.iterations,
            )),
            10 => Box::new(crate::producer::simd::SimdProducer::<T, 10>::new(
                self.iterations,
            )),
            11 => Box::new(crate::producer::simd::SimdProducer::<T, 11>::new(
                self.iterations,
            )),
            12 => Box::new(crate::producer::simd::SimdProducer::<T, 12>::new(
                self.iterations,
            )),
            13 => Box::new(crate::producer::simd::SimdProducer::<T, 13>::new(
                self.iterations,
            )),
            14 => Box::new(crate::producer::simd::SimdProducer::<T, 14>::new(
                self.iterations,
            )),
            15 => Box::new(crate::producer::simd::SimdProducer::<T, 15>::new(
                self.iterations,
            )),
            16 => Box::new(crate::producer::simd::SimdProducer::<T, 16>::new(
                self.iterations,
            )),
            17 => Box::new(crate::producer::simd::SimdProducer::<T, 17>::new(
                self.iterations,
            )),
            18 => Box::new(crate::producer::simd::SimdProducer::<T, 18>::new(
                self.iterations,
            )),
            19 => Box::new(crate::producer::simd::SimdProducer::<T, 19>::new(
                self.iterations,
            )),
            20 => Box::new(crate::producer::simd::SimdProducer::<T, 20>::new(
                self.iterations,
            )),
            21 => Box::new(crate::producer::simd::SimdProducer::<T, 21>::new(
                self.iterations,
            )),
            22 => Box::new(crate::producer::simd::SimdProducer::<T, 22>::new(
                self.iterations,
            )),
            23 => Box::new(crate::producer::simd::SimdProducer::<T, 23>::new(
                self.iterations,
            )),
            24 => Box::new(crate::producer::simd::SimdProducer::<T, 24>::new(
                self.iterations,
            )),
            25 => Box::new(crate::producer::simd::SimdProducer::<T, 25>::new(
                self.iterations,
            )),
            26 => Box::new(crate::producer::simd::SimdProducer::<T, 26>::new(
                self.iterations,
            )),
            27 => Box::new(crate::producer::simd::SimdProducer::<T, 27>::new(
                self.iterations,
            )),
            28 => Box::new(crate::producer::simd::SimdProducer::<T, 28>::new(
                self.iterations,
            )),
            29 => Box::new(crate::producer::simd::SimdProducer::<T, 29>::new(
                self.iterations,
            )),
            30 => Box::new(crate::producer::simd::SimdProducer::<T, 30>::new(
                self.iterations,
            )),
            31 => Box::new(crate::producer::simd::SimdProducer::<T, 31>::new(
                self.iterations,
            )),
            32 => Box::new(crate::producer::simd::SimdProducer::<T, 32>::new(
                self.iterations,
            )),
            33 => Box::new(crate::producer::simd::SimdProducer::<T, 33>::new(
                self.iterations,
            )),
            34 => Box::new(crate::producer::simd::SimdProducer::<T, 34>::new(
                self.iterations,
            )),
            35 => Box::new(crate::producer::simd::SimdProducer::<T, 35>::new(
                self.iterations,
            )),
            36 => Box::new(crate::producer::simd::SimdProducer::<T, 36>::new(
                self.iterations,
            )),
            37 => Box::new(crate::producer::simd::SimdProducer::<T, 37>::new(
                self.iterations,
            )),
            38 => Box::new(crate::producer::simd::SimdProducer::<T, 38>::new(
                self.iterations,
            )),
            39 => Box::new(crate::producer::simd::SimdProducer::<T, 39>::new(
                self.iterations,
            )),
            40 => Box::new(crate::producer::simd::SimdProducer::<T, 40>::new(
                self.iterations,
            )),
            41 => Box::new(crate::producer::simd::SimdProducer::<T, 41>::new(
                self.iterations,
            )),
            42 => Box::new(crate::producer::simd::SimdProducer::<T, 42>::new(
                self.iterations,
            )),
            43 => Box::new(crate::producer::simd::SimdProducer::<T, 43>::new(
                self.iterations,
            )),
            44 => Box::new(crate::producer::simd::SimdProducer::<T, 44>::new(
                self.iterations,
            )),
            45 => Box::new(crate::producer::simd::SimdProducer::<T, 45>::new(
                self.iterations,
            )),
            46 => Box::new(crate::producer::simd::SimdProducer::<T, 46>::new(
                self.iterations,
            )),
            47 => Box::new(crate::producer::simd::SimdProducer::<T, 47>::new(
                self.iterations,
            )),
            48 => Box::new(crate::producer::simd::SimdProducer::<T, 48>::new(
                self.iterations,
            )),
            49 => Box::new(crate::producer::simd::SimdProducer::<T, 49>::new(
                self.iterations,
            )),
            50 => Box::new(crate::producer::simd::SimdProducer::<T, 50>::new(
                self.iterations,
            )),
            51 => Box::new(crate::producer::simd::SimdProducer::<T, 51>::new(
                self.iterations,
            )),
            52 => Box::new(crate::producer::simd::SimdProducer::<T, 52>::new(
                self.iterations,
            )),
            53 => Box::new(crate::producer::simd::SimdProducer::<T, 53>::new(
                self.iterations,
            )),
            54 => Box::new(crate::producer::simd::SimdProducer::<T, 54>::new(
                self.iterations,
            )),
            55 => Box::new(crate::producer::simd::SimdProducer::<T, 55>::new(
                self.iterations,
            )),
            56 => Box::new(crate::producer::simd::SimdProducer::<T, 56>::new(
                self.iterations,
            )),
            57 => Box::new(crate::producer::simd::SimdProducer::<T, 57>::new(
                self.iterations,
            )),
            58 => Box::new(crate::producer::simd::SimdProducer::<T, 58>::new(
                self.iterations,
            )),
            59 => Box::new(crate::producer::simd::SimdProducer::<T, 59>::new(
                self.iterations,
            )),
            60 => Box::new(crate::producer::simd::SimdProducer::<T, 60>::new(
                self.iterations,
            )),
            61 => Box::new(crate::producer::simd::SimdProducer::<T, 61>::new(
                self.iterations,
            )),
            62 => Box::new(crate::producer::simd::SimdProducer::<T, 62>::new(
                self.iterations,
            )),
            63 => Box::new(crate::producer::simd::SimdProducer::<T, 63>::new(
                self.iterations,
            )),
            64 => Box::new(crate::producer::simd::SimdProducer::<T, 64>::new(
                self.iterations,
            )),
            _ => unreachable!("clap ensures that lanes should be between 0 and 64"),
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
