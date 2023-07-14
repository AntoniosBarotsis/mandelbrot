use astro_float::{BigFloat, Consts, RoundingMode};
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

pub fn mandelbrot_big(x: BigFloat, y: BigFloat) -> i32 {
  let mut depth = 0;
  let p = 128;
  let rm: RoundingMode = RoundingMode::None;

  // let x = BigFloat::from_f64(x, p);
  // let y = BigFloat::from_f64(y, p);

  let cr = x;
  let ci = y;
  let mut zr = BigFloat::from_word(0, p);
  let mut zi = BigFloat::from_word(0, p);

  while depth < MAX_DEPTH {
    let zr_squared = zr.powi(2, p, rm);
    let zi_squared = zi.powi(2, p, rm);

    let zr_tmp = zr_squared.sub(&zi_squared, p, rm).add(&cr, p, rm);
    let zi_tmp = (zr.mul(&zi, p, rm).mul(&BigFloat::from_word(2, p), p, rm)).add(&ci, p, rm);
    zr = zr_tmp;
    zi = zi_tmp;

    let zr_squared = zr.powi(2, p, rm);
    let zi_squared = zi.powi(2, p, rm);
    let square = zr_squared.add(&zi_squared, p, rm);

    if square > BigFloat::from_word(4, p) {
      return depth;
    }
    // if square.sub(&BigFloat::from_word(4), p, rm).is_positive() {
    //   dbg!("Exiting mandelbrot_big...");
    //   return depth;
    // }

    depth += 1;
  }

  depth
}

pub fn zoom(area: (f64, f64, f64, f64), amt: f64) -> (f64, f64, f64, f64) {
  (area.0 + amt, area.1 - amt, area.2 + amt, area.3 - amt)
}

#[cfg(test)]
mod tests {
    use super::zoom;

  #[test]
  fn test() {
    let area = (-2.0,0.0, -1.0,1.0);
    let expected = (-1.5,-0.5,-0.5,0.5);
    let actual = zoom(area, 0.5);

    assert_eq!(expected, actual);
  }
}

