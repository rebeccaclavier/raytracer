extern crate cgmath;
extern crate objekt;

use cgmath::{Vector3, InnerSpace};

use crate::ray::Ray;
use crate::materials::{Material, Empty};

type Vec3 = Vector3<f64>;

pub trait Hittable: std::marker::Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Box<dyn Material>
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3, mat: Box<dyn Material>) -> HitRecord {
        HitRecord {t, p, normal, mat}
    }

    pub fn empty() -> HitRecord {
        HitRecord::new(0f64, Vec3::new(0f64, 0f64, 0f64), Vec3::new(0f64, 0f64, 0f64), Box::new(Empty{}))
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Box<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Box<dyn Material>) -> Sphere {
        Sphere {center, radius, mat}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b*b - a*c;
        if discriminant > 0f64 {
            let mut root = (-b - discriminant.sqrt()) / a;
            if root < t_max && root > t_min {
                rec.t = root;
                rec.p = r.point_at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = self.mat.clone();
                return true
            }
            root = (-b + discriminant.sqrt()) / a;
            if root < t_max && root > t_min {
                rec.t = root;
                rec.p = r.point_at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = self.mat.clone();
                return true
            }
        }
        false
    }
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
    pub size: usize
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        let size = list.len();
        HittableList {list, size}
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut current_closest = t_max;
        for i in 0..self.size {
            if self.list[i].hit(r, t_min, current_closest, &mut temp_rec) {
                hit_anything = true;
                current_closest = temp_rec.t;
            }
        }
        *rec = temp_rec;

        hit_anything
    }
}