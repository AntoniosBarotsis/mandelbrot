
# Mandelbrot-rs

A Mandelbrot set renderer (with zoom!) I implemented for fun in Rust.


https://github.com/AntoniosBarotsis/mandelbrot/assets/50240570/8ab83659-81d9-47b7-bb88-d804ae88e872


[./assets/output2.mp4](https://github.com/AntoniosBarotsis/mandelbrot/assets/50240570/8ab83659-81d9-47b7-bb88-d804ae88e872)

## Usage

Running the project `cargo r -r` will create a "slide show" of separate frames in `./data`. These
can then be stitched together with [FFmpeg](https://ffmpeg.org/) with something like:

```sh
ffmpeg -framerate 30 -i 'data/img%03d.png' -pix_fmt yuv420p output.mp4
```

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


