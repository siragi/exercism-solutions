const scores = [_]u8{ 0, 1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10 };
pub fn score(s: []const u8) u32 {
    if (s.len == 0) {
        return 0;
    } else {
        // Absolute fan of bitwise and (& 31) to convert ascii lower and uppercase to the exact same index.
        // ascii 97 b(110 0001) and 31 b(1 1111) is 1 as well as
        // ascii 65 b(100 0001) and 31           is 1. =>  & 31 masks bits higher or eq to 2^5!
        return scores[(s[0] & 31)] + score(s[1..]);
    }
}
