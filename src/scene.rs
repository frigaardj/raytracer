use light::*;
use primitive::*;
use cgmath::*;
use std::f64;
use material::*;

pub struct Scene {
    pub primitives: Vec<Box<Primitive>>,
    pub lights: Vec<Box<Light>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene { primitives: Vec::new(), lights: Vec::new() }
    }
    pub fn raytrace(&self, ray: &Ray3<f64>) -> (u8, u8, u8) {
    	// for light in self.lights.iter() {
    		// check if it hits a light directly.
    	// }

    	let mut closest_dist = f64::INFINITY;
    	let mut closest_prim: Option<&Box<Primitive>> = None;
        for primitive in self.primitives.iter() {
            match primitive.intersects(&ray) {
                Some(distance) => {
                    // println!("{:?}", primitive.normal(&point));
                    if (distance < closest_dist) {
                    	closest_dist = distance;
                    	closest_prim = Some(primitive);
                    }
                },
                _ => {}
            };
        }
        match closest_prim {
        	None => (0u8, 0u8, 0u8),
        	Some(primitive) => {
		        let hitpoint = ray.origin.add_v(&ray.direction.mul_s(closest_dist));
		        let mut color = Vector3::new(0f64, 0f64, 0f64);
		        for light in self.lights.iter() {
		        	let to_light = light.get_center().sub_p(&hitpoint).normalize();
		        	let normal = primitive.normal(&hitpoint);
		        	let dot = normal.dot(&to_light);
		        	if (dot > 0f64) {
		        		let prim_mat = primitive.material_at_point(&hitpoint);
		        		let light_col = light.get_color();
		        		let diff = dot * prim_mat.diffuse;
		        		// println!("{:?}", prim_mat.color.z);
		        		color.add_self_v(&prim_mat.color.mul_s(diff).mul_v(&light_col));
		        	}
		        }
		        ((color.x.min(1f64) * 255f64) as u8, (color.y.min(1f64) * 255f64) as u8, (color.z.min(255f64) * 255f64) as u8)
        	}
        }
    }
}