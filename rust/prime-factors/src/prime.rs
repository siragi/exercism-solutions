//extern crate ramp;
//use ramp::Int;

/// https://medium.com/snips-ai/prime-number-generation-2a02f28508ff
fn div_small_primes(numb: &u64) -> bool {
    static SMALL_PRIMES: [u32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]; //, ... ,17807,17827,17837,17839,17851,17863]; // This does not compile
    for p in SMALL_PRIMES.into_iter() {
        if numb % &u64::from(*p) == 0 {
            return false;
        }
    }
    return true;
}

fn little_fermat(candidate: &u64) -> bool {
    use self::ramp::Randomu64;
    let mut rng = OsRng::new()
        .ok()o
        .expect("Failed to get OS random generator");
    let random: u64 = rng.gen_uu64_below(candidate);
    let result = u64::modpow(&random, &(candidate - &u64::one()), candidate);
    result == u64::one()
}

fn miller_rabin(candidate: &u64, limit: usize) -> bool {
    let (s, d) = rewrite(&(candidate - &u64::one()));
    let one = u64::one();
    let two = &one + &one;

    for _ in 0..limit {
        let basis = u64::sample_range(&two, &(candidate - &two));
        let mut y = u64::modpow(&basis, &d, candidate);

        if y == one || y == (candidate - &one) {
            continue;
        } else {
            for _ in one.clone()..s - one.clone() {
                y = u64::modpow(&y, &two, candidate);
                if y == one {
                    return false;
                } else if y == candidate - &one {
                    break;
                }
            }
            return false;
        }
    }
    true
}

pub fn is_prime(candidate: &u64) -> bool {
    // First, simple trial divide
    if !div_small_primes(candidate) {
        return false;
    }

    // Second, Fermat's little theorem test on the candidate
    if !little_fermat(candidate) {
        return false;
    }

    // Finally, Miller-Rabin test
    if !miller_rabin(candidate, 5) {
        return false;
    }
    true
}
