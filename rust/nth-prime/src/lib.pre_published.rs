/// Returns the nth prime number, where `nth(0)` is 2
// based on: https://exercism.io/tracks/rust/exercises/nth-prime/solutions/42cc6ddc5467486d872784b045c05b8a
// optimized (Iterators stepping through odds)
pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut primes = vec![3];
    let mut counter = primes[0] + 2;
    let index = (n - 1) as usize;

    while primes.len() <= index {
        // The maximum prime factor we need to divide by is the square root
        // (to get rid of double checking: ex. oddnum 15=3*5 and unnecessary 15=5*3)
        let max_factor = (counter as f64).sqrt() as u32;

        // Iterate through all primes less than the square root
        // If the counter is not divisible by any of those factors, it's prime
        if !primes
            .iter()
            .take_while(|prime| **prime <= max_factor) // dereference reference of reference
            .any(|prime| counter % prime == 0)
        {
            primes.push(counter);
        }

        counter += 2;
    }

    primes[index]
}
