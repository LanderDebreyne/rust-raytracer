use nalgebra::{Point3, Vector3};

use crate::materials::material::Material;
use crate::hitrecord::HitRecord;
use crate::render::ray::Ray;

#[derive(Clone)]
pub struct Triangle {
    pub p1: Point3<f64>,
    pub p2: Point3<f64>,
    pub p3: Point3<f64>,
    pub normals: [Vector3<f64>; 3],
    pub material: Material,
}

impl Triangle {
    pub fn new(p1: Point3<f64>, p2: Point3<f64>, p3: Point3<f64>, material: Material) -> Triangle {
        let n = (p2 - p1).cross(&(p3 - p1)).normalize();
        Triangle { p1, p2, p3, material, normals: [n; 3] }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
            let e1 = self.p2 - self.p1;
            let e2 = self.p3 - self.p1;
            let p = ray.direction.cross(&e2);
            let det = e1.dot(&p);
    
            // if determinant is near zero, ray lies in plane of triangle
            if det > -::std::f64::EPSILON && det < ::std::f64::EPSILON {
                return false
            }
    
            let inv_det = 1.0 / det;
            let s = ray.origin - self.p1;
            let beta = inv_det * s.dot(&p);
            if beta < 0.0 || beta > 1.0 { return false }
    
            let q = s.cross(&e1);
            let gamma = inv_det * ray.direction.dot(&q);
            if gamma < 0.0 || beta + gamma > 1.0 { return false }
    
            let t = inv_det * e2.dot(&q);
    
            if t < t_min || t > t_max {
                return false
            } else {
                let intersection_point = ray.origin + ray.direction.scale(t);
    
                let alpha = 1.0 - beta - gamma;
    
                // Interpolate normals at vertices to get normal
                let n = self.normals[0].scale(alpha) + self.normals[1].scale(beta) + self.normals[2].scale(gamma);

                hit_record.t = t;
                hit_record.hit_point = intersection_point;
                hit_record.normal = n;
                hit_record.material = self.material.clone();
            }

        true
    }
}