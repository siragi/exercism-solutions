module main

// is_leap_year_year returns true if the given year is a leap year in the Gregorian calendar
fn is_leap_year(year int) bool {
	mut yr := year
	mut answer := false

	// factorise year into 4 * 25 * 4 if possible
	if yr % 4 == 0 {  
		// divisble by 4: sofar it seems a leap year
		answer = true
		yr /= 4  // divide by 4
		
		if yr % 25 == 0 {  
			// except when it is also divisible by 25 (by 4*25 = 100) 
			answer = false
			yr /= 25
			
			if yr % 4 == 0 {
				// unless it is also divisible by 4 (by 4*4*25 = 400)
				answer = true
			}
		} 
	}

	return answer
}

// variant b:
// Nice Short version using else if (done with match true), but will everytime evaluate all 3 match clauses, because it is not nested.
// fn is_leap_year_year(year int) bool {
// 	return 
// 		match true {
// 			year % 400 == 0 {true}
// 			year % 100 == 0 {false}
// 			year % 4 == 0 {true}
// 			else {false}
// 		} 
// }


