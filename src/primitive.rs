extern crate cgmath;

use cgmath::*;
use primitives::*;
use util::*;
use material::*;

pub trait Primitive {
	// Check intersection, return None or the distance
    fn intersects(&self, ray: &Ray3<f64>) -> Option<f64>;
    fn normal(&self, intersection: &Point3<f64>) -> Vector3<f64>;
    fn material_at_point(&self, point: &Point3<f64>) -> &Material;
}

impl Primitive for PrimSphere {
    fn intersects(&self, ray: &Ray3<f64>) -> Option<f64> {
    	let L = ray.origin.sub_p(&self.center);
    	let a = ray.direction.dot(&ray.direction);
    	let b = 2f64 * ray.direction.dot(&L);
    	let c = L.dot(&L) - self.radius2;
    	let distances = solve_quadratic_real(&a, &b, &c);
    	distances.and_then(|res| {
    		if (res.0 < 0f64 && res.1 < 0f64) {
    			None
    		} else if res.0 < 0f64 {
				Some(res.1)
			} else if res.1 < 0f64 {
				Some(res.0)
			} else {
				Some(res.0.min(res.1))
			}
    	})
    }

    fn normal(&self, intersection: &Point3<f64>) -> Vector3<f64> {
    	intersection.sub_p(&self.center).normalize()
    }

    fn material_at_point(&self, point: &Point3<f64>) -> &Material {
    	&self.material
    }
}