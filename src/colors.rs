use image::Rgb;

const COLORS: [Rgb<u8>; 11] = [
  Rgb([0, 5, 91]),
  Rgb([6, 19, 92]),
  Rgb([12, 39, 110]),
  Rgb([39, 66, 129]),
  Rgb([66, 92, 147]),
  Rgb([95, 118, 166]),
  Rgb([125, 144, 184]),
  Rgb([156, 171, 202]),
  Rgb([188, 199, 220]),
  Rgb([221, 227, 237]),
  Rgb([0, 0, 0]),
];

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn map_to_gradient(depth: i32) -> Rgb<u8> {
  let i = f64::round(f64::log2(f64::from(depth)).clamp(0.0, 10.0)) as usize;

  COLORS[i]
}
