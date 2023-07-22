#![allow(dead_code)]

use image::{ImageBuffer, Rgb};

use crate::{colors::map_to_gradient, common::mandelbrot};

use super::map;

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn compute(
  width: u32,
  height: u32,
  area: (f64, f64, f64, f64),
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  let mut arr = vec![vec![0; height as usize]; width as usize].into_boxed_slice();

  arr.iter_mut().enumerate().for_each(|(x, slice)| {
    for y in 0..height {
      // -2,2 -> -2,0
      let x_tmp = f64::from(x as i32) / f64::from(width);
      let y_tmp = f64::from(y as i32) / f64::from(height);

      let x_scaled = map(x_tmp, 1.0, 0.0, area.1, area.0);
      let y_scaled = map(y_tmp, 1.0, 0.0, area.3, area.2);

      let depth = mandelbrot(x_scaled, y_scaled);

      slice[y as usize] = depth;
    }
  });

  ImageBuffer::from_fn(width, height, |x, y| {
    map_to_gradient(arr[x as usize][y as usize])
  })
}
