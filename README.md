# Mandelbrot
simple, efficient, generic, and well designed [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) explorer.

![Rendererd Mandelbrot Frame](./assets/mandelbrot.png)

### Features
 - Multithreaded
 - SIMD support, using Rust's Portable SIMD module ([std::simd](https://doc.rust-lang.org/stable/std/simd/index.html))
 - Generic over everything - from the type of the number to the renderer

