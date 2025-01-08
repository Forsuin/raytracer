use crate::hittable::{Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use rand::Rng;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            ..Default::default()
        }
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        // Render

        println!("P3\n{} {}\n255", self.width, self.height);

        for y in 0..self.height {
            if y % 10 == 0 {
                eprintln!("\rScanlines Remaining: {}", (self.height - y));
            }
            for x in 0..self.width {
                let mut pixel_color = Color::ZERO;

                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color += self.ray_color(&ray, self.max_depth, world);
                }

                write_color(&(self.pixel_samples_scale * pixel_color));
            }
        }

        // eprintln!("\rDone!");
    }

    fn initialize(&mut self) {
        let height = (self.width as f64 / self.aspect_ratio) as i32;
        self.height = if height < 1 { 1 } else { height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Vec3::ZERO;
        let focal_length = 1.0;

        // Determine viewport dimensions
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.width as f64 / self.height as f64);

        // calculate vectors across horizontal and vertical viewport edges

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // calculate horizontal and vertical delta between pixels

        self.pixel_delta_u = viewport_u / self.width as f64;
        self.pixel_delta_v = viewport_v / height as f64;

        // find location of upper left pixel

        let viewport_upper_left = self.center - Vec3::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::ZERO;
        }

        if let Some(hit) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            return if let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) {
                attenuation * self.ray_color(&scattered, depth - 1, world)
            } else {
                Color::ZERO
            }
        }

        // sky color

        let unit_direction = ray.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, x: i32, y: i32) -> Ray {
        // Construct a camera ray originating from the orign and directed at randomly sampled
        // point around the pixel location (x, y)

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset.x) * self.pixel_delta_u)
            + ((y as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-0.5, -0.5] - [0.5, 0.5] unit square
        let mut rng = rand::thread_rng();

        Vec3::new(rng.gen_range(0.0..=1.0) - 0.5,
                  rng.gen_range(0.0..=1.0) - 0.5,
                  0.)
    }
}

fn write_color(color: &Color) {
    let r = color.x;
    let g = color.y;
    let b = color.z;

    // Apply linear to gamma transform for gama 2
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    // Translate the [0, 1] component values to the range [0, 255]
    let intensity = Interval::new(0., 0.999);
    let r = (256. * intensity.clamp(r)) as u32;
    let g = (256. * intensity.clamp(g)) as u32;
    let b = (256. * intensity.clamp(b)) as u32;

    println!("{r} {g} {b}");
}

#[inline]
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}