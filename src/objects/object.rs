use crate::{hitrecord::HitRecord, render::ray::Ray};

use super::{sphere::Sphere, triangle::Triangle, plane::Plane, rectangle::Rectangle};

pub enum Object {
    SphereObj(Sphere),
    TriangleObj(Triangle),
    PlaneObj(Plane),
    RectObj(Rectangle)
    //Mesh(Mesh),
}

impl Hit for Object {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        match self {
            Object::SphereObj(sphere) => sphere.hit(r, t_min, t_max, hit_record),
            Object::TriangleObj(triangle) => triangle.hit(r, t_min, t_max, hit_record),
            Object::PlaneObj(plane) => plane.hit(r, t_min, t_max, hit_record),
            Object::RectObj(rect) => rect.hit(r, t_min, t_max, hit_record),
            //Object::Mesh(mesh) => mesh.hit(r, t_min, t_max, hit_record),
        }
    }
}

pub trait Hit: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}