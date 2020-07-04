/// calculate the prime factors using a ready to use library (cheating)
/// This version does nothing but squieze the results of the sieve into the wanted vector format
use primal; // For this to work add Cargo.toml dependencies: primal = "0.2"

pub fn factors(n: u64) -> Vec<u64> {
    // set up a sieve containing all possible primes to the limit of the max_factor
    let max_factor = (n as f64).sqrt() as usize; // possibility u64 bigger than f64 (!)
    let sieve = primal::Sieve::new(max_factor); // This creates a sieve containing a vector of tuples containing primes and the corresponding prime counter

    // Output vector for holding the primes, that we extract from the sieve:
    let mut vec: Vec<u64> = Vec::new();

    // When there are no primes  'if let' (single 'match' statement) will not attempt to extract anything from the sieve.
    if let Some(tuple_vec) = sieve.factor(n as usize).ok() {
        // Extraction via loop:
        for (prime, counter) in tuple_vec {
            // the tuple_vec contains counted primes
            // If the prime number is 3 and was counted 5 times (3, 5) it will append [3,3,3,3,3] using the vec! macro:
            vec.append(&mut vec![prime as u64; counter]);
        }
    }

    // There is also a folding version:
    /* if let Some(tuple_vec) = sieve.factor(n as usize).ok() {
        // Extraction via iteration
        vec = tuple_vec
            .iter()
            .fold(&mut vec, |acc, tupl| {
                acc.append(&mut vec![tupl.0 as u64; tupl.1]);
                acc
            })
            .to_vec();  // To turn a vec that looks like [[2; 2] [5; 5; 5]] into [2; 2; 5; 5; 5]
    } */

    // We could also pop the last tuple (results in a vector that holds the biggest primes first, so we have to reverse the result)
    /* if let Some(mut tuple_vec) = sieve.factor(n as usize).ok() {
        // Let's get the last tuple first (happens to be the biggest prime number)
        while let Some((prime, counter)) = tuple_vec.pop() {
            // instead of using the vec! macro we can also push via loop. Which is the most readable way in my eyes:
            for _ in 0..counter {
                vec.push(prime as u64);
            }
        }
    }
    vec.reverse(); */

    vec
}
