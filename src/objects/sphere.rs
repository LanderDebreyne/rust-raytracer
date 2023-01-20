use nalgebra::{Point3, Vector3};

use crate::materials::material::Material;
use crate::hitrecord::HitRecord;
use crate::render::ray::Ray;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Material) -> Sphere {
        Sphere { center, radius, material }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let half_b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
    
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
    
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
    
        hit_record.t = root;
        hit_record.hit_point = r.at(hit_record.t);
        hit_record.normal = (hit_record.hit_point - self.center) / self.radius;
        let normal = hit_record.normal.clone();
        hit_record.set_face_normal(&r, &normal);
        hit_record.material = self.material.clone();
    
        true
    }

    pub fn light_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let half_b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
    
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
    
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
    
        true
    }

    pub fn rand_pos(&self) -> Point3<f64> {
        let theta = rand::random::<f64>() * 2.0 * std::f64::consts::PI;
        let phi = rand::random::<f64>() * 2.0 * std::f64::consts::PI;
        let x = self.radius * theta.cos() * phi.cos();
        let y = self.radius * theta.sin() * phi.cos();
        let z = self.radius * phi.sin();
        self.center + Vector3::new(x, y, z)
    }

    pub fn area (&self) -> f64 {
        4.0 * std::f64::consts::PI * self.radius * self.radius
    }

    pub fn normalp(&self, p: &Point3<f64>) -> Vector3<f64> {
        (p - self.center) / self.radius
    }

}