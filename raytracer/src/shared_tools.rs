use crate::vec3::*;
use std::f64::consts::PI;

pub fn degree_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    // anti-aliasing
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

// pub fn random_i32()

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let ii = i as f64;
                let jj = j as f64;
                let kk = k as f64;
                accum += (ii * u + (1.0 - ii) * (1.0 - u))
                    * (jj * v + (1.0 - jj) * (1.0 - v))
                    * (kk * w + (1.0 - kk) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }
    accum
}

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}
impl Perlin {
    const POINT_COUNT: u32 = 256;
    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = vec![];
        for i in 0..Self::POINT_COUNT {
            p.push(i as usize);
        }
        Self::permute(&mut p, Perlin::POINT_COUNT);
        p
    }

    fn permute(p: &mut Vec<usize>, n: u32) {
        for i in (0..n).rev() {
            let i = i as usize;
            let target = rand::random::<usize>() % (i + 1);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let ii = i as f64;
                    let jj = j as f64;
                    let kk = k as f64;
                    let weight_v = Vec3::new(u - ii, v - jj, w - kk);
                    accum += (ii * u + (1.0 - ii) * (1.0 - u))
                        * (jj * v + (1.0 - jj) * (1.0 - v))
                        * (kk * w + (1.0 - kk) * (1.0 - w))
                        * (c[i][j][k] * weight_v);
                }
            }
        }
        accum
    }

    pub fn new() -> Self {
        let mut ranvec = vec![];
        for _i in 0..Self::POINT_COUNT {
            ranvec.push(Vec3::rand(-1.0, 1.0).unit());
        }
        Self {
            ranvec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[(i + di as i32) as usize & 255]
                        ^ self.perm_y[(j + dj as i32) as usize & 255]
                        ^ self.perm_z[(k + dk as i32) as usize & 255]];
                }
            }
        }
        Self::perlin_interp(c, u, v, w)

        // let i = (4.0 * p.x) as i32 & 255;
        // let j = (4.0 * p.y) as i32 & 255;
        // let k = (4.0 * p.z) as i32 & 255;

        // self.ranfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }

    pub fn turb(&self, p: &Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut tmp_p = *p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(&tmp_p);
            weight *= 0.5;
            tmp_p *= 2.0;
        }

        accum.abs()
    }
}
