use options;

pub const USAGE: &'static str = "
Fractastic! A CLI for generating fractal images

Usage:
  fractastic [options]

Options:
  -h --help                   Show this screen.
  --re=<re>                   Origin, real axis
  --im=<im>                   Origin, imaginary axis
  --zoom=<zoom>               Zoom factor
  --iterations=<iterations>   Number of iterations
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    flag_re:         Option<f32>,
    flag_im:         Option<f32>,
    flag_zoom:       Option<f32>,
    flag_iterations: Option<usize>
}

pub fn make_options_from_args(args: &Args) -> options::Options {
    let mut options = options::Options::default();

    if let Some(re) = args.flag_re {
        options.window.origin.re = re;
    }

    if let Some(im) = args.flag_im {
        options.window.origin.im = im;
    }

    if let Some(zoom) = args.flag_zoom {
        options.window.zoom = zoom;
    }

    if let Some(iterations) = args.flag_iterations {
        options.iterations = iterations;
    }

    options
}
