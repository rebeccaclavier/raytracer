extern crate image;
extern crate cgmath;

mod ray;

use image::{ImageBuffer, RgbImage};
use cgmath::{InnerSpace, Vector3};
use ray::Ray;

fn color(r: &Ray) -> Vector3<f64> {
    let unit_direction = r.direction / r.direction.magnitude();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn main() {
    let res_x = 200;
    let res_y = 100;
    let mut img: RgbImage = ImageBuffer::new(res_x, res_y);

    let lower_left_corner = Vector3::new(-2f64, -1f64, -1f64);
    let horizontal = Vector3::new(4f64, 0f64, 0f64);
    let vertical = Vector3::new(0f64, 2f64, 0f64);
    let origin = Vector3::new(0f64, 0f64, 0f64);

    for y in 0..res_y {
        for x in 0..res_x {
            let u = x as f64 / res_x as f64;
            let v = y as f64 / res_y as f64;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r);

            let scaled = col * 255.0;
            let r = scaled.x as u8;
            let g = scaled.y as u8;
            let b = scaled.z as u8;
            img.put_pixel(x, res_y - y - 1, image::Rgb([r, g, b]));
        }
    }

    img.save("jeff.png").unwrap();
}
