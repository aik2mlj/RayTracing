#![allow(dead_code)]

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
