use crate::from_f64::FromF64;
use crate::high_precision::HighPrecision;
use crate::types::{Dimensions, Pos, Size, View};

/// small utility that provides the calculations required to explore the mandelbrot set
pub struct Explorer {
    pub zoom: HighPrecision,
    pub offset: Pos,
}

impl Explorer {
    pub fn new(zoom: HighPrecision, offset: Pos) -> Self {
        Self { zoom, offset }
    }

    pub fn view(&self, dims: &Dimensions) -> View {
        let ratio_size = self.ratio_size(dims);
        let size = self.zoom_size(&ratio_size);
        let start = self.start(&ratio_size, &size);
        View { start, size }
    }

    pub fn ratio_size(&self, dims: &Dimensions) -> Size {
        let dims_w = HighPrecision::from_f64(dims.w as f64);
        let dims_h = HighPrecision::from_f64(dims.h as f64);
        let one = HighPrecision::from_f64(1.);
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

    pub fn zoom_size(&self, ratio_size: &Size) -> Size {
        Size {
            w: ratio_size.w * self.zoom,
            h: ratio_size.h * self.zoom,
        }
    }

    pub fn start(&self, ratio_size: &Size, zoom_size: &Size) -> Pos {
        let half = HighPrecision::from(0.5);
        Pos {
            x: self.offset.x + (ratio_size.w - zoom_size.w) * half,
            y: self.offset.y + (ratio_size.h - zoom_size.h) * half,
        }
    }
}
