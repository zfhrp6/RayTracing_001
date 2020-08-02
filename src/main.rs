use ray_tracing_001::color::color;
use ray_tracing_001::ray::Ray;
use ray_tracing_001::vec3::Vec3;
fn main() {
    let width = 200;
    let height = 100;

    let lower_left_corner = Vec3::from_i(-2, -1, -1);
    let horizontal = Vec3::from_i(4, 0, 0);
    let vertical = Vec3::from_i(0, 2, 0);
    let origin = Vec3::from_i(0, 0, 0);

    println!("P3\n{} {}\n255\n", width, height);
    for y in (0..height).rev() {
        for x in 0..width {
            let v = (y as f32) / (height as f32);
            let u = (x as f32) / (width as f32);
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let color = color(&ray);
            print!("{} {} {}\n", color.r, color.g, color.b);
        }
    }
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
