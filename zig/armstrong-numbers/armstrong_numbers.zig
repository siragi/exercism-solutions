const std = @import("std");
const math = std.math;

pub fn isArmstrongNumber(num: u128) bool {
    //@compileError("please implement the isArmstrongNumber function");
    if (num < 10) return true;
    var n = num;
    var p: usize = 0;
    var sum: u128 = 0;
    // for (&rem_list) |*item| {
    //     item.* = 0;
    // }
    // or faster:
    // rem_list= [_]u128{0} ** 128;
    var rem_list: [128]u128 = undefined;

    // find the power (p) and store remainders
    while (n > 0) : (p += 1) {
        rem_list[p] = n % 10;
        //std.debug.print("\np:{}, n:{}, rem:{}", .{ p, n, rem_list[p] });
        n /= 10;
    }

    // sum up the remainders^p
    var i: usize = 0;
    while (i < p) : (i += 1) {
        sum += math.pow(u128, rem_list[i], p);
        //std.debug.print("\nitem: {}, sum: {}", .{ rem_list[i], sum });
    }

    return sum == num;
}
