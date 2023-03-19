module main

fn is_triangle(a f64, b f64, c f64) bool {
	return a + b >= c && b + c >= a && c + a >= b && a > 0 && b > 0 && c > 0
}

fn is_isosceles(a f64, b f64, c f64) bool {
	return is_triangle(a, b, c) && (a == b || a == c || b == c)
}

fn is_equilateral(a f64, b f64, c f64) bool {
	return is_triangle(a, b, c) && a == b && a == c
}

fn is_scalene(a f64, b f64, c f64) bool {
	return is_triangle(a, b, c) && !is_isosceles(a, b, c)
}
