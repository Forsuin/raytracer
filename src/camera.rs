use crate::hittable::{Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub width: i32,
    height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            width: 100,
            ..Default::default()
        }
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        // Render

        println!("P3\n{} {}\n255", self.width, self.height);

        for y in 0..self.height {
            // eprintln!("\rScanlines Remaining: {}", y - 1);
            for x in 0..self.width {
                let pixel_center = self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);


                let pixel_color = self.ray_color(&ray, &world);

                write_color(&pixel_color);
            }
        }

        // eprintln!("\rDone!");
    }

    fn initialize(&mut self) {
        let height = (self.width as f64 / self.aspect_ratio) as i32;
        self.height = if height < 1 { 1 } else { height };

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

    fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        if let Some(hit) = world.hit(ray, Interval::new(0., f64::INFINITY)) {
            return 0.5 * (hit.normal + Color::new(1., 1., 1.));
        }

        // sky color

        let unit_direction = ray.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}

fn write_color(color: &Color) {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let r = (255.999 * r) as u32;
    let g = (255.999 * g) as u32;
    let b = (255.999 * b) as u32;

    println!("{r} {g} {b}");
}