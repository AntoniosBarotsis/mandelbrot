use nalgebra::Complex;

pub fn mandelbrot(x: f64, y: f64) -> i32 {
  let mut depth = 0;
  let c = Complex::new(x, y);
  let z_0 = Complex::new(0.0, 0.0);
  let mut z = z_0;

  while depth < 1000 {
    z = next(z, c);

    if z.norm_sqr() > 4.0 {
      return depth;
    }

    depth += 1;
  }
  
  depth
  // 1000.0
}

fn next(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
  z.powi(2) + c
}
