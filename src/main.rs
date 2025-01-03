use raytracer::ray::Ray;
use raytracer::vec3::*;

type Color = Vec3;

fn main() {
    const ASPECT_RATIO: f64 = 16. / 9.;
    const WIDTH: i32 = 400;

    // calculate image height
    let height = (WIDTH as f64 / ASPECT_RATIO) as i32;
    let height = if height < 1 { 1 } else { height };


    // camera

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (WIDTH as f64 / height as f64);
    let focal_length = 1.0;
    let camera_center = Vec3::ZERO;


    // calculate vectors across horizontal and vertical viewport edges

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // calculate horizontal and vertical delta between pixels

    let pixel_delta_u = viewport_u / WIDTH as f64;
    let pixel_delta_v = viewport_v / height as f64;

    // find location of upper left pixel

    let viewport_upper_left = camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    // Render

    println!("P3\n{} {}\n255", WIDTH, height);

    for y in 0..height {
        // eprintln!("\rScanlines Remaining: {}", y - 1);
        for x in 0..WIDTH {
            let pixel_center = pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);


            let pixel_color = ray_color(&ray);

            write_color(&pixel_color);
        }
    }

    // eprintln!("\rDone!");
}

pub fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = center - ray.origin;
    let a = ray.direction.dot(&ray.direction);
    let b = -2.0 * ray.direction.dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    discriminant >= 0.
}

pub fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(Vec3::new(0., 0., -1.), 0.5, ray) {
        return Color::new(1., 0., 0.);
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
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