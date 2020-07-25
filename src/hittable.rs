use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3, // normal dir
    pub t: f64,
}

pub trait Object {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}
impl Object for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.squared_length();
        let half_b = oc * r.dir;
        let c = oc.squared_length() - self.radius * self.radius;
        let delta = half_b * half_b - a * c;

        if delta > 0.0 {
            let delta_sqrt = delta.sqrt();
            let root = (-half_b - delta_sqrt) / a;

            if (root < t_max) && (root > t_min) {
                return Some(HitRecord {
                    t: root,
                    p: r.at(root),
                    normal: (r.at(root) - self.center) / self.radius,
                });
            }

            let root = (-half_b + delta_sqrt) / a;
            if root < t_max && root > t_min {
                return Some(HitRecord {
                    t: root,
                    p: r.at(root),
                    normal: (r.at(root) - self.center) / self.radius,
                });
            }
        }
        None
    }
}
