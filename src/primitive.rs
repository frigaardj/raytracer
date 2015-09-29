extern crate cgmath;

use cgmath::*;
use primitives::*;
use util::*;
use material::*;

pub trait Primitive {
	// Intersects the ray with the primitive. Returns the distance to the intersection point or None if no intersection
    fn intersects(&self, ray: &Ray3<f64>) -> Option<f64>;
    // Returns the normal at the given point (assumes this point is on the primitive)
    fn normal(&self, intersection: &Point3<f64>) -> Vector3<f64>;
    // Returns the material at the given point (assumes this point is on the primitive)
    fn material(&self) -> &Material;

    fn name(&self) -> &str;
}

impl<'a> Primitive for PrimSphere<'a> {
    fn intersects<'b>(&self, ray: &'b Ray3<f64>) -> Option<f64> {
    	let l = ray.origin.sub_p(&self.center);
    	let a = ray.direction.dot(&ray.direction);
    	let b = 2f64 * ray.direction.dot(&l);
    	let c = l.dot(&l) - self.radius2;
    	let distances = solve_quadratic_real(a, b, c);
    	distances.and_then(|res| {
    		if res.0 < 0f64 && res.1 < 0f64 {
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
    fn normal(&self, intersection: &Point3<f64>) -> Vector3<f64> {intersection.sub_p(&self.center).normalize() }
    fn material(&self) -> &Material { &self.material }
    fn name(&self) -> &str { &self.name }
}

impl<'a> Primitive for PrimPlane<'a> {
    fn intersects<'b>(&self, ray: &'b Ray3<f64>) -> Option<f64> {
    	// Ray starts at d and extends along e with distance t
    	// Plane has normal n and passes through a
    	// t = (n dot (a - d)) / (n dot e)
    	let n_dot_e = ray.direction.dot(&self.normal);
    	if n_dot_e == 0f64 {
    		return None;
    	}
    	let t = &self.normal.dot(&self.point.sub_p(&ray.origin)) / n_dot_e;
    	if t >= 0f64 {
    		return Some(t);
    	} else {
    		return None;
    	}
    }
    fn normal(&self, intersection: &Point3<f64>) -> Vector3<f64> { self.normal }
    fn material(&self) -> &Material { &self.material }
    fn name(&self) -> &str { &self.name }
}