/// Sum the multiples of all of unique factors which are below a given number(limit)
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&x| {
            factors.iter().any(|&f| f != 0 && x % f == 0) // take it when first factor has been found -> creates no duplicates
        })
        .sum()
}
