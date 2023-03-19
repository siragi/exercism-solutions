// https://exercism.io/tracks/rust/exercises/nth-prime/solutions/4ae020aee31e49edb0f31a72bb39f0c6
fn is_prime(n: u32) -> bool {
    let sqrt = (n as f64).sqrt() as u32;
    (2..=sqrt).all(|number| n % number != 0)
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|n| is_prime(*n)).nth(n as usize).unwrap()
}
