


pub trait Exp {
    fn exp(self) -> Self;
}

impl Exp for f32 {
    fn exp(self) -> Self {
        self.exp2()
    }
}

impl Exp for f64 {
    fn exp(self) -> Self {
        self.exp2()
    }
}

impl<E, F> Exp for flexfloat::flexfloat::FlexFloat<E, F>
where
    E: flexfloat::BitArrayArith,
    F: flexfloat::BitArrayArith,
{
    fn exp(self) -> Self {
        self.exp2()
    }
}


impl<const N: usize> Exp for fastnum::decimal::Decimal<N> {
    fn exp(self) -> Self {
        self.exp2()
    }
}
