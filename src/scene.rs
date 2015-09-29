use primitive::*;
use primitives::*;
use cgmath::*;
use std::f64;
use util::*;

pub struct Scene<'s> {
    pub primitives: Vec<&'s Primitive>,
    pub lights: Vec<&'s PrimSphere<'s>>
}

impl<'s> Scene<'s> {
    pub fn raytrace(&self, ray: &Ray3<f64>, depth: u8) -> Vector3<f64> {
    	if depth <= 0u8 {
    		return Vector3::new(0f64, 0f64, 0f64);
    	}
    	for light in &self.lights {
    		match light.intersects(&ray) {
    			Some(_) => {
    				println!("Direct hit on a light");
    				return Vector3::new(1f64, 1f64, 1f64);
    			}
    			_ => {}
    		}
    	}

    	// Find the closest primitive hit by the ray
    	let mut closest_dist = f64::INFINITY;
    	let mut closest_prim = None;
        for primitive in &self.primitives {
            match primitive.intersects(&ray) {
                Some(distance) => {
                	// println!("Hit!");
                    if distance < closest_dist {
                    	closest_dist = distance;
                    	closest_prim = Some(primitive);
                    }
                },
                _ => {}
            };
        }

        match closest_prim {
        	// We hit a primitive
        	Some(hit_primitive) => {
		        let hitpoint = ray.origin.add_v(&ray.direction.mul_s(closest_dist));
	        	let hit_normal = hit_primitive.normal(&hitpoint);
		        let mut color = Vector3::new(0f64, 0f64, 0f64);

		        // Generate shadow rays from hitpoint to all lightsources. Assumes point sources. 
		        for light in &self.lights {
		        	let mut shade = 1.0f64;
		        	let mut v_to_light = light.center.sub_p(&hitpoint);
		        	let light_dist = v_to_light.length();
		        	v_to_light.normalize_self();
		        	// Avoid hitting the primitive we are bouncing off..
		        	let r_to_light = Ray3::new(hitpoint.add_v(&v_to_light.mul_s(0.0001f64)), v_to_light);

		        	for shadow_primitive in &self.primitives {
		        		match shadow_primitive.intersects(&r_to_light) {
		        			Some(distance) => {
		        				if distance < light_dist {
		        					shade = 0f64;
			        				break;
		        				}
		        				// if hit_primitive.name() == shadow_primitive.name() && shadow_primitive.name().contains("Small") {
		        				// 	println!("shit");
		        				// 	println!("Shadow ray from {} hit {} at distance {}", hit_primitive.name(), shadow_primitive.name(), distance);
		        				// }
		        				// if shadow_primitive.name().contains("wall") {
		        				// 	println!("Shadow ray from {} hit {} at distance {}", hit_primitive.name(), shadow_primitive.name(), distance);
		        				// }
		        			},
		        			None => {}
		        		}
		        	}

		        	// diffuse shading
		        	let dot = hit_normal.dot(&r_to_light.direction);
		        	if dot > 0f64 {
		        		let prim_mat = hit_primitive.material();
		        		let light_col = light.material().color;
		        		let diff = dot * prim_mat.diffuse * shade;
		        		// println!("{:?}", prim_mat.color.z);
		        		color.add_self_v(&prim_mat.color.mul_s(diff).mul_v(&light_col));
		        	}
		        }

		        // Reflections
		        let refl = hit_primitive.material().reflection;
		        if refl > 0.0f64 {
		        	let reflected_v = mirror(&ray.direction, &hit_normal);
		        	let reflected_ray = Ray3::new(hitpoint.add_v(&reflected_v.mul_s(0.0001f64)), reflected_v);
		        	let reflected_color = self.raytrace(&reflected_ray, depth - 1).mul_s(refl).mul_v(&hit_primitive.material().color);
		        	color.add_self_v(&reflected_color);
		        }
		        color
        	}
        	// Did not hit anything - background color.
        	None => Vector3::new(0f64, 0f64, 0f64),
        }
    }
}