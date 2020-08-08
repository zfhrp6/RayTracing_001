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
        fn conv(o: usize) -> usize {
            (((o as f64) / 255.99).sqrt() * 255.99) as usize
        }
        Color::new(conv(self.r), conv(self.g), conv(self.b))
    }

    pub fn as_vec3(&self) -> Vec3 {
        fn conv(u: usize) -> f32 {
            (u as f32) / 255.99
        }
        Vec3::new(conv(self.r), conv(self.g), conv(self.b))
    }

    pub fn new(r: usize, g: usize, b: usize) -> Color {
        Color { r, g, b }
    }
}

impl From<(usize, usize, usize)> for Color {
    fn from(v: (usize, usize, usize)) -> Color {
        Color::new(v.0, v.1, v.2)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(v: (f32, f32, f32)) -> Color {
        fn conv(f: f32) -> usize {
            (f * 255.99) as usize
        }
        Color::new(conv(v.0), conv(v.1), conv(v.2))
    }
}
impl From<&Vec3> for Color {
    fn from(vec: &Vec3) -> Color {
        vec.as_color()
    }
}

impl ops::Div<f32> for Color {
    type Output = Color;
    fn div(self: Color, div: f32) -> Color {
        fn div_inner(u: usize, div: f32) -> usize {
            ((u as f32) / div) as usize
        }
        Color::new(
            div_inner(self.r, div),
            div_inner(self.g, div),
            div_inner(self.b, div),
        )
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
        fn mul_inner(u: usize, div: f32) -> usize {
            ((u as f32) * div) as usize
        }
        Color::new(
            mul_inner(self.r, div),
            mul_inner(self.g, div),
            mul_inner(self.b, div),
        )
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
