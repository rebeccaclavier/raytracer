extern crate cgmath;

use cgmath::Vector3;

use crate::ray::Ray;

type Vec3 = Vector3<f64>;
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(origin: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {
        Camera {origin, lower_left_corner, horizontal, vertical}
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin
        )
    }
}