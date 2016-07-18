extern crate image;
extern crate num_complex;
extern crate rand;

use std::fs::File;
use std::path::Path;

use num_complex::Complex;

struct Window {
    origin:   Complex<f32>,
    p_width:  u32,
    p_height: u32,
    zoom:     f32
}

impl Default for Window {
    fn default() -> Self {
        Window {
            origin:   Complex { re: -0.75f32, im: 0f32 },
            p_width:  2800,
            p_height: 2400,
            zoom:     1.0
        }
    }
}

impl Window {
    pub fn complex_from_point(&self, x: u32, y: u32) -> Complex<f32> {
        let re_proportion = x as f32 / self.p_width as f32;
        let im_proportion = y as f32 / self.p_height as f32;

        let real_size      = self.real_size();
        let imaginary_size = self.imaginary_size();

        Complex {
            re: re_proportion * real_size + (self.origin.re - (real_size / 2.0)),
            im: im_proportion * imaginary_size + (self.origin.im - (imaginary_size / 2.0))
        }
    }

    fn cartesian_to_pixel(&self) -> f32 {
        0.001275 / self.zoom
    }

    fn real_size(&self) -> f32 {
        self.p_width as f32 * self.cartesian_to_pixel()
    }

    fn imaginary_size(&self) -> f32 {
        self.p_height as f32 * self.cartesian_to_pixel()
    }
}

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

fn mandelbrot_escape_number(complex_point: Complex<f32>, escape_point: Option<u32>) -> Option<f32> {
    let mut z: Complex<f32> = Complex { re: 0.0, im: 0.0 };

    let epoint = match escape_point {
        Some(value) => value as f32,
        None        => 2.0
    };

    for i in 0..250 {
        z = z.powf(2.0) + complex_point;
        if z.norm() >= epoint {
            let log_zn = (z.im * z.im + z.re * z.re).log(10.0) / 2.0;
            let nu     = (log_zn / (2.0 as f32).log(10.0)).log(10.0) / (2.0 as f32).log(10.0);
            return Some(i as f32 + 1.0 - nu)
        }
    }

    None
}

fn choose_color<'a>(escape_number: Option<f32>) -> Color {
    match escape_number {
        Some(i) => {
            let frequency = 0.3;
            let phase_r   = 0.0;
            let phase_g   = 2.0;
            let phase_b   = 4.0;
            let width     = 127.0;
            let center    = 128.0;

            Color {
                r: ((frequency * i + phase_r).sin() * width + center) as u8,
                g: ((frequency * i + phase_g).sin() * width + center) as u8,
                b: ((frequency * i + phase_b).sin() * width + center) as u8
            }
        },

        None => Color { r: 0, g: 0, b: 0 }
    }
}

fn main () {
    let window = Window::default();

    let imgbuf = image::ImageBuffer::from_fn(window.p_width, window.p_height, |x, y| {
        let complex       = window.complex_from_point(x, y);
        let escape_number = mandelbrot_escape_number(complex, Some(8));
        let color         = choose_color(escape_number);

        color.to_pixel()
    });

    let ref mut fout = File::create(&Path::new("mandelbrot.png")).unwrap();
    let _ = image::ImageRgba8(imgbuf).save(fout, image::PNG).unwrap();
}
