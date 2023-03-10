use nalgebra::{Vector3, Point3};
use crate::{materials::diffuse::Diffuse, materials::{material::Material::{self, DiffMat}}};

use super::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub material: Material,
    pub hit_point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub depth: i32,
    pub t: f64,
    pub front_face: bool,
    pub is_light: bool,
    pub light: Vector3<f64>,
}

impl HitRecord {
    pub fn new(depth: i32) -> HitRecord{
        HitRecord {
            hit_point: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            depth: depth,
            material: DiffMat(Diffuse::new(Vector3::new(0.5, 0.5, 0.5), 0.79)),
            is_light: false,
            light: Vector3::new(5.0, 5.0, 5.0)
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f64>) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }

}
