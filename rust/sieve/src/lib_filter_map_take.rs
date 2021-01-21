// zrwi's solution: https://exercism.io/tracks/rust/exercises/sieve/solutions/5f64e30e37884b9aa001f7f45ca40579
use std::usize;

/// Construct a vector of all primes up to upper_bound
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (2..=upper_bound).map(Option::from).collect(); // Creates a Vec with 'Some(2), Some(3) .. '
    (0..numbers.len())
        .filter_map(|i| {
            let prime = numbers[i].take()?; // If possible take a num out of 'Some(num)' and exchanges with None OR by the magic of '?': throws an error (works on Options too because of the try trait)
            (prime..=upper_bound)
                .step_by(prime as usize)
                .skip(1)
                .for_each(|i| numbers[(i - 2) as usize] = None);
            Some(prime)
        })
        .collect()
}
