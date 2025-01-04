use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

struct Sphere {
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
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
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
        
        if root <= tmin || tmax <= root {
            root = (h + sqrtd) / a;
            if (root <= tmin || tmax <= root) {
                return None
            }
        }
        
        Some(HitRecord {
            point: ray.at(root),
            normal: (ray.at(root) - self.center) / self.radius,
            t: root,
        })
    }
}