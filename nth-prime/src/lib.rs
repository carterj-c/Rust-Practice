pub fn nth(n: u32) -> u32 {
    // There is no formula for finding primes so we will need to check each number
    // 2 is the only even prime, return it if 0 is passed in
    let mut primes: Vec<u32> = vec![2];
    let mut prime_candidate: u32 = 3;
    while primes.len() != (n + 1) as usize {
        if is_prime(prime_candidate) {
            primes.push(prime_candidate);
        }

        prime_candidate += 2;
    }
    primes[n as usize]
}

fn is_prime(prime_candidate: u32) -> bool {
    // To optimize the speed of this algorithm we can stop checking candidates at
    // sqrt(prime_candidate) since a number is prime if not
    for candidate in (3..=prime_candidate.isqrt()).step_by(2) {
        if prime_candidate % candidate == 0 {
            return false;
        }
    }
    true
}
