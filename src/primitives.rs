extern crate cgmath;

use cgmath::Point3;

pub struct PrimSphere {
    pub center: cgmath::Point3<f64>,
    // pub radius: f64
    pub radius2: f64
}

impl PrimSphere {
	pub fn new(center: Point3<f64>, radius : f64) -> PrimSphere {
		PrimSphere {center: center, radius2: radius.powi(2)}
	}
}