#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos<N> {
    pub x: N,
    pub y: N,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size<N> {
    pub w: N,
    pub h: N,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub w: usize,
    pub h: usize,
}
