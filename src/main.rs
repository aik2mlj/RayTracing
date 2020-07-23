#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::f64::consts::PI;

pub use vec3::Vec3;

fn main() {
    // let x = Vec3::new(1.0, 1.0, 1.0);
    // println!("{:?}", x);
    const SIZ: u32 = 512;

    let mut img: RgbImage = ImageBuffer::new(SIZ, SIZ);
    let bar = ProgressBar::new(SIZ.into()); // used for displaying progress in stdcerr

    // for x in 0..1024 {
    //     for y in 1..1024 {
    //         let pixel = img.get_pixel_mut(x, y);
    //         // let color = (x / 4) as u8;
    //         let r = (x / 4) as u8;
    //         let g = (y / 4) as u8;
    //         let b = (x & y) as u8;
    //         *pixel = image::Rgb([r, g, b]);
    //     }
    //     bar.inc(1);
    // }
    let rad: u32 = 200;
    let r: u8 = 25;
    let g: u8 = 255;
    let b: u8 = 255;
    let half = SIZ / 2;
    // get a circle
    for x in (half - rad)..(half + rad) {
        let y1 = half - ((rad * rad - (half - x) * (half - x)) as f64).sqrt() as u32;
        let y2 = half + ((rad * rad - (half - x) * (half - x)) as f64).sqrt() as u32;
        let pixel1 = img.get_pixel_mut(x, y1);
        *pixel1 = image::Rgb([r, g, b]);
        let pixel2 = img.get_pixel_mut(x, y2);
        *pixel2 = image::Rgb([r, g, b]);

        bar.inc(1);
    }
    for y in (half - rad)..(half + rad) {
        let x1 = half - ((rad * rad - (half - y) * (half - y)) as f64).sqrt() as u32;
        let x2 = half + ((rad * rad - (half - y) * (half - y)) as f64).sqrt() as u32;
        let pixel1 = img.get_pixel_mut(x1, y);
        *pixel1 = image::Rgb([r, g, b]);
        let pixel2 = img.get_pixel_mut(x2, y);
        *pixel2 = image::Rgb([r, g, b]);

        bar.inc(1);
    }

    struct Thread {
        l: u32,
        angle: f64,
    }
    let th: [Thread; 3] = [
        Thread {
            l: rad * 4 / 5,
            angle: rand::random::<f64>() * 2.0 * PI,
        },
        Thread {
            l: rad * 2 / 3,
            angle: rand::random::<f64>() * 2.0 * PI,
        },
        Thread {
            l: rad / 2,
            angle: rand::random::<f64>() * 2.0 * PI,
        },
    ];

    // let l1 = (rad * 4 / 5) as u32;
    // let l2 = (rad * 2 / 3) as u32;
    // let l3 =
    // for x in half..
    for wh in &th {
        let x0 = (wh.l as f64 * wh.angle.cos()) as i32;
        if x0 < 0 {
            for x in x0..1 {
                let y = ((x as f64 / x0 as f64) * wh.l as f64 * wh.angle.sin()) as i32;
                let pixel = img.get_pixel_mut((x + half as i32) as u32, (y + half as i32) as u32);
                *pixel = image::Rgb([r, g, b]);
            }
        } else {
            for x in 0..x0 + 1 {
                let y = ((x as f64 / x0 as f64) * wh.l as f64 * wh.angle.sin()) as i32;
                let pixel = img.get_pixel_mut((x + half as i32) as u32, (y + half as i32) as u32);
                *pixel = image::Rgb([r, g, b]);
            }
        }

        let y0 = (wh.l as f64 * wh.angle.sin()) as i32;
        if y0 < 0 {
            for y in y0..1 {
                let x = ((y as f64 / y0 as f64) * wh.l as f64 * wh.angle.cos()) as i32;
                let pixel = img.get_pixel_mut((x + half as i32) as u32, (y + half as i32) as u32);
                *pixel = image::Rgb([r, g, b]);
            }
        } else {
            for y in 0..y0 + 1 {
                let x = ((y as f64 / y0 as f64) * wh.l as f64 * wh.angle.cos()) as i32;
                let pixel = img.get_pixel_mut((x + half as i32) as u32, (y + half as i32) as u32);
                *pixel = image::Rgb([r, g, b]);
            }
        }
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
