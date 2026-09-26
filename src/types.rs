use crate::high_precision::HighPrecision;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pos<T = HighPrecision> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Size<T = HighPrecision> {
    pub w: T,
    pub h: T,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub w: u64,
    pub h: u64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct View<T = HighPrecision> {
    pub start: Pos<T>,
    pub size: Size<T>,
}
