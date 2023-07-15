pub mod parallel;
#[cfg(feature = "simd")]
pub mod simd;
pub mod simple;
#[macro_use]
mod simd_boilerplate;

fn map(point: f64, old_top: f64, old_bottom: f64, new_top: f64, new_bottom: f64) -> f64 {
  ((point - old_bottom) / (old_top - old_bottom)).mul_add(new_top - new_bottom, new_bottom)
}
