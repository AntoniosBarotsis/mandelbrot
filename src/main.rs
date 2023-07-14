#![cfg_attr(
  feature = "simd",
  feature(portable_simd, iter_array_chunks, split_array)
)]

mod colors;
mod common;
mod implementations;

use indicatif::ProgressBar;

use crate::common::zoom;

fn main() {
  let width = 1920;
  // let width = 480;

  let height = 9 * width / 16;

  create_frames(width, height, 30);
  // create_frames(width, height, 7 * 60);
}

// TODO: Maybe the scale should also be a big float
fn create_frames(width: u32, height: u32, n: u32) {
  #[allow(clippy::cast_lossless)]
  let pb = ProgressBar::new(n as u64);

  let point = (-0.669_611_276_569, -0.458_152_008_518);
  let mut area = (point.0 - 2.0, point.0 + 2.0, point.1 - 2.0, point.1 + 2.0);

  for i in 0..n {
    #[cfg(not(feature = "simd"))]
    let img = implementations::parallel::compute(width, height, area);
    #[cfg(feature = "simd")]
    let img = implementations::simd::compute_parallel(width, height, area);

    img
      .save(format!("data/img{i:0>3}.png"))
      .expect("Image saved");
    pb.inc(1);

    let scale_off = (area.1 - area.0).abs() / 64.0;
    area = zoom(area, scale_off);
  }
}
