#![allow(unused_imports, clippy::unwrap_used)]

mod colors;
mod implementations;
mod common;

use image::{GenericImage, GenericImageView};
use implementations::{simple, parallel};

fn main() {
  // let width = 100_000;
  // let width = 50_000;
  // let width = 10_000;
  // let width = 3840;
  let width = 1920;
  // let width = 800;

  let height = 9 * width / 16;

  // let img = simple::compute(width, height);
  let off_x = 1170/2;
  let off_y = 250/2;

  let scale_off = 3.0;
  let scale = 3.2 / scale_off;
  let point = (- (scale_off * width as f64) / 1.5, - (height as f64) / 2.0);


  let img = parallel::compute(width, height, scale, point);

  // let img = img.view(0, 0, width / 2, height / 2);
  // let img = img.to_image();
  
  img.save("out.png").unwrap();
}
