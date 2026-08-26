use std::ops::Add;

use num_traits::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos<N: Float> {
    pub x: N,
    pub y: N,
}

impl<N: Float> Add for Pos<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size<N: Float> {
    pub w: N,
    pub h: N,
}
