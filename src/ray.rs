extern crate cgmath;

use cgmath::Vector3;

type Vec3 = Vector3<f64>;

#[derive(Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn empty() -> Ray {
        Ray::new(Vec3::new(0f64, 0f64, 0f64), Vec3::new(0f64, 0f64, 0f64))
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}