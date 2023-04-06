const ascii = @import("std").ascii;

/// checks if str contains all letters of the alphabet
pub fn isPangram(str: []const u8) bool {
    const alphabet = "abcdefghijklmnopqrstuvwxyz";
    var ans = false;

    for (alphabet) |letter| {
        ans = false;
        for (str) |s| {
            if (letter == ascii.toLower(s)) {
                ans = true;
                continue;
            }
        }
        if (ans == false) break;
    }
    return ans;
}
