use crate::bvh::*;
use crate::material::*;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64::consts::PI;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Vec3,      // the hit point
    pub normal: Vec3, // normal dir (united)
    pub t: f64,

    // UV for texture
    pub u: f64,
    pub v: f64,

    pub front_face: bool,

    pub mat_ptr: Arc<dyn Material>,
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
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

pub struct HitTableList {
    // a list of hit-tables that have implemented Object trait
    pub objects: Vec<Arc<dyn Object>>,
}
impl HitTableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, new_item: Arc<dyn Object>) {
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

    // bound all the stuff together
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut first_box = true;
        let mut ret = AABB::default();
        for ob in self.objects.iter() {
            let tmp_ret = ob.bounding_box(t0, t1);
            if let Some(tmp_box) = tmp_ret {
                ret = if first_box {
                    tmp_box
                } else {
                    AABB::surrounding_box(tmp_box, ret)
                };
                first_box = false;
            } else {
                return None;
            }
        }
        Some(ret)
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
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
                let ret_p = r.at(root);
                let outward_normal = (ret_p - self.center) / self.radius;
                let (u, v) = Sphere::get_uv(outward_normal);
                let mut ret = HitRecord {
                    t: root,
                    p: ret_p,
                    normal: outward_normal,
                    front_face: false,
                    mat_ptr: self.mat_ptr.clone(),

                    u,
                    v,
                };
                ret.set_face_normal(&r, &outward_normal);
                return Some(ret);
            }

            let root = (-half_b + delta_sqrt) / a;
            if root < t_max && root > t_min {
                let ret_p = r.at(root);
                let outward_normal = (ret_p - self.center) / self.radius;
                let (u, v) = Sphere::get_uv(outward_normal);
                let mut ret = HitRecord {
                    t: root,
                    p: ret_p,
                    normal: outward_normal,
                    front_face: false,
                    mat_ptr: self.mat_ptr.clone(),

                    u,
                    v,
                };
                ret.set_face_normal(&r, &outward_normal);
                return Some(ret);
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        // use the outside BOX of this sphere
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
impl Sphere {
    fn get_uv(p: Vec3) -> (f64, f64) {
        // put a 2D UV onto the surface of a sphere
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        (1.0 - (phi + PI) / (2.0 * PI), (theta + PI / 2.0) / PI)
    }
}

#[derive(Clone)]
pub struct XYRect {
    pub mat_ptr: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}
impl Object for XYRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let mut ret = HitRecord {
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            t,
            normal: outward_normal,
            front_face: false,
            mat_ptr: self.mat_ptr.clone(),
            p: r.at(t),
        };
        ret.set_face_normal(r, &outward_normal);
        Some(ret)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mat_ptr,
        }
    }
}

#[derive(Clone)]
pub struct XZRect {
    pub mat_ptr: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}
impl Object for XZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.y) / r.dir.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let z = r.orig.z + t * r.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let mut ret = HitRecord {
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            t,
            normal: outward_normal,
            front_face: false,
            mat_ptr: self.mat_ptr.clone(),
            p: r.at(t),
        };
        ret.set_face_normal(r, &outward_normal);
        Some(ret)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}
impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mat_ptr,
        }
    }
}

#[derive(Clone)]
pub struct YZRect {
    pub mat_ptr: Arc<dyn Material>,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}
impl Object for YZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.x) / r.dir.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.orig.y + t * r.dir.y;
        let z = r.orig.z + t * r.dir.z;
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let mut ret = HitRecord {
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            t,
            normal: outward_normal,
            front_face: false,
            mat_ptr: self.mat_ptr.clone(),
            p: r.at(t),
        };
        ret.set_face_normal(r, &outward_normal);
        Some(ret)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mat_ptr,
        }
    }
}

pub struct Box {
    pub min: Vec3,
    pub max: Vec3,
    pub sides: HitTableList,
}
impl Object for Box {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}
impl Box {
    pub fn new(min: Vec3, max: Vec3, mat_ptr: Arc<dyn Material>) -> Self {
        let mut sides = HitTableList::new();
        sides.add(Arc::new(XYRect::new(
            min.x,
            max.x,
            min.y,
            max.y,
            min.z,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(XYRect::new(
            min.x,
            max.x,
            min.y,
            max.y,
            max.z,
            mat_ptr.clone(),
        )));

        sides.add(Arc::new(XZRect::new(
            min.x,
            max.x,
            min.z,
            max.z,
            min.y,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(XZRect::new(
            min.x,
            max.x,
            min.z,
            max.z,
            max.y,
            mat_ptr.clone(),
        )));

        sides.add(Arc::new(YZRect::new(
            min.y,
            max.y,
            min.z,
            max.z,
            min.x,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(YZRect::new(
            min.y,
            max.y,
            min.z,
            max.z,
            max.x,
            mat_ptr.clone(),
        )));

        Self { min, max, sides }
    }
}
