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

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {
    fn scatter(self: &Self, r_in: &Ray, record: &HitRecord) -> (bool, Color, Ray) {
        let reflected = reflect(r_in.direction(), &record.normal);
        let attenuation = Color::new(255, 255, 255);
        let (outward_normal, rri) = if r_in.direction().dot(&record.normal) > 0.0 {
            (-record.normal, self.ref_idx)
        } else {
            (record.normal, 1.0 / self.ref_idx)
        };
        let refracted = refract(r_in.direction(), &outward_normal, rri);
        if refracted.is_some() {
            (true, attenuation, Ray::new(record.p, refracted.unwrap()))
        } else {
            (true, attenuation, Ray::new(record.p, reflected))
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2.0 * n * v.dot(n))
}

fn refract(v: &Vec3, n: &Vec3, relative_refractive_index: f32) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant =
        1.0 - relative_refractive_index * relative_refractive_index * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(relative_refractive_index * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}
