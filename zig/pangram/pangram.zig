const std = @import("std");
const ascii = std.ascii;
const bit_set = std.bit_set;

pub fn isPangram(str: []const u8) bool {
    var alphabet_bitset = bit_set.StaticBitSet(26).initEmpty(); // depending on size -> IntegerBitSet or ArrayBitSetdoes
    // no allocation.

    for (str) |c| {
        if (!ascii.isAlphabetic(c)) continue;

        const index = @intCast(usize, ascii.toLower(c) - 'a');
        alphabet_bitset.set(index);

        if (alphabet_bitset.count() == 26) return true;
    }
    return false;
}
