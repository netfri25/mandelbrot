// TODO: to support future numbers with higher precision, switch these types to use a higher
//       precision type that implements `FromF64` and `Exp2`.
//
//       ideally, it will should also support `Copy`, but just `Clone` is also fine, but it will
//       require a bit of refactoring.

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub w: u64,
    pub h: u64,
}
