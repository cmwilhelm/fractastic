extern crate docopt;
extern crate rustc_serialize;
extern crate image;
extern crate num_complex;
extern crate rand;

use std::fs::File;
use std::path::Path;

mod args;
mod color;
mod mandelbrot;
mod options;
mod window;

fn main () {
    let args: args::Args = docopt::Docopt::new(args::USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let options = args::make_options_from_args(&args);
    let imgbuf  = mandelbrot::generate_image(&options);

    let ref mut fout = File::create(&Path::new("mandelbrot.png")).unwrap();
    let _ = image::ImageRgba8(imgbuf).save(fout, image::PNG).unwrap();
}
