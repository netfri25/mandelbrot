
pub trait FromF32 {
    fn from_f32(value: f32) -> Self;
}

impl FromF32 for f32 {
    fn from_f32(value: f32) -> Self {
        value
    }
}

impl FromF32 for f64 {
    fn from_f32(value: f32) -> Self {
        value as f64
    }
}

impl FromF32 for rug::Float {
    fn from_f32(value: f32) -> Self {
        Self::with_val_64(256, value)
    }
}

impl<const N: u32, const ES: u32, Int: fast_posit::Int, const RS: u32> FromF32 for fast_posit::Posit<N, ES, Int, RS> {
    fn from_f32(value: f32) -> Self {
        fast_posit::RoundFrom::round_from(value)
    }
}
