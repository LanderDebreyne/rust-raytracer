use nalgebra::{Point3, Vector3};

use crate::materials::material::Material;
use crate::hitrecord::HitRecord;
use crate::render::ray::Ray;

#[derive(Clone)]
pub struct Plane {
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: Material,
}

impl Plane {
    pub fn new(p: Point3<f64>, normal: Vector3<f64>, material: Material) -> Plane {
        Plane { p, material, normal }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let t = (self.p - ray.origin).dot(&self.normal) / ray.direction.dot(&self.normal);

        if t < t_min || t > t_max {
            return false;
        }

        hit_record.t = t;
        hit_record.hit_point = ray.at(t);
        hit_record.normal = self.normal;
        hit_record.material = self.material.clone();
        true
    }
}