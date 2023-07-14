#![allow(unsafe_code, unused)]

use std::sync::{Arc, Mutex};

use astro_float::{BigFloat, RoundingMode};
use image::{ImageBuffer, Rgb};
use rayon::prelude::{ParallelIterator, IntoParallelRefMutIterator, IndexedParallelIterator, IntoParallelIterator};

use crate::{colors::map_to_gradient, common::{mandelbrot, mandelbrot_big}};

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn compute_big(width: u32, height: u32, scale: &BigFloat, point: (f64, f64)) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  let p = 128;
  let rm = RoundingMode::None;

  let mut arr = vec![vec![0; height as usize]; width as usize].into_boxed_slice();

  arr.par_iter_mut().enumerate().for_each(|(x, slice)| {
    for y in 0..height {
      // -2,2 -> -2,0
      let old_bottom = 0.0;
      let old_top = 1.0;
      let new_bottom = BigFloat::from_f64(-2.0, p).div(scale, p, rm).sub(&BigFloat::from_f64(point.0, p), p, rm);
      let new_top = BigFloat::from_f64(2.0, p).div(scale, p, rm).sub(&BigFloat::from_f64(point.1, p), p, rm);

      let x_tmp = f64::from(x as i32) / f64::from(width);
      let y_tmp = f64::from(y as i32) / f64::from(height);
      let (x_scaled, y_scaled) = map_big((x_tmp, y_tmp), old_top, old_bottom, &new_top, &new_bottom, p, rm);

      // let depth = mandelbrot(x_scaled, y_scaled);
      let depth = mandelbrot_big(x_scaled, y_scaled);

      unsafe {
        *slice.get_unchecked_mut(y as usize) = depth;
      }
    }
  });

  ImageBuffer::from_fn(width, height, |x, y| {
    map_to_gradient(arr[x as usize][y as usize])
  })
}

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn compute(width: u32, height: u32, scale: f64, point: (f64, f64), area: (f64, f64, f64, f64)) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  // rayon::ThreadPoolBuilder::new().num_threads(12).build_global().unwrap();

  let mut arr = vec![vec![0; height as usize]; width as usize].into_boxed_slice();

  let mut x_min = Arc::new(Mutex::new(f64::MAX));
  let mut y_min = Arc::new(Mutex::new(f64::MAX));
  let mut x_max = Arc::new(Mutex::new(f64::MIN));
  let mut y_max = Arc::new(Mutex::new(f64::MIN));

  arr.par_iter_mut().enumerate().for_each(|(x, slice)| {
    for y in 0..height {
      // -2,2 -> -2,0
      let x_tmp = f64::from(x as i32) / f64::from(width);
      let y_tmp = f64::from(y as i32) / f64::from(height);

      let x_scaled = map(x_tmp, 1.0, 0.0, area.1, area.0);
      let y_scaled = map(y_tmp, 1.0, 0.0, area.3, area.2);

      if x_scaled > *x_max.lock().unwrap() { *x_max.lock().unwrap() = x_scaled }
      if y_scaled > *y_max.lock().unwrap() { *y_max.lock().unwrap() = y_scaled }
      if x_scaled < *x_min.lock().unwrap() { *x_min.lock().unwrap() = x_scaled }
      if y_scaled < *y_min.lock().unwrap() { *y_min.lock().unwrap() = y_scaled }

      let depth = mandelbrot(x_scaled, y_scaled);
      // let depth = mandelbrot_big(x_scaled, y_scaled);

      unsafe {
        *slice.get_unchecked_mut(y as usize) = depth;
      }
    }
  });

  // dbg!((x_min.lock().unwrap(), y_min.lock().unwrap()), (x_max.lock().unwrap(), y_max.lock().unwrap()));

  // dbg!((min_x_scaled.lock().unwrap(), max_x_scaled.lock().unwrap()), (min_y_scaled.lock().unwrap(), max_y_scaled.lock().unwrap()));

  ImageBuffer::from_fn(width, height, |x, y| {
    map_to_gradient(arr[x as usize][y as usize])
  })
}

fn map(point: f64, old_top: f64, old_bottom: f64, new_top: f64, new_bottom: f64) -> f64 {
  ((point - old_bottom) / (old_top - old_bottom)).mul_add(new_top - new_bottom, new_bottom)
}

fn map_big(point: (f64, f64), old_top: f64, old_bottom: f64, new_top: &BigFloat, new_bottom: &BigFloat, p: usize, rm: RoundingMode) -> (BigFloat, BigFloat) {
  let x = BigFloat::from_f64((point.0 - old_bottom) / (old_top - old_bottom), p);
  let x = x.mul(&new_top.sub(new_bottom, p, rm), p, rm).add(new_bottom, p, rm);

  let y = BigFloat::from_f64((point.1 - old_bottom) / (old_top - old_bottom), p);
  let y = y.mul(&new_top.sub(new_bottom, p, rm), p, rm).add(new_bottom, p, rm);

  (x, y)
}
