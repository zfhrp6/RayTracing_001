use ray_tracing_001::camera::Camera;
use ray_tracing_001::color::Color;
use ray_tracing_001::hitable::{Hitable, HitableList, Sphere};
use ray_tracing_001::material::{Dielectric, Lambertian, Metal};
use ray_tracing_001::misc::random;
use ray_tracing_001::ray::Ray;
use ray_tracing_001::vec3::Vec3;
use std::rc::Rc;

fn main() {
    let width = 200usize;
    let height = 100usize;

    let sampling_num = 100usize;

    let hitables: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            center: Vec3::from_i(0, 0, -1),
            radius: 0.5,
            material: Rc::new(Box::new(Lambertian::new(Color::from_f(0.1, 0.2, 0.5)))),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Rc::new(Box::new(Lambertian::new(Color::from_f(0.8, 0.8, 0.0)))),
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Rc::new(Box::new(Metal::new(Color::from_f(0.8, 0.6, 0.2), 0.0))),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Rc::new(Box::new(Dielectric { ref_idx: 1.5 })),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: -0.45,
            material: Rc::new(Box::new(Dielectric { ref_idx: 1.5 })),
        }),
    ];
    let world = HitableList::new(hitables);
    let camera = Camera::new(
        Vec3::from_i(-2, 2, 1),
        Vec3::from_i(0, 0, -1),
        Vec3::from_i(0, 1, 0),
        90.0,
        (width as f32) / (height as f32),
    );

    println!("P3\n{} {}\n255\n", width, height);
    for y in (0..height).rev() {
        for x in 0..width {
            // samping for anti-aliasing
            let mut temp_sum_color = Color { r: 0, g: 0, b: 0 };
            for _ in 0..sampling_num {
                let v = ((y as f32) + random()) / (height as f32);
                let u = ((x as f32) + random()) / (width as f32);
                let ray = &camera.get_ray(u, v);
                temp_sum_color = temp_sum_color + color(&ray, &world, 0);
            }
            let color = (temp_sum_color / (sampling_num as f32)).hoge_gamma();
            println!("{} {} {}", color.r, color.g, color.b);
        }
    }
}

fn color(r: &Ray, world: &HitableList, depth: isize) -> Color {
    let rec = world.hit(r, 0.0001, f32::MAX);

    // object
    if rec.is_some() {
        let temp_record = rec.unwrap();
        let material = &temp_record.material;
        let (is_scattered, attenuation, scattered) = material.scatter(r, &temp_record);
        if depth < 50 && is_scattered {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            return Color::new(0, 0, 0);
        }
    }

    // background
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

#[test]
fn as_color_test() {
    let col = Vec3::new(1.0, 0.0, 0.143).as_color();
    assert_eq!(
        col,
        Color {
            r: 255,
            g: 0,
            b: (0.143 * 255.99) as usize
        }
    );
}

#[test]
fn as_vec3_test() {
    let vec = Color {
        r: 0,
        g: 111,
        b: 255,
    }
    .as_vec3();
    assert_eq!(vec, Vec3::new(0.0, 111.0 / 255.99, 255.0 / 255.99));
}

#[test]
fn random_test() {
    assert!(random() > 0.0);
    assert!(random() < 1.0);
    assert_ne!(random(), random());
}
