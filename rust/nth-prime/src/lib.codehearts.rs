/// Returns the nth prime number, where `nth(0)` is 2
/// https://exercism.io/tracks/rust/exercises/nth-prime/solutions/42cc6ddc5467486d872784b045c05b8a
pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];
    let mut counter = primes[0] + 1;
    let index = n as usize;

    while primes.len() <= index {
        // The maximum prime factor we need to divide by is the square root
        let max_factor = (counter as f64).sqrt() as u32 + 1;

        // Iterate through all primes less than the square root
        // If the counter is not divisible by any of those factors, it's prime
        if primes
            .iter()
            .take_while(|prime| **prime < max_factor)
            .all(|prime| counter % prime != 0)
        {
            primes.push(counter);
        }

        counter += 2;
    }

    primes[index]
}
