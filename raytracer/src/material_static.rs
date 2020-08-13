#![allow(dead_code)]

use crate::hittable::HitRecord;
use crate::pdf::*;
use crate::ray::*;
use crate::shared_tools::*;
use crate::texture::*;
use crate::vec3::*;
use std::f64::consts::PI;
use std::sync::Arc;

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>, // TODO: enum
    pub attenuation: Vec3,
    pub pdf_ptr: Option<Arc<dyn PDF>>,
}

// TRAIT Material
pub trait Material: Send + Sync {
    // return: color, ray, pdf
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn scattering_pdf(&self, _ray_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
    // return: color
    fn emitted(&self, _ray_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::zero()
    }
}
//*******************

// Lambertian Material
pub struct Lambertian<T: Texture> {
    pub albedo: Arc<T>,
}
impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        // let uvw = ONB::build_from_w(&rec.normal);
        // let direction = uvw.local(&Vec3::rand_cosine_direction());
        // let scatter_dir = rec.normal + Vec3::random_unit_vector(); // Lambertian scattering
        // let scattered = Ray::new(rec.p, direction.unit());
        Some(ScatterRecord {
            specular_ray: None,
            attenuation: self.albedo.value(rec.u, rec.v, rec.p), // get color value in texture
            pdf_ptr: Some(Arc::new(CosinePDF::build_from_w(&rec.normal))),
        })
    }

    fn scattering_pdf(&self, _ray_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos = rec.normal * scattered.dir.unit();
        if cos < 0.0 {
            0.0
        } else {
            cos / PI
        }
    }
}
impl<T: Texture> Lambertian<T> {
    pub fn new_from_texture(other: Arc<T>) -> Self {
        Self { albedo: other }
    }
}
// impl<SolidColor: Texture> Lambertian<SolidColor> {
//     pub fn new(_al: Vec3) -> Self {
//         Self {
//             albedo: Arc::new(SolidColor::new(_al)),
//         }
//     }
// }
// impl From<Arc<dyn Texture>> for Lambertian {
//     fn from(other: Arc<dyn Texture>) -> Self {
//         Self { albedo: other }
//     }
// }

// lighting thing
pub struct DiffuseLight<T: Texture> {
    pub emit: Arc<T>,
    pub intensity: f64,
}
impl<T: Texture> Material for DiffuseLight<T> {
    fn emitted(&self, _ray_in: &Ray, _rec: &HitRecord, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p) * self.intensity
    }
}
impl<T: Texture> DiffuseLight<T> {
    // pub fn new(albedo: Vec3, intensity: f64) -> Self {
    //     Self {
    //         emit: Arc::new(SolidColor::new(albedo)),
    //         intensity,
    //     }
    // }
    pub fn new_from_texture(other: Arc<T>, intensity: f64) -> Self {
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
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(ray_in.dir.unit(), rec.normal); // the reflected dir
        Some(ScatterRecord {
            specular_ray: Some(Ray::new(
                rec.p,
                reflected + Vec3::rand_in_unit_sphere() * self.fuzz,
            )),
            attenuation: self.albedo,
            pdf_ptr: None,
        })
        // let scattered = Ray::new(rec.p, reflected + Vec3::rand_in_unit_sphere() * self.fuzz); // the reflected ray
        // if scattered.dir * rec.normal > 0.0 {
        //     // whether the reflected ray and the normal are in the same side
        //     Some((self.albedo, scattered, 0.0))
        // } else {
        //     None
        // }
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
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        // None
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
            Some(ScatterRecord {
                specular_ray: Some(Ray::new(rec.p, Vec3::reflect(unit_dir, rec.normal))),
                attenuation: Vec3::ones(),
                pdf_ptr: None,
            })
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            // proportion: some rays reflect & some refract
            if rand::random::<f64>() < reflect_prob {
                // reflect
                Some(ScatterRecord {
                    specular_ray: Some(Ray::new(rec.p, Vec3::reflect(unit_dir, rec.normal))),
                    attenuation: Vec3::ones(),
                    pdf_ptr: None,
                })
            } else {
                // refract
                Some(ScatterRecord {
                    specular_ray: Some(Ray::new(
                        rec.p,
                        Vec3::refract(unit_dir, rec.normal, etai_over_etat),
                    )),
                    attenuation: Vec3::ones(),
                    pdf_ptr: None,
                })
            }
        }
    }
}
impl Dielectric {
    pub fn new(_ref: f64) -> Self {
        Self { ref_idx: _ref }
    }
}

pub struct Isotropic<T: Texture> {
    pub albedo: Arc<T>,
}
impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            specular_ray: Some(Ray::new(rec.p, Vec3::rand_in_unit_sphere())),
            attenuation: self.albedo.value(rec.u, rec.v, rec.p),
            pdf_ptr: None,
        })
        // Some((
        //     self.albedo.value(rec.u, rec.v, rec.p),
        //     Ray::new(rec.p, Vec3::rand_in_unit_sphere()),
        //     0.0,
        // ))
    }
}
impl<T: Texture> Isotropic<T> {
    // pub fn new_from_color(albedo: Vec3) -> Self {
    //     Self {
    //         albedo: Arc::new(SolidColor::new(albedo)),
    //     }
    // }
    pub fn new_from_texture(albedo: Arc<T>) -> Self {
        Self { albedo }
    }
}
