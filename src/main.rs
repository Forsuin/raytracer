use rand::{random, thread_rng, Rng};
use raytracer::camera::Camera;
use raytracer::hittable::HittableList;
use raytracer::material::{Dialetric, Lambertian, Material, Metal};
use raytracer::sphere::Sphere;
use raytracer::vec3::*;
use std::sync::Arc;

fn main() {
    // World

    let mut world = HittableList { objects: vec![] };

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.add(Sphere::new(Vec3::new(0., -1000., 0.), 1000., material_ground));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let center = Vec3::new(a as f64 + 0.9 * random::<f64>(), 0.2, b as f64 + 0.9 * random::<f64>());

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Send>;

                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                }
                else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = thread_rng().gen_range(0.0..=0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                }
                else {
                    sphere_material = Arc::new(Dialetric::new(1.5));
                }

                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let glass = Arc::new(Dialetric::new(1.5));
    let diffuse = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let metal = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Sphere::new(Vec3::new(0., 1., 0.), 1.0, glass));
    world.add(Sphere::new(Vec3::new(-4., 1., 0.), 1.0, diffuse));
    world.add(Sphere::new(Vec3::new(4., 1., 0.), 1.0, metal));


    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Vec3::new(13., 2., 3.);
    camera.lookat = Vec3::new(0., 0., 0.);
    camera.vup = Vec3::new(0., 1., 0.);

    camera.defocus_angle = 0.6;
    camera.focus_distance = 10.0;

    camera.render(&world);
}