extern crate image;
extern crate cgmath;

mod ray;
mod objects;
mod camera;
mod materials;

use image::{ImageBuffer, RgbImage};
use cgmath::{InnerSpace, Vector3};
use rand::{Rng, thread_rng};
use rayon::prelude::*;
use std::f64;
use std::io;
use std::io::Write;

use ray::Ray;
use objects::{Hittable, HitRecord, Sphere, HittableList};
use camera::Camera;
use materials::{Matte, Metal};

type Vec3 = Vector3<f64>;

fn unit_vector(vector: &Vec3) -> Vec3 {
    vector / vector.magnitude()
}

fn rand_f64() -> f64 {
    thread_rng().gen::<f64>()
}


fn color(r: &Ray, world: &dyn Hittable, depth: usize) -> Vec3 {
    let mut rec = HitRecord::empty();
    if world.hit(r, 0.001, f64::MAX, &mut rec) {
        let mut scattered = Ray::empty();
        let mut attenuation = Vec3::new(0f64, 0f64, 0f64);

        if depth < 50 && rec.mat.scatter(&r, &mut rec.clone(), &mut attenuation, &mut scattered) {
            let res = color(&scattered, world, depth + 1);
            attenuation.x *= res.x;
            attenuation.y *= res.y;
            attenuation.z *= res.z;
            return attenuation
        } else {
            return Vec3::new(0f64, 0f64, 0f64)
        }
    }

    let unit_direction = unit_vector(&r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let res_x = 2000;
    let res_y = 1000;
    let n_passes = 500;
    let mut img: RgbImage = ImageBuffer::new(res_x, res_y);

    let camera = Camera::new(
        Vec3::new(0f64, 0f64, 0f64),
        Vec3::new(-2f64, -1f64, -1f64),
        Vec3::new(4f64, 0f64, 0f64),
        Vec3::new(0f64, 2f64, 0f64)
    );

    let list: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0f64, 0f64, -1f64), 0.5, Box::new(Matte::new(Vec3::new(0.2, 0.2, 0.8))))),
        Box::new(Sphere::new(Vec3::new(0f64, -100.5, -1f64), 100f64, Box::new(Matte::new(Vec3::new(0.7, 0.7, 0.7))))),
        Box::new(Sphere::new(Vec3::new(1f64, 0f64, -1f64), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1f64)))),
        Box::new(Sphere::new(Vec3::new(-1f64, 0f64, -1f64), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3))))
    ];

    let world = &HittableList::new(list);

    for y in 0..res_y {
        let progress = format!(" rendering row {} / {}", y + 1, res_y);
        print!("{}", progress);
        io::stdout().flush().unwrap();

        for x in 0..res_x {
            let passes: Vec<(f64, f64)> = (0..n_passes)
                .map(|_| (
                    (x as f64 + rand_f64()) / res_x as f64,
                    (y as f64 + rand_f64()) / res_y as f64
                ))
                .collect();
            
            let mut col: Vec3 = passes.par_iter()
                                      .map(|(u, v)| color(&camera.get_ray(*u, *v), world, 0))
                                      .sum();

            col /= n_passes as f64;
            col = col.map(|x| x.sqrt());

            let scaled = col * 255f64;
            let r = scaled.x as u8;
            let g = scaled.y as u8;
            let b = scaled.z as u8;

            img.put_pixel(x, res_y - y - 1, image::Rgb([r, g, b]));
        }
        
        let clear = (0..progress.len()).map(|_| "\r \r").collect::<String>();
        print!("{}", clear);
        io::stdout().flush().unwrap();
    }

    img.save("jeff.png").unwrap();
}
