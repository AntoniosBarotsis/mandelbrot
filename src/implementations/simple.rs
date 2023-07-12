#![allow(unused)]

use image::{ImageBuffer, Rgb};

use crate::{colors::map_to_gradient, common::mandelbrot};

pub fn compute(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::<Rgb<u8>, _>::new(width, height);
  let tmp_x = 3.3 / f64::from(width);
  let tmp_y = 2.8 / f64::from(height);

  for (x, y, pixel) in img.enumerate_pixels_mut() {
    let x = f64::from(x).mul_add(tmp_x, -2.15);
    let y = f64::from(y).mul_add(tmp_y, -1.4);

    let c = mandelbrot(x, y);

    *pixel = map_to_gradient(c);
  }

  img
}