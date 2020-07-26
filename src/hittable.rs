use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Default)]
pub struct HitRecord {
    pub p: Vec3,      // the hit point
    pub normal: Vec3, // normal dir
    pub t: f64,

    pub front_face: bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir * *outward_normal < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
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

#[derive(Clone, Debug)]
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
            let mut ret = HitRecord::default();

            if (root < t_max) && (root > t_min) {
                ret.t = root;
                ret.p = r.at(root);
                let outward_normal = (ret.p - self.center) / self.radius;
                ret.set_face_normal(&r, &outward_normal);
                return Some(ret);
            }

            let root = (-half_b + delta_sqrt) / a;
            if root < t_max && root > t_min {
                ret.t = root;
                ret.p = r.at(root);
                let outward_normal = (ret.p - self.center) / self.radius;
                ret.set_face_normal(&r, &outward_normal);
                return Some(ret);
            }
        }
        None
    }
}
