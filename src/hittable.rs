use crate::ray::Ray;
use crate::vec3::Vec3;

pub(crate) struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}