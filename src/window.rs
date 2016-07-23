use num_complex::Complex;

pub struct Window {
    pub p_width:  u32,
    pub p_height: u32,
    pub origin:   Complex<f32>,
    pub zoom:     f32
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
