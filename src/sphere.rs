use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self {
            center,
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        
        let discriminant = h * h - a * c;
        
        if discriminant < 0. {
            return None
        }
        
        let sqrtd = discriminant.sqrt();
        
        // Find the nearest root that lies in acceptable range
        
        let mut root = (h - sqrtd) / a;
        
        if root <= ray_t.min || ray_t.max <= root {
            root = (h + sqrtd) / a;
            if root <= ray_t.min || ray_t.max <= root {
                return None
            }
        }
        
        let mut hr = HitRecord {
            point: ray.at(root),
            normal: Vec3::ZERO,
            t: root,
            front_face: false,
        };

        let outward_normal = (hr.point - self.center) / self.radius;
        hr.set_face_normal(ray, outward_normal);

        Some(hr)
    }
}