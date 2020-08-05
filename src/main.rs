use ray_tracing_001::camera::Camera;
use ray_tracing_001::color::Color;
use ray_tracing_001::hitable::{HitableList, Sphere};
use ray_tracing_001::misc::{random, random_in_unit_sphere};
use ray_tracing_001::ray::Ray;
use ray_tracing_001::vec3::Vec3;

fn main() {
    let width = 200usize;
    let height = 100usize;

    let sampling_num = 100usize;

    let hitables = vec![
        Box::new(Sphere {
            center: Vec3::from_i(0, 0, -1),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];
    let world = HitableList::new(hitables);
    let camera = Camera::new();

    println!("P3\n{} {}\n255\n", width, height);
    for y in (0..height).rev() {
        for x in 0..width {
            // samping for anti-aliasing
            let mut temp_sum_color = Color { r: 0, g: 0, b: 0 };
            for _ in 0..sampling_num {
                let v = ((y as f32) + random()) / (height as f32);
                let u = ((x as f32) + random()) / (width as f32);
                let ray = &camera.get_ray(u, v);
                temp_sum_color = temp_sum_color + color(&ray, &world);
            }
            let color = (temp_sum_color / (sampling_num as f32)).hoge_gamma();
            println!("{} {} {}", color.r, color.g, color.b);
        }
    }
}

fn color(r: &Ray, world: &HitableList) -> Color {
    let rec = world.hit(r, 0.0001, f32::MAX);
    if rec.is_some() {
        let record = rec.unwrap();
        let target = record.p + record.normal + random_in_unit_sphere();
        return color(&Ray::new(record.p, target - record.p), world) * 0.5;
    }

    let ud = r.direction().unit_vector();
    let t = 0.5 * (ud.y + 1.0);
    ((1.0 - t) * Vec3::from_i(1, 1, 1) + t * Vec3::new(0.5, 0.7, 1.0)).as_color()
}

#[cfg(test)]
#[test]
fn unit_vector() {
    let v = Vec3::from_i(2, -2, 0);
    let uv = v.unit_vector();
    assert_eq!(
        uv,
        Vec3::new(
            (2.0 / (2.0 * 2.0 + 2.0 * 2.0 * 0.0 * 0.0) as f32).sqrt(),
            -(2.0 / (2.0 * 2.0 + 2.0 * 2.0 * 0.0 * 0.0) as f32).sqrt(),
            (0.0 / (2.0 * 2.0 + 2.0 * 2.0 * 0.0 * 0.0) as f32).sqrt()
        )
    );
}

#[cfg(test)]
#[test]
fn random_test() {
    assert!(random() > 0.0);
    assert!(random() < 1.0);
    assert_ne!(random(), random());
}
