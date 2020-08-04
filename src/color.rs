use std::ops;

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
