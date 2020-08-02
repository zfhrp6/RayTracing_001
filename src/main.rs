use std::ops;

fn main() {
    let width = 200;
    let height = 100;

    let lower_left_corner = Vec3::from_i(-2, -1, -1);
    let horizontal = Vec3::from_i(4, 0, 0);
    let vertical = Vec3::from_i(0, 2, 0);
    let origin = Vec3::from_i(0, 0, 0);

    println!("P3\n{} {}\n255\n", width, height);
    for y in (0..height).rev() {
        for x in 0..width {
            let v = (y as f32) / (height as f32);
            let u = (x as f32) / (width as f32);
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let color = color(&ray);
            print!("{} {} {}\n", color.r, color.g, color.b);
        }
    }
}

struct Color {
    r: isize,
    g: isize,
    b: isize,
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
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<'a> Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn from_i(x: isize, y: isize, z: isize) -> Vec3 {
        Vec3 {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }

    fn inverse(self: Vec3) -> Vec3 {
        Vec3::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z)
    }

    fn dot(self: &Vec3, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(self: &Vec3, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
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
            r: ((255.99 * self.x) as isize),
            g: ((255.99 * self.y) as isize),
            b: ((255.99 * self.z) as isize),
        }
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
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
        Vec3::new(self.x * f, self.y * f, self.z * f)
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
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
    let center_vector = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * center_vector.dot(r.direction());
    let c = center_vector.dot(&center_vector) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

struct Ray {
    o: Vec3,
    d: Vec3,
}

impl<'a> Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray { o, d }
    }

    fn origin(&self) -> &Vec3 {
        &self.o
    }

    fn direction(&self) -> &Vec3 {
        &self.d
    }

    fn point_at_parameter(&self, f: f32) -> Vec3 {
        self.o + f * self.d
    }
}

fn color(r: &Ray) -> Color {
    let t = hit_sphere(&Vec3::from_i(0, 0, -1), 0.5, r);
    if t > 0.0 {
        let n = (&(r.point_at_parameter(t)) - &Vec3::from_i(0, 0, -1)).unit_vector();
        return (0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)).as_color();
    }

    let ud = r.direction().unit_vector();
    let t = 0.5 * (ud.y + 1.0);
    ((1.0 - t) * Vec3::from_i(1, 1, 1) + t * Vec3::new(0.5, 0.7, 1.0)).as_color()
}

#[cfg(test)]
#[test]
fn unit_vector() {
    let v = Vec3::from_i(2, -2, 0);
    let uv = v.unit_vector();
    assert_eq!(
        uv,
        Vec3::new(
            (2.0 / (2.0 * 2.0 + 2.0 * 2.0 * 0.0 * 0.0) as f32).sqrt(),
            -(2.0 / (2.0 * 2.0 + 2.0 * 2.0 * 0.0 * 0.0) as f32).sqrt(),
            (0.0 / (2.0 * 2.0 + 2.0 * 2.0 * 0.0 * 0.0) as f32).sqrt()
        )
    );
}
