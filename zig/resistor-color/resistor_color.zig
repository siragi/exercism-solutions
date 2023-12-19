const std = @import("std");
const enums = std.enums;

pub fn colorCode(color: ColorBand) usize {
    return @intFromEnum(color);
}

pub fn colors() []const ColorBand {
    // const len = @typeInfo(ColorBand).Enum.fields.len;
    // // const len = @intFromEnum(ColorBand.white) + 1; //also possible
    // var result: [len]ColorBand = undefined;
    // for (0..10) |i| {
    //     result[i] = @enumFromInt(i);
    // }
    // return &result;
    return enums.values(ColorBand);
}

pub const ColorBand = enum(usize) { black, brown, red, orange, yellow, green, blue, violet, grey, white, _ };
