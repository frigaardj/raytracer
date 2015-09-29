extern crate cgmath;

use cgmath::*;
use material::*;

// Sphere primitive
pub struct PrimSphere<'a> {
    pub center: Point3<f64>,
    pub radius2: f64,
    pub material: &'a Material,
    pub name: String,
}

impl<'a> PrimSphere<'a> {
	pub fn new(center: Point3<f64>, radius : f64, material: &Material, name: String) -> PrimSphere {
		PrimSphere {
			center: center, 
			radius2: radius.powi(2),
			material: material,
			name: name
		}
	}
}

// Infinite plane primitive
pub struct PrimPlane<'a> {
	pub point: Point3<f64>,
	pub normal: Vector3<f64>,
	pub material: &'a Material,
    pub name: String,
}

impl<'a> PrimPlane<'a> {
	pub fn new(point: Point3<f64>, normal: Vector3<f64>, material: &Material, name: String) -> PrimPlane {
		PrimPlane {
			point: point, 
			normal: normal.normalize(),
			material: material,
			name: name
		}
	}
}