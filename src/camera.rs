extern crate cgmath;

use cgmath::{Vector3, InnerSpace};

use crate::ray::Ray;

fn unit_vector(vector: &Vec3) -> Vec3 {
    vector / vector.magnitude()
}

type Vec3 = Vector3<f64>;
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov*std::f64::consts::PI/180f64;
        let half_height = (theta/2f64).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&vup.cross(w));
        let v = w.cross(u);
        let lower_left_corner = origin - half_width*u - half_height*v - w;
        let horizontal = 2f64*half_width*u;
        let vertical = 2f64*half_height*v;
             
        Camera {origin, lower_left_corner, horizontal, vertical}
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin
        )
    }
}
