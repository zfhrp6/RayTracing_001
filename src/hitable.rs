use crate::ray::Ray;
use crate::vec3::Vec3;
use std::cell::RefCell;
use std::ops::Index;

#[derive(Clone)]
pub struct HitRecord {
    t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn null() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::from_i(0, 0, 0),
            normal: Vec3::from_i(0, 0, 0),
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

impl Sphere {
    fn hit_sphere(self, r: &Ray) -> (f32, f32, f32) {
        let center_vector = r.origin() - &self.center;
        let a = r.direction().dot(r.direction());
        let b = center_vector.dot(r.direction());
        let c = center_vector.dot(&center_vector) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        (
            discriminant,
            (-b - discriminant.sqrt()) / a,
            (-b + discriminant.sqrt()) / a,
        )
    }
}

impl Hitable for Sphere {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let (discriminant, negative_root, positive_root) = self.hit_sphere(r);
        if discriminant < 0.0
            || (!(t_min < negative_root && negative_root < t_max)
                && !(t_min < positive_root && positive_root < t_max))
        {
            return (false, HitRecord::null());
        }

        let temp = if t_min < negative_root && negative_root < t_max {
            negative_root
        } else {
            positive_root
        };

        let p = r.point_at_parameter(temp);
        return (
            true,
            HitRecord {
                t: temp,
                p: p,
                normal: (p - &self.center) / self.radius,
            },
        );
    }
}

pub struct HitableList {
    pub list: RefCell<Vec<Box<Sphere>>>,
    pub size: isize,
}

impl HitableList {
    pub fn new(list: Vec<Box<Sphere>>) -> HitableList {
        let len = list.len() as isize;
        HitableList {
            list: RefCell::new(list),
            size: len,
        }
    }
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_record = HitRecord::null();

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
                closest_so_far = record.t;
                temp_record = record.clone();
            }
        }
        (hit_anything, temp_record)
    }
}
