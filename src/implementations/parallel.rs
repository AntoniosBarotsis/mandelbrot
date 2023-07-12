#![allow(unsafe_code, unused)]

use image::{ImageBuffer, Rgb};
use indicatif::ProgressBar;
use rayon::prelude::{ParallelIterator, IntoParallelRefMutIterator, IndexedParallelIterator};

use crate::{colors::map_to_gradient, common::mandelbrot};

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn compute(width: u32, height: u32, scale: f64, point: (f64, f64)) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  // rayon::ThreadPoolBuilder::new().num_threads(12).build_global().unwrap();

  let mut arr = vec![vec![0; height as usize]; width as usize].into_boxed_slice();

  let pb = ProgressBar::new(u64::from(width));

  arr.par_iter_mut().enumerate().for_each(|(x, slice)| {
    for y in 0..height {
      let x_scaled = f64::from(x as i32 + (point.0) as i32) * scale / (width as f64);
      let y_scaled = f64::from(y as i32 + (point.1) as i32) * scale / (height as f64);

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
