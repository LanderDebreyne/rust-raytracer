use nalgebra::{Point3, Vector3};
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use image::{ImageBuffer, RgbImage};
use std::f64::INFINITY;
use super::{scene::Scene, hitrecord::HitRecord, ray::Ray};
use crate::{objects::{world::World, object::{Hit, Pos}}, render::hitrecord, materials::material::Scatter};


pub struct PinholeCamera {
    pub eye: Point3<f64>,
    pub lookat: Point3<f64>,
    pub up: Vector3<f64>,
    pub u: Vector3<f64>,
    pub v: Vector3<f64>,
    pub w: Vector3<f64>,
    pub exposure_time: f64,
    pub d: f64,
    pub zoom: f64,
    pub scene: Box<Scene>,
}



impl PinholeCamera {
    pub fn new(scene: Box<Scene>,eye: &Point3<f64>, lookat: &Point3<f64>, view_distance: f64) -> PinholeCamera {
        let up = Vector3::new(0.0, 1.0, 0.0);
        let w = (eye - lookat).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        PinholeCamera {
            eye: *eye,
            lookat: *lookat,
            up,
            u,
            v,
            w,
            exposure_time: 1.0,
            d: view_distance,
            zoom: 1.0,
            scene: scene,
        }
    }

    pub fn render(&self, world: &World) -> () {
        println!("Pinhole camera rendering");
        // render   
        let mut pixels = vec![0; self.scene.width * self.scene.height * 3];
        let pixrows: Vec<(usize, &mut [u8])> = pixels.chunks_mut(self.scene.width * 3).enumerate().collect();
        println!("Rendering... ");
        let samples = self.scene.samples;
        pixrows.into_par_iter().for_each(|(i, row)| {
            let mut rng = rand::thread_rng();
            row.chunks_mut(3).enumerate().into_iter().for_each(|(j, pixel)| {
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                (0..samples).into_iter().for_each(|sample| {
                    // jittered sampling
                    let vs = ((sample / 8) as f64) / (samples as f64).sqrt();
                    let us = ((sample % 8) as f64) / (samples as f64).sqrt();
                    let ss = 1.0 / (sample as f64).sqrt();
                    let u = ((j as f64) + us +  (ss * rng.gen::<f64>())) / (self.scene.width-1) as f64;
                    let v = ((i as f64) + vs + (ss * rng.gen::<f64>())) / (self.scene.height-1)  as f64;
                    let ray = self.ray(u, v);
                    let mut hit_record = hitrecord::HitRecord::new(self.scene.max_depth.try_into().unwrap());
                    let pixel_color = self.ray_color(&ray, &world, &mut hit_record);
                    r = r + (pixel_color.x / (samples as f64));
                    g = g + (pixel_color.y / (samples as f64));
                    b = b + (pixel_color.z / (samples as f64));
                });
                let scale = 1.0;
                pixel[0] = ((scale*r).sqrt().clamp(0.0, 0.999)*256.0) as u8;
                pixel[1] = ((scale*g).sqrt().clamp(0.0, 0.999)*256.0) as u8;
                pixel[2] = ((scale*b).sqrt().clamp(0.0, 0.999)*256.0) as u8;
            });
        });
        println!("Done!");

        // reverse
        println!("Reversing image... ");
        pixels.reverse();
        println!("Done!");
        // saving image
        println!("Writing image to file... ");
        // Construct a new RGB ImageBuffer with the specified width and height.
        let mut img: RgbImage = ImageBuffer::new(self.scene.width as u32, self.scene.height as u32);
        img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let i = (y * (self.scene.width as u32) + x) as usize;
            pixel[2] = pixels[i * 3];
            pixel[1] = pixels[i * 3 + 1];
            pixel[0] = pixels[i * 3 + 2];
        });
        img.save("image.png").unwrap();
        println!("Done!");
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let horizontal = self.u * self.scene.width as f64 * self.zoom;
        let vertical = self.v * self.scene.height as f64 * self.zoom;
        let lower_left_corner = self.eye - horizontal / 2.0 - vertical / 2.0 - self.w * self.d;
        let horizontal = horizontal * u;
        let vertical = vertical * v;
        let origin = self.eye;
        let direction = lower_left_corner + horizontal + vertical - origin;
        Ray::new(origin, direction)
    }

    pub fn ray_color(&self, r: &Ray, world: &World, hit_record: &mut HitRecord) -> Vector3<f64> {
        if hit_record.depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        let hit = world.hit(r, 0.001, INFINITY, hit_record);
        if hit {
            if hit_record.is_light {
                return hit_record.light;
            }
            let mut dir = Vector3::new(0.0, 0.0, 0.0);
            // direct illumination
            // check if lights are visible
            for light in &world.lights {
                let light_pos = light.1.rand_pos();
                let light_dir = light_pos - hit_record.hit_point;
                let light_intensity = light_dir.dot(&light.1.normalp(&light_pos)); // cosine of angle between light and normal
                if light_intensity < 1e-6 {
                    let light_distance = light_dir.norm();
                    let light_dir = light_dir.normalize();
                    let light_ray = Ray::new(hit_record.hit_point, light_dir);
                    let light_hit = world.light_hit(&light_ray, 0.001, light_distance-0.001);
                    if !light_hit {
                        dir += light.0 * light_intensity.abs() * light.1.area() / (light_distance * light_distance);
                    }
                }
            }
            dir = dir.component_mul(&hit_record.material.albedo());
            // indirect illumination
            let mut scatter = Ray::new(hit_record.hit_point, hit_record.normal); // dummy
            if hit_record.material.scatter(r, hit_record, &mut scatter) {
                return dir + hit_record.material.albedo().normalize().scale(hit_record.material.reflectivity()).component_mul(&self.ray_color(&scatter, world, hit_record));
            }
        }
        Vector3::new(0.0, 0.0, 0.0)
    }
}