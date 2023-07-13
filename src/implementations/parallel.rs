#![allow(unsafe_code, unused)]

use image::{ImageBuffer, Rgb};
use rayon::prelude::{ParallelIterator, IntoParallelRefMutIterator, IndexedParallelIterator};

use crate::{colors::map_to_gradient, common::mandelbrot};

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn compute(width: u32, height: u32, scale: f64, point: (f64, f64)) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  // rayon::ThreadPoolBuilder::new().num_threads(12).build_global().unwrap();

  let mut arr = vec![vec![0; height as usize]; width as usize].into_boxed_slice();

  arr.par_iter_mut().enumerate().for_each(|(x, slice)| {
    for y in 0..height {
      // -2,2 -> -2,0
      let old_bottom = 0.0;
      let old_top = 1.0;
      let new_bottom = -2.0 / scale - point.0;
      let new_top = 2.0 / scale - point.1;

      let x_tmp = f64::from(x as i32) / f64::from(width);
      let y_tmp = f64::from(y as i32) / f64::from(height);
      let (x_scaled, y_scaled) = map((x_tmp, y_tmp), old_top, old_bottom, new_top, new_bottom);

      let depth = mandelbrot(x_scaled, y_scaled);

      unsafe {
        *slice.get_unchecked_mut(y as usize) = depth;
      }
    }
  });

  // dbg!((min_x_scaled.lock().unwrap(), max_x_scaled.lock().unwrap()), (min_y_scaled.lock().unwrap(), max_y_scaled.lock().unwrap()));

  ImageBuffer::from_fn(width, height, |x, y| {
    map_to_gradient(arr[x as usize][y as usize])
  })
}

fn map(point: (f64, f64), old_top: f64, old_bottom: f64, new_top: f64, new_bottom: f64) -> (f64, f64) {
  let x = ((point.0 - old_bottom) / (old_top - old_bottom)).mul_add(new_top - new_bottom, new_bottom);
  let y = ((point.1 - old_bottom) / (old_top - old_bottom)).mul_add(new_top - new_bottom, new_bottom);

  (x, y)
}

// new_value - 
//   (old_value - old_bottom) / (old_top - old_bottom) *
//   (new_top - new_bottom) + new_bottom

// Y=(X-A)/(B-A) * (D-C)+C
