#![feature(portable_simd)]

use clap::Parser;

mod config;
mod exp2;
mod fast_float;
mod from_f64;
mod high_precision;
mod producer;
mod renderer;
mod types;

fn main() {
    let config = config::Config::parse();
    let window_config = config.window_config();
    let program = config.create_macroquad_program();
    macroquad::Window::from_config(window_config, program);
}
