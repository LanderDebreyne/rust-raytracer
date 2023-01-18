use nalgebra::Vector3;

use crate::{hitrecord::HitRecord, render::ray::Ray};

pub mod material;
pub mod metal;
pub mod lambertian;


pub trait Scatter: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3<f64>, ray_scattered: &mut Ray) -> bool;
}