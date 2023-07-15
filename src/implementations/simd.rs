#![allow(
  dead_code,
  unsafe_code,
  clippy::similar_names,
  clippy::cast_possible_truncation,
  clippy::cast_possible_wrap
)]

use crate::{colors::map_to_gradient, common::MAX_DEPTH};
use image::{ImageBuffer, Rgb};
use packed_simd::{f64x2, f64x8, Simd, SimdArray};
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

      let x_scaled = map_simd_f64x2(xs_tmp, 1.0, 0.0, area.1, area.0);
      let y_scaled = map_simd_f64x2(ys_tmp, 1.0, 0.0, area.3, area.2);

      let depth = mandelbrot_simd_f64x2(x_scaled, y_scaled);

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
      let xs_tmp = f64x2::new(
        f64::from(*x1 as i32) / f64::from(width),
        f64::from(*x2 as i32) / f64::from(width),
      );
      let ys_tmp = f64x2::splat(f64::from(y as i32) / f64::from(height));

      let x_scaled = map_simd_f64x2(xs_tmp, 1.0, 0.0, area.1, area.0);
      let y_scaled = map_simd_f64x2(ys_tmp, 1.0, 0.0, area.3, area.2);

      let depth = mandelbrot_simd_f64x2(x_scaled, y_scaled);

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

pub fn compute_parallel_f64x8(
  width: u32,
  height: u32,
  area: (f64, f64, f64, f64),
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  let mut arr = vec![vec![0; height as usize]; width as usize].into_boxed_slice();

  arr.par_iter_mut().enumerate().chunks(8).for_each(|mut a| {
    let [
      (x1, slice1),
      (x2, slice2),
      (x3, slice3),
      (x4, slice4),
      (x5, slice5),
      (x6, slice6),
      (x7, slice7),
      (x8, slice8),
      ] = unsafe { a.get_many_unchecked_mut([0,1,2,3,4,5,6,7]) };

    for y in 0..height {
      // -2,2 -> -2,0
      let xs_tmp = f64x8::new(
        f64::from(*x1 as i32) / f64::from(width),
        f64::from(*x2 as i32) / f64::from(width),
        f64::from(*x3 as i32) / f64::from(width),
        f64::from(*x4 as i32) / f64::from(width),
        f64::from(*x5 as i32) / f64::from(width),
        f64::from(*x6 as i32) / f64::from(width),
        f64::from(*x7 as i32) / f64::from(width),
        f64::from(*x8 as i32) / f64::from(width),
      );
      let ys_tmp = f64x8::splat(
        f64::from(y as i32) / f64::from(height),
      );

      let x_scaled = map_simd_f64x8(xs_tmp, 1.0, 0.0, area.1, area.0);
      let y_scaled = map_simd_f64x8(ys_tmp, 1.0, 0.0, area.3, area.2);

      let depth = mandelbrot_simd_f64x8(x_scaled, y_scaled);

      unsafe {
        *slice1.get_unchecked_mut(y as usize) = depth.extract(0);
        *slice2.get_unchecked_mut(y as usize) = depth.extract(1);
        *slice3.get_unchecked_mut(y as usize) = depth.extract(2);
        *slice4.get_unchecked_mut(y as usize) = depth.extract(3);
        *slice5.get_unchecked_mut(y as usize) = depth.extract(4);
        *slice6.get_unchecked_mut(y as usize) = depth.extract(5);
        *slice7.get_unchecked_mut(y as usize) = depth.extract(6);
        *slice8.get_unchecked_mut(y as usize) = depth.extract(7);
      }
    }
  });

  ImageBuffer::from_fn(width, height, |x, y| {
    map_to_gradient(arr[x as usize][y as usize])
  })
}

crate::simd_boilerplate!(8);
crate::simd_boilerplate!(2);
