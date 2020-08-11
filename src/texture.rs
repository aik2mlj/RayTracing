use image::GenericImageView;
use std::path::Path;

use crate::shared_tools::*;
use crate::vec3::Vec3;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct SolidColor {
    color_value: Vec3,
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color_value
    }
}
impl SolidColor {
    pub fn new(color: Vec3) -> Self {
        Self { color_value: color }
    }
    pub fn new_from_f64(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: Vec3::new(r, g, b),
        }
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sine = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sine < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
impl CheckerTexture {
    pub fn new(color1: Vec3, color2: Vec3) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(color1)),
            even: Arc::new(SolidColor::new(color2)),
        }
    }
}

pub struct ImageTexture {
    img: image::DynamicImage,
    width: u32,
    height: u32,
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Vec3 {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);

        let i = (u * self.width as f64) as u32;
        let j = (v * self.height as f64) as u32;

        let i = i.min(self.width - 1);
        let j = j.min(self.height - 1);

        let pixel = self.img.get_pixel(i, j);
        Vec3::new(
            pixel[0] as f64 / 255.0,
            pixel[1] as f64 / 255.0,
            pixel[2] as f64 / 255.0,
        )
    }
}
impl ImageTexture {
    pub fn new(inputpath: &str) -> Self {
        let img = image::open(&Path::new(inputpath)).unwrap();
        let width = img.dimensions().0;
        let height = img.dimensions().1;

        Self { img, width, height }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}
impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::ones() * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(&p, 7)).sin())
    }
}
impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}
