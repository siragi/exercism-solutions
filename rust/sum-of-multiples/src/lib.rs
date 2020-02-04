/// Sum the multiples of all of unique factors which are below a given number(limit)
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = vec![];
    for &f in factors {
        if f == 0 {
            multiples.push(0);
        } else {
            (1..limit)
                .filter(|&x| x % f == 0) // filters all multiples (creates duplicates)
                .for_each(|m| multiples.push(m))
        }
    }
    multiples.sort(); // how to get unique entries? sort ...
    multiples.dedup(); // ... and deduplicate
    multiples.iter().sum()
}
