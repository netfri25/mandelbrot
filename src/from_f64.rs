// NOTE: this trait exists since I want `f32: From<f64>`, but it's not a thing sadly.
pub trait FromF64 {
    fn from_f64(value: f64) -> Self;
}

impl FromF64 for f32 {
    fn from_f64(value: f64) -> Self {
        value as Self
    }
}

impl FromF64 for f64 {
    fn from_f64(value: f64) -> Self {
        value as Self
    }
}

impl<const N: u32, const ES: u32, Int: fast_posit::Int, const RS: u32> FromF64
    for fast_posit::Posit<N, ES, Int, RS>
{
    fn from_f64(value: f64) -> Self {
        fast_posit::RoundFrom::round_from(value)
    }
}
