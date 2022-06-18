use std::io::{stderr, Write};

use image::{RgbImage, Rgb};
use ray_tracer::ray::Ray;
use ray_tracer::Vec3;

fn main() {

    // camera setup

    let ratio = 16./9.; 
    let width = 400;
    let height = (width as f32 / ratio) as u32;

    let mut img = RgbImage::new(width, height);

    let view_height = 2.0;
    let view_width = ratio * view_height;
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(view_width, 0.0, 0.0);
    let vertical = Vec3(0.0, view_height, 0.0);
    let focal_ray = Vec3(0.0, 0.0, focal_length);
    let lower_left_corner = origin - horizontal/2. - vertical/2. - focal_ray;
    // render

    for x in 0..img.width() {
        for y in 0..img.height() {
            
            let u = (x as f32) / ((width - 1) as f32);
            let v = (y as f32) / ((width - 1) as f32);

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);

            img.put_pixel(x, y, ray_color(&ray));
        }
    }


    img.save("test.png").unwrap();
}

fn ray_color(ray: &Ray) -> Rgb<u8>{

    let unit_dir = ray.direction.to_unit();
    let t = unit_dir.1;
    let color = Vec3(0.5, 0.7, 1.0) * t.abs();
    Rgb([(color.0 * 255.) as u8, 
        (color.1 * 255.) as u8, 
        (color.2 * 255.) as u8])
}

