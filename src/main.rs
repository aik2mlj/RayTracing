mod bvh;
mod camera;
mod hittable;
mod material;
mod ray;
mod scenes;
mod shared_tools;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use std::sync::Arc;

pub use bvh::*;
pub use camera::Camera;
pub use hittable::*;
pub use material::*;
pub use ray::*;
pub use shared_tools::*;
pub use texture::*;
pub use vec3::Vec3;

// Image
const SIZ: u32 = 1080;
const RADIO: f64 = 16.0 / 9.0;
const IMAGE_W: u32 = (SIZ as f64 * RADIO) as u32;
const IMAGE_H: u32 = SIZ;
const SAMPLE_PER_PIXEL: u32 = 256;
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
fn ray_color(r: &Ray, background: &Vec3, world: &HitTableList, depth: u32) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Vec3::zero();
    }
    let t = world.hit(r, 0.001, f64::MAX); // 0.001: get rid of shadow acnes
    if let Some(rec) = t {
        let emitted_value = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);
        let scattered_value = rec.mat_ptr.scatter(r, &rec);
        if let Some((attenuation, scattered)) = scattered_value {
            return emitted_value
                + ray_color(&scattered, &background, world, depth - 1).elemul(attenuation);
        } else {
            emitted_value
        }
    // recurse to add in the child rays
    } else {
        // If the ray hits nothing, return the background color.
        *background
    }

    // let unit_dir = r.dir.unit();
    // let t = 0.5 * (unit_dir.y + 1.0);
    // Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(IMAGE_W, IMAGE_H);
    let bar = ProgressBar::new(SIZ.into()); // used for displaying progress in stdcerr

    // THE WORLD!
    let mut world = HitTableList::new();
    let mut background = Vec3::new(0.7, 0.8, 1.0);
    let mut lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let mut lookat = Vec3::zero();
    let mut vfov = 20.0;
    let mut dist_to_focus = 10.0;
    let mut aperture = 0.0;
    match 1 {
        1 => {
            world = scenes::big_random_scene();
            background = Vec3::zero();
            lookfrom = Vec3::new(23.0, 3.0, 5.0);
            lookat = Vec3::new(0.0, 0.7, 0.0);
            dist_to_focus = 23.0;
            aperture = 0.1;
        }
        2 => {
            world = scenes::two_spheres();
        }
        3 => {
            world = scenes::one_ball();
        }
        4 => {
            world = scenes::earth();
        }
        _ => {
            world = scenes::simple_light();
            background = Vec3::zero();
            lookfrom = Vec3::new(26.0, 3.0, 6.0);
            lookat = Vec3::new(0.0, 2.0, 0.0);
        }
    }
    let mut world0 = HitTableList::new();
    // use BVH
    world0.add(Arc::new(BVHNode::new(&mut world, 0.0, 1.0)));

    // Camera
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let cam = Camera::new(lookfrom, lookat, v_up, vfov, RADIO, aperture, dist_to_focus);

    // Render
    for j in (0..IMAGE_H).rev() {
        for i in 0..IMAGE_W {
            let mut pixel_color = Vec3::zero();
            for _s in 0..SAMPLE_PER_PIXEL {
                // write each sample
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_W - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_H - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &background, &world0, MAX_DEPTH);
            }
            write_color(i, j, &mut img, pixel_color);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
