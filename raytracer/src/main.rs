mod bvh;
mod camera;
mod hittable;
mod material;
mod material_static;
mod onb;
mod pdf;
mod ray;
mod scenes;
mod shared_tools;
mod texture;
// mod hittable_static;
#[allow(clippy::float_cmp)]
mod vec3;

use image::{imageops, ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;

pub use bvh::*;
pub use camera::Camera;
pub use hittable::*;
pub use material::*;
pub use pdf::*;
pub use ray::*;
pub use shared_tools::*;
pub use texture::*;
pub use vec3::Vec3;

// Image
const MAX_DEPTH: u32 = 50;

// put pixel onto the image
#[allow(clippy::eq_op)]
#[allow(clippy::float_cmp)]
fn write_color(pixel_x: u32, pixel_y: u32, sample_per_pixel: u32, img: &mut RgbImage, rgb: Vec3) {
    // sqrt for Gamma Correction: gamma = 2.0
    let mut r = rgb.x;
    let mut g = rgb.y;
    let mut b = rgb.z;
    // Remove acne(NaN != NaN)
    if r != r {
        r = 0.0
    }
    if g != g {
        g = 0.0
    }
    if b != b {
        b = 0.0
    }
    let r = (r / sample_per_pixel as f64).sqrt();
    let g = (g / sample_per_pixel as f64).sqrt();
    let b = (b / sample_per_pixel as f64).sqrt();
    img.put_pixel(
        pixel_x,
        pixel_y,
        Rgb([
            (clamp(r, 0.0, 0.999) * 255.99) as u8,
            (clamp(g, 0.0, 0.999) * 255.99) as u8,
            (clamp(b, 0.0, 0.999) * 255.99) as u8,
        ]),
    );
}

// get the ray color within the depth
fn ray_color(
    r: &Ray,
    background: &Vec3,
    objects: &HitTableList,
    lights: Arc<HitTableList>,
    depth: u32,
) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Vec3::zero();
    }
    let t = objects.hit(r, 0.001, f64::MAX); // 0.001: get rid of shadow acnes
    if let Some(rec) = t {
        let emitted_value = rec.mat_ptr.emitted(r, &rec, rec.u, rec.v, rec.p);
        let scattered_value = rec.mat_ptr.scatter(r, &rec);
        if let Some(srec) = scattered_value {
            // let on_light = Vec3::new(random_f64(213.0, 343.0), 554.0, random_f64(227.0, 332.0));
            // let to_light = on_light - rec.p;
            // let distance_squared = to_light.squared_length();
            // let to_light = to_light.unit();
            // if to_light * rec.normal < 0.0 {
            //     return emitted_value;
            // }
            // let light_area = ((343 - 213) * (332 - 227)) as f64;
            // let light_cosine = to_light.y.abs();
            // // if light_cosine < 0.000001 {
            // //     return emitted_value;
            // // }

            // let light_shape = Arc::new(XZRect::new(
            //     213.0,
            //     343.0,
            //     227.0,
            //     332.0,
            //     554.0,
            //     Arc::new(Lambertian::new(Vec3::zero())),
            // ));
            if let Some(specular_ray) = srec.specular_ray {
                return ray_color(&specular_ray, &background, objects, lights, depth - 1)
                    .elemul(srec.attenuation);
            }
            // let p1 = Arc::new(CosinePDF::build_from_w(&rec.normal));
            if srec.pdf_ptr.is_none() {
                panic!("pdf_ptr is None!");
            }
            let p = if lights.objects.is_empty() {
                let pdf_ptr = srec.pdf_ptr.unwrap();
                MixturePDF::new(pdf_ptr.clone(), pdf_ptr)
            } else {
                let light_ptr = Arc::new(HittablePDF::new(lights.clone(), rec.p));
                MixturePDF::new(light_ptr, srec.pdf_ptr.unwrap())
            };

            // let p = srec.pdf_ptr.unwrap();

            // let p = CosinePDF::build_from_w(&rec.normal);
            let scattered = Ray::new(rec.p, p.generate());
            let pdf = p.value(scattered.dir);

            emitted_value
                + ray_color(&scattered, &background, objects, lights, depth - 1)
                    .elemul(srec.attenuation)
                    * rec.mat_ptr.scattering_pdf(r, &rec, &scattered)
                    / pdf
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

#[allow(unused_assignments)]
fn main() {
    let (tx, rx) = channel();
    let n_jobs: usize = 32;
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);

    // let mut img: RgbImage = ImageBuffer::new(IMAGE_W, IMAGE_H);
    let bar = ProgressBar::new(n_jobs as u64); // used for displaying progress in stdcerr

    // THE WORLD!
    let mut siz: u32 = 400;
    let mut ratio: f64 = 16.0 / 9.0;
    let mut sample_per_pixel: u32 = 256;

    let mut lights = HitTableList::default();
    let mut objects = HitTableList::default();
    let mut background = Vec3::new(0.7, 0.8, 1.0);
    let mut lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let mut lookat = Vec3::zero();
    let mut vfov = 20.0;
    let mut dist_to_focus = 10.0;
    let mut aperture = 0.0;
    match 7 {
        0 => {
            objects = scenes::former_three_ball_scene();
        }
        1 => {
            // siz = 1080;
            objects = scenes::big_random_scene();
            background = Vec3::zero();
            lookfrom = Vec3::new(23.0, 3.0, 5.0);
            lookat = Vec3::new(0.0, 0.7, 0.0);
            dist_to_focus = 23.0;
            aperture = 0.1;
        }
        2 => {
            objects = scenes::two_spheres();
        }
        3 => {
            objects = scenes::one_ball();
        }
        4 => {
            objects = scenes::earth();
        }
        5 => {
            objects = scenes::simple_light();
            background = Vec3::zero();
            lookfrom = Vec3::new(26.0, 3.0, 6.0);
            lookat = Vec3::new(0.0, 2.0, 0.0);
        }
        6 => {
            objects = scenes::book2_final_scene();
            siz = 1600;
            sample_per_pixel = 1000;
            ratio = 1.0;
            background = Vec3::zero();
            lookfrom = Vec3::new(478.0, 278.0, -600.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            lights.add(Arc::new(XZRect::new(
                123.0,
                423.0,
                147.0,
                412.0,
                554.0,
                Arc::new(Lambertian::new(Vec3::zero())),
            )));
            lights.add(Arc::new(Sphere::new(
                Vec3::new(260.0, 150.0, 45.0),
                50.0,
                Arc::new(Lambertian::new(Vec3::zero())),
            )));
        }
        7 => {
            siz = 1000;
            ratio = 1.0;
            objects = scenes::cornell_box();
            background = Vec3::zero();
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            sample_per_pixel = 256;
            lights.add(Arc::new(XZRect::new(
                213.0,
                343.0,
                227.0,
                332.0,
                554.0,
                Arc::new(Lambertian::new(Vec3::zero())),
            )));
            lights.add(Arc::new(Sphere::new(
                Vec3::new(190.0, 90.0, 190.0),
                90.0,
                Arc::new(Lambertian::new(Vec3::zero())),
            )));
            // denoise the reflection
            let box_up = Arc::new(XZRect::new(
                0.0,
                165.0,
                0.0,
                165.0,
                330.0,
                Arc::new(Lambertian::new(Vec3::zero())),
            ));
            let box_up = Arc::new(RotateY::new(box_up, 38.0));
            let box_up = Arc::new(Translate::new(box_up, Vec3::new(265.0, 0.0, 295.0)));
            lights.add(box_up);
        }
        _ => {
            // static bvh
            siz = 1080;
            sample_per_pixel = 256;
            objects = scenes::static_scene();
            background = Vec3::zero();
            lookfrom = Vec3::new(23.0, 3.0, 5.0);
            lookat = Vec3::new(0.0, 0.7, 0.0);
            dist_to_focus = 23.0;
            aperture = 0.1;
        }
    }
    let image_w: u32 = (siz as f64 * ratio) as u32;
    let image_h: u32 = siz;

    // use BVH
    // let mut world = HitTableList::new();
    // world.add(Arc::new(BVHNode::new(&mut objects, 0.0, 1.0)));
    // let world = Arc::new(world);
    let lights = Arc::new(lights);
    // let lights = Arc::new(XZRect::new(
    //     213.0,
    //     343.0,
    //     227.0,
    //     332.0,
    //     554.0,
    //     Arc::new(Lambertian::new(Vec3::zero())),
    // ));
    // not use BVH
    let world = Arc::new(objects);

    // Camera
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let cam = Camera::new(lookfrom, lookat, v_up, vfov, ratio, aperture, dist_to_focus);
    let cam = Arc::new(cam);

    // Render

    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ptr = world.clone();
        let cam_ptr = cam.clone();
        let lights_ptr = lights.clone();
        pool.execute(move || {
            let row_begin = image_h as usize * i / n_jobs;
            let row_end = image_h as usize * (i + 1) / n_jobs;
            let render_height = (row_end - row_begin) as u32;
            let mut img_tmp: RgbImage = ImageBuffer::new(image_w, render_height); // a part of image

            for i in 0..image_w {
                for (img_j, j) in (row_begin..row_end).enumerate() {
                    let img_j = img_j as u32; // 0..row_end - row_begin
                    let j = j as u32; // row_begin..row_end
                    let mut pixel_color = Vec3::zero();
                    for _s in 0..sample_per_pixel {
                        // write each sample
                        let u = (i as f64 + rand::random::<f64>()) / (image_w - 1) as f64;
                        let v = (j as f64 + rand::random::<f64>()) / (image_h - 1) as f64;

                        let r = cam_ptr.get_ray(u, v);
                        pixel_color +=
                            ray_color(&r, &background, &world_ptr, lights_ptr.clone(), MAX_DEPTH);
                    }
                    write_color(i, img_j, sample_per_pixel, &mut img_tmp, pixel_color);
                }
            }
            tx.send((row_begin..row_end, img_tmp))
                .expect("failed to send result");
        });
    }

    let mut result_img: RgbImage = ImageBuffer::new(image_w, image_h);
    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..image_w {
                *result_img.get_pixel_mut(col, row as u32) = *data.get_pixel(col, idx as u32);
            }
        }
        bar.inc(1);
    }

    // flip & turn the image
    let result_img = imageops::flip_horizontal(&imageops::rotate180(&result_img));
    result_img.save("output/test.png").unwrap();
    bar.finish();
}
