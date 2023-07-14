#![allow(unused_imports, clippy::unwrap_used)]

mod colors;
mod implementations;
mod common;

use std::fmt::format;
use astro_float::{BigFloat, RoundingMode};
use indicatif::ProgressBar;

use image::{GenericImage, GenericImageView};
use implementations::{simple, parallel};

use crate::common::zoom;

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

  // create_frames(width, height, 510);
  // create_frames_2(width, height, 30);
  create_frames_2(width, height, 7 * 60);
}

// fn create_frames(width: u32, height: u32, n: u32) {
//   let pb = ProgressBar::new(n as u64);
//   let mut first_frame = None;

//   let mut scale_off = 1.0;
//   for i in 0..n {
//     if i < 30 {
//       if first_frame.as_ref().is_none() {
//         let scale = 3.0 / scale_off;
//         let point = (- (0.900 * scale_off * width as f64) / 1.5, - (height as f64) / 2.0);
    
//         first_frame = Some(parallel::compute(width, height, scale, point));
//         scale_off += 0.005;
//       }
//       first_frame.clone().unwrap().save(format!("data/img{i:0>3}.png")).unwrap();
//       pb.inc(1);
//       continue;
//     }
//     let scale = 3.0 / scale_off;
//     let point = (- (0.900 * scale_off * width as f64) / 1.5, - (height as f64) / 2.0);


//     let img = parallel::compute(width, height, scale, point);
    
//     img.save(format!("data/img{i:0>3}.png")).unwrap();
//     pb.inc(1);

//     if i < 120 {
//       scale_off += 0.005;
//     } else {
//       scale_off += 0.1;
//     }
//   }
// }

// TODO: Maybe the scale should also be a big float
fn create_frames_2(width: u32, height: u32, n: u32) {
  let pb = ProgressBar::new(n as u64);

  let mut scale = 1.0;
  // -4.78,1.22, -3.0,3.0
  let point = ( 	-0.669611276569,  	-0.458152008518);
  let mut area = (point.0 - 2.0, point.0 + 2.0, point.1 - 2.0, point.1 + 2.0);


  // let p = 128;
  // let rm = RoundingMode::None;
  // // let mut scale = BigFloat::from_f64(1.0, p);
  // let mut scale = BigFloat::from_f64(50_000.0, p);
  // let scale_off = BigFloat::from_f64(scale_off, p);


  for i in 0..n {
    // let point = (400.0, 000.0);
    // let point = (-600.0, - 100.0);


    // let area = (0.365,0.385,0.265,0.285);
    let img = parallel::compute(width, height, scale, (0.0, 0.0), area);
    // let img = parallel::compute_big(width, height, &scale, (1.0, 0.0));
    
    img.save(format!("data/img{i:0>3}.png")).unwrap();
    pb.inc(1);

    // dbg!(scale);
    // let scale_off = 0.1;
    let scale_off = (area.1 - area.0).abs() / 64.0;
    area = zoom(area, scale_off);
    // scale *= scale_off;
    // scale = scale.mul(&scale_off, p, rm);

    // if i < 120 {
    //   scale_off += 0.005;
    // } else {
    //   scale_off += 0.1;
    // }
  }
}