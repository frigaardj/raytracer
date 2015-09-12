extern crate cgmath;

use cgmath::*;
use primitives::*;
use util::*;

pub trait Intersectable {
	// Check intersection, return None or the distance
    fn intersects(&self, ray: &Ray<f64, Point3<f64>, Vector3<f64>>) -> Option<f64>;
}

impl Intersectable for PrimSphere {
    fn intersects(&self, ray: &Ray<f64, Point3<f64>, Vector3<f64>>) -> Option<f64> {
    	let L = ray.origin.sub_p(&self.center);
    	let a = ray.direction.dot(&ray.direction);
    	let b = 2f64 * ray.direction.dot(&L);
    	let c = L.dot(&L) - self.radius2;
    	let xs = solve_quadratic_real(&a, &b, &c);
    	match xs {
    		None => None,
    		Some(res) => {
	    		let min = res.0.min(res.1);
	    		if min > 0f64 {
	    			Some(min)
	    		} else {
	    			let max = res.0.max(res.1);
	    			if max < 0f64 {
	    				None 
	    			} else {
	    				Some(max)
	    			}
	    		}
    		},
    	}
    }
}