use image::{ImageBuffer, Rgba};
use num_complex::Complex;

use color::Color;
use options::Options;

pub fn generate_image(options: &Options) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let ref window = options.window;

    ImageBuffer::from_fn(window.p_width, window.p_height, |x, y| {
        let complex = window.complex_from_point(x, y);
        let escape_number = escape_number(
            complex,
            Some(8),
            options.iterations
        );
        let color = choose_color(escape_number);

        color.to_pixel()
    })
}

fn is_cardioid_member(point: &Complex<f32>) -> bool {
    let q     = (point.re - 0.25).powf(2.0) + point.im.powf(2.0);
    let left  = q * (q + point.re - 0.25);
    let right = 0.25 * point.im.powf(2.0);

    left < right
}

fn is_bulb_member(point: &Complex<f32>) -> bool {
    let term1 = (point.re + 1.0).powf(2.0);
    let term2 = point.im.powf(2.0);

    term1 + term2 < 0.0625
}

fn escape_number(
    complex_point: Complex<f32>,
    escape_point:  Option<u32>,
    iterations:    usize
) -> Option<f32> {
    let mut z: Complex<f32> = Complex { re: 0.0, im: 0.0 };

    if is_cardioid_member(&complex_point) || is_bulb_member(&complex_point) {
        return None;
    }

    let epoint = match escape_point {
        Some(value) => value as f32,
        None        => 2.0
    };

    for i in 0..iterations {
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
