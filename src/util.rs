pub fn solve_quadratic_real(a: &f64, b: &f64, c: &f64) -> Option<(f64, f64)> {
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