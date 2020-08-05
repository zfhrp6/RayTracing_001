use crate::vec3::Vec3;
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random(), random(), random()) - Vec3::from_i(1, 1, 1);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn random() -> f32 {
    rand::random::<f32>()
}
