use crate::hitable::HitableList;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::ops;

pub fn color(r: &Ray, world: &HitableList) -> Color {
    let (is_hit, rec) = world.hit(r, 0.0, f32::MAX);
    if is_hit {
        return (0.5
            * Vec3::new(
                rec.normal.borrow().x + 1.0,
                rec.normal.borrow().y + 1.0,
                rec.normal.borrow().z + 1.0,
            ))
        .as_color();
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
