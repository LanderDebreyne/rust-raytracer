use nalgebra::{Vector3, Point3};
use crate::{hitrecord::HitRecord, render::{camera::Camera, ray::Ray}};
use crate::materials::{material::Material::{DiffMat, SpecMat}, diffuse::Diffuse, specular::Specular};

use super::{object::{Object, Hit}, object::Object::{SphereObj, PlaneObj, TriangleObj, RectObj}, sphere::Sphere, triangle::Triangle, plane::Plane, rectangle::Rectangle};

pub struct World {
    pub objects: Vec<Box<Object>>,
    pub camera: Box<Camera>,
    pub lights: Vec<(Vector3<f64>, Box<Object>)>,
}

impl World {
    pub fn new(camera: Box<Camera>) -> World {
        let mut world = World {
            objects: Vec::new(),
            lights: Vec::new(),
            camera: camera,
        };
        world.build();
        world
    }

    pub fn add(&mut self, object: Box<Object>) {
        self.objects.push(object);
    }

    pub fn addlight(&mut self, ls: Vector3<f64>, light: Box<Object>) {
        self.lights.push((ls, light));
    }

    pub fn build(&mut self) -> () {
        let material_sphere = DiffMat(Diffuse::new(Vector3::new(0.7, 0.3, 0.3), 0.5));
        let material_sphere2 = SpecMat(Specular::new(0.9));
        let sphere = SphereObj(Sphere::new(Point3::new(0.4, 0.4, -0.4), 0.4, material_sphere));
        let sphere2 = SphereObj(Sphere::new(Point3::new(-0.3, 1.2, -0.6), 0.4, material_sphere2));
        let _triangle = TriangleObj(Triangle::new(Point3::new(-0.9, 1.9, 0.6), Point3::new(0.9, 1.9, 0.6), Point3::new(0.0, 0.1, 0.6), material_sphere2));
        let w = DiffMat(Diffuse::new(Vector3::new(1.0, 1.0, 1.0), 0.5));
        let rw = DiffMat(Diffuse::new(Vector3::new(1.0, 0.9, 0.9), 0.5));
        let r = DiffMat(Diffuse::new(Vector3::new(1.0, 0.1, 0.1), 0.5));
        let g = DiffMat(Diffuse::new(Vector3::new(0.1, 1.0, 0.1), 0.5));
        let left = PlaneObj(Plane::new(Point3::new(-1.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), r));
        let back = PlaneObj(Plane::new(Point3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, -1.0), w));
        let right = PlaneObj(Plane::new(Point3::new(1.0, 0.0, 0.0), Vector3::new(-1.0, 0.0, 0.0), g));
        let ceil = PlaneObj(Plane::new(Point3::new(0.0, 2.0, 0.0), Vector3::new(0.0, -1.0, 0.0), w));
        let floor = PlaneObj(Plane::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), w));
        let light = RectObj(Rectangle::new(Point3::new(-0.25, 1.9, 0.0), Vector3::new(0.5, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.5), rw));
        self.add(Box::new(sphere));
        self.add(Box::new(sphere2));
        self.add(Box::new(back));
        self.add(Box::new(left));
        self.add(Box::new(right));
        self.add(Box::new(ceil));
        self.add(Box::new(floor));
        self.addlight(Vector3::new(2.5, 1.5, 2.0) ,Box::new(light));
    }

}

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let depth = hit_record.depth;
        let mut temp_hit_record = HitRecord::new(0);
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_hit_record) {
                hit_anything = true;
                let t = temp_hit_record.clone();
                closest_so_far = t.t;
                *hit_record = temp_hit_record;
            }
        }

        for light in &self.lights {
            if light.1.hit(r, t_min, closest_so_far, &mut temp_hit_record) {
                hit_anything = true;
                let t = temp_hit_record.clone();
                closest_so_far = t.t;
                *hit_record = temp_hit_record;
                hit_record.is_light = true;
                hit_record.light = light.0;
            }
        }

        hit_record.depth = depth - 1;

        hit_anything
    }

    fn light_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for object in &self.objects {
            if object.light_hit(r, t_min, t_max) {
                return true;
            }
        }

        for light in &self.lights {
            if light.1.light_hit(r, t_min, t_max) {
                return true;
            }
        }
        false
    }
}