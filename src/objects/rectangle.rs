use nalgebra::{Point3, Vector3};

use crate::materials::material::Material;
use crate::hitrecord::HitRecord;
use crate::render::ray::Ray;

#[derive(Clone)]
pub struct Rectangle {
    pub p: Point3<f64>,
    pub x: Vector3<f64>,
    pub y: Vector3<f64>,
    pub material: Material,
}

impl Rectangle {
    pub fn new(p: Point3<f64>, x: Vector3<f64>, y: Vector3<f64>, material: Material) -> Rectangle {
        Rectangle { p, x, y, material }
    }

    pub fn normal(&self) -> Vector3<f64> {
        self.x.cross(&self.y).normalize()
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let normal = self.normal();
        let det = r.direction.dot(&normal);
        if det > -1e-6 {
            return false;
        }
        let det2 = (self.p - r.origin).dot(&normal);
        if det2 > -1e-6 {
            return false;
        }
        let t = det2 / det;
        if t < t_min || t > t_max {
            return false;
        }
        let hit_point = r.at(t);
        let x = (hit_point - self.p).dot(&self.x.normalize());
        let y = (hit_point - self.p).dot(&self.y.normalize());
        if x < 0.0 || x > self.x.norm() || y < 0.0 || y > self.y.norm() {
            return false;
        }
        hit_record.t = t;
        hit_record.hit_point = hit_point;
        hit_record.normal = normal;                
        hit_record.set_face_normal(&r, &normal);
        hit_record.material = self.material.clone();
        true
    }

    pub fn light_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let normal = self.normal();
        let det = r.direction.dot(&normal);
        if det > -1e-6 {
            return false;
        }
        let det2 = (self.p - r.origin).dot(&normal);
        if det2 > -1e-6 {
            return false;
        }
        let t = det2 / det;
        if t < t_min || t > t_max {
            return false;
        }
        let hit_point = r.at(t);
        let x = (hit_point - self.p).dot(&self.x.normalize());
        let y = (hit_point - self.p).dot(&self.y.normalize());
        if x < 0.0 || x > self.x.norm() || y < 0.0 || y > self.y.norm() {
            return false;
        }
        return true;
    }

    pub fn rand_pos(&self) -> Point3<f64> {
        self.p + self.x * rand::random::<f64>() + self.y * rand::random::<f64>()
    }

    pub fn area(&self) -> f64 {
        self.x.norm() * self.y.norm()
    }

    pub fn normalp(&self, _p: &Point3<f64>) -> Vector3<f64> {
        self.x.cross(&self.y).normalize()
    }
}