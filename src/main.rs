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
  let img = parallel::compute(width, height);

  // let img = img.view(width / 2, height / 14, width / 4, height / 4);
  // let img = img.to_image();
  
  img.save("out.png").unwrap();
}
