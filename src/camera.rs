use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Camera {
    lower_left_coner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_coner: Vec3::from_i(-2, -1, -1),
            horizontal: Vec3::from_i(4, 0, 0),
            vertical: Vec3::from_i(0, 2, 0),
            origin: Vec3::from_i(0, 0, 0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_coner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
