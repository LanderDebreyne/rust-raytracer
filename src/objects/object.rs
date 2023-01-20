use nalgebra::{Point3, Vector3};

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

    fn light_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        match self {
            Object::SphereObj(sphere) => sphere.light_hit(r, t_min, t_max),
            Object::TriangleObj(triangle) => triangle.light_hit(r, t_min, t_max),
            Object::PlaneObj(plane) => plane.light_hit(r, t_min, t_max),
            Object::RectObj(rect) => rect.light_hit(r, t_min, t_max),
            //Object::Mesh(mesh) => mesh.light_hit(r, t_min, t_max),
        }
    }
}

impl Pos for Object {

    fn area(&self) -> f64 {
        match self {
            Object::SphereObj(sphere) => sphere.area(),
            Object::TriangleObj(triangle) => triangle.area(),
            Object::PlaneObj(plane) => plane.area(),
            Object::RectObj(rect) => rect.area(),
            //Object::Mesh(mesh) => mesh.area(),
        }
    }

    fn rand_pos(&self) -> Point3<f64> {
        match self {
            Object::SphereObj(sphere) => sphere.rand_pos(),
            Object::TriangleObj(triangle) => triangle.rand_pos(),
            Object::PlaneObj(plane) => plane.rand_pos(),
            Object::RectObj(rect) => rect.rand_pos(),
            //Object::Mesh(mesh) => mesh.rand_pos(),
        }
    }

    fn normalp(&self, p: &Point3<f64>) -> Vector3<f64> {
        match self {
            Object::SphereObj(sphere) => sphere.normalp(p),
            Object::TriangleObj(triangle) => triangle.normalp(p),
            Object::PlaneObj(plane) => plane.normalp(p),
            Object::RectObj(rect) => rect.normalp(p),
            //Object::Mesh(mesh) => mesh.normal(p),
        }
    }
}

pub trait Hit: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
    fn light_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool;
}

pub trait Pos: Sync + Send {
    fn rand_pos(&self) -> Point3<f64>;
    fn area(&self) -> f64;
    fn normalp(&self, p: &Point3<f64>) -> Vector3<f64>;
}