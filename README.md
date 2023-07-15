
# Mandelbrot-rs

A Mandelbrot set renderer (with zoom!) I implemented for fun in Rust.

[./assets/output2.mp4](https://github.com/AntoniosBarotsis/mandelbrot/assets/50240570/8ab83659-81d9-47b7-bb88-d804ae88e872)

## Usage

Running the project `cargo r -r` will create a "slide show" of separate frames in `./data`. These
can then be stitched together with [FFmpeg](https://ffmpeg.org/) with something like:

```sh
ffmpeg -framerate 30 -i 'data/img%03d.png' -pix_fmt yuv420p output.mp4
```

> **Note** If you're on Windows, you can install FFmpeg via
> [Chocolatey](https://community.chocolatey.org/) with
>
> ```sh
> choco install ffmpeg -y
> ```

## Configurations

Currently, there's no "nice" way for users to do any sort of configuration without editing the code
directly. As I worked on this purely for my own fun, I'm not too interested in implementing that.

There's a few things you might want to edit:

| What | Where | Use |
|:---:|:---:|:---:|
| `point` | `main.rs/create_frames()` | This is the point the video will be zooming towards |
| `scale_off` | `main.rs/create_frames()` | This determines the amount each thread will zoom by. Changing the denominator is an easy way to affect how quickly/slowly the zoom happens |
| `width` | `main.rs` | The output image width. The FFmpeg command I mention works with 1920 and 480 so these are the two I left here for now. Use 480 for testing and 1920 for a nicer render. This can in theory just be whatever number you want (I rendered a 50kp frame at some point) |
| `COLORS` | `colors.rs` | The color pallete. This should be 11 elements, if you want to use more, you probably need to make sure the `depth` works and is handled correctly. |
| `MAX_DEPTH` | `common.rs` | The maximum depth used in the Mandelbrot calculations. I have not played with this at all 👍 |

## SIMD

My SIMD code relies on a bunch of features that have not landed in stable Rust yet (it didn't
really need to use any of them but I just wanted to try some new stuff out) and thus you need
the Nightly toolchain to build those. They are put behind a `simd` feature so that you can still
tinker with the rest of the project in stable Rust.

```sh
rustup default nightly
# Might also need a `rustup update`
cargo b -r -F simd
```

## Benchmarks

I ran some simple benchmarks with [hyperfine](https://github.com/sharkdp/hyperfine) and got the
following results:

```
1920x1080p - 30 frames

Benchmark 1: Simple
  Time (mean ± σ):     68.014 s ±  1.744 s    [User: 66.984 s, System: 0.239 s]
  Range (min … max):   66.985 s … 71.085 s    5 runs
  
Benchmark 2: Parallel (2 threads)
  Time (mean ± σ):     41.453 s ±  4.840 s    [User: 75.976 s, System: 0.227 s]
  Range (min … max):   38.249 s … 49.833 s    5 runs

Benchmark 3: SIMD
  Time (mean ± σ):     39.394 s ±  0.522 s    [User: 38.457 s, System: 0.203 s]
  Range (min … max):   38.724 s … 39.954 s    5 runs
  
Benchmark 4: Parallel
  Time (mean ± σ):     20.054 s ±  1.140 s    [User: 123.051 s, System: 0.649 s]
  Range (min … max):   18.492 s … 21.706 s    5 runs
  
Benchmark 5: SIMD + Parallel
  Time (mean ± σ):     14.300 s ±  0.235 s    [User: 70.323 s, System: 0.490 s]
  Range (min … max):   13.912 s … 14.477 s    5 runs
  
Benchmark 6: SIMD (f64x8) + Parallel
  Time (mean ± σ):     10.257 s ±  1.228 s    [User: 54.645 s, System: 0.259 s]
  Range (min … max):    8.062 s … 10.859 s    5 runs
```

## Limitations

Towards the ends of both videos in the [assets](./assets/) folder, you can see what I presume to be
the limitations of 64-bit floating number accuracy. Ways to go past that include using
[128-bit floats](https://crates.io/crates/f128) or 
[multiple-precision floating-point numbers](https://crates.io/crates/gmp-mpfr-sys). Neither of these
natively support Windows which is why I chose to not go any further.

There's also [`astro-float`](https://crates.io/crates/astro-float) which is written in native Rust
but do keep in mind that it is much slower than using standard floats (as is likely the case with
the 2 other alternatives I mentioned previously, I've just happened to have only used this one).

## Stuff that Helped me

- [Render the Julia set in 3 dozen lines of Rust code](https://www.youtube.com/watch?v=g4vN2Z0JuZI)
- [Online Mandelbrot Set Plotter](https://sciencedemos.org.uk/mandelbrot.php)
- [Brute Force Processing](https://youtu.be/PBvLs88hvJ8)
