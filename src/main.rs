use std::ops;

fn main() {
    let width = 200;
    let height = 100;

    let lower_left_corner = Vec3 {
        x: -2.0,
        y: -1.0,
        z: -1.0,
    };
    let horizontal = Vec3 {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };
    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    println!("P3\n{} {}\n255\n", width, height);
    for y in (0..height).rev() {
        for x in 0..width {
            let v = (y as f32) / (height as f32);
            let u = (x as f32) / (width as f32);
            let ray = Ray {
                o: origin,
                d: lower_left_corner + u * horizontal + v * vertical,
            };
            let color = color(&ray).as_color();
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

#[derive(Copy, Clone, PartialEq, Debug)]
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

impl<'a> Vec3 {
    fn inverse(self: Vec3) -> Vec3 {
        Vec3 {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
        }
    }

    fn length(self: &Vec3) -> f32 {
        self.squared_length().sqrt()
    }

    fn squared_length(self: &Vec3) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn unit_vector(self: &Vec3) -> Vec3 {
        self.clone() / self.length()
    }

    fn as_color(self: &Vec3) -> Color {
        Color {
            r: ((255.99 * self.x) as i32),
            g: ((255.99 * self.y) as i32),
            b: ((255.99 * self.z) as i32),
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

struct Ray {
    o: Vec3,
    d: Vec3,
}

impl<'a> Ray {
    #[allow(dead_code)]
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray { o, d }
    }
    #[allow(dead_code)]
    fn origin(&self) -> &Vec3 {
        &self.o
    }

    #[allow(dead_code)]
    fn direction(&self) -> &Vec3 {
        &self.d
    }

    #[allow(dead_code)]
    fn point_at_parameter(self, f: f32) -> Vec3 {
        self.o + f * self.d
    }
}

fn color(r: &Ray) -> Vec3 {
    let uv = r.direction().unit_vector();
    let t = 0.5 * (uv.y + 1.0);
    (1.0 - t)
        * Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + t * Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
}

#[cfg(test)]
#[test]
fn unit_vector() {
    let v = Vec3 {
        x: 2.0,
        y: -2.0,
        z: 0.0,
    };
    let uv = v.unit_vector();
    assert_eq!(
        uv,
        Vec3 {
            x: (2.0 / (2.0 * 2.0 + 2.0 * 2.0 * 0.0 * 0.0) as f32).sqrt(),
            y: -(2.0 / (2.0 * 2.0 + 2.0 * 2.0 * 0.0 * 0.0) as f32).sqrt(),
            z: (0.0 / (2.0 * 2.0 + 2.0 * 2.0 * 0.0 * 0.0) as f32).sqrt()
        }
    );
}
