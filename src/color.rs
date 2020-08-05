use crate::vec3::Vec3;
use std::ops;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Color {
    pub r: usize,
    pub g: usize,
    pub b: usize,
}

impl Color {
    pub fn hoge_gamma(&self) -> Color {
        Color {
            r: (((self.r as f64) / 255.99).sqrt() * 255.99) as usize,
            g: (((self.g as f64) / 255.99).sqrt() * 255.99) as usize,
            b: (((self.b as f64) / 255.99).sqrt() * 255.99) as usize,
        }
    }

    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(
            (self.r as f32) / 255.99,
            (self.g as f32) / 255.99,
            (self.b as f32) / 255.99,
        )
    }

    pub fn new(r: usize, g: usize, b: usize) -> Color {
        Color { r, g, b }
    }

    pub fn from_f(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: (r * 255.99) as usize,
            g: (g * 255.99) as usize,
            b: (b * 255.99) as usize,
        }
    }
}

impl ops::Div<f32> for Color {
    type Output = Color;
    fn div(self: Color, div: f32) -> Color {
        Color {
            r: ((self.r as f32) / div) as usize,
            g: ((self.g as f32) / div) as usize,
            b: ((self.b as f32) / div) as usize,
        }
    }
}

impl ops::Mul for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        (self.as_vec3() * other.as_vec3()).as_color()
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self: Color, div: f32) -> Color {
        Color {
            r: ((self.r as f32) * div) as usize,
            g: ((self.g as f32) * div) as usize,
            b: ((self.b as f32) * div) as usize,
        }
    }
}
impl ops::Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}
