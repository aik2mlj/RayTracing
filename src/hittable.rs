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

pub struct HitTableList {
    // a list of hit-tables that have implemented Object trait
    pub objects: Vec<Box<dyn Object>>,
}
impl HitTableList {
    pub fn add(&mut self, new_item: Box<dyn Object>) {
        self.objects.push(new_item);
    }
}
impl Object for HitTableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut ret = None;

        for ob in self.objects.iter() {
            let tmp_rec = ob.hit(r, t_min, closest_so_far);
            if let Some(rec_value) = tmp_rec {
                closest_so_far = rec_value.t;
                ret = Some(rec_value);
            }
        }
        ret
    }
}

#[derive(Debug)]
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
