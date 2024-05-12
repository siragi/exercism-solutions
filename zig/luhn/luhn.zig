const std = @import("std");
const print = std.debug.print; // use 2>&1 |

pub fn isValid(s: []const u8) bool {
    if (s.len == 0) return false;

    var luhn: u32 = 0;

    // define heap general purpose allocator
    // https://zig.guide/standard-library/allocators

    // a) heap alloc
    // var gpa = std.heap.GeneralPurposeAllocator(.{}){}; // gpa instance
    // const allocator = gpa.allocator();
    // defer std.debug.assert(gpa.deinit() == .ok); // assert deinit-status is .ok

    // b.) lets try allocation on stack instead of heap (even if fba is found in std.heap.FixedBufferAllocator)
    var buffer: [65]u8 = undefined; // longest test has 40 bytes, 25 bytes seem to be overhead needed by the fba or the Arraylist.
    var fba = std.heap.FixedBufferAllocator.init(&buffer);
    const allocator = fba.allocator();

    var filtered = std.ArrayList(u8).init(allocator);
    defer filtered.deinit();

    for (s) |byte| switch (byte) {
        ' ' => continue,
        '0'...'9' => filtered.append(byte - '0') catch unreachable,
        else => return false,
    };

    if (filtered.items.len < 2) return false;

    while (filtered.items.len > 0) {
        luhn += filtered.pop();

        if (filtered.popOrNull()) |value| {
            luhn += if (value > 4) value * 2 - 9 else value * 2;
        }
    }
    return luhn % 10 == 0;
}

// absolute clever (https://exercism.org/tracks/zig/exercises/luhn/solutions/szczescie):
// pub fn isValid(s: []const u8) bool {
//     return luhn(s, s.len - 1, 0, 0);
// }
// fn luhn(s: []const u8, i: u64, n: u8, sum: u8) bool {
//     if (s[i] == ' ' and i > 0) return luhn(s, i - 1, n, sum);
//     if (s[i] < '0' or s[i] > '9') return false;
//     const digit = (s[i] - '0') * (n % 2 + 1);
//     const mod = (sum + digit + @intFromBool(digit > 9)) % 10;
//     return if (i > 0) luhn(s, i - 1, n + 1, mod) else n > 0 and mod == 0;
// }
