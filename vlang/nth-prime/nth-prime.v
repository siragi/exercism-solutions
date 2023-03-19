module main

fn nth_prime(n int) !int {
	if n <= 0 {
		return error('n must be greater than 0')
	}

	mut primes := [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
		73, 79, 83, 89, 97] // just to speed up: the first 25 primes (being below 100)

	for n > primes.len {
		mut is_prime := false
		mut candidate := primes.last()

		for !is_prime {
			candidate += 2 // let's check only odd numbers
			is_prime = true // assume as long proven otherwise

			for prime in primes[1..] { // primes [1..] omits first prime '2', since we step up by 2
				if prime * prime > candidate {
					// if all smaller primes up to current prime are no factors of 'candidate' candidate, then there is no way an even bigger number could be a factor, since it would have to pair up with a smaller than current prime (which are already excluded)
					break
				} else if candidate % prime == 0 {
					is_prime = false
					break
				}
			}
		}
		primes << candidate
	}

	return primes[n - 1]
}
