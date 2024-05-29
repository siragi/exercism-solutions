const std = @import("std");
const print = std.debug.print;
// to make use of prints in your code add '2>&1 | cat' to your test command or prints will be scrambled (overwritten/randomly omitted).
// zig test test_isbn_verifier.zig 2>&1 | cat

pub fn isValidIsbn10(s: []const u8) bool {
    var checksum: u32 = 0;
    var multiplier: u8 = 10;

    for (s) |byte| {
        if (multiplier == 0) {
            print("{s} too long to be ISBN\n", .{s});
            return false;
        }
        switch (byte) {
            '-' => continue,

            '0'...'9' => {
                const digit: u8 = byte & 0b0000_1111; // only last for bits important for numbers in ascii (bitwise-and = masking)
                print("\n +{d}x{d}", .{ digit, multiplier });
                checksum += digit * multiplier;
            },
            'X', 'x' => {
                if (multiplier == 1) checksum += 10 else return false;
            },
            else => {
                print("\nnot allowed char {c}\n", .{byte});
                return false;
            },
        }
        multiplier -= 1;

        print(", checksum: {d}", .{checksum});
    }

    // print("\n chksum: {}", .{checksum});
    // print(", isbn: {}\n", .{checksum % 11 == 0});

    return checksum % 11 == 0 and multiplier == 0;
}
