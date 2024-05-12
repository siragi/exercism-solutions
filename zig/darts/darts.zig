const std = @import("std");

pub const Coordinate = struct {
    x: f32,
    y: f32,

    pub fn init(x_coord: f32, y_coord: f32) Coordinate {
        return Coordinate{
            .x = x_coord,
            .y = y_coord,
        };
    }
    pub fn score(self: Coordinate) usize {
        const radius = std.math.hypot(self.x, self.y);
        const border: u8 = @intFromFloat(@ceil(radius));
        return switch (border) {
            0...1 => 10,
            2...5 => 5,
            6...10 => 1,
            else => 0,
        };
    }
};
