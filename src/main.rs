extern crate cgmath;
mod primitive;
mod util;
mod primitives;
mod ppm;
mod scene;
mod light;
mod material;

use primitive::*;
use primitives::*;
use util::*;
use cgmath::*;
use scene::*;
use std::f64;
use material::*;

use ppm::*;

fn main() {
    let width = 800;
    let height = 800;

    let mut scene = Scene::new();

    let material1 = Material::new(Vector3::new(0.3f64, 0.4f64, 0.2f64), 0.8f64, 0.5f64);
    let material2 = Material::new(Vector3::new(1f64, 1f64, 1f64), 1f64, 0.2f64);
    let material3 = Material::new(Vector3::new(1f64, 1f64, 1f64), 0.5f64, 0.5f64);
    let material4 = Material::new(Vector3::new(0.8f64, 0.2f64, 0.3f64), 0.9f64, 0.1f64);

    let sphere1 = PrimSphere::new(Point3::new(10f64, 0f64, 0f64), 2f64, material1);
    let sphere2 = PrimSphere::new(Point3::new(15f64, 5f64, 3f64), 4f64, material4);
    scene.primitives.push(Box::new(sphere1));
    scene.primitives.push(Box::new(sphere2));

    let light1 = PrimSphere::new(Point3::new(0f64, 5f64, 5f64), 2f64, material2);
    let light2 = PrimSphere::new(Point3::new(-15f64, -30f64, -30f64), 3f64, material3);
    scene.lights.push(Box::new(light1));
    scene.lights.push(Box::new(light2));

    let camera = Point3::new(-60f64, 0f64, 0f64);
    let img_x = -50f64;
    let img_max_y = 1f64;
    let img_min_y = -1f64;
    let img_max_z = 1f64;
    let img_min_z = -1f64;


    // let hit = sphere.intersects(&Ray3::new(camera, Vector3::new(1f64, 0.04f64, 0f64)));
    // println!("{:?}", hit);
    let mut image = PPM::new(height, width);
    for y in 0..width {
        for z in 0..height {
            let img_point = Point3::new(
                img_x,
                img_min_y + (img_max_y - img_min_y) * (y as f64 / width as f64),
                img_min_z + (img_max_z - img_min_z) * (z as f64/ height as f64)
            );
            let pixel_ray = Ray3::new(camera, img_point.sub_p(&camera).normalize());
            let color = scene.raytrace(&pixel_ray);
            let pixel = RGB {r: color.0, g: color.1, b: color.2};
            image.set_pixel(y, z, pixel);
        }
    }
    image.write_file("image.ppm");
}
