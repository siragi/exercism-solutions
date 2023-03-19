// adapted from glennpratt's solution: https://exercism.io/tracks/rust/exercises/sieve/solutions/a5112b3d37a04dd49bbba597fd03f050
use std::usize;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let max = upper_bound as usize;

    let mut marks = vec![false; max + 1];
    (2..max + 1)
        .filter_map(|n| {
            // gives back: Option
            if marks[n] {
                return None;
            }
            for m in 1..(max / n + 1) {
                // divide by n, because ...
                marks[m * n] = true // ... here we multiply by n
            }
            Some(n as u64)
        })
        .collect()
}
