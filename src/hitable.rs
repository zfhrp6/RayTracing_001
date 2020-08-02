use crate::ray::Ray;
use crate::vec3::Vec3;

struct hit_record {
    t: f32,
    p: Vec3,
    normal: Vec3,
}
struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32, record: &mut hit_record) -> bool {
        let center_vector = r.origin() - &self.center;
        let a = r.direction().dot(r.direction());
        let b = center_vector.dot(r.direction());
        let c = center_vector.dot(&center_vector) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = r.point_at_parameter(record.t);
                record.normal = (&record.p - &self.center) / self.radius;
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if t_min < temp && temp < t_max {
                record.t = temp;
                record.p = r.point_at_parameter(record.t);
                record.normal = (&record.p - &self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}
