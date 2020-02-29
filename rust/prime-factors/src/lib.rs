/// calculate the prime factors
use primal; // Cargo.toml dependencies: primal = "0.2"

pub fn factors(n: u64) -> Vec<u64> {
    // ("This should calculate the prime factors of {}", n)
    // set up a sieve containing all possible primes to the limit of the max_factor
    let max_factor = (n as f64).sqrt() as usize; // possibility u64 bigger than f64 (!)
    let sieve = primal::Sieve::new(max_factor);

    let mut vec: Vec<u64> = Vec::new();

    if let Some(tuple_vec) = sieve.factor(n as usize).ok() {
        for (prime, counter) in tuple_vec {
            // the tuple_vec contains counted primes
            // If the prime number is 3 and was counted 5 times it will append [3,3,3,3,3]:
            vec.append(&mut vec![prime as u64; counter]);
        }
    }

    // So many possibilities:
    // a)
    // while let Some((prime, counter)) = factorsvec.iter().next().cloned().unwrap().pop() {
    //     for _i in 0..counter {
    //         vec.push(prime as u64);
    //     }
    // }

    // b)
    // if let Some(tuple_vec) = sieve.factor(n as usize).ok() {
    //     for (prime, counter) in tuple_vec {
    //         for _ in 0..counter {
    //             vec.push(prime as u64);
    //         }
    //     }
    // }
    /* c)
    sieve.factor(n as usize).ok().iter().for_each(|f| {
        for (prime, counter) in f {
            for _ in 1..=(*counter as u64) {
                vec.push(*prime as u64)
            }
        }
    });
    */

    // d)
    // sieve.factor(n as usize).ok().iter().for_each(|f| {
    //     f.iter().for_each(|(prime, counter)| {
    //         for _ in 0..*counter {
    //             vec.push(*prime as u64)
    //         }
    //     })
    // });

    // e)
    // if let Some(mut primefactors) = sieve.factor(n as usize).ok() {
    //     while let Some((prime, counter)) = primefactors.pop() {
    //         for _ in 0..counter {
    //             vec.push(prime as u64);
    //         }
    //     }
    // }
    // vec.reverse();
    vec
}
