pub fn answer() -> u64 {
    let total = 10001;
    let mut primes: Vec<u64> = Vec::with_capacity(total);
    let mut count = 2;
    let mut p = 3;
    primes.push(2);
    primes.push(3);
    while count < total {
        p = p + 2;
        if is_prime(p, &primes) {
            primes.push(p);
            count += 1;
        }
    }
    p
}

fn is_prime(n: u64, primes: &Vec<u64>) -> bool {
    let mut result = true;
    let half = &(n / 2);
    for p in primes {
        if p >= half {
            break;
        }
        if n % p == 0 {
            result = false;
            break;
        }
    }
    result
}
