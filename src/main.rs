#![allow(unused_imports, clippy::unwrap_used)]

mod colors;
mod implementations;
mod common;

use implementations::{simple, parallel};

fn main() {
  let width = 50_000;
  // let width = 3840;
  // let width = 1920;
  // let width = 800;

  let height = 9 * width / 16;

  // let img = simple::compute(width, height);
  let img = parallel::compute(width, height);
  img.save("out.png").unwrap();
}
