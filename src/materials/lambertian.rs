use nalgebra::Vector3;

use crate::{render::{hitrecord::HitRecord, ray::Ray}, misc::{near_zero, random_unit_vector}};

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vector3<f64>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f64>) -> Lambertian {
        Lambertian {
            albedo,
        }
    }

    pub fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3<f64>, ray_scattered: &mut Ray) -> bool {
        let scatter_direction = hit_record.normal + random_unit_vector();
        if near_zero(&scatter_direction) {
            ray_scattered.direction = hit_record.normal;
        } else {
            ray_scattered.direction = scatter_direction;
        }
        ray_scattered.origin = hit_record.hit_point;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        true
    }
}