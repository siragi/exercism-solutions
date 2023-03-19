module main

import arrays
import maps

const factors = {
	3: 'Pling'
	5: 'Plang'
	7: 'Plong'
}

fn raindrops(number int) string {
	result := arrays.fold(factors.keys(), '', fn [number] (acc string, factor int) string {
		// the fold operation is an anonymus function than encloses [number] -> a closure.
		return match true {
			number % factor == 0 { acc + factors[factor] }
			else { acc }
		}
	})
	return if result == '' { number.str() } else { result }
}
