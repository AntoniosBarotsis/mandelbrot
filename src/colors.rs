use image::Rgb;

const COLORS: [Rgb<u8>; 11] = [
  // Rgb([6, 19, 92]),
  // Rgb([76, 19, 99]),
  // Rgb([120, 21, 101]),
  // Rgb([158, 34, 98]),
  // Rgb([189, 57, 92]),
  // Rgb([214, 86, 85]),
  // Rgb([231, 118, 78]),
  // Rgb([240, 152, 76]),
  // Rgb([243, 186, 81]),
  // Rgb([239, 221, 99]),
  
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
  // let i = (depth / 100) as usize;
  COLORS[i]
}
