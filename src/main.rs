fn main() {
    let width = 200;
    let height = 100;
    println!("P3\n{} {}\n255\n", width, height);
    for y in (0..height).rev() {
        for x in 0..width {
            let r = ((x as f32) / (width as f32) * 255.99).round() as i32;
            let g = ((y as f32) / (height as f32) * 255.99).round() as i32;
            let b = (0.2f32 * 255.99).round() as i32;
            print!("{} {} {}\n", r, g, b);
        }
    }
}
