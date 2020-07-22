#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 1024);
    let bar = ProgressBar::new(1024); // used for displaying progress in stdcerr

    for x in 0..1024 {
        for y in 1..1024 {
            let pixel = img.get_pixel_mut(x, y);
            // let color = (x / 4) as u8;
            let r = (x / 4) as u8;
            let g = (y / 4) as u8;
            let b = (x & y) as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
