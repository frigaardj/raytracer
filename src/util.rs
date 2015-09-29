extern crate cgmath;
use cgmath::*;

pub fn solve_quadratic_real(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
	let discr = b * b - 4f64 * a * c;
	if discr < 0f64 {
		return None;
	} else if discr == 0f64 {
		let x = - 0.5f64 * b / a;
		return Some((x, x));
	} else {
		// Better conditioned quadratic formula
        let q = -0.5f64 * (b + b.signum() * discr.sqrt()); 
        let x1 = q / a;
        let x2 = c / q;
        return Some((x1, x2));
	}
}

pub fn lerp(a: f64, b: f64, r: f64) -> f64{
	a + (b - a) * r
}

pub fn mirror(incident: &Vector3<f64>, mirror_normal: &Vector3<f64>) -> Vector3<f64> {
	let dot = incident.dot(&mirror_normal);
	incident.sub_v(&mirror_normal.mul_s(dot * 2.0f64))
}
