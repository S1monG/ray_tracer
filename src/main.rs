use image::{RgbImage, Rgb};
fn main() {


    let mut img = RgbImage::new(255, 255);

    for x in 0..255 {
        for y in 0..255 {
            img.put_pixel(x, y, Rgb([x as u8, y as u8, 100]));
        }
    }


    img.save("test.png").unwrap();
}