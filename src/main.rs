use std::rc::Rc;
use raytracer::camera::Camera;
use raytracer::hittable::HittableList;
use raytracer::material::{Lambertian, Metal};
use raytracer::sphere::Sphere;
use raytracer::vec3::*;

fn main() {
    // World

    let mut world = HittableList { objects: vec![] };

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));


    world.add(Sphere::new(Vec3::new(0., -100.5, -1.), 100., material_ground));
    world.add(Sphere::new(Vec3::new(0., 0., -1.2), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1., 0., -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(1., 0., -0.7), 0.5, material_right));


    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.width = 600;
    camera.samples_per_pixel = 100;
    camera.max_depth= 40;

    camera.render(&world)
}