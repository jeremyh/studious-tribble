A simple ray tracer for experimentation with Rust.

If you have a [Rust toolchain installed](https://rustup.rs/), use `./run-and-show` on Mac and Linux to make a simple image.

Or build and run it with your own options:

```
❯ chambray --help
Create a ray-traced image

Usage: chambray [OPTIONS] [OUTPUT_FILE]

Arguments:
  [OUTPUT_FILE]  Supports image extensions .ppm, .tga or .ff [default: image.ppm]

Options:
      --width <WIDTH>      [default: 400]
      --height <HEIGHT>    [default: 200]
      --samples <SAMPLES>  [default: 64]
      --threads <THREADS>  [default: 8]
  -h, --help               Print help
  -V, --version            Print version


```

Most OSes support `.ppm` images. The other two formats were added for fun. 
