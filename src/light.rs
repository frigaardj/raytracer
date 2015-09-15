extern crate cgmath;

use cgmath::*;
use primitives::*;

pub trait Light {
   	fn get_center(&self) -> Point3<f64>;
   	fn get_color(&self) -> Vector3<f64>;
}

impl Light for PrimSphere {
	fn get_center(&self) -> Point3<f64> {
		self.center
	}
	fn get_color(&self) -> Vector3<f64> {
		self.material.color
	}
}