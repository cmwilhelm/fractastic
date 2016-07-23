use image;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_pixel(&self) -> image::Rgba<u8> {
        image::Rgba([self.r as u8, self.g as u8, self.b as u8, 255])
    }
}
