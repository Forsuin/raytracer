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

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut closest_so_far = tmax;
        let mut hit_anything = None;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, tmin, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }
}

