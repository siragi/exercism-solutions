module main

fn two_fer(name string) string {
	you := if name.is_blank() { 'you' } else { name }
	return 'One for ${you}, one for me.'
}
