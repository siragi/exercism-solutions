/// returns grains of rice on the square number s on a chessboard, where grains on each field double.
pub fn square(s: u32) -> u64 {
    // how many grains are on a given square
    assert!(s > 0 && s < 65, "Square must be between 1 and 64");
    2_u64.pow(s - 1)
}

// the total number of grains on the chessboard
pub fn total() -> u64 {
    (1..=64).fold(0, |acc, x| acc + square(x))
}
