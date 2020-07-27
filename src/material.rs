use crate::hittable::HitRecord;
use crate::ray::*;
use crate::shared_tools::*;
use crate::vec3::*;

// TRAIT Material
pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }
}
//*******************

// Lambertian Material
pub struct Lambertian {
    pub albedo: Vec3,
}
impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let scatter_dir = rec.normal + Vec3::random_unit_vector(); // Lambertian scattering
        Some((self.albedo, Ray::new(rec.p, scatter_dir)))
    }
}
impl Lambertian {
    pub fn new(_al: Vec3) -> Self {
        Self { albedo: _al }
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
