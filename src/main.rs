use nalgebra::Point3;
use objects::world::World;
use render::{hitrecord, camera::Renderer, pinholecamera::PinholeCamera};
use crate::render::{camera, scene::Scene};

mod objects;
mod materials;
mod render;
mod misc;

fn main() {
    // scene
    let aspect_ratio = 16.0 / 9.0;
    let width = 1920;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples = 100;
    let max_depth = 50;

    let scene = Scene::new(width, height, samples, max_depth);
    
    // camera
    let eye = Point3::new(0.0, 0.5, -2.3);
    let lookat = Point3::new(0.0, 0.5, 0.5);
    let view_distance = 500.0;
    let camera = PinholeCamera::new(Box::new(scene), &eye, &lookat, view_distance);

    // world
    let mut world = World::new(Box::new(camera::Camera::Pinhole(camera)));
    world.build();
    world.camera.render(&world);
}