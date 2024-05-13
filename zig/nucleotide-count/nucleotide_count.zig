pub const NucleotideError = error{Invalid};

pub const Counts = struct {
    a: u32 = 0,
    c: u32 = 0,
    g: u32 = 0,
    t: u32 = 0,
};

pub fn countNucleotides(s: []const u8) NucleotideError!Counts {
    var acgt = Counts{};

    for (s) |byte| switch (byte) {
        // 'A' => acgt.a += 1,
        // 'C' => acgt.c += 1,
        // 'G' => acgt.g += 1,
        // 'T' => acgt.t += 1,
        // lets use the inline switch prong:
        inline 'A', 'C', 'G', 'T' => |char| @field(acgt, &.{char | 0b10_0000}) += 1, // bitwise or to add 6th bit (2^5 = 0b10_0000 = 0x20 = 32), which makes the difference between upper and lowercase in ascii. the binary number represents space, so char ' ' works too!
        else => return NucleotideError.Invalid,
    };
    return acgt;
}
