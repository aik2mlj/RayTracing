use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(_orig: Vec3, _dir: Vec3) -> Self {
        Self {
            orig: _orig,
            dir: _dir,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}
