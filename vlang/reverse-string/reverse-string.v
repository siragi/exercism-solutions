module main

// reverse_string returns a given string in reverse order
fn reverse_string(str string) string {
	runes := str.runes()
	mut answer := []rune{cap: str.len}

	for i:= str.len-1 ; i >= 0 ; i-- {
		answer << runes[i]
	}
	return answer.string()
}