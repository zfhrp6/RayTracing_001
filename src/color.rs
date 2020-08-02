use crate::ray::Ray;
use crate::vec3::Vec3;
use std::ops;

pub fn color(r: &Ray) -> Color {
    let t = hit_sphere(&Vec3::from_i(0, 0, -1), 0.5, r);
    if t > 0.0 {
        let n = (&(r.point_at_parameter(t)) - &Vec3::from_i(0, 0, -1)).unit_vector();
        return (0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)).as_color();
    }

    let ud = r.direction().unit_vector();
    let t = 0.5 * (ud.y + 1.0);
    ((1.0 - t) * Vec3::from_i(1, 1, 1) + t * Vec3::new(0.5, 0.7, 1.0)).as_color()
}

pub struct Color {
    pub r: isize,
    pub g: isize,
    pub b: isize,
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
pub fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
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
