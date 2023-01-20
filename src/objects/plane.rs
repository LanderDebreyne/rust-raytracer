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

    pub fn light_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let t = (self.p - ray.origin).dot(&self.normal) / ray.direction.dot(&self.normal);

        if t < t_min || t > t_max {
            return false;
        }

        true
    }

    // not actually correct because of the margin, but it's a start
    pub fn rand_pos(&self) -> Point3<f64> {
        let margin = 200.0;
        let x = (self.p.x + margin * (rand::random::<f64>() - 0.5)) * self.normal.x;
        let y = (self.p.y + margin * (rand::random::<f64>() - 0.5)) * self.normal.y;
        let z = (self.p.z + margin * (rand::random::<f64>() - 0.5)) * self.normal.z;
        Point3::new(x, y, z)
    }

    pub fn area(&self) -> f64 {
        400000.0
    }

    pub fn normalp(&self, _p: &Point3<f64>) -> Vector3<f64> {
        self.normal
    }
}