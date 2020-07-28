use crate::vec3::Vec3;
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct SolidColor {
    color_value: Vec3,
}
impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
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
