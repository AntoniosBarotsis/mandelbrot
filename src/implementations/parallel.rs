#![allow(unsafe_code, unused)]

use image::{ImageBuffer, Rgb};
use indicatif::ProgressBar;
use rayon::prelude::{ParallelIterator, IntoParallelRefMutIterator, IndexedParallelIterator};

use crate::{colors::map_to_gradient, common::mandelbrot};

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn compute(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  // rayon::ThreadPoolBuilder::new().num_threads(12).build_global().unwrap();

  let mut arr = vec![vec![0; height as usize]; width as usize].into_boxed_slice();

  let tmp_x = 3.3 / f64::from(width);
  let tmp_y = 2.8 / f64::from(height);
  let pb = ProgressBar::new(u64::from(width));

  arr.par_iter_mut().enumerate().for_each(|(x, slice)| {
    for y in 0..height {
      let x_scaled = f64::from(x as i32).mul_add(tmp_x, -2.15);
      let y_scaled = f64::from(y).mul_add(tmp_y, -1.4);
  
      let depth = mandelbrot(x_scaled, y_scaled);

      unsafe {
        *slice.get_unchecked_mut(y as usize) = depth;
      }
    }
    pb.inc(1);
  });

  ImageBuffer::from_fn(width, height, |x, y| {
    map_to_gradient(arr[x as usize][y as usize])
  })
}
