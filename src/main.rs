mod camera;
mod hittable;
mod material;
mod ray;
mod shared_tools;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use std::f64::consts::PI;
use std::sync::Arc;

pub use camera::Camera;
pub use hittable::*;
pub use material::*;
pub use ray::*;
pub use shared_tools::*;
pub use vec3::Vec3;

// Image
const SIZ: u32 = 512;
const RADIO: f64 = 16.0 / 9.0;
const IMAGE_W: u32 = (SIZ as f64 * RADIO) as u32;
const IMAGE_H: u32 = SIZ;
const SAMPLE_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

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
        let scattered_value = rec.mat_ptr.scatter(r, &rec);
        if let Some((attenuation, scattered)) = scattered_value {
            return ray_color(&scattered, world, depth - 1).elemul(attenuation);
        }
        return Vec3::zero();
        // recurse to add in the child rays
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

    let material_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat_ptr: material_ground.clone(),
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_center.clone(),
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_left.clone(),
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.4,
        mat_ptr: material_left.clone(),
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_right.clone(),
    }));

    // Camera
    let v_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let cam = Camera::new(
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        v_up,
        40.0,
        RADIO,
    );

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
