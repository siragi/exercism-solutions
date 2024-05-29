const std = @import("std");
const print = std.debug.print;

pub const ColorBand = enum {
    black,
    brown,
    red,
    orange,
    yellow,
    green,
    blue,
    violet,
    grey,
    white,

    fn value(self: ColorBand) usize {
        return @intFromEnum(self);
    }
};

pub fn colorCode(colors: [2]ColorBand) usize {
    // a) works
    // const c0: usize = @intFromEnum(colors[0]);
    // const c1: usize = @intFromEnum(colors[1]);
    // return c0 * 10 + c1;

    // b) or oneliner which is less readable, because it uses casting to avoid integer overflow (since enum has small type)
    // return @intFromEnum(colors[0]) * @as(usize, 10) + @intFromEnum(colors[1]);

    // c) use self made enum method that converts to usize.
    return colors[0].value() * 10 + colors[1].value();
}
