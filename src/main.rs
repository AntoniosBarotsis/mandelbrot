#![allow(unused_imports, clippy::unwrap_used)]

mod colors;
mod implementations;
mod common;

use std::fmt::format;
use indicatif::ProgressBar;

use image::{GenericImage, GenericImageView};
use implementations::{simple, parallel};

fn main() {
  // let width = 100_000;
  // let width = 50_000;
  // let width = 10_000;
  // let width = 3840;
  let width = 1920;
  // let width = 720;
  // let width = 480;

  let height = 9 * width / 16;

  // let scale_off = 3.0;
  // let scale = 3.2 / scale_off;
  // let point = (- (scale_off * width as f64) / 1.5, - (height as f64) / 2.0);
  // let img = parallel::compute(width, height, scale, point);
  // img.save("out.png").unwrap();

  create_frames(width, height, 510);
}
//50
fn create_frames(width: u32, height: u32, n: u32) {
  let pb = ProgressBar::new(n as u64);
  let mut first_frame = None;

  let mut scale_off = 1.0;
  for i in 0..n {
    if i < 30 {
      if first_frame.as_ref().is_none() {
        let scale = 3.0 / scale_off;
        let point = (- (0.900 * scale_off * width as f64) / 1.5, - (height as f64) / 2.0);
    
        first_frame = Some(parallel::compute(width, height, scale, point));
        scale_off += 0.005;
      }
      first_frame.clone().unwrap().save(format!("data/img{i:0>3}.png")).unwrap();
      pb.inc(1);
      continue;
    }
    let scale = 3.0 / scale_off;
    let point = (- (0.900 * scale_off * width as f64) / 1.5, - (height as f64) / 2.0);


    let img = parallel::compute(width, height, scale, point);
    
    img.save(format!("data/img{i:0>3}.png")).unwrap();
    pb.inc(1);

    if i < 120 {
      scale_off += 0.005;
    } else {
      scale_off += 0.1;
    }
  }
}
