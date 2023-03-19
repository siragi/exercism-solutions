pub fn nth(n: u32) -> u32 {
    //unimplemented!("What is the 0-indexed {}th prime number?", n)
    let mut primes: Vec<u32> = Vec::new();
    let mut nth_prime: u32 = 2;

    primes.push(2);
    if n > 0 {
        for _i in 1..=n {
            next_prime(&mut primes);
        }

        fn next_prime(primes: &mut Vec<u32>) {
            //let lastprime = primes[primes.len() - 1];
            let lastprime: u32 = *primes.last().unwrap();
            let mut next = lastprime + 1;

            loop {
                if is_prime(next, &primes) {
                    primes.push(next);
                    break;
                } else {
                    next += 1;
                }
            }
        }

        fn is_prime(x: u32, primes: &Vec<u32>) -> bool {
            let mut is_a_prime = true;
            for i in primes {
                if x % i == 0 {
                    is_a_prime = false;
                    break;
                }
            }
            is_a_prime
        }

        //0.) -> nicest Variant, but unstable pattern matching (not working!)
        /*  match primes.as_slice() {
            [.., last] => nth_prime = *last,
            _ => unreachable!(),
        } */

        // -> all variants below work!

        //1.)
        //nth_prime = primes[primes.len() - 1];
        //2.)
        //nth_prime = *primes.get(n as usize).unwrap();

        //3.)
        /* let last = &primes[n as usize];
        nth_prime = *last; */

        //4.) Nice because get() returns a Option, that can be used for error handling
        /* match primes.get(n as usize) {
            Some(last) => nth_prime = *last,
            None => println!("No element!"),
        } */

        //5.) Since last() returns also an Option, that can be used for error handling. Chosen.
        match primes.last() {
            Some(lastprime) => nth_prime = *lastprime,
            None => println!("No element!"),
        }

        //6.)
        // nth_prime = primes[n as usize];
    }
    nth_prime
}
