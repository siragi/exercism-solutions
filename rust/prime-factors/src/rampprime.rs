/// https://medium.com/snips-ai/prime-number-generation-2a02f28508ff

fn is_prime_naive(numb: &Int) -> bool {
    let mut i = Int::from(3);
    while &i < numb {
        if numb % &i == 0 {
            return false
        }
        i = i + 2;
    }
    return true;
}

fn genprime(n: usize) -> Int {
    use self::ramp::RandomInt;
    let mut rng = OsRng::new().ok().expect("Failed to get OS random generator");
    loop {
        let mut candidate:Int = rng.gen_uint(n); 
        candidate.set_bit(0, true);
        candidate.set_bit((n-1) as u32, true);
        if is_prime(&candidate) == true { 
            return candidate;
        }
    }
}


fn div_small_primes(numb: &Int) -> bool { 
    static SMALL_PRIMES: [u32; 2048] = [2,3,5,7,11,13,17,19,23,29, ... ,17807,17827,17837,17839,17851,17863]; // This does not compile
    for p in SMALL_PRIMES.into_iter() {
        if numb % &Int::from(*p) == 0 {
            return false;
        }
    }
    return true;
}


fn little_fermat(candidate: &Int) -> bool {
    use self::ramp::RandomInt;
    let mut rng = OsRng::new().ok().expect("Failed to get OS random generator");
    let random:Int = rng.gen_uint_below(candidate); 
    let result = Int::modpow(&random, &(candidate - &Int::one()), candidate);
    result == Int::one()
}

fn miller_rabin(candidate: &Int, limit: usize) -> bool {
    let (s,d) = rewrite(&(candidate - &Int::one()));
    let one = Int::one();
    let two = &one + &one;

    for _ in 0..limit {
        let basis = Int::sample_range(&two, &(candidate-&two));
        let mut y = Int::modpow(&basis, &d, candidate);

        if y == one || y == (candidate - &one) {
            continue;
        } else {
            for _ in one.clone()..s-one.clone() {
                y = Int::modpow(&y, &two, candidate);
                if y == one {
                    return false
                } else if y == candidate - &one {
                    break;
                }
            }
            return false;
        }
    }
    true
}


fn is_prime(candidate: &Int) -> bool { 
    // First, simple trial divide
    if !div_small_primes(candidate){
        return false;
    }

     // Second, Fermat's little theo test on the candidate
    if !little_fermat(candidate) {
        return false;
    }

    // Finally, Miller-Rabin test
    if !miller_rabin(candidate, 5) {
        return false;
    }
    true
}