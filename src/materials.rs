extern crate cgmath;
extern crate objekt;

use cgmath::{Vector3, InnerSpace};
use rand::{Rng, thread_rng};

use crate::ray::Ray;
use crate::objects::{HitRecord};

type Vec3 = Vector3<f64>;

fn rand_f64() -> f64 {
    thread_rng().gen::<f64>()
}

fn unit_vector(vector: &Vec3) -> Vec3 {
    vector / vector.magnitude()
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    while {
        p = 2f64 * Vec3::new(rand_f64(), rand_f64(), rand_f64()) - Vec3::new(1f64, 1f64, 1f64);

        p.magnitude2() >= 1f64
    } {}

    p
}

pub trait Material: std::marker::Sync + objekt::Clone {
    fn scatter(&self, r: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

objekt::clone_trait_object!(Material);

#[derive(Clone)]
pub struct Empty {}

impl Material for Empty {
    fn scatter(&self, r: &Ray, _rec: &mut HitRecord, _attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *scattered = r.clone();
        true
    }
}

#[derive(Clone)]
pub struct Matte {
    albedo: Vec3
}

impl Matte {
    pub fn new(albedo: Vec3) -> Matte {
        Matte {albedo}
    }
}

impl Material for Matte {
    fn scatter(&self, _r: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, f: f64) -> Metal {
        let fuzz = if f < 1f64 { f } else { 1f64 };
        Metal {albedo, fuzz}
    }

    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2f64 * v.dot(n) * n
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Metal::reflect(unit_vector(&r.direction), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz*random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction.dot(rec.normal) > 0f64
    }
}