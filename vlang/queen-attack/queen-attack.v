module main

import math { abs }

fn can_queen_attack(w string, b string) !bool {
	if valid_square(w)! == valid_square(b)! {
		return error('queens on same square')
	}

	is_diagonal := abs(int(w[0]) - int(b[0])) == abs(int(w[1]) - int(b[1]))

	return w[0] == b[0] || w[1] == b[1] || is_diagonal
}

fn valid_square(s string) !string {
	if s.len != 2 || s[0] < `a` || s[0] > `h` || s[1] < `1` || s[1] > `8` {
		return error('${s} is not a valid square')
	}
	return s
}
