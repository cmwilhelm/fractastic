extern crate image;
extern crate num_complex;
extern crate rand;

use std::fs::File;
use std::path::Path;

use num_complex::Complex;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn to_pixel(&self) -> image::Rgba<u8> {
        image::Rgba([self.r as u8, self.g as u8, self.b as u8, 255])
    }
}

fn complex_from_point(x: u32, y: u32, xmax: u32, ymax: u32) -> Complex<f32> {
    Complex {
        re: (x as f32 / xmax as f32) * 3.0 - 2.5,
        im: (y as f32 / ymax as f32) * 2.0 - 1.0
    }
}

fn mandelbrot_escape_number(complex_point: Complex<f32>, escape_point: Option<u32>) -> Option<usize> {
    let mut z: Complex<f32> = Complex { re: 0.0, im: 0.0 };

    let epoint = match escape_point {
        Some(value) => value as f32,
        None        => 2.0
    };

    for i in 0..250 {
        z = z.powf(2.0) + complex_point;
        if z.norm() >= epoint {
            return Some(i)
        }
    }

    None
}

fn choose_color<'a>(escape_number: Option<usize>) -> Color {
    match escape_number {
        Some(i) => {
            let frequency = 0.3;
            let phase_r   = 0.0;
            let phase_g   = 2.0;
            let phase_b   = 4.0;
            let width     = 127.0;
            let center    = 128.0;

            Color {
                r: ((frequency * i as f32 + phase_r).sin() * width + center) as u8,
                g: ((frequency * i as f32 + phase_g).sin() * width + center) as u8,
                b: ((frequency * i as f32 + phase_b).sin() * width + center) as u8
            }
        },

        None => Color { r: 0, g: 0, b: 0 }
    }
}

fn main () {
    let xdim: u32 = 1400;
    let ydim: u32 = 800;

    let imgbuf = image::ImageBuffer::from_fn(xdim, ydim, |x, y| {
        let complex       = complex_from_point(x, y, xdim, ydim);
        let escape_number = mandelbrot_escape_number(complex, None);
        let color         = choose_color(escape_number);

        color.to_pixel()
    });

    let ref mut fout = File::create(&Path::new("mandelbrot.png")).unwrap();
    let _ = image::ImageRgba8(imgbuf).save(fout, image::PNG).unwrap();
}
