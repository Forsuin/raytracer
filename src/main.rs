use raytracer::camera::Camera;
use raytracer::hittable::HittableList;
use raytracer::material::{Dialetric, Lambertian, Metal};
use raytracer::sphere::Sphere;
use raytracer::vec3::*;
use std::rc::Rc;

fn main() {
    // World

    let mut world = HittableList { objects: vec![] };

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dialetric::new(1.50));
    let material_bubble = Rc::new(Dialetric::new(1.00 / 1.50 ));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));


    world.add(Sphere::new(Vec3::new(0., -100.5, -1.), 100., material_ground));
    world.add(Sphere::new(Vec3::new(0., 0., -1.2), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1., 0., -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, material_bubble));
    world.add(Sphere::new(Vec3::new(1., 0., -0.7), 0.5, material_right));

    //
    // let R = (PI / 4.0).cos();
    // let material_left = Rc::new(Lambertian::new(Color::new(0., 0., 1.)));
    // let material_right = Rc::new(Lambertian::new(Color::new(1., 0., 0.)));
    //
    // world.add(Sphere::new(Vec3::new(-R, 0., -1.), R, material_left));
    // world.add(Sphere::new(Vec3::new(R, 0., -1.), R, material_right));


    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.width = 1920;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Vec3::new(-2., 2., 1.);
    camera.lookat = Vec3::new(0., 0., -1.);
    camera.vup = Vec3::new(0., 1., 0.);

    camera.defocus_angle = 10.0;
    camera.focus_distance = 3.4;

    camera.render(&world)
}