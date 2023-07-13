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
      let new_bottom = -2.0 * scale - 1.0;
      let new_top = 2.0 * scale + 0.0;

      let x_tmp = f64::from(x as i32) / f64::from(width);
      let y_tmp = f64::from(y as i32) / f64::from(height);
      let (x_scaled, y_scaled) = map((x_tmp, y_tmp), old_top, old_bottom, new_top, new_bottom);

      // dbg!((x_scaled, y_scaled));

      // let x_scaled = f64::from((scale * 4.0 * (x as f64)) as i32) / (width as f64) - scale * 2.0;
      // let y_scaled = f64::from((scale * 4.0 * (y as f64)) as i32) / (height as f64) - scale * 2.0;

      // let frac_tl = screen_to_world((0,0), (0.0,0.0), scale);
      // let frac_br = screen_to_world((width as i32, height as i32), (0.0,0.0), scale);
      // let x_scaled = (frac_br.0 - frac_tl.0) / width as f64;
      // let x_scaled = (frac_br.1 - frac_tl.1) / height as f64;
      // let c = (x as f64 * x_scaled + frac_tl.0, y as f64 * y_scaled + frac_tl.1);
      // let depth = mandelbrot(c.0, c.1);

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

fn screen_to_world(n: (i32, i32), offset: (f64, f64), scale: f64) -> (f64, f64) {
  (n.0 as f64 / scale + offset.0, n.1 as f64 / scale + offset.1)
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
