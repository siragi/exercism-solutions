module main

const alphabet = 'abcdefghijklmnopqrstuvwxyz'.runes()

fn is_pangram(phrase string) bool {
	runes := phrase.to_lower().runes()
	return alphabet.all(it in runes)
}
