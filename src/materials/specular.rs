use nalgebra::Vector3;
use crate::{hitrecord::HitRecord, render::ray::Ray, misc::reflect};

#[derive(Clone, Copy)]
pub struct Specular {
    pub albedo: Vector3<f64>,
    pub reflectivity: f64,
}

// TODO: add paramater for fuzziness

impl Specular {
    pub fn new(reflectivity: f64) -> Specular {
        Specular {
            albedo: Vector3::new(0.01, 0.01, 0.01),
            reflectivity
        }
    }

    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, ray_scattered: &mut Ray) -> bool {
        let refl = reflect(&ray_in.direction.normalize(), &hit_record.normal);
        ray_scattered.direction = refl.normalize();
        ray_scattered.origin = hit_record.hit_point;
        ray_scattered.direction.dot(&hit_record.normal) > 0.0
    }
}