use std::ops;

fn main() {
    let width = 200;
    let height = 100;
    println!("P3\n{} {}\n255\n", width, height);
    for y in (0..height).rev() {
        for x in 0..width {
            let color = Color {
                r: ((x as f32) / (width as f32) * 255.99).round() as i32,
                g: ((y as f32) / (height as f32) * 255.99).round() as i32,
                b: (0.2f32 * 255.99).round() as i32,
            };
            print!("{} {} {}\n", color.r, color.g, color.b);
        }
    }
}

struct Color {
    r: i32,
    g: i32,
    b: i32,
}

impl ops::Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            r: (self.r + other.r) / 2,
            g: (self.g + other.g) / 2,
            b: (self.b + other.b) / 2,
        }
    }
}

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    fn inverse(self: Vec3) -> Vec3 {
        Vec3 {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        self * other.inverse()
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f32) -> Vec3 {
        Vec3 {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, f: f32) -> Vec3 {
        self * (1.0 / f)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[allow(dead_code)]
fn length(vec: Vec3) -> f32 {
    squared_length(vec).sqrt()
}

#[allow(dead_code)]
fn squared_length(vec: Vec3) -> f32 {
    vec.x * vec.x + vec.y * vec.y + vec.z + vec.z
}
