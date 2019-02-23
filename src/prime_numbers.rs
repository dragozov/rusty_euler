pub fn is_prime(n: u64, primes: &Vec<u64>) -> bool {
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