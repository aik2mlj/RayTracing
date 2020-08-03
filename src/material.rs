use crate::hittable::HitRecord;
use crate::ray::*;
use crate::shared_tools::*;
use crate::texture::*;
use crate::vec3::*;
use std::convert::From;
use std::sync::Arc;

// TRAIT Material
pub trait Material: Send + Sync {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }
    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::zero()
    }
}
//*******************

// Lambertian Material
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}
impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let scatter_dir = rec.normal + Vec3::random_unit_vector(); // Lambertian scattering
        Some((
            self.albedo.value(rec.u, rec.v, rec.p), // get color value in texture
            Ray::new(rec.p, scatter_dir),
        ))
    }
}
impl Lambertian {
    pub fn new(_al: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(_al)),
        }
    }
    pub fn new_from_texture(other: Arc<dyn Texture>) -> Self {
        Self { albedo: other }
    }
}
impl From<Arc<dyn Texture>> for Lambertian {
    fn from(other: Arc<dyn Texture>) -> Self {
        Self { albedo: other }
    }
}

// lighting thing
pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
    pub intensity: f64,
}
impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p) * self.intensity
    }
}
impl DiffuseLight {
    pub fn new(albedo: Vec3, intensity: f64) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(albedo)),
            intensity,
        }
    }
    pub fn new_from_texture(other: Arc<dyn Texture>, intensity: f64) -> Self {
        Self {
            emit: other,
            intensity,
        }
    }
}

// Metal Material
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64, // Fuzzy reflection
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Vec3::reflect(ray_in.dir.unit(), rec.normal); // the reflected dir
        let scattered = Ray::new(rec.p, reflected + Vec3::rand_in_unit_sphere() * self.fuzz); // the reflected ray
        if scattered.dir * rec.normal > 0.0 {
            // whether the reflected ray and the normal are in the same side
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
impl Metal {
    pub fn new(_al: Vec3, _fuzz: f64) -> Self {
        Self {
            albedo: _al,
            fuzz: _fuzz,
        }
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}
impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_dir = ray_in.dir.unit();

        let cos_theta = if (-unit_dir) * rec.normal < 1.0 {
            (-unit_dir) * rec.normal
        } else {
            1.0
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            // Total internal reflection
            Some((
                Vec3::ones(),
                Ray::new(rec.p, Vec3::reflect(unit_dir, rec.normal)),
            ))
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            // proportion: some rays reflect & some refract
            if rand::random::<f64>() < reflect_prob {
                // reflect
                Some((
                    Vec3::ones(),
                    Ray::new(rec.p, Vec3::reflect(unit_dir, rec.normal)),
                ))
            } else {
                // refract
                Some((
                    Vec3::ones(),
                    Ray::new(rec.p, Vec3::refract(unit_dir, rec.normal, etai_over_etat)),
                ))
            }
        }
    }
}
impl Dielectric {
    pub fn new(_ref: f64) -> Self {
        Self { ref_idx: _ref }
    }
}

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}
impl Material for Isotropic {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        Some((
            self.albedo.value(rec.u, rec.v, rec.p),
            Ray::new(rec.p, Vec3::rand_in_unit_sphere()),
        ))
    }
}
impl Isotropic {
    pub fn new_from_color(albedo: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(albedo)),
        }
    }
    pub fn new_from_texture(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}
