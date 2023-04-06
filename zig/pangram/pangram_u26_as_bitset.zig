const ascii = @import("std").ascii;

pub fn isPangram(str: []const u8) bool {
    var set: u26 = 0; // std.StaticBitSet also works.

    for (str) |c| {
        if (!ascii.isAlphabetic(c)) continue;
        set |= @as(u26, 1) << @intCast(u5, ascii.toLower(c) - 'a'); //bitwise or does set bit to 1, regardless of the current state. why u5? compiler suggestion, obv. u8 - 'a' will only generate u5 values.
        if (set == 0b1111111111_1111111111_111111) return true; // 26 bits are set to 1
        //if (set == 0x3FF_FFFF) return true; // hex representation : each F represents 4 bits = 2^4 and leading 3 ar 2 bits
    }
    return false;
}
