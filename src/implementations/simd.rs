#![allow(dead_code, unsafe_code)]

use crate::{colors::map_to_gradient, common::MAX_DEPTH};
use image::{ImageBuffer, Rgb};
use packed_simd::{f64x2, i32x2, Simd};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn compute(
  width: u32,
  height: u32,
  area: (f64, f64, f64, f64),
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  let mut arr = vec![vec![0; height as usize]; width as usize].into_boxed_slice();

  for [(x1, slice1), (x2, slice2)] in &mut arr.iter_mut().enumerate().array_chunks() {
    for y in 0..height {
      // -2,2 -> -2,0
      let xs_tmp = f64x2::new(
        f64::from(x1 as i32) / f64::from(width),
        f64::from(x2 as i32) / f64::from(width),
      );
      let ys_tmp = f64x2::new(
        f64::from(y as i32) / f64::from(height),
        f64::from(y as i32) / f64::from(height),
      );

      let x_scaled = map_simd(xs_tmp, 1.0, 0.0, area.1, area.0);
      let y_scaled = map_simd(ys_tmp, 1.0, 0.0, area.3, area.2);

      let depth = mandelbrot_simd(x_scaled, y_scaled);

      unsafe {
        *slice1.get_unchecked_mut(y as usize) = depth.extract(0);
        *slice2.get_unchecked_mut(y as usize) = depth.extract(1);
      }
    }
  }

  ImageBuffer::from_fn(width, height, |x, y| {
    map_to_gradient(arr[x as usize][y as usize])
  })
}

pub fn compute_parallel(
  width: u32,
  height: u32,
  area: (f64, f64, f64, f64),
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  let mut arr = vec![vec![0; height as usize]; width as usize].into_boxed_slice();

  arr.par_iter_mut().enumerate().chunks(2).for_each(|mut a| {
    let ([(x1, slice1)], [(x2, slice2)]) = a.split_at_mut(1) else {
      panic!()
    };

    for y in 0..height {
      // -2,2 -> -2,0
      let xs_tmp = f64x2::new(
        f64::from(*x1 as i32) / f64::from(width),
        f64::from(*x2 as i32) / f64::from(width),
      );
      let ys_tmp = f64x2::new(
        f64::from(y as i32) / f64::from(height),
        f64::from(y as i32) / f64::from(height),
      );

      let x_scaled = map_simd(xs_tmp, 1.0, 0.0, area.1, area.0);
      let y_scaled = map_simd(ys_tmp, 1.0, 0.0, area.3, area.2);

      let depth = mandelbrot_simd(x_scaled, y_scaled);

      unsafe {
        *slice1.get_unchecked_mut(y as usize) = depth.extract(0);
        *slice2.get_unchecked_mut(y as usize) = depth.extract(1);
      }
    }
  });

  ImageBuffer::from_fn(width, height, |x, y| {
    map_to_gradient(arr[x as usize][y as usize])
  })
}

fn map_simd(
  point: Simd<[f64; 2]>,
  old_top: f64,
  old_bottom: f64,
  new_top: f64,
  new_bottom: f64,
) -> Simd<[f64; 2]> {
  let tmp_1 = f64x2::new(new_top - new_bottom, new_top - new_bottom);
  let tmp_2 = f64x2::new(new_bottom, new_bottom);
  ((point - old_bottom) / (old_top - old_bottom)).mul_add(tmp_1, tmp_2)
}

pub fn mandelbrot_simd(x: Simd<[f64; 2]>, y: Simd<[f64; 2]>) -> Simd<[i32; 2]> {
  let mut depth = i32x2::new(0, 0);
  let cr = x;
  let ci = y;
  let mut zr = f64x2::splat(0.0);
  let mut zi = f64x2::splat(0.0);
  let two = f64x2::splat(2.0);

  for _ in 0..MAX_DEPTH {
    let zr_tmp = zr.powf(two) - zi.powf(two) + cr;
    let zi_tmp = zr * zi * two + ci;
    zr = zr_tmp;
    zi = zi_tmp;

    let zr_squared = zr.powf(two);
    let zi_squared = zi.powf(two);
    let square = zr_squared + zi_squared;

    let out = square.le(f64x2::splat(4.0));

    if out.none() {
      break;
    }

    depth += out.select(i32x2::splat(1), i32x2::splat(0));
  }

  depth
}
