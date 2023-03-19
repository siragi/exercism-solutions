module main

const secs_per_earthyear = 31557600

fn age(seconds f64, planet string) !f64 {
	// age should return an error if the planet is not one of the 8 listed
	//secs_per_earthyear := 31557600

	return seconds / secs_per_earthyear / match planet {
		'Mercury' { 0.2408467 }
		'Venus' { 0.61519726 }
		'Earth' { 1.0 }
		'Mars' { 1.8808158 }
		'Jupiter' { 11.862615 }
		'Saturn' { 29.447498 }
		'Uranus' { 84.016846 }
		'Neptune' { 164.79132 }
		else { return error('${planet} is not a valid planet') }
	}
}
