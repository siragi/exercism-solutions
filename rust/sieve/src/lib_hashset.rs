// KoStard's solution: https://exercism.io/tracks/rust/exercises/sieve/solutions/e039e95f34a14201b57d4890541cec00
use std::collections::HashSet;

/// Construct a vector of all primes up to upper_bound
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut marked: HashSet<u64> = HashSet::new(); // Using this to get better algorithm complexity
    let mut res = Vec::new();
    for i in 2..=upper_bound {
        // N times
        if marked.contains(&i) {
            // O(1)
            continue;
        }
        res.push(i);
        let mut c = i * 2;
        while c <= upper_bound {
            // < O(N/Pi) - works P times (count of prime numbers in the range)
            marked.insert(c);
            c += i;
        }
    }
    res
}
