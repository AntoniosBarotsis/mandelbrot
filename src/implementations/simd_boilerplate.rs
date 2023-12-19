#[macro_export]
macro_rules! simd_boilerplate {
  ($N: literal) => {
    paste::item! {
      fn [< map_simd_f64x $N >](
        point: Simd<f64, $N>,
        old_top: f64,
        old_bottom: f64,
        new_top: f64,
        new_bottom: f64,
      ) -> Simd<f64, $N>
      // where
      //   [f64; $N]: SimdArray
      {
        let tmp_1 = Simd::<f64, $N>::splat(new_top - new_bottom);
        let tmp_2 = Simd::<f64, $N>::splat(new_bottom);

        let old_bottom = Simd::<f64, $N>::splat(old_bottom);
        let old_top = Simd::<f64, $N>::splat(old_top);
        ((point - old_bottom) / (old_top - old_bottom)).mul_add(tmp_1, tmp_2)
      }

      pub fn [< mandelbrot_simd_f64x $N >](x: Simd<f64, $N>, y: Simd<f64, $N>) -> Simd<i32, $N> {
        let mut depth = Simd::<i32, $N>::splat(0);
        let cr = x;
        let ci = y;
        let mut zr = Simd::<f64, $N>::splat(0.0);
        let mut zi = Simd::<f64, $N>::splat(0.0);
        let two = Simd::<f64, $N>::splat(2.0);

        for _ in 0..MAX_DEPTH {
          let zr_tmp = (zr * zr) - (zi * zi) + cr;
          let zi_tmp = zr * zi * two + ci;
          zr = zr_tmp;
          zi = zi_tmp;

          let zr_squared = zr * zr;
          let zi_squared = zi * zi;
          let square = zr_squared + zi_squared;

          let out = square.simd_lt(Simd::<f64, $N>::splat(4.0)).cast();

          if out.to_bitmask() == 0 {
            break;
          }

          depth += out.select(Simd::<i32, $N>::splat(1), Simd::<i32, $N>::splat(0));
        }

        depth
      }
    }
  };
}
