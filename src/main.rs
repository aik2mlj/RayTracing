mod camera;
mod hittable;
mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use std::f64::consts::PI;

pub use camera::Camera;
pub use hittable::*;
pub use ray::*;
pub use vec3::Vec3;

// Image
const SIZ: u32 = 512;
const RADIO: f64 = 16.0 / 9.0;
const IMAGE_W: u32 = (SIZ as f64 * RADIO) as u32;
const IMAGE_H: u32 = SIZ;
const SAMPLE_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn degree_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    // anti-aliasing
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

// put pixel onto the image
fn write_color(x: u32, y: u32, img: &mut RgbImage, rgb: Vec3) {
    // sqrt for Gamma Correction: gamma = 2.0
    let r = (rgb.x / SAMPLE_PER_PIXEL as f64).sqrt();
    let g = (rgb.y / SAMPLE_PER_PIXEL as f64).sqrt();
    let b = (rgb.z / SAMPLE_PER_PIXEL as f64).sqrt();
    img.put_pixel(
        x,
        IMAGE_H - y - 1,
        Rgb([
            (clamp(r, 0.0, 0.999) * 255.99) as u8,
            (clamp(g, 0.0, 0.999) * 255.99) as u8,
            (clamp(b, 0.0, 0.999) * 255.99) as u8,
        ]),
    );
}

// get the ray color within the depth
fn ray_color(r: &Ray, world: &HitTableList, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::zero();
    }
    let t = world.hit(r, 0.001, f64::MAX); // 0.001: get rid of shadow acnes
    if let Some(rec) = t {
        let target = rec.p + rec.normal + Vec3::rand_in_unit_sphere();
        return ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5; // recurse to add in the child rays
    }

    let unit_dir = r.dir.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(IMAGE_W, IMAGE_H);
    let bar = ProgressBar::new(SIZ.into()); // used for displaying progress in stdcerr

    // THE WORLD!
    let mut world = HitTableList { objects: vec![] };
    world.add(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    // Camera
    let cam = Camera::new(RADIO);

    // Render
    for j in (0..IMAGE_H).rev() {
        for i in 0..IMAGE_W {
            let mut pixel_color = Vec3::zero();
            for _s in 0..SAMPLE_PER_PIXEL {
                // write each sample
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_W - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_H - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(i, j, &mut img, pixel_color);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
