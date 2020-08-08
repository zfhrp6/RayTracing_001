use crate::misc::degree_to_radian;
use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Camera {
    origin: Vec3,
    lower_left_coner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        v_field_of_view: f32, // from top to bottom in degrees
        aspect: f32,
    ) -> Camera {
        let theta = degree_to_radian(v_field_of_view);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit_vector();
        let u = view_up.cross(&w).unit_vector();
        let v = w.cross(&u);
        Camera {
            lower_left_coner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.09 * half_height * v,
            origin: look_from,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_coner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
