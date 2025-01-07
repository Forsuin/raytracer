use raytracer::camera::Camera;
use raytracer::hittable::HittableList;
use raytracer::sphere::Sphere;
use raytracer::vec3::*;

fn main() {
    // World

    let mut world = HittableList { objects: vec![] };

    world.add(Sphere::new(Vec3::new(0., 0., -1.), 0.5));
    world.add(Sphere::new(Vec3::new(0., -100.5, -1.), 100.));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth= 40;

    camera.render(&world)
}