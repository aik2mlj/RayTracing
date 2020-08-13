// BVH is used to fast the code, making the running time logarithmic to the numbers of objects

use crate::hittable::*;
use crate::ray::*;
use crate::vec3::Vec3;
use std::cmp::Ordering;
use std::sync::Arc;

// a bounding box
#[derive(Default, Clone)]
pub struct AABB {
    pub _min: Vec3,
    pub _max: Vec3,
}
impl AABB {
    pub fn new(_mn: Vec3, _mx: Vec3) -> Self {
        Self {
            _min: _mn,
            _max: _mx,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut tmin = t_min;
        let mut tmax = t_max;

        let invd = 1.0 / r.dir.x;
        let mut t0 = (self._min.x - r.orig.x) * invd;
        let mut t1 = (self._max.x - r.orig.x) * invd;
        if invd < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        tmin = if t0 > tmin { t0 } else { tmin };
        tmax = if t1 < tmax { t1 } else { tmax };
        if tmax <= tmin {
            return false;
        }

        let invd = 1.0 / r.dir.y;
        let mut t0 = (self._min.y - r.orig.y) * invd;
        let mut t1 = (self._max.y - r.orig.y) * invd;
        if invd < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        tmin = if t0 > tmin { t0 } else { tmin };
        tmax = if t1 < tmax { t1 } else { tmax };
        if tmax <= tmin {
            return false;
        }

        let invd = 1.0 / r.dir.z;
        let mut t0 = (self._min.z - r.orig.z) * invd;
        let mut t1 = (self._max.z - r.orig.z) * invd;
        if invd < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        tmin = if t0 > tmin { t0 } else { tmin };
        tmax = if t1 < tmax { t1 } else { tmax };
        if tmax <= tmin {
            return false;
        }

        true
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        Self::new(
            Vec3::new(
                box0._min.x.min(box1._min.x),
                box0._min.y.min(box1._min.y),
                box0._min.z.min(box1._min.z),
            ),
            Vec3::new(
                box0._max.x.max(box1._max.x),
                box0._max.y.max(box1._max.y),
                box0._max.z.max(box1._max.z),
            ),
        )
    }
}

// a node that bound two node
// real objects(Sphere, etc.) live at the leaves of this hierarchy
pub struct BVHNodeStatic<L: Hittable, R: Hittable> {
    pub left: Arc<L>,  // left child
    pub right: Arc<R>, // right child
    pub _box: AABB,
}
impl<L: Hittable, R: Hittable> Hittable for BVHNodeStatic<L, R> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self._box.hit(r, t_min, t_max) {
            return None;
        }

        let left_tmp_ret = self.left.hit(r, t_min, t_max);
        if let Some(left_rec) = left_tmp_ret {
            let right_tmp_ret = self.right.hit(r, t_min, left_rec.t);
            if right_tmp_ret.is_some() {
                right_tmp_ret
            } else {
                Some(left_rec)
            }
        } else {
            let right_tmp_ret = self.right.hit(r, t_min, t_max);
            if right_tmp_ret.is_some() {
                right_tmp_ret
            } else {
                None
            }
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self._box.clone())
    }
}
impl<L: Hittable, R: Hittable> BVHNodeStatic<L, R> {
    pub fn construct(left: Arc<L>, right: Arc<R>, time0: f64, time1: f64) -> Self {
        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();

        Self {
            left,
            right,
            _box: AABB::surrounding_box(box_left, box_right),
        }
    }
}

pub struct BVHNode {
    pub left: Arc<dyn Hittable>,  // left child
    pub right: Arc<dyn Hittable>, // right child
    pub _box: AABB,
}
impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self._box.hit(r, t_min, t_max) {
            return None;
        }

        let left_tmp_ret = self.left.hit(r, t_min, t_max);
        if let Some(left_rec) = left_tmp_ret {
            let right_tmp_ret = self.right.hit(r, t_min, left_rec.t);
            if right_tmp_ret.is_some() {
                right_tmp_ret
            } else {
                Some(left_rec)
            }
        } else {
            let right_tmp_ret = self.right.hit(r, t_min, t_max);
            if right_tmp_ret.is_some() {
                right_tmp_ret
            } else {
                None
            }
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self._box.clone())
    }
}
impl BVHNode {
    pub fn new(list: &mut HitTableList, time0: f64, time1: f64) -> Self {
        let len = list.objects.len();
        Self::new_(&mut list.objects, 0, len, time0, time1)
    }
    fn new_(
        objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = rand::random::<usize>() % 3;
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };
        let span = end - start;
        let mut left = objects[start].clone();
        let mut right = objects[start].clone();

        if span == 1 {
        } else if span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
            }
        } else {
            let objects_slice = &mut objects[start..end]; // mutable slice
            objects_slice.sort_by(|a, b| comparator(a, b)); // sort the slice

            let mid = (start + end) >> 1; // half divide and recurse
            left = Arc::new(BVHNode::new_(objects, start, mid, time0, time1));
            right = Arc::new(BVHNode::new_(objects, mid, end, time0, time1));
        }

        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();

        Self {
            left,
            right,
            _box: AABB::surrounding_box(box_left, box_right),
        }
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        let box_a = a.bounding_box(0.0, 0.0).unwrap();
        let box_b = b.bounding_box(0.0, 0.0).unwrap();
        box_a._min.x.partial_cmp(&box_b._min.x).unwrap()
    }
    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        let box_a = a.bounding_box(0.0, 0.0).unwrap();
        let box_b = b.bounding_box(0.0, 0.0).unwrap();
        box_a._min.y.partial_cmp(&box_b._min.y).unwrap()
    }
    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        let box_a = a.bounding_box(0.0, 0.0).unwrap();
        let box_b = b.bounding_box(0.0, 0.0).unwrap();
        box_a._min.z.partial_cmp(&box_b._min.z).unwrap()
    }
}
