extern crate cgmath;
use cgmath::*;

pub struct Material {
    pub color: Vector3<f64>,
    pub diffuse: f64,
    pub reflection: f64,
}

impl Material {
	pub fn new(color: Vector3<f64>, diffuse: f64, reflection: f64) -> Material {
		Material {
			color: color,
			diffuse: diffuse.min(1.0f64).max(0.0f64),
			reflection: reflection.min(1.0f64).max(0.0f64)
		}
	}
}