mod hittable;
mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
// use std::f64::consts::PI;

pub use hittable::Object;
pub use ray::Ray;
pub use vec3::Vec3;

// Image
const SIZ: u32 = 512;
const RADIO: f64 = 16.0 / 9.0;
const IMAGE_W: u32 = (SIZ as f64 * RADIO) as u32;
const IMAGE_H: u32 = SIZ;

// Camera
const VIEWPORT_H: f64 = 2.0;
const VIEWPORT_W: f64 = RADIO * VIEWPORT_H;
const FOCAL_LEN: f64 = 1.0;

fn write_color(x: u32, y: u32, img: &mut RgbImage, rgb: Vec3) {
    img.put_pixel(
        x,
        IMAGE_H - y - 1,
        Rgb([
            (rgb.x * 255.99) as u8,
            (rgb.y * 255.99) as u8,
            (rgb.z * 255.99) as u8,
        ]),
    );
}

fn ray_color(r: &Ray, sph: &hittable::HitTableList) -> Vec3 {
    let t = sph.hit(r, 0.0, f64::MAX);
    if let Some(rec) = t {
        return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_dir = r.dir.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(IMAGE_W, IMAGE_H);
    let bar = ProgressBar::new(SIZ.into()); // used for displaying progress in stdcerr

    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_W, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_H, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LEN);
    println!("{:?}", lower_left_corner);

    // Render
    let mut world = hittable::HitTableList { objects: vec![] };
    let sphere = hittable::Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    world.add(Box::new(sphere));

    for j in (0..IMAGE_H).rev() {
        for i in 0..IMAGE_W {
            let u = i as f64 / (IMAGE_W - 1) as f64;
            let v = j as f64 / (IMAGE_H - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(&r, &world);
            write_color(i, j, &mut img, pixel_color);
        }
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
