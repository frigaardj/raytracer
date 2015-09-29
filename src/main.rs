extern crate cgmath;
extern crate time;
mod primitive;
mod util;
mod primitives;
mod ppm;
mod scene;
mod material;

use primitive::*;
use util::*;
use primitives::*;
use cgmath::*;
use scene::*;
use material::*;

use ppm::*;

fn main() {
    let width = 1000;
    let height = 1000;

    let white_matte = Material::new(Vector3::new(1f64, 1f64, 1f64), 1f64, 0f64);
    let light_blue = Material::new(Vector3::new(0.4f64, 0.2f64, 1f64), 1f64, 0f64);
    let white_mirror = Material::new(Vector3::new(1f64, 1f64, 1f64), 0.3f64, 0.9f64);
    let light_red = Material::new(Vector3::new(1f64, 0.2f64, 0.3f64), 19f64, 0f64);

    let s1 = PrimSphere::new(Point3::new(10f64, 0f64, 0f64), 3f64, &white_mirror, "Big spehre".to_string());
    let s2 = PrimSphere::new(Point3::new(10f64, 7f64, 0f64), 3f64, &white_mirror, "Small Sphere".to_string());
    
    let p1 = PrimPlane::new(Point3::new(30f64, 0f64, 0f64), Vector3::new(-1f64, 0f64, 0f64), &light_blue, "Back wall".to_string());
    let p2 = PrimPlane::new(Point3::new(0f64, -7f64, 0f64), Vector3::new(0f64, 1f64, 0f64), &white_matte, "RH wall".to_string());
    let p3 = PrimPlane::new(Point3::new(0f64, 0f64, -5f64), Vector3::new(0f64, 0f64, 1f64), &light_red, "Bottom wall".to_string());

    let l1 = PrimSphere::new(Point3::new(-50f64, 20f64, 20f64), 0.2f64, &white_matte, "Left Light".to_string());
    let l2 = PrimSphere::new(Point3::new(-20f64, 63f64, 20f64), 0.2f64, &white_matte, "RH Light".to_string());
    // let l2 = PrimSphere::new(Point3::new(0f64, -3f64, 0f64), 1f64, &material4);
    
    let mut prims: Vec<&Primitive> = Vec::new();
    let mut lights: Vec<&PrimSphere> = Vec::new();

    prims.push(&s1);
    prims.push(&s2);
    prims.push(&p1);
    prims.push(&p2);
    prims.push(&p3);

    lights.push(&l1);
    lights.push(&l2);


    let scene = Scene { 
        primitives: prims, 
        lights: lights
    };

    let camera = Point3::new(-100f64, 0f64, 0f64);
    let img_x = -90f64;
    let img_max_y = 1f64;
    let img_min_y = -1f64;
    let img_max_z = 1f64;
    let img_min_z = -1f64;


    // let hit = sphere.intersects(&Ray3::new(camera, Vector3::new(1f64, 0.04f64, 0f64)));
    // println!("{:?}", hit);
    let start_time = time::now();
    println!("Started rendering at {:?}", start_time);
    let mut image = PPM::new(height, width);
    for y in 0..width {
        if y % 100 == 0 {
            println!("Row {}/{}", y, width);   
        }
        for z in 0..height {
            // println!("Computing pixel {};{}", y, z);
            let img_point = Point3::new(
                img_x,
                lerp(img_max_y, img_min_y, y as f64 / width as f64),
                lerp(img_max_z, img_min_z, z as f64 / height as f64)
            );
            let pixel_ray = Ray3::new(camera, img_point.sub_p(&camera).normalize());
            let color = scene.raytrace(&pixel_ray, 5);
            let pixel = RGB {
                r: (color.x.min(1f64) * 255f64) as u8,
                g: (color.y.min(1f64) * 255f64) as u8,
                b: (color.z.min(1f64) * 255f64) as u8
            };
            image.set_pixel(y, z, pixel);
        }
    }
    let end_time = time::now();
    println!("Finished rendering at {:?}", end_time);
    let duration = end_time - start_time;
    println!("Rendering took {:?}", duration);
    image.write_file("image.ppm");
}
