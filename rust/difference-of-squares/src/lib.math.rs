// According to https://en.wikipedia.org/wiki/Square_pyramidal_number
pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

// According to https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF
pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
}

pub fn difference(number: u32) -> u32 {
    square_of_sum(number) - sum_of_squares(number)
}
