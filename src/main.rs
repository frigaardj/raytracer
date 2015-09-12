extern crate cgmath;
extern crate rand;
mod intersectable;
mod util;
mod primitives;
mod ppm;

use rand::*;
use intersectable::*;
// use intersectable::Intersectable;
use primitives::*;
use util::*;
use cgmath::*;

use ppm::*;

fn main() {
    let width = 800;
    let height = 600;


    let mut scene : Vec<Box<Intersectable>>= Vec::new();
    let sphere_center = Point3::new(10f64, 0f64, 0f64);
    let sphere = PrimSphere::new(sphere_center, 2f64);
    scene.push(Box::new(sphere));

    let camera = Point3::new(-30f64, 0f64, 0f64);
    let img_x = -20f64;
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
            // println!("{:?}", img_point);
            let pixel_ray = Ray3::new(camera, img_point.sub_p(&camera).normalize());
            for intersectable in scene.iter() {
                let hitpoint = intersectable.intersects(&pixel_ray);
                let color = match hitpoint {
                    Some(point) => {
                        println!("Hit");
                        255u8
                    },
                    None => 0u8,
                };

                let pixel = RGB {r: color, g: color, b: color};
                image.set_pixel(y, z, pixel);

            }

            // let pixel = RGB {r: random::<u8>(), g: random::<u8>(), b: random::<u8>()};


            // image.set_pixel(x, y, pixel);
        }
    }
    image.write_file("image.ppm");
}
