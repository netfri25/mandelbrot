use std::ops::{Add, Div, Mul, Sub};

use crate::from_f64::FromF64;
use crate::high_precision::HighPrecision;
use crate::types::{Dimensions, Pos, Size, View};

/// small utility that provides the calculations required to explore the mandelbrot set
pub struct Explorer<T = HighPrecision> {
    pub zoom: T,
    pub offset: Pos<T>,
}

impl<T> Explorer<T>
where
    T: FromF64 + PartialOrd + Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Div<T, Output = T>,
{
    pub fn new(zoom: T, offset: Pos<T>) -> Self {
        Self { zoom, offset }
    }

    pub fn view(&self, dims: &Dimensions) -> View<T> {
        let ratio_size = self.ratio_size(dims);
        let size = self.zoom_size(&ratio_size);
        let start = self.start(&ratio_size, &size);
        View { start, size }
    }

    pub fn ratio_size(&self, dims: &Dimensions) -> Size<T> {
        let dims_w = T::from_f64(dims.w as f64);
        let dims_h = T::from_f64(dims.h as f64);
        let one = T::from_f64(1.);
        let [ratio_w, ratio_h] = if dims_w > dims_h {
            [one, dims_h / dims_w]
        } else {
            [dims_w / dims_h, one]
        };

        Size {
            w: ratio_w,
            h: ratio_h,
        }
    }

    pub fn zoom_size(&self, ratio_size: &Size<T>) -> Size<T> {
        Size {
            w: ratio_size.w * self.zoom,
            h: ratio_size.h * self.zoom,
        }
    }

    pub fn start(&self, ratio_size: &Size<T>, zoom_size: &Size<T>) -> Pos<T> {
        let half = T::from_f64(0.5);
        Pos {
            x: self.offset.x + (ratio_size.w - zoom_size.w) * half,
            y: self.offset.y + (ratio_size.h - zoom_size.h) * half,
        }
    }
}
