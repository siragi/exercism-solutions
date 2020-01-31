/// Returns the nth prime number, where `nth(0)` is 2
// based upon https://exercism.io/tracks/rust/exercises/nth-prime/solutions/4ae020aee31e49edb0f31a72bb39f0c6
// optimized (Iterators stepping through odds)
fn is_oddnum_prime(oddnum: u32) -> bool {
    // sqrt is the maximum oddnum we have to check  (to get rid of double checking ex. oddnum 15=3*5 and unnecessary 15=5*3)
    let sqrt = (oddnum as f64).sqrt() as u32; // turn oddnum into f64 to get sqrt and back

    !(3..=sqrt) // not(!) divisible (prime)
        .step_by(2) // 3,5,7,..
        .any(|number| oddnum % number == 0) // any divisible (not prime)
}

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        // special case (the only even prime number)
        return 2;
    }
    (3..) // Creates an Iterator with u32 Elements
        .step_by(2) // 3,5,7,..
        .filter(|odd| is_oddnum_prime(*odd)) // only primes
        .nth((n - 1) as usize) // -1 because of special prime 2
        .unwrap() // get rid of iterator wrapping the u32
}
