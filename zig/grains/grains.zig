pub const ChessboardError = error{IndexOutOfBounds};

pub fn square(index: usize) ChessboardError!u64 {
    if (index < 1 or index > 64) return ChessboardError.IndexOutOfBounds;
    // right-bitshift: 2^(index-1)
    return @as(u64, 1) << @intCast(index - 1);
}
// pub fn square(index: usize) ChessboardError!u64 {
//     if (index < 1 or index > 64) return ChessboardError.IndexOutOfBounds;
//     if (index == 1) return 1;

//     var ans: u64 = 1;
//     for (1..index) |_| {
//         ans *= 2;
//     }
//     return ans;
// }

pub fn total() u64 {
    return 0xffff_ffff_ffff_ffff; // 0xf == 0b1111 (4 bit) -> 64/4 = 16 f needed.
}
// pub fn total() u64 {
//     var ans: u64 = 0;
//     for (1..65) |i| {
//         ans += square(i) catch unreachable;
//     }
//     return ans;
// }
