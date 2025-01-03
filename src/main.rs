use raytracer::vec3::*;

type Color = Vec3;

fn main() {
    const WIDTH: i32 = 256;
    const HEIGHT: i32 = 256;

    // Render

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        eprintln!("\rScanlines Remaining: {}", y - 1);
        for x in 0..WIDTH {
            let r = x as f64 / (WIDTH - 1) as f64;
            let g = y as f64 / (HEIGHT - 1) as  f64;
            let b = 0.0;

            let pixel_color = Color::new(r, g, b);

            write_color(&pixel_color);
        }
    }

    eprintln!("\rDone!");
}

pub fn write_color(color: &Color) {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let r = (255.999 * r) as u32;
    let g = (255.999 * g) as u32;
    let b = (255.999 * b) as u32;

    println!("{r} {g} {b}");
}