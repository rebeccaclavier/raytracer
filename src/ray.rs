extern crate cgmath;

use cgmath::Vector3;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}