use rand::random;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let mut direction = hit.normal + random_unit_vector();

        // catch degenerate scatter direction
        if direction.near_zero() {
            direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, direction);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&ray.direction, &hit.normal);
        let reflected = reflected.unit_vector() + (self.fuzz * random_unit_vector());
        let scattered = Ray::new(hit.point, reflected);
        let attenuation = self.albedo;

        if scattered.direction.dot(&hit.normal) < 0. {
            return None;
        }

        Some((scattered, attenuation))
    }
}

pub struct Dialetric {
    refraction_index: f64,
}

impl Dialetric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index,
        }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dialetric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::ONE;
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.unit_vector();
        let cos_theta = f64::min((-unit_direction).dot(&hit.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_reflect = ri * sin_theta > 1.0;
        let direction: Vec3;

        if (cannot_reflect || Self::reflectance(cos_theta, ri) > random::<f64>()) {
            direction = reflect(&unit_direction, &hit.normal);
        }
        else {
            direction = refract(&unit_direction, &hit.normal, ri);
        }


        let scattered = Ray::new(hit.point, direction);

        Some((scattered, attenuation))
    }
}


fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1., 1.);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        // in same hemisphere as normal
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}


fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(-uv.dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::abs(1.0 - r_out_perp.length_squared()).sqrt() * n;
    r_out_perp + r_out_parallel
}