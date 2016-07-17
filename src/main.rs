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

struct Palette {
    options:  Vec<Color>,
    backdrop: Color
}

impl Palette {
    fn new(size: usize) -> Self {
        let options = (0..size).into_iter().map(|i| {
            Color {
                r: rand::random::<u8>(),
                g: rand::random::<u8>(),
                b: rand::random::<u8>()
            }
        }).collect::<Vec<Color>>();

        Palette {
            options:  options,
            backdrop: Color { r: 0, g: 0, b: 0 }
        }
    }
}

fn complex_from_point(x: u32, y: u32, xmax: u32, ymax: u32) -> Complex<f32> {
    Complex {
        re: (x as f32 / xmax as f32) * 3.0 - 2.5,
        im: (y as f32 / ymax as f32) * 2.0 - 1.0
    }
}

fn mandelbrot_escape_number(complex_point: Complex<f32>) -> Option<usize> {
    let mut z: Complex<f32> = Complex { re: 0.0, im: 0.0 };

    for i in 0..250 {
        z = z.powf(2.0) + complex_point;
        if z.norm() >= 2.0 {
            return Some(i)
        }
    }

    None
}

fn choose_color<'a>(escape_number: Option<usize>, palette: &'a Palette) -> &'a Color {
    match escape_number {
        Some(i) => &palette.options[i as usize % palette.options.len()],
        None    => &palette.backdrop
    }
}

fn main () {
    let xdim: u32 = 5600;
    let ydim: u32 = 3200;

    let palette = Palette::new(15);

    let imgbuf = image::ImageBuffer::from_fn(xdim, ydim, |x, y| {
        let complex       = complex_from_point(x, y, xdim, ydim);
        let escape_number = mandelbrot_escape_number(complex);
        let color         = choose_color(escape_number, &palette);

        color.to_pixel()
    });

    let ref mut fout = File::create(&Path::new("mandelbrot.png")).unwrap();
    let _ = image::ImageRgba8(imgbuf).save(fout, image::PNG).unwrap();
}
