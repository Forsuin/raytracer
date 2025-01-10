use crate::hittable::{Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use rand::{random, Rng};
#[cfg(not(feature = "singlethread"))]
use rayon::prelude::*;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_distance: f64,

    height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,

    // Defocus disk horizontal/vertical radius
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Vec3::ZERO,
            lookat: Vec3::new(0., 0., -1.),
            vup: Vec3::new(0., 1., 0.),
            defocus_angle: 0.0,
            focus_distance: 10.0,
            ..Default::default()
        }
    }

    #[cfg(not(feature = "singlethread"))]
    fn render_band(&self, band: &mut [Vec3], world: &HittableList, y: i32) {
        for x in 0..self.width {
            let mut pixel_color = Color::ZERO;

            for _sample in 0..self.samples_per_pixel {
                let ray = self.get_ray(x, y);
                pixel_color += self.ray_color(&ray, self.max_depth, world);
            }

            band[x as usize] = self.pixel_samples_scale * pixel_color;
        }
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        // Render

        #[cfg(not(feature = "singlethread"))] {
            let mut pixels = vec![Vec3::ZERO; (self.width * self.height) as usize];
            let bands: Vec<(usize, &mut [Vec3])> = pixels.chunks_mut(self.width as usize).enumerate().collect();


            println!("P3\n{} {}\n255", self.width, self.height);


            bands.into_par_iter().for_each(|(i, band)| {
                self.render_band(band, world, i as i32);
            });

            for pixel in pixels {
                write_color(&pixel);
            }
        }


        #[cfg(feature = "singlethread")] {
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

            eprintln!("\rDone!");
        }
    }

    fn initialize(&mut self) {
        let height = (self.width as f64 / self.aspect_ratio) as i32;
        self.height = if height < 1 { 1 } else { height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        // Determine viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width = viewport_height * (self.width as f64 / self.height as f64);


        // Calculate u,v,w unit basis vectors for camera coordinate frame
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);


        // calculate vectors across horizontal and vertical viewport edges

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // calculate horizontal and vertical delta between pixels

        self.pixel_delta_u = viewport_u / self.width as f64;
        self.pixel_delta_v = viewport_v / height as f64;

        // find location of upper left pixel

        let viewport_upper_left = self.center - (self.focus_distance * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calcuate the camera defocus disk basis vectors

        let defocus_radius = self.focus_distance * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
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
            };
        }

        // sky color

        let unit_direction = ray.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, x: i32, y: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at randomly sampled
        // point around the pixel location (x, y)

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset.x) * self.pixel_delta_u)
            + ((y as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let point = random_in_unit_disk();
        self.center + (point.x * self.defocus_disk_u) + (point.y * self.defocus_disk_v)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-0.5, -0.5] - [0.5, 0.5] unit square
        let mut rng = rand::thread_rng();

        Vec3::new(rng.gen_range(0.0..=1.0) - 0.5,
                  rng.gen_range(0.0..=1.0) - 0.5,
                  0.)
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let point = Vec3::new(random(), random(), 0.0);
        if point.length_squared() < 1.0 {
            return point;
        }
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