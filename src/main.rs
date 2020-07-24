mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
// use std::f64::consts::PI;

pub use ray::Ray;
pub use vec3::Vec3;

// Image
const SIZ: u32 = 512;
const RADIO: f64 = 16.0 / 9.0;
const image_w: u32 = (SIZ as f64 * RADIO) as u32;
const image_h: u32 = SIZ;

// Camera
const viewport_height: f64 = 2.0;
const viewport_width: f64 = RADIO * viewport_height;
const focal_length: f64 = 1.0;

fn write_color(x: u32, y: u32, img: &mut RgbImage, rgb: Vec3) {
    img.put_pixel(
        x,
        image_h - y - 1,
        Rgb([
            (rgb.x * 255.99) as u8,
            (rgb.y * 255.99) as u8,
            (rgb.z * 255.99) as u8,
        ]),
    );
}

fn ray_color(r: &Ray) -> Vec3 {
    let unit_dir = r.dir.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(image_w, image_h);
    let bar = ProgressBar::new(SIZ.into()); // used for displaying progress in stdcerr

    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    println!("{:?}", lower_left_corner);

    // Render
    for j in (0..image_h).rev() {
        for i in 0..image_w {
            let u = i as f64 / (image_w - 1) as f64;
            let v = j as f64 / (image_h - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(&r);
            write_color(i, j, &mut img, pixel_color);
        }
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
