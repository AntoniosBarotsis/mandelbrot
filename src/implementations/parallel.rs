#![allow(unsafe_code)]

use image::{ImageBuffer, Rgb};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{colors::map_to_gradient, common::mandelbrot};

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn compute(
  width: u32,
  height: u32,
  area: (f64, f64, f64, f64),
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  let mut arr = vec![vec![0; height as usize]; width as usize].into_boxed_slice();

  arr.par_iter_mut().enumerate().for_each(|(x, slice)| {
    for y in 0..height {
      // -2,2 -> -2,0
      let x_tmp = f64::from(x as i32) / f64::from(width);
      let y_tmp = f64::from(y as i32) / f64::from(height);

      let x_scaled = map(x_tmp, 1.0, 0.0, area.1, area.0);
      let y_scaled = map(y_tmp, 1.0, 0.0, area.3, area.2);

      let depth = mandelbrot(x_scaled, y_scaled);

      unsafe {
        *slice.get_unchecked_mut(y as usize) = depth;
      }
    }
  });

  ImageBuffer::from_fn(width, height, |x, y| {
    map_to_gradient(arr[x as usize][y as usize])
  })
}

fn map(point: f64, old_top: f64, old_bottom: f64, new_top: f64, new_bottom: f64) -> f64 {
  ((point - old_bottom) / (old_top - old_bottom)).mul_add(new_top - new_bottom, new_bottom)
}
