use crate::color::Color;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct HitRecord {
    t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<Box<dyn Material>>,
}

impl HitRecord {
    pub fn null() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::from_i(0, 0, 0),
            normal: Vec3::from_i(0, 0, 0),
            material: Rc::new(Box::new(Lambertian::new(Color::new(0, 0, 0)))),
        }
    }
}

pub trait Hitable {
    fn hit(self: &Self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<Box<dyn Material>>,
}

impl Sphere {
    fn hit_sphere(self: &Self, r: &Ray) -> (f32, f32, f32) {
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
    fn hit(self: &Self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (discriminant, negative_root, positive_root) = self.hit_sphere(r);
        if discriminant < 0.0
            || (!(t_min < negative_root && negative_root < t_max)
                && !(t_min < positive_root && positive_root < t_max))
        {
            return None;
        }

        let temp = if t_min < negative_root && negative_root < t_max {
            negative_root
        } else {
            positive_root
        };

        let p = r.point_at_parameter(temp);
        return Some(HitRecord {
            t: temp,
            p: p,
            normal: (p - &self.center) / self.radius,
            material: self.material.clone(),
        });
    }
}

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
    pub size: isize,
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        let len = list.len() as isize;
        HitableList { list, size: len }
    }
    pub fn hit(self: &Self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_record = HitRecord::null();

        for idx in self.list.iter() {
            let a = idx;
            let record = a.hit(r, t_min, closest_so_far);
            if record.is_some() {
                hit_anything = true;
                let tmp = record.unwrap();
                closest_so_far = tmp.t;
                temp_record = tmp;
            }
        }
        if hit_anything {
            Some(temp_record)
        } else {
            None
        }
    }
}
