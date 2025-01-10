use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use enum_iterator::{all, Sequence};

#[derive(Debug, PartialEq, Copy, Clone, Sequence)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn from_points(a: Vec3, b: Vec3) -> Self {
        Self {
            x: if a.x <= b.x { Interval::new(a.x, b.x) } else { Interval::new(b.x, a.x) },
            y: if a.y <= b.y { Interval::new(a.y, b.y) } else { Interval::new(b.y, a.y) },
            z: if a.z <= b.z { Interval::new(a.z, b.z) } else { Interval::new(b.z, a.z) },
        }
    }

    pub fn axis_interval(&self, axis: Axis) -> Interval {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn hit(&self, ray: Ray, ray_t: &mut Interval) -> bool {
        let ray_orig = ray.origin;
        let ray_dir = ray.direction;

        for axis in all::<Axis>() {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir.get_axis(axis);

            let t0 = (ax.min - ray_orig.get_axis(axis)) * adinv;
            let t1 = (ax.max - ray_orig.get_axis(axis)) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min { ray_t.min = t0; }
                if t1 < ray_t.max { ray_t.max = t1; }
            } else {
                if t0 > ray_t.min { ray_t.min = t1; }
                if t1 < ray_t.max { ray_t.max = t0; }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}