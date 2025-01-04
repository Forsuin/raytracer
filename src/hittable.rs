use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_vector: Vec3) {
        // outward_vector assumed to have unit length

        self.front_face = ray.direction.dot(&outward_vector) < 0.;
        self.normal = if self.front_face {
            outward_vector
        }
        else {
            -outward_vector
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}