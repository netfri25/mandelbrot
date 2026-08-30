
use crate::high_precision::HighPrecision;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pos {
    pub x: HighPrecision,
    pub y: HighPrecision,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Size {
    pub w: HighPrecision,
    pub h: HighPrecision,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub w: u64,
    pub h: u64,
}
