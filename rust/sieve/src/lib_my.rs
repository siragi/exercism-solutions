use std::usize;

/// Construct a vector of all primes up to upper_bound
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    } else if upper_bound == 2 {
        return vec![2];
    }

    let mut primes: Vec<u64> = vec![];
    let marks = vec![false; (upper_bound - 2 + 1) as usize]; // because we omit first '0' and '1' later in the sieve (-2), but since it starts at '0' we have to add 1
    let mut sieve: Vec<(u64, bool)> = (2..=upper_bound).zip(marks).collect();

    // goes throug all multiples of a given prime and marks them
    fn mark(base: u64, sieve: &mut Vec<(u64, bool)>) {
        for i in ((base - 2) as usize..sieve.len()).step_by(base as usize) {
            // sieve[0] holds '2' -> base - 2
            sieve[i].1 = true;
        }
    }

    // Find first not marked number
    fn not_marked(sieve: &mut Vec<(u64, bool)>) -> Option<u64> {
        match sieve.into_iter().find(|(_, marked)| *marked == false) {
            Some((prime, _)) => return Some(*prime),
            None => None,
        }
    }

    // Main loop
    while let Some(num) = not_marked(&mut sieve) {
        mark(num, &mut sieve);
        primes.push(num);
    }
    primes
}
