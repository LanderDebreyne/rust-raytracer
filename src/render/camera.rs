use crate::objects::world::World;

use super::pinholecamera::PinholeCamera;



pub trait Renderer {
    fn render(&self, world: &World) -> ();
}

pub enum Camera {
    Pinhole(PinholeCamera),
}

impl Renderer for Camera {
    fn render(&self, world: &World) -> () {
        match self {
            Camera::Pinhole(camera) => camera.render(world),
        }
    }
}