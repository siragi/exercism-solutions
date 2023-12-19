const std = @import("std");

pub fn isIsogram(str: []const u8) bool {
    if (str.len == 0) return true;

    for (str, 0..) |char, i| {
        if (!inAlphabet(char)) continue;

        for (str[i + 1 .. str.len]) |next_char| {
            const same_lowercase_ascii = ((char | 32) == (next_char | 32));

            if (same_lowercase_ascii) {
                // std.debug.print("\n no isogram -> equal ascii:  {}", .{char});
                return false;
            }
            // else std.debug.print("\n isogram last char:  {}", .{char});
        }
    }
    return true;
}

inline fn inAlphabet(char: u8) bool {
    const lowercase_ascii = char | 32;
    const ans = (97 <= lowercase_ascii) and (lowercase_ascii <= 122);
    // if (!ans) std.debug.print("\n inAlpha? {}: ", .{ans});
    return ans;
}
