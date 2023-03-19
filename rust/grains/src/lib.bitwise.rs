/// returns grains of rice on the square number s on a chessboard, where grains on each field double.
pub fn square(s: u32) -> u64 {
    match s {
        //1..=64 => 1_u64.wrapping_shl(s - 1), which shifts the bits to the left (doubling) like shl(), but respects the boundery of the type.
        1..=64 => 1_u64 << (s - 1), // short form for method shl() of Trait std::ops::Shl -> 1_u64.shl(s - 1)
        _ => panic!("Square must be between 1 and 64"),
    }
}

// the total number of grains on the chessboard
pub fn total() -> u64 {
    (0 as u64).wrapping_sub(1) // substract 1 from 0, wrapping around to the biggest u64
    // u64::max_value()
}
