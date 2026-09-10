# Mandelbrot
simple, efficient, generic, and well designed [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) explorer.

![Rendererd Mandelbrot Frame](./assets/mandelbrot.png)

### Features
 - Multithreaded
 - SIMD support, using Rust's Portable SIMD module ([std::simd](https://doc.rust-lang.org/stable/std/simd/index.html))
 - Generic over everything - the type of the number used for calculations, the renderer, the method for producing values, etc.
 - Configurable amount of SIMD lanes (from 1 to 64, inclusive)
 - Supports multiple number formats:
    - 32 bit float (with and without SIMD)
    - 64 bit float (with and without SIMD)
    - 32 bit float with ignored non-associativity[^1]
    - 64 bit float with ignored non-associativity[^1]
    - [posit](https://en.wikipedia.org/wiki/Unum_(number_format)#Unum_III)
    - fixed point (named `high-precision`) I10F118 (10 bits for integer part, 118 bits for fractional part)

[^1]: [the IEEE-754 format isn't associative](https://en.wikipedia.org/wiki/Associative_property#Nonassociativity_of_floating-point_calculation), meaning that `(a + b) + c` may not be equal to `a + (b + c)` (give it a try with 0.1, 0.2, 0.3). these types are defined with arithmetic that ignores this fact and treats the floats as associative, which allows the compiler to optimize even further by reordering operations and applying auto-vectorization.
