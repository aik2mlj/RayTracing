use crate::hittable::*;
use crate::onb::*;
use crate::shared_tools::*;
use crate::vec3::*;
use std::f64::consts::PI;
use std::sync::Arc;

pub trait PDF {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePDF {
    pub uvw: ONB,
}
impl PDF for CosinePDF {
    fn value(&self, direction: Vec3) -> f64 {
        let cos = direction.unit() * self.uvw.w;
        if cos <= 0.0 {
            0.0
        } else {
            cos / PI
        }
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local(&Vec3::rand_cosine_direction())
    }
}
impl CosinePDF {
    pub fn build_from_w(w: &Vec3) -> Self {
        Self {
            uvw: ONB::build_from_w(w),
        }
    }
}

pub struct NonePDF {}
impl PDF for NonePDF {
    fn value(&self, _direction: Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::zero()
    }
}

pub struct HittablePDF {
    pub o: Vec3,
    pub ptr: Arc<dyn Hittable>,
}
impl PDF for HittablePDF {
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
}
impl HittablePDF {
    pub fn new(ptr: Arc<dyn Hittable>, o: Vec3) -> Self {
        Self { o, ptr }
    }
}

pub struct MixturePDF {
    pub p: [Arc<dyn PDF>; 2],
}
impl PDF for MixturePDF {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_f64(0.0, 1.0) < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
impl MixturePDF {
    pub fn new(p0: Arc<dyn PDF>, p1: Arc<dyn PDF>) -> Self {
        Self { p: [p0, p1] }
    }
}
