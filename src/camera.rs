use crate::ray::*;
use crate::vec3::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    pub fn new(radio: f64) -> Self {
        let viewport_h = 2.0;
        let viewport_w = viewport_h * radio;
        let focal_len = 1.0;
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(viewport_w, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_h, 0.0),
            lower_left_corner: Vec3::new(0.0, 0.0, 0.0)
                - Vec3::new(viewport_w, 0.0, 0.0) / 2.0
                - Vec3::new(0.0, viewport_h, 0.0) / 2.0
                - Vec3::new(0.0, 0.0, focal_len),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
