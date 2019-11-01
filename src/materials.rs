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

#[derive(Clone)]
pub struct Glass {
    attenuation: Vec3,
    ref_idx: f64,
}

impl Glass {
    pub fn new(attenuation: Vec3, ref_idx: f64) -> Glass {
        Glass {attenuation, ref_idx}
    }

    fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
        let uv = unit_vector(v);
        let dt = uv.dot(*n);
        let discriminant = 1f64 - ni_over_nt.powi(2)*(1f64-dt.powi(2));
        if discriminant > 0f64 {
            *refracted = ni_over_nt*(uv - n*dt) - n*discriminant.sqrt();
            return true
        }
        
        false
    }

    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2f64 * v.dot(n) * n
    }

    fn schlick(&self, cosine: f64) -> f64 {
        let r0 = ((1f64-self.ref_idx) / (1f64+self.ref_idx)).powi(2);
        r0 + (1f64-r0)*(1f64-cosine).powi(5)
    }
}

impl Material for Glass {
    fn scatter(&self, r: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let outward_normal: Vec3; 
        let reflected = Glass::reflect(r.direction, rec.normal);
        let ni_over_nt: f64;
        *attenuation = self.attenuation;
        let mut refracted = Vec3::new(0f64, 0f64, 0f64);

        let reflect_prob: f64;
        let cosine: f64;
        
        if r.direction.dot(rec.normal) > 0f64 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r.direction.dot(rec.normal) / r.direction.magnitude();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1f64 / self.ref_idx;
            cosine = -(r.direction.dot(rec.normal) / r.direction.magnitude());
        }

        if Glass::refract(&r.direction, &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = self.schlick(cosine);
        } else {
            reflect_prob = 1f64;
        }

        if rand_f64() < reflect_prob {
            *scattered = Ray::new(rec.p, reflected);
        } else {
            *scattered = Ray::new(rec.p, refracted);
        }

        true
    }
}

