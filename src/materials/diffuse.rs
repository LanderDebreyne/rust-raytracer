use nalgebra::Vector3;

use crate::{render::{hitrecord::HitRecord, ray::Ray}, misc::{near_zero, random_unit_vector}};

#[derive(Clone, Copy)]
pub struct Diffuse {
    pub albedo: Vector3<f64>,
    pub reflectivity: f64,
}

impl Diffuse {
    pub fn new(albedo: Vector3<f64>, reflectivity: f64) -> Diffuse {
        Diffuse {
            albedo,
            reflectivity
        }
    }

    pub fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord, ray_scattered: &mut Ray) -> bool {
        let scatter_direction = (hit_record.normal + random_unit_vector()).normalize();
        if near_zero(&scatter_direction) {
            ray_scattered.direction = hit_record.normal;
        } else {
            ray_scattered.direction = scatter_direction;
        }
        ray_scattered.origin = hit_record.hit_point;
        true
    }
}