const std = @import("std");
const mem = std.mem;

pub fn score(s: []const u8) u32 {
    var ans: u32 = 0;

    const scrabble = "aAeEiIoOuUlLnNrRsStTdDgGbBcCmMpPfFhHvVwWyYkKjJxXqQzZ";

    for (s) |char| {
        var maybe_pos = mem.indexOf(u8, scrabble, &[1]u8{char});
        ans += switch (maybe_pos orelse undefined) {
            // 'A', 'E', I, O, U, L, N, R, S, T => 1,
            0...19 => 1,
            // D, G => 2,
            20...23 => 2,
            // B, C, M, P => 3,
            24...31 => 3,
            // F, H, V, W, Y => 4,
            32...41 => 4,
            // K => 5,
            42, 43 => 5,
            // J, X => 8,
            44...47 => 8,
            // Q, Z => 10,
            48...51 => 10,
            else => 0,
        };
    }
    return ans;
}
