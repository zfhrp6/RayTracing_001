use crate::ray::Ray;
use crate::vec3::Vec3;
use std::cell::RefCell;
use std::ops::Index;

#[derive(Clone)]
pub struct HitRecord {
    t: RefCell<f32>,
    p: RefCell<Vec3>,
    pub normal: RefCell<Vec3>,
}

impl HitRecord {
    pub fn null() -> HitRecord {
        HitRecord {
            t: RefCell::new(0.0),
            p: RefCell::new(Vec3::from_i(0, 0, 0)),
            normal: RefCell::new(Vec3::from_i(0, 0, 0)),
        }
    }
}

pub trait Hitable {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord);
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hitable for Sphere {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let center_vector = r.origin() - &self.center;
        let a = r.direction().dot(r.direction());
        let b = center_vector.dot(r.direction());
        let c = center_vector.dot(&center_vector) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                (
                    true,
                    HitRecord {
                        t: RefCell::new(temp),
                        p: RefCell::new(p),
                        normal: RefCell::new((p - &self.center) / self.radius),
                    },
                );
            }
            temp = (-b + discriminant.sqrt()) / a;
            if t_min < temp && temp < t_max {
                let p = r.point_at_parameter(temp);
                (
                    true,
                    HitRecord {
                        t: RefCell::new(temp),
                        p: RefCell::new(p),
                        normal: RefCell::new((p - &self.center) / self.radius),
                    },
                );
            }
        }
        return (false, HitRecord::null());
    }
}

pub struct HitableList {
    pub list: RefCell<Vec<Box<Sphere>>>,
    pub size: isize,
}

impl HitableList {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let temp_record: HitRecord = HitRecord::null();

        for idx in 0..self.size {
            let (is_hit, record) =
                self.list
                    .borrow()
                    .as_slice()
                    .index(idx as usize)
                    .hit(r, t_min, closest_so_far);
            //        for hittable in self.list.borrow().as_slice().iter() {
            if is_hit {
                hit_anything = true;
                closest_so_far = *record.t.borrow();
            }
        }
        (hit_anything, temp_record.clone())
    }
}
