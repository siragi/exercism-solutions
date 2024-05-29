const std = @import("std");

pub const TriangleError = error{
    Invalid,
};
pub const Triangle = struct {
    a: f64,
    b: f64,
    c: f64,

    pub fn init(a: f64, b: f64, c: f64) TriangleError!Triangle {
        // if (a + b >= c and b + c >= a and c + a >= b and (a * b * c != 0)) {
        // above works but is way too easy to understand, so we looked up equivalent: max(a,b,c) < a+b+c - max(a,b,c)
        // since std.mem.max fn needs '[]const T' we give what can be coerced/cast the address to adhoc f64 slice
        if (2 * std.mem.max(f64, &[_]f64{ a, b, c }) < a + b + c and (a * b * c != 0)) {
            return .{
                .a = a,
                .b = b,
                .c = c,
            };
        } else {
            return TriangleError.Invalid;
        }
    }

    pub fn isEquilateral(self: Triangle) bool {
        return self.a == self.b and self.b == self.c;
    }

    pub fn isIsosceles(self: Triangle) bool {
        return self.a == self.b or self.b == self.c or self.c == self.a;
    }

    pub fn isScalene(self: Triangle) bool {
        return !isIsosceles(self);
    }
};
