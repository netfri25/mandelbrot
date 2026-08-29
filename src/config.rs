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

    /// rendering resolution
    #[arg(short, long, value_parser = Resolution::parse, default_value = "0.5")]
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

fn make_threaded<F, P, T>(threads: u32, make_producer: F) -> Box<dyn Producer<T> + Send>
where
    ThreadedProducer<F>: Producer<T>,
    F: FnMut() -> P + Send + 'static,
    P: Producer<T> + Send,
{
    Box::new(ThreadedProducer::new(threads as usize, make_producer))
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
            window_resizable: false,
            platform: macroquad::miniquad::conf::Platform {
                linux_backend: macroquad::miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn create_naive_producer<T>(&self) -> Box<dyn Producer<T> + Send>
    where
        crate::producer::naive::NaiveProducer: Producer<T>,
    {
        Box::new(crate::producer::naive::NaiveProducer::new(self.iterations))
    }

    // NOTE: this can probably be done with code generation, but a using `g<c-a>` in vim is enough
    fn create_simd_producer<T>(&self, lanes: u8) -> Box<dyn Producer<T> + Send>
    where
        crate::producer::simd::SimdProducer<1>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<2>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<3>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<4>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<5>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<6>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<7>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<8>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<9>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<10>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<11>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<12>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<13>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<14>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<15>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<16>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<17>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<18>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<19>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<20>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<21>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<22>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<23>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<24>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<25>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<26>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<27>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<28>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<29>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<30>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<31>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<32>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<33>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<34>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<35>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<36>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<37>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<38>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<39>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<40>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<41>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<42>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<43>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<44>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<45>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<46>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<47>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<48>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<49>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<50>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<51>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<52>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<53>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<54>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<55>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<56>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<57>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<58>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<59>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<60>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<61>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<62>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<63>: crate::producer::Producer<T>,
        crate::producer::simd::SimdProducer<64>: crate::producer::Producer<T>,
    {
        match lanes {
            1 => Box::new(crate::producer::simd::SimdProducer::<1>::new(
                self.iterations,
            )),
            2 => Box::new(crate::producer::simd::SimdProducer::<2>::new(
                self.iterations,
            )),
            3 => Box::new(crate::producer::simd::SimdProducer::<3>::new(
                self.iterations,
            )),
            4 => Box::new(crate::producer::simd::SimdProducer::<4>::new(
                self.iterations,
            )),
            5 => Box::new(crate::producer::simd::SimdProducer::<5>::new(
                self.iterations,
            )),
            6 => Box::new(crate::producer::simd::SimdProducer::<6>::new(
                self.iterations,
            )),
            7 => Box::new(crate::producer::simd::SimdProducer::<7>::new(
                self.iterations,
            )),
            8 => Box::new(crate::producer::simd::SimdProducer::<8>::new(
                self.iterations,
            )),
            9 => Box::new(crate::producer::simd::SimdProducer::<9>::new(
                self.iterations,
            )),
            10 => Box::new(crate::producer::simd::SimdProducer::<10>::new(
                self.iterations,
            )),
            11 => Box::new(crate::producer::simd::SimdProducer::<11>::new(
                self.iterations,
            )),
            12 => Box::new(crate::producer::simd::SimdProducer::<12>::new(
                self.iterations,
            )),
            13 => Box::new(crate::producer::simd::SimdProducer::<13>::new(
                self.iterations,
            )),
            14 => Box::new(crate::producer::simd::SimdProducer::<14>::new(
                self.iterations,
            )),
            15 => Box::new(crate::producer::simd::SimdProducer::<15>::new(
                self.iterations,
            )),
            16 => Box::new(crate::producer::simd::SimdProducer::<16>::new(
                self.iterations,
            )),
            17 => Box::new(crate::producer::simd::SimdProducer::<17>::new(
                self.iterations,
            )),
            18 => Box::new(crate::producer::simd::SimdProducer::<18>::new(
                self.iterations,
            )),
            19 => Box::new(crate::producer::simd::SimdProducer::<19>::new(
                self.iterations,
            )),
            20 => Box::new(crate::producer::simd::SimdProducer::<20>::new(
                self.iterations,
            )),
            21 => Box::new(crate::producer::simd::SimdProducer::<21>::new(
                self.iterations,
            )),
            22 => Box::new(crate::producer::simd::SimdProducer::<22>::new(
                self.iterations,
            )),
            23 => Box::new(crate::producer::simd::SimdProducer::<23>::new(
                self.iterations,
            )),
            24 => Box::new(crate::producer::simd::SimdProducer::<24>::new(
                self.iterations,
            )),
            25 => Box::new(crate::producer::simd::SimdProducer::<25>::new(
                self.iterations,
            )),
            26 => Box::new(crate::producer::simd::SimdProducer::<26>::new(
                self.iterations,
            )),
            27 => Box::new(crate::producer::simd::SimdProducer::<27>::new(
                self.iterations,
            )),
            28 => Box::new(crate::producer::simd::SimdProducer::<28>::new(
                self.iterations,
            )),
            29 => Box::new(crate::producer::simd::SimdProducer::<29>::new(
                self.iterations,
            )),
            30 => Box::new(crate::producer::simd::SimdProducer::<30>::new(
                self.iterations,
            )),
            31 => Box::new(crate::producer::simd::SimdProducer::<31>::new(
                self.iterations,
            )),
            32 => Box::new(crate::producer::simd::SimdProducer::<32>::new(
                self.iterations,
            )),
            33 => Box::new(crate::producer::simd::SimdProducer::<33>::new(
                self.iterations,
            )),
            34 => Box::new(crate::producer::simd::SimdProducer::<34>::new(
                self.iterations,
            )),
            35 => Box::new(crate::producer::simd::SimdProducer::<35>::new(
                self.iterations,
            )),
            36 => Box::new(crate::producer::simd::SimdProducer::<36>::new(
                self.iterations,
            )),
            37 => Box::new(crate::producer::simd::SimdProducer::<37>::new(
                self.iterations,
            )),
            38 => Box::new(crate::producer::simd::SimdProducer::<38>::new(
                self.iterations,
            )),
            39 => Box::new(crate::producer::simd::SimdProducer::<39>::new(
                self.iterations,
            )),
            40 => Box::new(crate::producer::simd::SimdProducer::<40>::new(
                self.iterations,
            )),
            41 => Box::new(crate::producer::simd::SimdProducer::<41>::new(
                self.iterations,
            )),
            42 => Box::new(crate::producer::simd::SimdProducer::<42>::new(
                self.iterations,
            )),
            43 => Box::new(crate::producer::simd::SimdProducer::<43>::new(
                self.iterations,
            )),
            44 => Box::new(crate::producer::simd::SimdProducer::<44>::new(
                self.iterations,
            )),
            45 => Box::new(crate::producer::simd::SimdProducer::<45>::new(
                self.iterations,
            )),
            46 => Box::new(crate::producer::simd::SimdProducer::<46>::new(
                self.iterations,
            )),
            47 => Box::new(crate::producer::simd::SimdProducer::<47>::new(
                self.iterations,
            )),
            48 => Box::new(crate::producer::simd::SimdProducer::<48>::new(
                self.iterations,
            )),
            49 => Box::new(crate::producer::simd::SimdProducer::<49>::new(
                self.iterations,
            )),
            50 => Box::new(crate::producer::simd::SimdProducer::<50>::new(
                self.iterations,
            )),
            51 => Box::new(crate::producer::simd::SimdProducer::<51>::new(
                self.iterations,
            )),
            52 => Box::new(crate::producer::simd::SimdProducer::<52>::new(
                self.iterations,
            )),
            53 => Box::new(crate::producer::simd::SimdProducer::<53>::new(
                self.iterations,
            )),
            54 => Box::new(crate::producer::simd::SimdProducer::<54>::new(
                self.iterations,
            )),
            55 => Box::new(crate::producer::simd::SimdProducer::<55>::new(
                self.iterations,
            )),
            56 => Box::new(crate::producer::simd::SimdProducer::<56>::new(
                self.iterations,
            )),
            57 => Box::new(crate::producer::simd::SimdProducer::<57>::new(
                self.iterations,
            )),
            58 => Box::new(crate::producer::simd::SimdProducer::<58>::new(
                self.iterations,
            )),
            59 => Box::new(crate::producer::simd::SimdProducer::<59>::new(
                self.iterations,
            )),
            60 => Box::new(crate::producer::simd::SimdProducer::<60>::new(
                self.iterations,
            )),
            61 => Box::new(crate::producer::simd::SimdProducer::<61>::new(
                self.iterations,
            )),
            62 => Box::new(crate::producer::simd::SimdProducer::<62>::new(
                self.iterations,
            )),
            63 => Box::new(crate::producer::simd::SimdProducer::<63>::new(
                self.iterations,
            )),
            64 => Box::new(crate::producer::simd::SimdProducer::<64>::new(
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
