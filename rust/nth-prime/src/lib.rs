/// Returns the nth prime number, where `nth(0)` is 2
// based on: https://exercism.io/tracks/rust/exercises/nth-prime/solutions/42cc6ddc5467486d872784b045c05b8a
// optimized (stepping through odds)
pub fn nth(n: u32) -> u32 {
    if n == 0 {
        // reflecting 0th prime
        return 2;
    }
    let mut primes = vec![3];
    let index = (n - 1) as usize; // since primes vector holds 1..(n-1)th prime

    let mut odds = (5..).step_by(2);

    while primes.len() <= index {
        // current holds next number to test
        let current = odds.next().unwrap();
        // The maximum prime factor we need to divide by is the square root
        let max_factor = (current as f64).sqrt() as u32;

        // If current number is not divisible by any of the prime factors computed before (below max factor), it's a prime
        if !primes
            .iter()
            .take_while(|&&prime| prime <= max_factor) // while referenced reference of u32 is lt sqrt :-)
            .any(|prime| current % prime == 0)
        {
            primes.push(current);
        }
    }

    primes[index]
}
