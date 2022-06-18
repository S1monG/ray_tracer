use image::{RgbImage, Rgb};
use ray_tracer::ray::Ray;
use ray_tracer::Vec3;

fn main() {

    // camera setup

    let ratio = 16./9.; 
    let width = 400;
    let height = (width as f32 / ratio) as u32;

    let mut img = RgbImage::new(width, height);

    let focal_length = 100.;
    let origin = Vec3(width as f32/2., height as f32/2., 0.,);  // middle of the image with z to 0

    // render

    for x in 0..img.width() {
        for y in 0..img.height() {
            
            let ray = Ray {
                origin,
                direction: Vec3(x as f32, y as f32, focal_length) - origin
            };

            img.put_pixel(x, y, ray_color(ray));
        }
    }


    img.save("test.png").unwrap();
}

fn ray_color(ray: Ray) -> Rgb<u8>{


    /* Rgb([
        (unit_direction.0 * 255.) as u8, 
        (unit_direction.1 * 255.) as u8, 
        (unit_direction.2 * 255.) as u8
    ]) */

    Rgb([ray.direction.0 as u8, ray.direction.1 as u8, 200])
}

