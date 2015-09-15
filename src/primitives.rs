extern crate cgmath;

use cgmath::Point3;
use material::*;

pub struct PrimSphere {
    pub center: cgmath::Point3<f64>,
    pub radius2: f64,
    pub material: Material,
}

impl PrimSphere {
	pub fn new(center: Point3<f64>, radius : f64, material: Material) -> PrimSphere {
		PrimSphere {
			center: center, 
			radius2: radius.powi(2),
			material: material,
		}
	}
}