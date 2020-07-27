use crate::ray::*;
use crate::shared_tools::*;
use crate::vec3::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    // lookfrom: the point you look from, lookat: the same
    // view_up: a conventional view_up direction, usually (0, 1, 0)
    // vfov: an angle to decide the amount of zoom-out
    pub fn new(lookfrom: Vec3, lookat: Vec3, view_up: Vec3, vfov: f64, radio: f64) -> Self {
        let theta = degree_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_h = 2.0 * h;
        let viewport_w = viewport_h * radio;

        let w = (lookfrom - lookat).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);

        Self {
            origin: lookfrom,
            horizontal: u * viewport_w,
            vertical: v * viewport_h,
            lower_left_corner: lookfrom - u * viewport_w / 2.0 - v * viewport_h / 2.0 - w,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
