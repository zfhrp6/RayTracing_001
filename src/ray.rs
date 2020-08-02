use crate::vec3::Vec3;

pub struct Ray {
    o: Vec3,
    d: Vec3,
}

impl<'a> Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray { o, d }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.o
    }

    pub fn direction(&self) -> &Vec3 {
        &self.d
    }

    pub fn point_at_parameter(&self, f: f32) -> Vec3 {
        self.o + f * self.d
    }
}
