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
    let aspect_ratio = 1.0;
    let width = 800;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples = 256;    
    let max_depth = 10;

    let scene = Scene::new(width, height, samples, max_depth);
    
    // camera
    let eye = Point3::new(0.0, 0.5, -2.3);
    let lookat = Point3::new(0.0, 0.5, 0.5);
    let view_distance = 600.0;
    let camera = PinholeCamera::new(Box::new(scene), &eye, &lookat, view_distance);

    // world
    let mut world = World::new(Box::new(camera::Camera::Pinhole(camera)));
    world.build();
    world.camera.render(&world);
}