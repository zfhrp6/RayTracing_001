use rand::rngs;
use ray_tracing_001::camera::Camera;
use ray_tracing_001::color::Color;
use ray_tracing_001::hitable::{Hitable, HitableList, Sphere};
use ray_tracing_001::material::{Dielectric, Lambertian, Material, Metal};
use ray_tracing_001::misc::{random, random_seed};
use ray_tracing_001::ray::Ray;
use ray_tracing_001::vec3::Vec3;
use std::rc::Rc;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 200usize;
    let height = ((width as f64) / aspect_ratio) as usize;

    let sampling_num = 100usize;

    let rng = &mut random_seed(153);

    let hitables = random_scene(rng);
    let world = HitableList::new(hitables);
    // settings of camera
    let look_from = (13, 2, 3).into();
    let look_at: Vec3 = (0, 0, 0).into();
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        (0, 1, 0).into(),
        20.0,
        (width as f32) / (height as f32),
        aperture,
        10.0,
    );

    println!("P3\n{} {}\n255\n", width, height);
    for y in (0..height).rev() {
        for x in 0..width {
            // samping for anti-aliasing
            let mut temp_sum_color = Color { r: 0, g: 0, b: 0 };
            for _ in 0..sampling_num {
                let v = ((y as f32) + random(rng)) / (height as f32);
                let u = ((x as f32) + random(rng)) / (width as f32);
                let ray = &camera.get_ray(u, v, rng);
                temp_sum_color = temp_sum_color + color(&ray, &world, 0, rng);
            }
            let color = (temp_sum_color / (sampling_num as f32)).hoge_gamma();
            println!("{} {} {}", color.r, color.g, color.b);
        }
    }
}

fn color(r: &Ray, world: &HitableList, depth: isize, rng: &mut rngs::StdRng) -> Color {
    let rec = world.hit(r, 0.0001, f32::MAX);

    // object
    if rec.is_some() {
        let temp_record = rec.unwrap();
        let material = &temp_record.material;
        let (is_scattered, attenuation, scattered) = material.scatter(r, &temp_record, rng);
        if depth < 50 && is_scattered {
            return attenuation * color(&scattered, world, depth + 1, rng);
        } else {
            return Color::new(0, 0, 0);
        }
    }

    // background
    let ud = r.direction().unit_vector();
    let t = 0.5 * (ud.y + 1.0);
    ((1.0 - t) * Vec3::from_i(1, 1, 1) + t * Vec3::new(0.5, 0.7, 1.0)).as_color()
}

fn random_scene(rng: &mut rngs::StdRng) -> Vec<Box<dyn Hitable>> {
    enum Materials {
        Lambertian,
        Metal,
        Dielectric,
    };
    fn choose_random_material(rng: &mut rngs::StdRng) -> Materials {
        let r = random(rng);
        match r {
            _ if r < 0.8 => Materials::Lambertian,
            _ if r < 0.95 => Materials::Metal,
            _ => Materials::Dielectric,
        }
    }
    fn create_object(center: Vec3, material: Rc<Box<dyn Material>>) -> Box<dyn Hitable> {
        Box::new(Sphere {
            center,
            radius: 0.2,
            material,
        })
    }
    fn random_material(en: Materials, rng: &mut rngs::StdRng) -> Rc<Box<dyn Material>> {
        match en {
            Materials::Lambertian => Rc::new(Box::new(Lambertian::new(
                (
                    random(rng) * random(rng),
                    random(rng) * random(rng),
                    random(rng) * random(rng),
                )
                    .into(),
            ))),
            Materials::Metal => Rc::new(Box::new(Metal::new(
                (
                    0.5 * (1.0 + random(rng)),
                    0.5 * (1.0 + random(rng)),
                    0.5 * (1.0 + random(rng)),
                )
                    .into(),
                0.5 * random(rng),
            ))),
            Materials::Dielectric => Rc::new(Box::new(Dielectric { ref_idx: 1.5 })),
        }
    }
    let mut objects: Vec<Box<dyn Hitable>> = vec![];
    for cx in -11..11 {
        for cz in -11..11 {
            let (cxf, czf) = (cx as f32, cz as f32);
            let center = Vec3::new(cxf + 0.9 * random(rng), 0.2, czf + 0.9 * random(rng));
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                objects.push(create_object(
                    center,
                    random_material(choose_random_material(rng), rng),
                ));
            }
        }
    }
    let ground_sphere = Box::new(Sphere {
        center: (0, -1000, 0).into(),
        radius: 1000.0,
        material: Rc::new(Box::new(Lambertian::new((0.5, 0.5, 0.5).into()))),
    });
    let defined_sphere1 = Box::new(Sphere {
        center: (0, 1, 0).into(),
        radius: 1.0,
        material: Rc::new(Box::new(Dielectric { ref_idx: 1.5 })),
    });
    let defined_sphere2 = Box::new(Sphere {
        center: (-4, 1, 0).into(),
        radius: 1.0,
        material: Rc::new(Box::new(Lambertian::new((0.4, 0.2, 0.1).into()))),
    });
    let defined_sphere3 = Box::new(Sphere {
        center: (4, 1, 0).into(),
        radius: 1.0,
        material: Rc::new(Box::new(Metal::new((0.7, 0.6, 0.5).into(), 0.0))),
    });
    objects.push(ground_sphere);
    objects.push(defined_sphere1);
    objects.push(defined_sphere2);
    objects.push(defined_sphere3);
    objects
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
