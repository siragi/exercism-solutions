module main

fn collatz(number int) !int {
	if number < 1 {
		return error('Err: Number below one.')
	}
	return collatz_helper(number)
}

fn collatz_helper(number int) int {
	// Base case of recursion
	if number == 1 {
		return 0
	}

	return 1 + if number & 0b1 == 1 {
		// odd: multiply n by 3 and add 1 to get 3n + 1.
		// n = 3*n + 1
		collatz_helper(number * 3 + 1)
	} else {
		// even: divide n by 2 to get n / 2.
		collatz_helper(number / 2)
	}
}
