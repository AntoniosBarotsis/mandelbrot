#![allow(clippy::similar_names)]

use nalgebra::Complex;

const MAX_DEPTH: i32 = 1000;

pub fn mandelbrot(x: f64, y: f64) -> i32 {
  let mut depth = 0;
  let c = Complex::new(x, y);
  let mut z = Complex::new(0.0, 0.0);

  while depth < MAX_DEPTH {
    z = z.powi(2) + c;

    if z.norm_sqr() > 4.0 {
      return depth;
    }

    depth += 1;
  }

  depth
}

pub fn zoom(area: (f64, f64, f64, f64), amt: f64) -> (f64, f64, f64, f64) {
  (area.0 + amt, area.1 - amt, area.2 + amt, area.3 - amt)
}
