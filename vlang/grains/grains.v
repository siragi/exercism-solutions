module main

fn grains_on_square(square i8) !u64 {
	if square < 1 || square > 64 {
		return error('square must be between 1 and 64')
	}
	return u64(1) << square - 1
}

fn total_grains_on_board() u64 {
	return 0xFFFFFFFF_FFFFFFFF
}
