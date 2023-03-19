// https://exercism.io/tracks/rust/exercises/nth-prime/solutions/03d468d4e88d42d594bcfad8be6fa5d6
pub fn nth(n: u32) -> u32 {
    let mut found = vec![2];
    if n == 0 {
        return found[0];
    }
    let mut current = 3;
    let mut n = n;
    loop {
        match found.iter().find(|&x| current % x == 0) {
            None => {
                n -= 1;
                if n == 0 {
                    return current;
                }
                found.push(current);
                current += 2; // even numbers other than 2 are not prime number
            }
            _ => current += 2,
        }
    }
}
