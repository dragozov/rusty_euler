use super::prime_numbers;

pub fn answer() -> u64 {
    let total = 10001;
    let mut primes: Vec<u64> = Vec::with_capacity(total);
    let mut count = 2;
    let mut p = 3;
    primes.push(2);
    primes.push(3);
    while count < total {
        p = p + 2;
        if prime_numbers::is_prime(p, &primes) {
            primes.push(p);
            count += 1;
        }
    }
    p
}
