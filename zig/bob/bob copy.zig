const std = @import("std");
const ascii = std.ascii;
const print = std.debug.print;

pub fn response(s: []const u8) []const u8 {
    const bobs_answers = [5][]const u8{
        "Whatever.",
        "Fine. Be that way!",
        "Sure.",
        "Whoa, chill out!",
        "Calm down, I know what I'm doing!",
    };

    if (s.len == 0) return bobs_answers[1];

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const spaced = allocator.dupe(u8, s) catch unreachable;
    defer allocator.free(spaced);

    for ([_]u8{ '\n', '\t', '\r' }) |spacechar| {
        std.mem.replaceScalar(u8, spaced, spacechar, ' ');
    }

    const trimmed = std.mem.trim(u8, spaced, " ");
    if (trimmed.len == 0) return bobs_answers[1];

    var up: u8 = 0;
    var lo: u8 = 0;
    for (trimmed) |c| {
        if (ascii.isAlphabetic(c)) {
            if (ascii.isUpper(c)) up += 1 else lo += 1;
        }
    }

    if (up > 0 and lo == 0) { //upper, no lower
        if (trimmed[trimmed.len - 1] == '?') return bobs_answers[4] else return bobs_answers[3]; // yelled question vs. just yelled
    }
    if (trimmed[trimmed.len - 1] == '?') return bobs_answers[2]; // simple question

    return bobs_answers[0]; // whatever
}
