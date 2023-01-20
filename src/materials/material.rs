use nalgebra::Vector3;

use crate::{hitrecord::HitRecord, render::ray::Ray};

use super::{diffuse::Diffuse, specular::Specular};

#[derive(Clone, Copy)]
pub enum Material {
    DiffMat(Diffuse),
    SpecMat(Specular),
}

impl Scatter for Material {

    fn albedo(&self) -> Vector3<f64> {
        match self {
            Material::DiffMat(diffuse) => diffuse.albedo,
            Material::SpecMat(specular) => specular.albedo,
        }
    }

    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, ray_scattered: &mut Ray) -> bool {
        match self {
            Material::DiffMat(diffuse) => diffuse.scatter(ray_in, hit_record,  ray_scattered),
            Material::SpecMat(specular) => specular.scatter(ray_in, hit_record,  ray_scattered),
        }
    }

    fn reflectivity(&self) -> f64 {
        match self {
            Material::DiffMat(diffuse) => diffuse.reflectivity,
            Material::SpecMat(specular) => specular.reflectivity,
        }
    }
}

pub trait Scatter: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, ray_scattered: &mut Ray) -> bool;
    fn albedo(&self) -> Vector3<f64>;
    fn reflectivity(&self) -> f64;
}