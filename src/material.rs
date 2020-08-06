use crate::color::Color;
use crate::hitable::HitRecord;
use crate::misc::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(self: &Self, r_in: &Ray, _record: &HitRecord) -> (bool, Color, Ray);
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(self: &Self, _r_in: &Ray, record: &HitRecord) -> (bool, Color, Ray) {
        let target = record.p + record.normal + random_in_unit_sphere();
        let scatterd = Ray::new(record.p, target - record.p);
        let attenuation = self.albedo;
        (true, attenuation, scatterd)
    }
}

pub struct Metal {
    albedo: Color,
    fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f32) -> Metal {
        Metal {
            albedo,
            fuzziness: match fuzziness {
                _ if fuzziness < 0.0 => 0.0,
                _ if fuzziness > 1.0 => 1.0,
                _ => fuzziness,
            },
        }
    }
}

impl Material for Metal {
    fn scatter(self: &Self, r_in: &Ray, record: &HitRecord) -> (bool, Color, Ray) {
        let reflected = reflect(&r_in.direction().unit_vector(), &record.normal);
        let scattered = Ray::new(
            record.p,
            reflected + self.fuzziness * random_in_unit_sphere(),
        );
        let attenuation = self.albedo;
        (
            scattered.direction().dot(&record.normal) > 0.0,
            attenuation,
            scattered,
        )
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2.0 * n * v.dot(n))
}
