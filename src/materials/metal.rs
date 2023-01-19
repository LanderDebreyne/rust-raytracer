use nalgebra::Vector3;
use crate::{hitrecord::HitRecord, render::ray::Ray, misc::reflect};

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vector3<f64>,
}

impl Metal {
    pub fn new(albedo: Vector3<f64>) -> Metal {
        Metal {
            albedo,
        }
    }

    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation : &mut Vector3<f64>, ray_scattered: &mut Ray) -> bool {
        let refl = reflect(&ray_in.direction.normalize(), &hit_record.normal);
        ray_scattered.direction = refl.normalize();
        ray_scattered.origin = hit_record.hit_point;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        ray_scattered.direction.dot(&hit_record.normal) > 0.0
    }
}