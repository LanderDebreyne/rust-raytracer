use nalgebra::Vector3;

use crate::{hitrecord::HitRecord, render::ray::Ray};

use super::{lambertian::Lambertian, metal::Metal};

#[derive(Clone, Copy)]
pub enum Material {
    MatLam(Lambertian),
    MatMet(Metal),
}

impl Scatter for Material {

    fn albedo(&self) -> Vector3<f64> {
        match self {
            Material::MatLam(lambertian) => lambertian.albedo,
            Material::MatMet(metal) => metal.albedo,
        }
    }

    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3<f64>, ray_scattered: &mut Ray) -> bool {
        match self {
            Material::MatLam(lambertian) => lambertian.scatter(ray_in, hit_record, attenuation, ray_scattered),
            Material::MatMet(metal) => metal.scatter(ray_in, hit_record, attenuation, ray_scattered),
        }
    }
}

pub trait Scatter: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3<f64>, ray_scattered: &mut Ray) -> bool;
    fn albedo(&self) -> Vector3<f64>;
}